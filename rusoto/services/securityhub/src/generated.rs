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
use serde_json;
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AcceptAdministratorInvitationRequest {
    /// <p>The account ID of the Security Hub administrator account that sent the invitation.</p>
    #[serde(rename = "AdministratorId")]
    pub administrator_id: String,
    /// <p>The identifier of the invitation sent from the Security Hub administrator account.</p>
    #[serde(rename = "InvitationId")]
    pub invitation_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AcceptAdministratorInvitationResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AcceptInvitationRequest {
    /// <p>The identifier of the invitation sent from the Security Hub administrator account.</p>
    #[serde(rename = "InvitationId")]
    pub invitation_id: String,
    /// <p>The account ID of the Security Hub administrator account that sent the invitation.</p>
    #[serde(rename = "MasterId")]
    pub master_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AcceptInvitationResponse {}

/// <p>The details of an AWS account.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AccountDetails {
    /// <p>The ID of an AWS account.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The email of an AWS account.</p>
    #[serde(rename = "Email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

/// <p><p>Provides details about one of the following actions that affects or that was taken on a resource:</p> <ul> <li> <p>A remote IP address issued an AWS API call</p> </li> <li> <p>A DNS request was received</p> </li> <li> <p>A remote IP address attempted to connect to an EC2 instance</p> </li> <li> <p>A remote IP address attempted a port probe on an EC2 instance</p> </li> </ul></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Action {
    /// <p><p>The type of action that was detected. The possible action types are:</p> <ul> <li> <p> <code>NETWORK<em>CONNECTION</code> </p> </li> <li> <p> <code>AWS</em>API<em>CALL</code> </p> </li> <li> <p> <code>DNS</em>REQUEST</code> </p> </li> <li> <p> <code>PORT_PROBE</code> </p> </li> </ul></p>
    #[serde(rename = "ActionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type: Option<String>,
    /// <p>Included if <code>ActionType</code> is <code>AWS_API_CALL</code>. Provides details about the API call that was detected. </p>
    #[serde(rename = "AwsApiCallAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_api_call_action: Option<AwsApiCallAction>,
    /// <p>Included if <code>ActionType</code> is <code>DNS_REQUEST</code>. Provides details about the DNS request that was detected. </p>
    #[serde(rename = "DnsRequestAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_request_action: Option<DnsRequestAction>,
    /// <p>Included if <code>ActionType</code> is <code>NETWORK_CONNECTION</code>. Provides details about the network connection that was detected.</p>
    #[serde(rename = "NetworkConnectionAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_connection_action: Option<NetworkConnectionAction>,
    /// <p>Included if <code>ActionType</code> is <code>PORT_PROBE</code>. Provides details about the port probe that was detected. </p>
    #[serde(rename = "PortProbeAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_probe_action: Option<PortProbeAction>,
}

/// <p>Provides information about the IP address where the scanned port is located.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ActionLocalIpDetails {
    /// <p>The IP address.</p>
    #[serde(rename = "IpAddressV4")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_v4: Option<String>,
}

/// <p>For <code>NetworkConnectionAction</code> and <code>PortProbeDetails</code>, <code>LocalPortDetails</code> provides information about the local port that was involved in the action.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ActionLocalPortDetails {
    /// <p>The number of the port.</p>
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    /// <p>The port name of the local connection.</p>
    #[serde(rename = "PortName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_name: Option<String>,
}

/// <p>For <code>AwsApiAction</code>, <code>NetworkConnectionAction</code>, and <code>PortProbeAction</code>, <code>RemoteIpDetails</code> provides information about the remote IP address that was involved in the action.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ActionRemoteIpDetails {
    /// <p>The city where the remote IP address is located.</p>
    #[serde(rename = "City")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<City>,
    /// <p>The country where the remote IP address is located.</p>
    #[serde(rename = "Country")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Country>,
    /// <p>The coordinates of the location of the remote IP address.</p>
    #[serde(rename = "GeoLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo_location: Option<GeoLocation>,
    /// <p>The IP address.</p>
    #[serde(rename = "IpAddressV4")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_v4: Option<String>,
    /// <p>The internet service provider (ISP) organization associated with the remote IP address.</p>
    #[serde(rename = "Organization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<IpOrganizationDetails>,
}

/// <p>Provides information about the remote port that was involved in an attempted network connection.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ActionRemotePortDetails {
    /// <p>The number of the port.</p>
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    /// <p>The port name of the remote connection.</p>
    #[serde(rename = "PortName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_name: Option<String>,
}

/// <p>An <code>ActionTarget</code> object.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ActionTarget {
    /// <p>The ARN for the target action.</p>
    #[serde(rename = "ActionTargetArn")]
    pub action_target_arn: String,
    /// <p>The description of the target action.</p>
    #[serde(rename = "Description")]
    pub description: String,
    /// <p>The name of the action target.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

/// <p>Represents a Security Hub administrator account designated by an organization management account.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AdminAccount {
    /// <p>The AWS account identifier of the Security Hub administrator account.</p>
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>The current status of the Security Hub administrator account. Indicates whether the account is currently enabled as a Security Hub administrator.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Information about an Availability Zone.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AvailabilityZone {
    /// <p>The ID of the subnet. You can specify one subnet per Availability Zone.</p>
    #[serde(rename = "SubnetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    /// <p>The name of the Availability Zone.</p>
    #[serde(rename = "ZoneName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_name: Option<String>,
}

/// <p>Provided if <code>ActionType</code> is <code>AWS_API_CALL</code>. It provides details about the API call that was detected.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsApiCallAction {
    /// <p>Identifies the resources that were affected by the API call.</p>
    #[serde(rename = "AffectedResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affected_resources: Option<::std::collections::HashMap<String, String>>,
    /// <p>The name of the API method that was issued.</p>
    #[serde(rename = "Api")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api: Option<String>,
    /// <p>Indicates whether the API call originated from a remote IP address (<code>remoteip</code>) or from a DNS domain (<code>domain</code>).</p>
    #[serde(rename = "CallerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caller_type: Option<String>,
    /// <p>Provided if <code>CallerType</code> is <code>domain</code>. Provides information about the DNS domain that the API call originated from.</p>
    #[serde(rename = "DomainDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_details: Option<AwsApiCallActionDomainDetails>,
    /// <p>An ISO8601-formatted timestamp that indicates when the API call was first observed.</p>
    #[serde(rename = "FirstSeen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_seen: Option<String>,
    /// <p>An ISO8601-formatted timestamp that indicates when the API call was most recently observed.</p>
    #[serde(rename = "LastSeen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_seen: Option<String>,
    /// <p>Provided if <code>CallerType</code> is <code>remoteIp</code>. Provides information about the remote IP address that the API call originated from.</p>
    #[serde(rename = "RemoteIpDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_ip_details: Option<ActionRemoteIpDetails>,
    /// <p>The name of the AWS service that the API method belongs to.</p>
    #[serde(rename = "ServiceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
}

/// <p>Provided if <code>CallerType</code> is <code>domain</code>. It provides information about the DNS domain that issued the API call.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsApiCallActionDomainDetails {
    /// <p>The name of the DNS domain that issued the API call.</p>
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
}

/// <p>Contains information about settings for logging access for the stage.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsApiGatewayAccessLogSettings {
    /// <p>The ARN of the CloudWatch Logs log group that receives the access logs.</p>
    #[serde(rename = "DestinationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_arn: Option<String>,
    /// <p>A single-line format of the access logs of data, as specified by selected <code>$context</code> variables. The format must include at least <code>$context.requestId</code>.</p>
    #[serde(rename = "Format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

/// <p>Contains information about settings for canary deployment in the stage.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsApiGatewayCanarySettings {
    /// <p>The deployment identifier for the canary deployment.</p>
    #[serde(rename = "DeploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    /// <p>The percentage of traffic that is diverted to a canary deployment.</p>
    #[serde(rename = "PercentTraffic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_traffic: Option<f64>,
    /// <p>Stage variables that are overridden in the canary release deployment. The variables include new stage variables that are introduced in the canary.</p> <p>Each variable is represented as a string-to-string map between the stage variable name and the variable value.</p>
    #[serde(rename = "StageVariableOverrides")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_variable_overrides: Option<::std::collections::HashMap<String, String>>,
    /// <p>Indicates whether the canary deployment uses the stage cache.</p>
    #[serde(rename = "UseStageCache")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_stage_cache: Option<bool>,
}

/// <p>Contains information about the endpoints for the API.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsApiGatewayEndpointConfiguration {
    /// <p>A list of endpoint types for the REST API.</p> <p>For an edge-optimized API, the endpoint type is <code>EDGE</code>. For a Regional API, the endpoint type is <code>REGIONAL</code>. For a private API, the endpoint type is <code>PRIVATE</code>.</p>
    #[serde(rename = "Types")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<String>>,
}

/// <p>Defines settings for a method for the stage.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsApiGatewayMethodSettings {
    /// <p>Indicates whether the cached responses are encrypted. </p>
    #[serde(rename = "CacheDataEncrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_data_encrypted: Option<bool>,
    /// <p>Specifies the time to live (TTL), in seconds, for cached responses. The higher the TTL, the longer the response is cached.</p>
    #[serde(rename = "CacheTtlInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_ttl_in_seconds: Option<i64>,
    /// <p>Indicates whether responses are cached and returned for requests. For responses to be cached, a cache cluster must be enabled on the stage.</p>
    #[serde(rename = "CachingEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caching_enabled: Option<bool>,
    /// <p>Indicates whether data trace logging is enabled for the method. Data trace logging affects the log entries that are pushed to CloudWatch Logs.</p>
    #[serde(rename = "DataTraceEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_trace_enabled: Option<bool>,
    /// <p>The HTTP method. You can use an asterisk (*) as a wildcard to apply method settings to multiple methods.</p>
    #[serde(rename = "HttpMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_method: Option<String>,
    /// <p>The logging level for this method. The logging level affects the log entries that are pushed to CloudWatch Logs.</p> <p>If the logging level is <code>ERROR</code>, then the logs only include error-level entries.</p> <p>If the logging level is <code>INFO</code>, then the logs include both <code>ERROR</code> events and extra informational events.</p> <p>Valid values: <code>OFF</code> | <code>ERROR</code> | <code>INFO</code> </p>
    #[serde(rename = "LoggingLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_level: Option<String>,
    /// <p>Indicates whether CloudWatch metrics are enabled for the method. </p>
    #[serde(rename = "MetricsEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics_enabled: Option<bool>,
    /// <p>Indicates whether authorization is required for a cache invalidation request.</p>
    #[serde(rename = "RequireAuthorizationForCacheControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_authorization_for_cache_control: Option<bool>,
    /// <p>The resource path for this method. Forward slashes (/) are encoded as ~1 . The initial slash must include a forward slash.</p> <p>For example, the path value <code>/resource/subresource</code> must be encoded as <code>/~1resource~1subresource</code>.</p> <p>To specify the root path, use only a slash (/). You can use an asterisk (*) as a wildcard to apply method settings to multiple methods.</p>
    #[serde(rename = "ResourcePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_path: Option<String>,
    /// <p>The throttling burst limit for the method.</p>
    #[serde(rename = "ThrottlingBurstLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttling_burst_limit: Option<i64>,
    /// <p>The throttling rate limit for the method.</p>
    #[serde(rename = "ThrottlingRateLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttling_rate_limit: Option<f64>,
    /// <p>Indicates how to handle unauthorized requests for cache invalidation.</p> <p>Valid values: <code>FAIL_WITH_403</code> | <code>SUCCEED_WITH_RESPONSE_HEADER</code> | <code>SUCCEED_WITHOUT_RESPONSE_HEADER</code> </p>
    #[serde(rename = "UnauthorizedCacheControlHeaderStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unauthorized_cache_control_header_strategy: Option<String>,
}

/// <p>Contains information about a REST API in version 1 of Amazon API Gateway.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsApiGatewayRestApiDetails {
    /// <p>The source of the API key for metering requests according to a usage plan.</p> <p> <code>HEADER</code> indicates whether to read the API key from the X-API-Key header of a request.</p> <p> <code>AUTHORIZER</code> indicates whether to read the API key from the <code>UsageIdentifierKey</code> from a custom authorizer.</p>
    #[serde(rename = "ApiKeySource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_source: Option<String>,
    /// <p>The list of binary media types supported by the REST API.</p>
    #[serde(rename = "BinaryMediaTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary_media_types: Option<Vec<String>>,
    /// <p>Indicates when the API was created.</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    /// <p>A description of the REST API.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The endpoint configuration of the REST API.</p>
    #[serde(rename = "EndpointConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_configuration: Option<AwsApiGatewayEndpointConfiguration>,
    /// <p>The identifier of the REST API.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The minimum size in bytes of a payload before compression is enabled.</p> <p>If <code>null</code>, then compression is disabled.</p> <p>If 0, then all payloads are compressed.</p>
    #[serde(rename = "MinimumCompressionSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_compression_size: Option<i64>,
    /// <p>The name of the REST API.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The version identifier for the REST API.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>Provides information about a version 1 Amazon API Gateway stage.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsApiGatewayStageDetails {
    /// <p>Settings for logging access for the stage.</p>
    #[serde(rename = "AccessLogSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_log_settings: Option<AwsApiGatewayAccessLogSettings>,
    /// <p>Indicates whether a cache cluster is enabled for the stage.</p>
    #[serde(rename = "CacheClusterEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_cluster_enabled: Option<bool>,
    /// <p>If a cache cluster is enabled, the size of the cache cluster.</p>
    #[serde(rename = "CacheClusterSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_cluster_size: Option<String>,
    /// <p>If a cache cluster is enabled, the status of the cache cluster.</p>
    #[serde(rename = "CacheClusterStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_cluster_status: Option<String>,
    /// <p>Information about settings for canary deployment in the stage.</p>
    #[serde(rename = "CanarySettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canary_settings: Option<AwsApiGatewayCanarySettings>,
    /// <p>The identifier of the client certificate for the stage.</p>
    #[serde(rename = "ClientCertificateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_certificate_id: Option<String>,
    /// <p>Indicates when the stage was created.</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    /// <p>The identifier of the deployment that the stage points to.</p>
    #[serde(rename = "DeploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    /// <p>A description of the stage.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The version of the API documentation that is associated with the stage.</p>
    #[serde(rename = "DocumentationVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_version: Option<String>,
    /// <p>Indicates when the stage was most recently updated.</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "LastUpdatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date: Option<String>,
    /// <p>Defines the method settings for the stage.</p>
    #[serde(rename = "MethodSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method_settings: Option<Vec<AwsApiGatewayMethodSettings>>,
    /// <p>The name of the stage.</p>
    #[serde(rename = "StageName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_name: Option<String>,
    /// <p>Indicates whether active tracing with AWS X-Ray is enabled for the stage.</p>
    #[serde(rename = "TracingEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracing_enabled: Option<bool>,
    /// <p><p>A map that defines the stage variables for the stage.</p> <p>Variable names can have alphanumeric and underscore characters.</p> <p>Variable values can contain the following characters:</p> <ul> <li> <p>Uppercase and lowercase letters</p> </li> <li> <p>Numbers</p> </li> <li> <p>Special characters -._~:/?#&amp;=,</p> </li> </ul></p>
    #[serde(rename = "Variables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<::std::collections::HashMap<String, String>>,
    /// <p>The ARN of the web ACL associated with the stage.</p>
    #[serde(rename = "WebAclArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_acl_arn: Option<String>,
}

/// <p>Contains information about a version 2 API in Amazon API Gateway.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsApiGatewayV2ApiDetails {
    /// <p>The URI of the API. </p> <p>Uses the format <code> <i>&lt;api-id&gt;</i>.execute-api.<i>&lt;region&gt;</i>.amazonaws.com</code> </p> <p>The stage name is typically appended to the URI to form a complete path to a deployed API stage.</p>
    #[serde(rename = "ApiEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_endpoint: Option<String>,
    /// <p>The identifier of the API.</p>
    #[serde(rename = "ApiId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_id: Option<String>,
    /// <p>An API key selection expression. Supported only for WebSocket APIs. </p>
    #[serde(rename = "ApiKeySelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_selection_expression: Option<String>,
    /// <p>A cross-origin resource sharing (CORS) configuration. Supported only for HTTP APIs.</p>
    #[serde(rename = "CorsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cors_configuration: Option<AwsCorsConfiguration>,
    /// <p>Indicates when the API was created.</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    /// <p>A description of the API.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the API.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The API protocol for the API.</p> <p>Valid values: <code>WEBSOCKET</code> | <code>HTTP</code> </p>
    #[serde(rename = "ProtocolType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_type: Option<String>,
    /// <p>The route selection expression for the API.</p> <p>For HTTP APIs, must be <code>${request.method} ${request.path}</code>. This is the default value for HTTP APIs.</p> <p>For WebSocket APIs, there is no default value.</p>
    #[serde(rename = "RouteSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_selection_expression: Option<String>,
    /// <p>The version identifier for the API.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>Contains route settings for a stage.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsApiGatewayV2RouteSettings {
    /// <p>Indicates whether data trace logging is enabled. Data trace logging affects the log entries that are pushed to CloudWatch Logs. Supported only for WebSocket APIs.</p>
    #[serde(rename = "DataTraceEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_trace_enabled: Option<bool>,
    /// <p>Indicates whether detailed metrics are enabled.</p>
    #[serde(rename = "DetailedMetricsEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detailed_metrics_enabled: Option<bool>,
    /// <p>The logging level. The logging level affects the log entries that are pushed to CloudWatch Logs. Supported only for WebSocket APIs.</p> <p>If the logging level is <code>ERROR</code>, then the logs only include error-level entries.</p> <p>If the logging level is <code>INFO</code>, then the logs include both <code>ERROR</code> events and extra informational events.</p> <p>Valid values: <code>OFF</code> | <code>ERROR</code> | <code>INFO</code> </p>
    #[serde(rename = "LoggingLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_level: Option<String>,
    /// <p>The throttling burst limit.</p>
    #[serde(rename = "ThrottlingBurstLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttling_burst_limit: Option<i64>,
    /// <p>The throttling rate limit.</p>
    #[serde(rename = "ThrottlingRateLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttling_rate_limit: Option<f64>,
}

/// <p>Contains information about a version 2 stage for Amazon API Gateway.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsApiGatewayV2StageDetails {
    /// <p>Information about settings for logging access for the stage.</p>
    #[serde(rename = "AccessLogSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_log_settings: Option<AwsApiGatewayAccessLogSettings>,
    /// <p>Indicates whether the stage is managed by API Gateway.</p>
    #[serde(rename = "ApiGatewayManaged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_gateway_managed: Option<bool>,
    /// <p>Indicates whether updates to an API automatically trigger a new deployment.</p>
    #[serde(rename = "AutoDeploy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_deploy: Option<bool>,
    /// <p>The identifier of a client certificate for a stage. Supported only for WebSocket API calls.</p>
    #[serde(rename = "ClientCertificateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_certificate_id: Option<String>,
    /// <p>Indicates when the stage was created.</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    /// <p>Default route settings for the stage.</p>
    #[serde(rename = "DefaultRouteSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_route_settings: Option<AwsApiGatewayV2RouteSettings>,
    /// <p>The identifier of the deployment that the stage is associated with. </p>
    #[serde(rename = "DeploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    /// <p>The description of the stage.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The status of the last deployment of a stage. Supported only if the stage has automatic deployment enabled.</p>
    #[serde(rename = "LastDeploymentStatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_deployment_status_message: Option<String>,
    /// <p>Indicates when the stage was most recently updated.</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "LastUpdatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date: Option<String>,
    /// <p>The route settings for the stage.</p>
    #[serde(rename = "RouteSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_settings: Option<AwsApiGatewayV2RouteSettings>,
    /// <p>The name of the stage.</p>
    #[serde(rename = "StageName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_name: Option<String>,
    /// <p><p>A map that defines the stage variables for the stage.</p> <p>Variable names can have alphanumeric and underscore characters.</p> <p>Variable values can contain the following characters:</p> <ul> <li> <p>Uppercase and lowercase letters</p> </li> <li> <p>Numbers</p> </li> <li> <p>Special characters -._~:/?#&amp;=,</p> </li> </ul></p>
    #[serde(rename = "StageVariables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_variables: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Provides details about an auto scaling group.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsAutoScalingAutoScalingGroupDetails {
    /// <p>Indicates when the auto scaling group was created.</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    /// <p>The amount of time, in seconds, that Amazon EC2 Auto Scaling waits before it checks the health status of an EC2 instance that has come into service.</p>
    #[serde(rename = "HealthCheckGracePeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_grace_period: Option<i64>,
    /// <p>The service to use for the health checks.</p>
    #[serde(rename = "HealthCheckType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_type: Option<String>,
    /// <p>The name of the launch configuration.</p>
    #[serde(rename = "LaunchConfigurationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_configuration_name: Option<String>,
    /// <p>The list of load balancers associated with the group.</p>
    #[serde(rename = "LoadBalancerNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_names: Option<Vec<String>>,
}

/// <p>Provides details about an AWS Certificate Manager certificate.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsCertificateManagerCertificateDetails {
    /// <p>The ARN of the private certificate authority (CA) that will be used to issue the certificate.</p>
    #[serde(rename = "CertificateAuthorityArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_authority_arn: Option<String>,
    /// <p>Indicates when the certificate was requested.</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// <p>The fully qualified domain name (FQDN), such as www.example.com, that is secured by the certificate.</p>
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// <p>Contains information about the initial validation of each domain name that occurs as a result of the <code>RequestCertificate</code> request.</p> <p>Only provided if the certificate type is <code>AMAZON_ISSUED</code>.</p>
    #[serde(rename = "DomainValidationOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_validation_options:
        Option<Vec<AwsCertificateManagerCertificateDomainValidationOption>>,
    /// <p>Contains a list of Extended Key Usage X.509 v3 extension objects. Each object specifies a purpose for which the certificate public key can be used and consists of a name and an object identifier (OID).</p>
    #[serde(rename = "ExtendedKeyUsages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_key_usages: Option<Vec<AwsCertificateManagerCertificateExtendedKeyUsage>>,
    /// <p>For a failed certificate request, the reason for the failure.</p> <p>Valid values: <code>NO_AVAILABLE_CONTACTS</code> | <code>ADDITIONAL_VERIFICATION_REQUIRED</code> | <code>DOMAIN_NOT_ALLOWED</code> | <code>INVALID_PUBLIC_DOMAIN</code> | <code>DOMAIN_VALIDATION_DENIED</code> | <code>CAA_ERROR</code> | <code>PCA_LIMIT_EXCEEDED</code> | <code>PCA_INVALID_ARN</code> | <code>PCA_INVALID_STATE</code> | <code>PCA_REQUEST_FAILED</code> | <code>PCA_NAME_CONSTRAINTS_VALIDATION</code> | <code>PCA_RESOURCE_NOT_FOUND</code> | <code>PCA_INVALID_ARGS</code> | <code>PCA_INVALID_DURATION</code> | <code>PCA_ACCESS_DENIED</code> | <code>SLR_NOT_FOUND</code> | <code>OTHER</code> </p>
    #[serde(rename = "FailureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>Indicates when the certificate was imported. Provided if the certificate type is <code>IMPORTED</code>.</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "ImportedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imported_at: Option<String>,
    /// <p>The list of ARNs for the AWS resources that use the certificate.</p>
    #[serde(rename = "InUseBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_use_by: Option<Vec<String>>,
    /// <p>Indicates when the certificate was issued. Provided if the certificate type is <code>AMAZON_ISSUED</code>.</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "IssuedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_at: Option<String>,
    /// <p>The name of the certificate authority that issued and signed the certificate.</p>
    #[serde(rename = "Issuer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    /// <p>The algorithm that was used to generate the public-private key pair.</p> <p>Valid values: <code>RSA_2048</code> | <code>RSA_1024</code> |<code> RSA_4096</code> | <code>EC_prime256v1</code> | <code>EC_secp384r1</code> | <code>EC_secp521r1</code> </p>
    #[serde(rename = "KeyAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_algorithm: Option<String>,
    /// <p>A list of key usage X.509 v3 extension objects.</p>
    #[serde(rename = "KeyUsages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_usages: Option<Vec<AwsCertificateManagerCertificateKeyUsage>>,
    /// <p>The time after which the certificate becomes invalid.</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "NotAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_after: Option<String>,
    /// <p>The time before which the certificate is not valid.</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "NotBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_before: Option<String>,
    /// <p>Provides a value that specifies whether to add the certificate to a transparency log.</p>
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<AwsCertificateManagerCertificateOptions>,
    /// <p>Whether the certificate is eligible for renewal.</p> <p>Valid values: <code>ELIGIBLE</code> | <code>INELIGIBLE</code> </p>
    #[serde(rename = "RenewalEligibility")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_eligibility: Option<String>,
    /// <p>Information about the status of the AWS Certificate Manager managed renewal for the certificate. Provided only when the certificate type is <code>AMAZON_ISSUED</code>.</p>
    #[serde(rename = "RenewalSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_summary: Option<AwsCertificateManagerCertificateRenewalSummary>,
    /// <p>The serial number of the certificate.</p>
    #[serde(rename = "Serial")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial: Option<String>,
    /// <p>The algorithm that was used to sign the certificate.</p>
    #[serde(rename = "SignatureAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_algorithm: Option<String>,
    /// <p>The status of the certificate.</p> <p>Valid values: <code>PENDING_VALIDATION</code> | <code>ISSUED</code> | <code>INACTIVE</code> | <code>EXPIRED</code> | <code>VALIDATION_TIMED_OUT</code> | <code>REVOKED</code> | <code>FAILED</code> </p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The name of the entity that is associated with the public key contained in the certificate.</p>
    #[serde(rename = "Subject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// <p>One or more domain names (subject alternative names) included in the certificate. This list contains the domain names that are bound to the public key that is contained in the certificate.</p> <p>The subject alternative names include the canonical domain name (CN) of the certificate and additional domain names that can be used to connect to the website.</p>
    #[serde(rename = "SubjectAlternativeNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_alternative_names: Option<Vec<String>>,
    /// <p>The source of the certificate. For certificates that AWS Certificate Manager provides, <code>Type</code> is <code>AMAZON_ISSUED</code>. For certificates that are imported with <code>ImportCertificate</code>, <code>Type</code> is <code>IMPORTED</code>.</p> <p>Valid values: <code>IMPORTED</code> | <code>AMAZON_ISSUED</code> | <code>PRIVATE</code> </p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p><p>Contains information about one of the following:</p> <ul> <li> <p>The initial validation of each domain name that occurs as a result of the <code>RequestCertificate</code> request</p> </li> <li> <p>The validation of each domain name in the certificate, as it pertains to AWS Certificate Manager managed renewal</p> </li> </ul></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsCertificateManagerCertificateDomainValidationOption {
    /// <p>A fully qualified domain name (FQDN) in the certificate.</p>
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// <p>The CNAME record that is added to the DNS database for domain validation.</p>
    #[serde(rename = "ResourceRecord")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_record: Option<AwsCertificateManagerCertificateResourceRecord>,
    /// <p>The domain name that AWS Certificate Manager uses to send domain validation emails.</p>
    #[serde(rename = "ValidationDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_domain: Option<String>,
    /// <p>A list of email addresses that AWS Certificate Manager uses to send domain validation emails.</p>
    #[serde(rename = "ValidationEmails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_emails: Option<Vec<String>>,
    /// <p>The method used to validate the domain name.</p>
    #[serde(rename = "ValidationMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_method: Option<String>,
    /// <p>The validation status of the domain name.</p>
    #[serde(rename = "ValidationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_status: Option<String>,
}

/// <p>Contains information about an extended key usage X.509 v3 extension object.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsCertificateManagerCertificateExtendedKeyUsage {
    /// <p>The name of an extension value. Indicates the purpose for which the certificate public key can be used.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>An object identifier (OID) for the extension value.</p> <p>The format is numbers separated by periods.</p>
    #[serde(rename = "OId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_id: Option<String>,
}

/// <p>Contains information about a key usage X.509 v3 extension object.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsCertificateManagerCertificateKeyUsage {
    /// <p>The key usage extension name.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Contains other options for the certificate.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsCertificateManagerCertificateOptions {
    /// <p>Whether to add the certificate to a transparency log.</p> <p>Valid values: <code>DISABLED</code> | <code>ENABLED</code> </p>
    #[serde(rename = "CertificateTransparencyLoggingPreference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_transparency_logging_preference: Option<String>,
}

/// <p>Contains information about the AWS Certificate Manager managed renewal for an <code>AMAZON_ISSUED</code> certificate.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsCertificateManagerCertificateRenewalSummary {
    /// <p>Information about the validation of each domain name in the certificate, as it pertains to AWS Certificate Manager managed renewal. Provided only when the certificate type is <code>AMAZON_ISSUED</code>.</p>
    #[serde(rename = "DomainValidationOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_validation_options:
        Option<Vec<AwsCertificateManagerCertificateDomainValidationOption>>,
    /// <p>The status of the AWS Certificate Manager managed renewal of the certificate.</p> <p>Valid values: <code>PENDING_AUTO_RENEWAL</code> | <code>PENDING_VALIDATION</code> | <code>SUCCESS</code> | <code>FAILED</code> </p>
    #[serde(rename = "RenewalStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_status: Option<String>,
    /// <p>The reason that a renewal request was unsuccessful.</p> <p>Valid values: <code>NO_AVAILABLE_CONTACTS</code> | <code>ADDITIONAL_VERIFICATION_REQUIRED</code> | <code>DOMAIN_NOT_ALLOWED</code> | <code>INVALID_PUBLIC_DOMAIN</code> | <code>DOMAIN_VALIDATION_DENIED</code> | <code>CAA_ERROR</code> | <code>PCA_LIMIT_EXCEEDED</code> | <code>PCA_INVALID_ARN</code> | <code>PCA_INVALID_STATE</code> | <code>PCA_REQUEST_FAILED</code> | <code>PCA_NAME_CONSTRAINTS_VALIDATION</code> | <code>PCA_RESOURCE_NOT_FOUND</code> | <code>PCA_INVALID_ARGS</code> | <code>PCA_INVALID_DURATION</code> | <code>PCA_ACCESS_DENIED</code> | <code>SLR_NOT_FOUND</code> | <code>OTHER</code> </p>
    #[serde(rename = "RenewalStatusReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_status_reason: Option<String>,
    /// <p>Indicates when the renewal summary was last updated.</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "UpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// <p>Provides details about the CNAME record that is added to the DNS database for domain validation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsCertificateManagerCertificateResourceRecord {
    /// <p>The name of the resource.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The type of resource.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The value of the resource.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>Information about a cache behavior for the distribution.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsCloudFrontDistributionCacheBehavior {
    /// <p><p>The protocol that viewers can use to access the files in an origin. You can specify the following options:</p> <ul> <li> <p> <code>allow-all</code> - Viewers can use HTTP or HTTPS.</p> </li> <li> <p> <code>redirect-to-https</code> - CloudFront responds to HTTP requests with an HTTP status code of 301 (Moved Permanently) and the HTTPS URL. The viewer then uses the new URL to resubmit.</p> </li> <li> <p> <code>https-only</code> - CloudFront responds to HTTP request with an HTTP status code of 403 (Forbidden).</p> </li> </ul></p>
    #[serde(rename = "ViewerProtocolPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewer_protocol_policy: Option<String>,
}

/// <p>Provides information about caching for the distribution.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsCloudFrontDistributionCacheBehaviors {
    /// <p>The cache behaviors for the distribution.</p>
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<AwsCloudFrontDistributionCacheBehavior>>,
}

/// <p>Contains information about the default cache configuration for the distribution.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsCloudFrontDistributionDefaultCacheBehavior {
    /// <p><p>The protocol that viewers can use to access the files in an origin. You can specify the following options:</p> <ul> <li> <p> <code>allow-all</code> - Viewers can use HTTP or HTTPS.</p> </li> <li> <p> <code>redirect-to-https</code> - CloudFront responds to HTTP requests with an HTTP status code of 301 (Moved Permanently) and the HTTPS URL. The viewer then uses the new URL to resubmit.</p> </li> <li> <p> <code>https-only</code> - CloudFront responds to HTTP request with an HTTP status code of 403 (Forbidden).</p> </li> </ul></p>
    #[serde(rename = "ViewerProtocolPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewer_protocol_policy: Option<String>,
}

/// <p>A distribution configuration.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsCloudFrontDistributionDetails {
    /// <p>Provides information about the cache configuration for the distribution.</p>
    #[serde(rename = "CacheBehaviors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_behaviors: Option<AwsCloudFrontDistributionCacheBehaviors>,
    /// <p>The default cache behavior for the configuration.</p>
    #[serde(rename = "DefaultCacheBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_cache_behavior: Option<AwsCloudFrontDistributionDefaultCacheBehavior>,
    /// <p>The object that CloudFront sends in response to requests from the origin (for example, index.html) when a viewer requests the root URL for the distribution (http://www.example.com) instead of an object in your distribution (http://www.example.com/product-description.html). </p>
    #[serde(rename = "DefaultRootObject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_root_object: Option<String>,
    /// <p>The domain name corresponding to the distribution.</p>
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// <p>The entity tag is a hash of the object.</p>
    #[serde(rename = "ETag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    /// <p>Indicates when that the distribution was last modified.</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
    /// <p>A complex type that controls whether access logs are written for the distribution.</p>
    #[serde(rename = "Logging")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<AwsCloudFrontDistributionLogging>,
    /// <p>Provides information about the origin groups in the distribution.</p>
    #[serde(rename = "OriginGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_groups: Option<AwsCloudFrontDistributionOriginGroups>,
    /// <p>A complex type that contains information about origins for this distribution.</p>
    #[serde(rename = "Origins")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origins: Option<AwsCloudFrontDistributionOrigins>,
    /// <p>Indicates the current status of the distribution.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>A unique identifier that specifies the AWS WAF web ACL, if any, to associate with this distribution.</p>
    #[serde(rename = "WebAclId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_acl_id: Option<String>,
}

/// <p>A complex type that controls whether access logs are written for the distribution.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsCloudFrontDistributionLogging {
    /// <p>The Amazon S3 bucket to store the access logs in.</p>
    #[serde(rename = "Bucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    /// <p>With this field, you can enable or disable the selected distribution.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>Specifies whether you want CloudFront to include cookies in access logs.</p>
    #[serde(rename = "IncludeCookies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_cookies: Option<bool>,
    /// <p>An optional string that you want CloudFront to use as a prefix to the access log filenames for this distribution.</p>
    #[serde(rename = "Prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

/// <p>Information about an origin group for the distribution.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsCloudFrontDistributionOriginGroup {
    /// <p>Provides the criteria for an origin group to fail over.</p>
    #[serde(rename = "FailoverCriteria")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failover_criteria: Option<AwsCloudFrontDistributionOriginGroupFailover>,
}

/// <p>Provides information about when an origin group fails over.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsCloudFrontDistributionOriginGroupFailover {
    /// <p>Information about the status codes that cause an origin group to fail over.</p>
    #[serde(rename = "StatusCodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_codes: Option<AwsCloudFrontDistributionOriginGroupFailoverStatusCodes>,
}

/// <p>The status codes that cause an origin group to fail over.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsCloudFrontDistributionOriginGroupFailoverStatusCodes {
    /// <p>The list of status code values that can cause a failover to the next origin.</p>
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<i64>>,
    /// <p>The number of status codes that can cause a failover.</p>
    #[serde(rename = "Quantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
}

/// <p>Provides information about origin groups that are associated with the distribution.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsCloudFrontDistributionOriginGroups {
    /// <p>The list of origin groups.</p>
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<AwsCloudFrontDistributionOriginGroup>>,
}

/// <p>A complex type that describes the Amazon S3 bucket, HTTP server (for example, a web server), Amazon Elemental MediaStore, or other server from which CloudFront gets your files.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsCloudFrontDistributionOriginItem {
    /// <p>Amazon S3 origins: The DNS name of the Amazon S3 bucket from which you want CloudFront to get objects for this origin.</p>
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// <p>A unique identifier for the origin or origin group.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>An optional element that causes CloudFront to request your content from a directory in your Amazon S3 bucket or your custom origin.</p>
    #[serde(rename = "OriginPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_path: Option<String>,
    /// <p>An origin that is an S3 bucket that is not configured with static website hosting.</p>
    #[serde(rename = "S3OriginConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_origin_config: Option<AwsCloudFrontDistributionOriginS3OriginConfig>,
}

/// <p>Information about an origin that is an S3 bucket that is not configured with static website hosting.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsCloudFrontDistributionOriginS3OriginConfig {
    /// <p>The CloudFront origin access identity to associate with the origin.</p>
    #[serde(rename = "OriginAccessIdentity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_access_identity: Option<String>,
}

/// <p>A complex type that contains information about origins and origin groups for this distribution.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsCloudFrontDistributionOrigins {
    /// <p>A complex type that contains origins or origin groups for this distribution.</p>
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<AwsCloudFrontDistributionOriginItem>>,
}

/// <p>Provides details about a CloudTrail trail.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsCloudTrailTrailDetails {
    /// <p>The ARN of the log group that CloudTrail logs are delivered to.</p>
    #[serde(rename = "CloudWatchLogsLogGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_log_group_arn: Option<String>,
    /// <p>The ARN of the role that the CloudWatch Logs endpoint assumes when it writes to the log group.</p>
    #[serde(rename = "CloudWatchLogsRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_role_arn: Option<String>,
    /// <p>Indicates whether the trail has custom event selectors.</p>
    #[serde(rename = "HasCustomEventSelectors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_custom_event_selectors: Option<bool>,
    /// <p>The Region where the trail was created.</p>
    #[serde(rename = "HomeRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_region: Option<String>,
    /// <p>Indicates whether the trail publishes events from global services such as IAM to the log files.</p>
    #[serde(rename = "IncludeGlobalServiceEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_global_service_events: Option<bool>,
    /// <p>Indicates whether the trail applies only to the current Region or to all Regions.</p>
    #[serde(rename = "IsMultiRegionTrail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_multi_region_trail: Option<bool>,
    /// <p>Whether the trail is created for all accounts in an organization in AWS Organizations, or only for the current AWS account.</p>
    #[serde(rename = "IsOrganizationTrail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_organization_trail: Option<bool>,
    /// <p>The AWS KMS key ID to use to encrypt the logs.</p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>Indicates whether CloudTrail log file validation is enabled.</p>
    #[serde(rename = "LogFileValidationEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_file_validation_enabled: Option<bool>,
    /// <p>The name of the trail.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The name of the S3 bucket where the log files are published.</p>
    #[serde(rename = "S3BucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_name: Option<String>,
    /// <p>The S3 key prefix. The key prefix is added after the name of the S3 bucket where the log files are published.</p>
    #[serde(rename = "S3KeyPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key_prefix: Option<String>,
    /// <p>The ARN of the SNS topic that is used for notifications of log file delivery.</p>
    #[serde(rename = "SnsTopicArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arn: Option<String>,
    /// <p>The name of the SNS topic that is used for notifications of log file delivery.</p>
    #[serde(rename = "SnsTopicName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_name: Option<String>,
    /// <p>The ARN of the trail.</p>
    #[serde(rename = "TrailArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_arn: Option<String>,
}

/// <p>Information about an AWS CodeBuild project.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsCodeBuildProjectDetails {
    /// <p>The AWS Key Management Service (AWS KMS) customer master key (CMK) used to encrypt the build output artifacts.</p> <p>You can specify either the ARN of the CMK or, if available, the CMK alias (using the format alias/alias-name). </p>
    #[serde(rename = "EncryptionKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<String>,
    /// <p>Information about the build environment for this build project.</p>
    #[serde(rename = "Environment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<AwsCodeBuildProjectEnvironment>,
    /// <p>The name of the build project.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The ARN of the IAM role that enables AWS CodeBuild to interact with dependent AWS services on behalf of the AWS account.</p>
    #[serde(rename = "ServiceRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role: Option<String>,
    /// <p>Information about the build input source code for this build project.</p>
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<AwsCodeBuildProjectSource>,
    /// <p>Information about the VPC configuration that AWS CodeBuild accesses.</p>
    #[serde(rename = "VpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<AwsCodeBuildProjectVpcConfig>,
}

/// <p>Information about the build environment for this build project.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsCodeBuildProjectEnvironment {
    /// <p>The certificate to use with this build project.</p>
    #[serde(rename = "Certificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    /// <p>The type of credentials AWS CodeBuild uses to pull images in your build.</p> <p>Valid values:</p> <ul> <li> <p> <code>CODEBUILD</code> specifies that AWS CodeBuild uses its own credentials. This requires that you modify your ECR repository policy to trust the AWS CodeBuild service principal.</p> </li> <li> <p> <code>SERVICE_ROLE</code> specifies that AWS CodeBuild uses your build project's service role.</p> </li> </ul> <p>When you use a cross-account or private registry image, you must use <code>SERVICE_ROLE</code> credentials. When you use an AWS CodeBuild curated image, you must use <code>CODEBUILD</code> credentials.</p>
    #[serde(rename = "ImagePullCredentialsType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_pull_credentials_type: Option<String>,
    /// <p>The credentials for access to a private registry.</p>
    #[serde(rename = "RegistryCredential")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_credential: Option<AwsCodeBuildProjectEnvironmentRegistryCredential>,
    /// <p>The type of build environment to use for related builds.</p> <p>The environment type <code>ARM_CONTAINER</code> is available only in Regions US East (N. Virginia), US East (Ohio), US West (Oregon), Europe (Ireland), Asia Pacific (Mumbai), Asia Pacific (Tokyo), Asia Pacific (Sydney), and Europe (Frankfurt).</p> <p>The environment type <code>LINUX_CONTAINER</code> with compute type build.general1.2xlarge is available only in Regions US East (N. Virginia), US East (N. Virginia), US West (Oregon), Canada (Central), Europe (Ireland), Europe (London), Europe (Frankfurt), Asia Pacific (Tokyo), Asia Pacific (Seoul), Asia Pacific (Singapore), Asia Pacific (Sydney), China (Beijing), and China (Ningxia).</p> <p>The environment type <code>LINUX_GPU_CONTAINER</code> is available only in Regions US East (N. Virginia), US East (N. Virginia), US West (Oregon), Canada (Central), Europe (Ireland), Europe (London), Europe (Frankfurt), Asia Pacific (Tokyo), Asia Pacific (Seoul), Asia Pacific (Singapore), Asia Pacific (Sydney), China (Beijing), and China (Ningxia).</p> <p>Valid values: <code>WINDOWS_CONTAINER</code> | <code>LINUX_CONTAINER</code> | <code>LINUX_GPU_CONTAINER</code> | <code>ARM_CONTAINER</code> </p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>The credentials for access to a private registry.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsCodeBuildProjectEnvironmentRegistryCredential {
    /// <p><p>The ARN or name of credentials created using AWS Secrets Manager.</p> <note> <p>The credential can use the name of the credentials only if they exist in your current AWS Region. </p> </note></p>
    #[serde(rename = "Credential")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential: Option<String>,
    /// <p>The service that created the credentials to access a private Docker registry.</p> <p>The valid value,<code> SECRETS_MANAGER</code>, is for AWS Secrets Manager.</p>
    #[serde(rename = "CredentialProvider")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_provider: Option<String>,
}

/// <p>Information about the build input source code for this build project.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsCodeBuildProjectSource {
    /// <p>Information about the Git clone depth for the build project.</p>
    #[serde(rename = "GitCloneDepth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_clone_depth: Option<i64>,
    /// <p>Whether to ignore SSL warnings while connecting to the project source code.</p>
    #[serde(rename = "InsecureSsl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insecure_ssl: Option<bool>,
    /// <p><p>Information about the location of the source code to be built.</p> <p>Valid values include:</p> <ul> <li> <p>For source code settings that are specified in the source action of a pipeline in AWS CodePipeline, location should not be specified. If it is specified, AWS CodePipeline ignores it. This is because AWS CodePipeline uses the settings in a pipeline&#39;s source action instead of this value.</p> </li> <li> <p>For source code in an AWS CodeCommit repository, the HTTPS clone URL to the repository that contains the source code and the build spec file (for example, <code>https://git-codecommit.region-ID.amazonaws.com/v1/repos/repo-name</code> ).</p> </li> <li> <p>For source code in an S3 input bucket, one of the following.</p> <ul> <li> <p>The path to the ZIP file that contains the source code (for example, <code>bucket-name/path/to/object-name.zip</code>).</p> </li> <li> <p> The path to the folder that contains the source code (for example, <code>bucket-name/path/to/source-code/folder/</code>).</p> </li> </ul> </li> <li> <p>For source code in a GitHub repository, the HTTPS clone URL to the repository that contains the source and the build spec file.</p> </li> <li> <p>For source code in a Bitbucket repository, the HTTPS clone URL to the repository that contains the source and the build spec file. </p> </li> </ul></p>
    #[serde(rename = "Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// <p><p>The type of repository that contains the source code to be built. Valid values are:</p> <ul> <li> <p> <code>BITBUCKET</code> - The source code is in a Bitbucket repository.</p> </li> <li> <p> <code>CODECOMMIT</code> - The source code is in an AWS CodeCommit repository.</p> </li> <li> <p> <code>CODEPIPELINE</code> - The source code settings are specified in the source action of a pipeline in AWS CodePipeline.</p> </li> <li> <p> <code>GITHUB</code> - The source code is in a GitHub repository.</p> </li> <li> <p> <code>GITHUB<em>ENTERPRISE</code> - The source code is in a GitHub Enterprise repository.</p> </li> <li> <p> <code>NO</em>SOURCE</code> - The project does not have input source code.</p> </li> <li> <p> <code>S3</code> - The source code is in an S3 input bucket. </p> </li> </ul></p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Information about the VPC configuration that AWS CodeBuild accesses.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsCodeBuildProjectVpcConfig {
    /// <p>A list of one or more security group IDs in your Amazon VPC.</p>
    #[serde(rename = "SecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    /// <p>A list of one or more subnet IDs in your Amazon VPC.</p>
    #[serde(rename = "Subnets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnets: Option<Vec<String>>,
    /// <p>The ID of the VPC.</p>
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

/// <p>Contains the cross-origin resource sharing (CORS) configuration for the API. CORS is only supported for HTTP APIs.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsCorsConfiguration {
    /// <p>Indicates whether the CORS request includes credentials.</p>
    #[serde(rename = "AllowCredentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_credentials: Option<bool>,
    /// <p>The allowed headers for CORS requests.</p>
    #[serde(rename = "AllowHeaders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_headers: Option<Vec<String>>,
    /// <p>The allowed methods for CORS requests.</p>
    #[serde(rename = "AllowMethods")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_methods: Option<Vec<String>>,
    /// <p>The allowed origins for CORS requests.</p>
    #[serde(rename = "AllowOrigins")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_origins: Option<Vec<String>>,
    /// <p>The exposed headers for CORS requests.</p>
    #[serde(rename = "ExposeHeaders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expose_headers: Option<Vec<String>>,
    /// <p>The number of seconds for which the browser caches preflight request results.</p>
    #[serde(rename = "MaxAge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_age: Option<i64>,
}

/// <p>Contains a definition of an attribute for the table.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsDynamoDbTableAttributeDefinition {
    /// <p>The name of the attribute.</p>
    #[serde(rename = "AttributeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_name: Option<String>,
    /// <p>The type of the attribute.</p>
    #[serde(rename = "AttributeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_type: Option<String>,
}

/// <p>Provides information about the billing for read/write capacity on the table.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsDynamoDbTableBillingModeSummary {
    /// <p>The method used to charge for read and write throughput and to manage capacity.</p>
    #[serde(rename = "BillingMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_mode: Option<String>,
    /// <p>If the billing mode is <code>PAY_PER_REQUEST</code>, indicates when the billing mode was set to that value.</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "LastUpdateToPayPerRequestDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_to_pay_per_request_date_time: Option<String>,
}

/// <p>Provides details about a DynamoDB table.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsDynamoDbTableDetails {
    /// <p>A list of attribute definitions for the table.</p>
    #[serde(rename = "AttributeDefinitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_definitions: Option<Vec<AwsDynamoDbTableAttributeDefinition>>,
    /// <p>Information about the billing for read/write capacity on the table.</p>
    #[serde(rename = "BillingModeSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_mode_summary: Option<AwsDynamoDbTableBillingModeSummary>,
    /// <p>Indicates when the table was created.</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "CreationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<String>,
    /// <p>List of global secondary indexes for the table.</p>
    #[serde(rename = "GlobalSecondaryIndexes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_secondary_indexes: Option<Vec<AwsDynamoDbTableGlobalSecondaryIndex>>,
    /// <p>The version of global tables being used.</p>
    #[serde(rename = "GlobalTableVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_table_version: Option<String>,
    /// <p>The number of items in the table.</p>
    #[serde(rename = "ItemCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_count: Option<i64>,
    /// <p>The primary key structure for the table.</p>
    #[serde(rename = "KeySchema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_schema: Option<Vec<AwsDynamoDbTableKeySchema>>,
    /// <p>The ARN of the latest stream for the table.</p>
    #[serde(rename = "LatestStreamArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_stream_arn: Option<String>,
    /// <p>The label of the latest stream. The label is not a unique identifier.</p>
    #[serde(rename = "LatestStreamLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_stream_label: Option<String>,
    /// <p>The list of local secondary indexes for the table.</p>
    #[serde(rename = "LocalSecondaryIndexes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_secondary_indexes: Option<Vec<AwsDynamoDbTableLocalSecondaryIndex>>,
    /// <p>Information about the provisioned throughput for the table.</p>
    #[serde(rename = "ProvisionedThroughput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_throughput: Option<AwsDynamoDbTableProvisionedThroughput>,
    /// <p>The list of replicas of this table.</p>
    #[serde(rename = "Replicas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicas: Option<Vec<AwsDynamoDbTableReplica>>,
    /// <p>Information about the restore for the table.</p>
    #[serde(rename = "RestoreSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_summary: Option<AwsDynamoDbTableRestoreSummary>,
    /// <p>Information about the server-side encryption for the table.</p>
    #[serde(rename = "SseDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sse_description: Option<AwsDynamoDbTableSseDescription>,
    /// <p>The current DynamoDB Streams configuration for the table.</p>
    #[serde(rename = "StreamSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_specification: Option<AwsDynamoDbTableStreamSpecification>,
    /// <p>The identifier of the table.</p>
    #[serde(rename = "TableId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_id: Option<String>,
    /// <p>The name of the table.</p>
    #[serde(rename = "TableName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    /// <p>The total size of the table in bytes.</p>
    #[serde(rename = "TableSizeBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_size_bytes: Option<i64>,
    /// <p>The current status of the table.</p>
    #[serde(rename = "TableStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_status: Option<String>,
}

/// <p>Information abut a global secondary index for the table.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsDynamoDbTableGlobalSecondaryIndex {
    /// <p>Whether the index is currently backfilling.</p>
    #[serde(rename = "Backfilling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backfilling: Option<bool>,
    /// <p>The ARN of the index.</p>
    #[serde(rename = "IndexArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_arn: Option<String>,
    /// <p>The name of the index.</p>
    #[serde(rename = "IndexName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    /// <p>The total size in bytes of the index.</p>
    #[serde(rename = "IndexSizeBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_size_bytes: Option<i64>,
    /// <p>The current status of the index.</p>
    #[serde(rename = "IndexStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_status: Option<String>,
    /// <p>The number of items in the index.</p>
    #[serde(rename = "ItemCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_count: Option<i64>,
    /// <p>The key schema for the index.</p>
    #[serde(rename = "KeySchema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_schema: Option<Vec<AwsDynamoDbTableKeySchema>>,
    /// <p>Attributes that are copied from the table into an index.</p>
    #[serde(rename = "Projection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projection: Option<AwsDynamoDbTableProjection>,
    /// <p>Information about the provisioned throughput settings for the indexes.</p>
    #[serde(rename = "ProvisionedThroughput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_throughput: Option<AwsDynamoDbTableProvisionedThroughput>,
}

/// <p>A component of the key schema for the DynamoDB table, a global secondary index, or a local secondary index.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsDynamoDbTableKeySchema {
    /// <p>The name of the key schema attribute.</p>
    #[serde(rename = "AttributeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_name: Option<String>,
    /// <p>The type of key used for the key schema attribute.</p>
    #[serde(rename = "KeyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_type: Option<String>,
}

/// <p>Information about a local secondary index for a DynamoDB table.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsDynamoDbTableLocalSecondaryIndex {
    /// <p>The ARN of the index.</p>
    #[serde(rename = "IndexArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_arn: Option<String>,
    /// <p>The name of the index.</p>
    #[serde(rename = "IndexName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    /// <p>The complete key schema for the index.</p>
    #[serde(rename = "KeySchema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_schema: Option<Vec<AwsDynamoDbTableKeySchema>>,
    /// <p>Attributes that are copied from the table into the index. These are in addition to the primary key attributes and index key attributes, which are automatically projected.</p>
    #[serde(rename = "Projection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projection: Option<AwsDynamoDbTableProjection>,
}

/// <p>For global and local secondary indexes, identifies the attributes that are copied from the table into the index.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsDynamoDbTableProjection {
    /// <p>The nonkey attributes that are projected into the index. For each attribute, provide the attribute name.</p>
    #[serde(rename = "NonKeyAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_key_attributes: Option<Vec<String>>,
    /// <p>The types of attributes that are projected into the index.</p>
    #[serde(rename = "ProjectionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projection_type: Option<String>,
}

/// <p>Information about the provisioned throughput for the table or for a global secondary index.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsDynamoDbTableProvisionedThroughput {
    /// <p>Indicates when the provisioned throughput was last decreased.</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "LastDecreaseDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_decrease_date_time: Option<String>,
    /// <p>Indicates when the provisioned throughput was last increased.</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "LastIncreaseDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_increase_date_time: Option<String>,
    /// <p>The number of times during the current UTC calendar day that the provisioned throughput was decreased.</p>
    #[serde(rename = "NumberOfDecreasesToday")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_decreases_today: Option<i64>,
    /// <p>The maximum number of strongly consistent reads consumed per second before DynamoDB returns a <code>ThrottlingException</code>.</p>
    #[serde(rename = "ReadCapacityUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_capacity_units: Option<i64>,
    /// <p>The maximum number of writes consumed per second before DynamoDB returns a <code>ThrottlingException</code>.</p>
    #[serde(rename = "WriteCapacityUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_capacity_units: Option<i64>,
}

/// <p>Replica-specific configuration for the provisioned throughput.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsDynamoDbTableProvisionedThroughputOverride {
    /// <p>The read capacity units for the replica.</p>
    #[serde(rename = "ReadCapacityUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_capacity_units: Option<i64>,
}

/// <p>Information about a replica of a DynamoDB table.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsDynamoDbTableReplica {
    /// <p>List of global secondary indexes for the replica.</p>
    #[serde(rename = "GlobalSecondaryIndexes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_secondary_indexes: Option<Vec<AwsDynamoDbTableReplicaGlobalSecondaryIndex>>,
    /// <p>The identifier of the AWS KMS customer master key (CMK) that will be used for AWS KMS encryption for the replica.</p>
    #[serde(rename = "KmsMasterKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_master_key_id: Option<String>,
    /// <p>Replica-specific configuration for the provisioned throughput.</p>
    #[serde(rename = "ProvisionedThroughputOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_throughput_override: Option<AwsDynamoDbTableProvisionedThroughputOverride>,
    /// <p>The name of the Region where the replica is located.</p>
    #[serde(rename = "RegionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_name: Option<String>,
    /// <p>The current status of the replica.</p>
    #[serde(rename = "ReplicaStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_status: Option<String>,
    /// <p>Detailed information about the replica status.</p>
    #[serde(rename = "ReplicaStatusDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_status_description: Option<String>,
}

/// <p>Information about a global secondary index for a DynamoDB table replica.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsDynamoDbTableReplicaGlobalSecondaryIndex {
    /// <p>The name of the index.</p>
    #[serde(rename = "IndexName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    /// <p>Replica-specific configuration for the provisioned throughput for the index.</p>
    #[serde(rename = "ProvisionedThroughputOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_throughput_override: Option<AwsDynamoDbTableProvisionedThroughputOverride>,
}

/// <p>Information about the restore for the table.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsDynamoDbTableRestoreSummary {
    /// <p>Indicates the point in time that the table was restored to.</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "RestoreDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_date_time: Option<String>,
    /// <p>Whether a restore is currently in progress.</p>
    #[serde(rename = "RestoreInProgress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_in_progress: Option<bool>,
    /// <p>The ARN of the source backup from which the table was restored.</p>
    #[serde(rename = "SourceBackupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_backup_arn: Option<String>,
    /// <p>The ARN of the source table for the backup.</p>
    #[serde(rename = "SourceTableArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_table_arn: Option<String>,
}

/// <p>Information about the server-side encryption for the table.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsDynamoDbTableSseDescription {
    /// <p>If the key is inaccessible, the date and time when DynamoDB detected that the key was inaccessible.</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "InaccessibleEncryptionDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inaccessible_encryption_date_time: Option<String>,
    /// <p>The ARN of the AWS KMS customer master key (CMK) that is used for the AWS KMS encryption.</p>
    #[serde(rename = "KmsMasterKeyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_master_key_arn: Option<String>,
    /// <p>The type of server-side encryption.</p>
    #[serde(rename = "SseType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sse_type: Option<String>,
    /// <p>The status of the server-side encryption.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>The current DynamoDB Streams configuration for the table.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsDynamoDbTableStreamSpecification {
    /// <p>Indicates whether DynamoDB Streams is enabled on the table.</p>
    #[serde(rename = "StreamEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_enabled: Option<bool>,
    /// <p>Determines the information that is written to the table.</p>
    #[serde(rename = "StreamViewType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_view_type: Option<String>,
}

/// <p>Information about an Elastic IP address.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsEc2EipDetails {
    /// <p>The identifier that AWS assigns to represent the allocation of the Elastic IP address for use with Amazon VPC.</p>
    #[serde(rename = "AllocationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocation_id: Option<String>,
    /// <p>The identifier that represents the association of the Elastic IP address with an EC2 instance.</p>
    #[serde(rename = "AssociationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,
    /// <p>The domain in which to allocate the address.</p> <p>If the address is for use with EC2 instances in a VPC, then <code>Domain</code> is <code>vpc</code>. Otherwise, <code>Domain</code> is <code>standard</code>. </p>
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// <p>The identifier of the EC2 instance.</p>
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// <p>The name of the location from which the Elastic IP address is advertised.</p>
    #[serde(rename = "NetworkBorderGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_border_group: Option<String>,
    /// <p>The identifier of the network interface.</p>
    #[serde(rename = "NetworkInterfaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_id: Option<String>,
    /// <p>The AWS account ID of the owner of the network interface.</p>
    #[serde(rename = "NetworkInterfaceOwnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_owner_id: Option<String>,
    /// <p>The private IP address that is associated with the Elastic IP address.</p>
    #[serde(rename = "PrivateIpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
    /// <p>A public IP address that is associated with the EC2 instance.</p>
    #[serde(rename = "PublicIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_ip: Option<String>,
    /// <p>The identifier of an IP address pool. This parameter allows Amazon EC2 to select an IP address from the address pool.</p>
    #[serde(rename = "PublicIpv4Pool")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_ipv_4_pool: Option<String>,
}

/// <p>The details of an EC2 instance.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsEc2InstanceDetails {
    /// <p>The IAM profile ARN of the instance.</p>
    #[serde(rename = "IamInstanceProfileArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_instance_profile_arn: Option<String>,
    /// <p>The Amazon Machine Image (AMI) ID of the instance.</p>
    #[serde(rename = "ImageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    /// <p>The IPv4 addresses associated with the instance.</p>
    #[serde(rename = "IpV4Addresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_v4_addresses: Option<Vec<String>>,
    /// <p>The IPv6 addresses associated with the instance.</p>
    #[serde(rename = "IpV6Addresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_v6_addresses: Option<Vec<String>>,
    /// <p>The key name associated with the instance.</p>
    #[serde(rename = "KeyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,
    /// <p>Indicates when the instance was launched.</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "LaunchedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launched_at: Option<String>,
    /// <p>The identifiers of the network interfaces for the EC2 instance. The details for each network interface are in a corresponding <code>AwsEc2NetworkInterfacesDetails</code> object.</p>
    #[serde(rename = "NetworkInterfaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interfaces: Option<Vec<AwsEc2InstanceNetworkInterfacesDetails>>,
    /// <p>The identifier of the subnet that the instance was launched in.</p>
    #[serde(rename = "SubnetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    /// <p>The instance type of the instance. </p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The identifier of the VPC that the instance was launched in.</p>
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

/// <p>Identifies a network interface for the EC2 instance.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsEc2InstanceNetworkInterfacesDetails {
    /// <p>The identifier of the network interface. The details are in a corresponding <code>AwsEc2NetworkInterfacesDetails</code> object.</p>
    #[serde(rename = "NetworkInterfaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_id: Option<String>,
}

/// <p>An association between the network ACL and a subnet.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsEc2NetworkAclAssociation {
    /// <p>The identifier of the association between the network ACL and the subnet.</p>
    #[serde(rename = "NetworkAclAssociationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_acl_association_id: Option<String>,
    /// <p>The identifier of the network ACL.</p>
    #[serde(rename = "NetworkAclId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_acl_id: Option<String>,
    /// <p>The identifier of the subnet that is associated with the network ACL.</p>
    #[serde(rename = "SubnetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
}

/// <p>Contains details about an EC2 network access control list (ACL).</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsEc2NetworkAclDetails {
    /// <p>Associations between the network ACL and subnets.</p>
    #[serde(rename = "Associations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associations: Option<Vec<AwsEc2NetworkAclAssociation>>,
    /// <p>The set of rules in the network ACL.</p>
    #[serde(rename = "Entries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<AwsEc2NetworkAclEntry>>,
    /// <p>Whether this is the default network ACL for the VPC.</p>
    #[serde(rename = "IsDefault")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    /// <p>The identifier of the network ACL.</p>
    #[serde(rename = "NetworkAclId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_acl_id: Option<String>,
    /// <p>The identifier of the AWS account that owns the network ACL.</p>
    #[serde(rename = "OwnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    /// <p>The identifier of the VPC for the network ACL.</p>
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

/// <p>A rule for the network ACL. Each rule allows or denies access based on the IP address, traffic direction, port, and protocol.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsEc2NetworkAclEntry {
    /// <p>The IPV4 network range for which to deny or allow access.</p>
    #[serde(rename = "CidrBlock")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_block: Option<String>,
    /// <p>Whether the rule is an egress rule. An egress rule is a rule that applies to traffic that leaves the subnet.</p>
    #[serde(rename = "Egress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress: Option<bool>,
    /// <p>The Internet Control Message Protocol (ICMP) type and code for which to deny or allow access.</p>
    #[serde(rename = "IcmpTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icmp_type_code: Option<IcmpTypeCode>,
    /// <p>The IPV6 network range for which to deny or allow access.</p>
    #[serde(rename = "Ipv6CidrBlock")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv_6_cidr_block: Option<String>,
    /// <p>For TCP or UDP protocols, the range of ports that the rule applies to.</p>
    #[serde(rename = "PortRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_range: Option<PortRangeFromTo>,
    /// <p>The protocol that the rule applies to. To deny or allow access to all protocols, use the value -1.</p>
    #[serde(rename = "Protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// <p>Whether the rule is used to allow access or deny access.</p>
    #[serde(rename = "RuleAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_action: Option<String>,
    /// <p>The rule number. The rules are processed in order by their number.</p>
    #[serde(rename = "RuleNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_number: Option<i64>,
}

/// <p>Information about the network interface attachment.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsEc2NetworkInterfaceAttachment {
    /// <p>Indicates when the attachment initiated.</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "AttachTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_time: Option<String>,
    /// <p>The identifier of the network interface attachment</p>
    #[serde(rename = "AttachmentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_id: Option<String>,
    /// <p>Indicates whether the network interface is deleted when the instance is terminated.</p>
    #[serde(rename = "DeleteOnTermination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_on_termination: Option<bool>,
    /// <p>The device index of the network interface attachment on the instance.</p>
    #[serde(rename = "DeviceIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_index: Option<i64>,
    /// <p>The ID of the instance.</p>
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// <p>The AWS account ID of the owner of the instance.</p>
    #[serde(rename = "InstanceOwnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_owner_id: Option<String>,
    /// <p>The attachment state.</p> <p>Valid values: <code>attaching</code> | <code>attached</code> | <code>detaching</code> | <code>detached</code> </p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Details about the network interface</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsEc2NetworkInterfaceDetails {
    /// <p>The network interface attachment.</p>
    #[serde(rename = "Attachment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment: Option<AwsEc2NetworkInterfaceAttachment>,
    /// <p>The IPv6 addresses associated with the network interface.</p>
    #[serde(rename = "IpV6Addresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_v6_addresses: Option<Vec<AwsEc2NetworkInterfaceIpV6AddressDetail>>,
    /// <p>The ID of the network interface.</p>
    #[serde(rename = "NetworkInterfaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_id: Option<String>,
    /// <p>The private IPv4 addresses associated with the network interface.</p>
    #[serde(rename = "PrivateIpAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ip_addresses: Option<Vec<AwsEc2NetworkInterfacePrivateIpAddressDetail>>,
    /// <p>The public DNS name of the network interface.</p>
    #[serde(rename = "PublicDnsName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_dns_name: Option<String>,
    /// <p>The address of the Elastic IP address bound to the network interface.</p>
    #[serde(rename = "PublicIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_ip: Option<String>,
    /// <p>Security groups for the network interface.</p>
    #[serde(rename = "SecurityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<AwsEc2NetworkInterfaceSecurityGroup>>,
    /// <p>Indicates whether traffic to or from the instance is validated.</p>
    #[serde(rename = "SourceDestCheck")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_dest_check: Option<bool>,
}

/// <p>Provides information about an IPV6 address that is associated with the network interface.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsEc2NetworkInterfaceIpV6AddressDetail {
    /// <p>The IPV6 address.</p>
    #[serde(rename = "IpV6Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_v6_address: Option<String>,
}

/// <p>Provides information about a private IPv4 address that is with the network interface.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsEc2NetworkInterfacePrivateIpAddressDetail {
    /// <p>The private DNS name for the IP address.</p>
    #[serde(rename = "PrivateDnsName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_dns_name: Option<String>,
    /// <p>The IP address.</p>
    #[serde(rename = "PrivateIpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
}

/// <p>A security group associated with the network interface.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsEc2NetworkInterfaceSecurityGroup {
    /// <p>The ID of the security group.</p>
    #[serde(rename = "GroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// <p>The name of the security group.</p>
    #[serde(rename = "GroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
}

/// <p>Details about an EC2 security group.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsEc2SecurityGroupDetails {
    /// <p>The ID of the security group.</p>
    #[serde(rename = "GroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// <p>The name of the security group.</p>
    #[serde(rename = "GroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// <p>The inbound rules associated with the security group.</p>
    #[serde(rename = "IpPermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_permissions: Option<Vec<AwsEc2SecurityGroupIpPermission>>,
    /// <p>[VPC only] The outbound rules associated with the security group.</p>
    #[serde(rename = "IpPermissionsEgress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_permissions_egress: Option<Vec<AwsEc2SecurityGroupIpPermission>>,
    /// <p>The AWS account ID of the owner of the security group.</p>
    #[serde(rename = "OwnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    /// <p>[VPC only] The ID of the VPC for the security group.</p>
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

/// <p>An IP permission for an EC2 security group.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsEc2SecurityGroupIpPermission {
    /// <p>The start of the port range for the TCP and UDP protocols, or an ICMP/ICMPv6 type number.</p> <p>A value of -1 indicates all ICMP/ICMPv6 types. If you specify all ICMP/ICMPv6 types, you must specify all codes. </p>
    #[serde(rename = "FromPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_port: Option<i64>,
    /// <p>The IP protocol name (<code>tcp</code>, <code>udp</code>, <code>icmp</code>, <code>icmpv6</code>) or number.</p> <p>[VPC only] Use <code>-1</code> to specify all protocols.</p> <p>When authorizing security group rules, specifying -1 or a protocol number other than <code>tcp</code>, <code>udp</code>, <code>icmp</code>, or <code>icmpv6</code> allows traffic on all ports, regardless of any port range you specify.</p> <p>For <code>tcp</code>, <code>udp</code>, and <code>icmp</code>, you must specify a port range.</p> <p>For <code>icmpv6</code>, the port range is optional. If you omit the port range, traffic for all types and codes is allowed. </p>
    #[serde(rename = "IpProtocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_protocol: Option<String>,
    /// <p>The IPv4 ranges.</p>
    #[serde(rename = "IpRanges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_ranges: Option<Vec<AwsEc2SecurityGroupIpRange>>,
    /// <p>The IPv6 ranges.</p>
    #[serde(rename = "Ipv6Ranges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv_6_ranges: Option<Vec<AwsEc2SecurityGroupIpv6Range>>,
    /// <p>[VPC only] The prefix list IDs for an AWS service. With outbound rules, this is the AWS service to access through a VPC endpoint from instances associated with the security group.</p>
    #[serde(rename = "PrefixListIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_list_ids: Option<Vec<AwsEc2SecurityGroupPrefixListId>>,
    /// <p>The end of the port range for the TCP and UDP protocols, or an ICMP/ICMPv6 code.</p> <p>A value of -1 indicates all ICMP/ICMPv6 codes. If you specify all ICMP/ICMPv6 types, you must specify all codes.</p>
    #[serde(rename = "ToPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_port: Option<i64>,
    /// <p>The security group and AWS account ID pairs.</p>
    #[serde(rename = "UserIdGroupPairs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_group_pairs: Option<Vec<AwsEc2SecurityGroupUserIdGroupPair>>,
}

/// <p>A range of IPv4 addresses.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsEc2SecurityGroupIpRange {
    /// <p>The IPv4 CIDR range. You can specify either a CIDR range or a source security group, but not both. To specify a single IPv4 address, use the /32 prefix length.</p>
    #[serde(rename = "CidrIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_ip: Option<String>,
}

/// <p>A range of IPv6 addresses.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsEc2SecurityGroupIpv6Range {
    /// <p>The IPv6 CIDR range. You can specify either a CIDR range or a source security group, but not both. To specify a single IPv6 address, use the /128 prefix length.</p>
    #[serde(rename = "CidrIpv6")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_ipv_6: Option<String>,
}

/// <p>A prefix list ID.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsEc2SecurityGroupPrefixListId {
    /// <p>The ID of the prefix.</p>
    #[serde(rename = "PrefixListId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_list_id: Option<String>,
}

/// <p>A relationship between a security group and a user.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsEc2SecurityGroupUserIdGroupPair {
    /// <p>The ID of the security group.</p>
    #[serde(rename = "GroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// <p>The name of the security group.</p>
    #[serde(rename = "GroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// <p>The status of a VPC peering connection, if applicable.</p>
    #[serde(rename = "PeeringStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peering_status: Option<String>,
    /// <p>The ID of an AWS account.</p> <p>For a referenced security group in another VPC, the account ID of the referenced security group is returned in the response. If the referenced security group is deleted, this value is not returned.</p> <p>[EC2-Classic] Required when adding or removing rules that reference a security group in another VPC. </p>
    #[serde(rename = "UserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// <p>The ID of the VPC for the referenced security group, if applicable.</p>
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// <p>The ID of the VPC peering connection, if applicable.</p>
    #[serde(rename = "VpcPeeringConnectionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_peering_connection_id: Option<String>,
}

/// <p>Contains information about a subnet in EC2.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsEc2SubnetDetails {
    /// <p>Whether to assign an IPV6 address to a network interface that is created in this subnet.</p>
    #[serde(rename = "AssignIpv6AddressOnCreation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assign_ipv_6_address_on_creation: Option<bool>,
    /// <p>The Availability Zone for the subnet.</p>
    #[serde(rename = "AvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    /// <p>The identifier of the Availability Zone for the subnet.</p>
    #[serde(rename = "AvailabilityZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone_id: Option<String>,
    /// <p>The number of available IPV4 addresses in the subnet. Does not include addresses for stopped instances.</p>
    #[serde(rename = "AvailableIpAddressCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_ip_address_count: Option<i64>,
    /// <p>The IPV4 CIDR block that is assigned to the subnet.</p>
    #[serde(rename = "CidrBlock")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_block: Option<String>,
    /// <p>Whether this subnet is the default subnet for the Availability Zone.</p>
    #[serde(rename = "DefaultForAz")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_for_az: Option<bool>,
    /// <p>The IPV6 CIDR blocks that are associated with the subnet.</p>
    #[serde(rename = "Ipv6CidrBlockAssociationSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv_6_cidr_block_association_set: Option<Vec<Ipv6CidrBlockAssociation>>,
    /// <p>Whether instances in this subnet receive a public IP address.</p>
    #[serde(rename = "MapPublicIpOnLaunch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_public_ip_on_launch: Option<bool>,
    /// <p>The identifier of the AWS account that owns the subnet.</p>
    #[serde(rename = "OwnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    /// <p>The current state of the subnet.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The ARN of the subnet.</p>
    #[serde(rename = "SubnetArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_arn: Option<String>,
    /// <p>The identifier of the subnet.</p>
    #[serde(rename = "SubnetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    /// <p>The identifier of the VPC that contains the subnet.</p>
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

/// <p>An attachment to an AWS EC2 volume.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsEc2VolumeAttachment {
    /// <p>The datetime when the attachment initiated.</p>
    #[serde(rename = "AttachTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_time: Option<String>,
    /// <p>Whether the EBS volume is deleted when the EC2 instance is terminated.</p>
    #[serde(rename = "DeleteOnTermination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_on_termination: Option<bool>,
    /// <p>The identifier of the EC2 instance.</p>
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// <p>The attachment state of the volume.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Details about an EC2 volume.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsEc2VolumeDetails {
    /// <p>The volume attachments.</p>
    #[serde(rename = "Attachments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<AwsEc2VolumeAttachment>>,
    /// <p>Indicates when the volume was created.</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// <p>Whether the volume is encrypted.</p>
    #[serde(rename = "Encrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    /// <p>The ARN of the AWS Key Management Service (AWS KMS) customer master key (CMK) that was used to protect the volume encryption key for the volume.</p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>The size of the volume, in GiBs.</p>
    #[serde(rename = "Size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// <p>The snapshot from which the volume was created.</p>
    #[serde(rename = "SnapshotId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
    /// <p>The volume state.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Details about an EC2 VPC.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsEc2VpcDetails {
    /// <p>Information about the IPv4 CIDR blocks associated with the VPC.</p>
    #[serde(rename = "CidrBlockAssociationSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_block_association_set: Option<Vec<CidrBlockAssociation>>,
    /// <p>The identifier of the set of Dynamic Host Configuration Protocol (DHCP) options that are associated with the VPC. If the default options are associated with the VPC, then this is default.</p>
    #[serde(rename = "DhcpOptionsId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dhcp_options_id: Option<String>,
    /// <p>Information about the IPv6 CIDR blocks associated with the VPC.</p>
    #[serde(rename = "Ipv6CidrBlockAssociationSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv_6_cidr_block_association_set: Option<Vec<Ipv6CidrBlockAssociation>>,
    /// <p>The current state of the VPC.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// <p>Indicates whether to enable CloudWatch Container Insights for the ECS cluster.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsEcsClusterClusterSettingsDetails {
    /// <p>The name of the setting.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The value of the setting.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>The run command configuration for the cluster.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsEcsClusterConfigurationDetails {
    /// <p>Contains the run command configuration for the cluster.</p>
    #[serde(rename = "ExecuteCommandConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execute_command_configuration:
        Option<AwsEcsClusterConfigurationExecuteCommandConfigurationDetails>,
}

/// <p>Contains the run command configuration for the cluster.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsEcsClusterConfigurationExecuteCommandConfigurationDetails {
    /// <p>The identifier of the KMS key that is used to encrypt the data between the local client and the container.</p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>The log configuration for the results of the run command actions. Required if <code>Logging</code> is <code>NONE</code>.</p>
    #[serde(rename = "LogConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_configuration:
        Option<AwsEcsClusterConfigurationExecuteCommandConfigurationLogConfigurationDetails>,
    /// <p>The log setting to use for redirecting logs for run command results.</p>
    #[serde(rename = "Logging")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<String>,
}

/// <p>The log configuration for the results of the run command actions.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsEcsClusterConfigurationExecuteCommandConfigurationLogConfigurationDetails {
    /// <p>Whether to enable encryption on the CloudWatch logs.</p>
    #[serde(rename = "CloudWatchEncryptionEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_encryption_enabled: Option<bool>,
    /// <p>The name of the CloudWatch log group to send the logs to.</p>
    #[serde(rename = "CloudWatchLogGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_log_group_name: Option<String>,
    /// <p>The name of the S3 bucket to send logs to.</p>
    #[serde(rename = "S3BucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_name: Option<String>,
    /// <p>Whether to encrypt the logs that are sent to the S3 bucket.</p>
    #[serde(rename = "S3EncryptionEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_encryption_enabled: Option<bool>,
    /// <p>Identifies the folder in the S3 bucket to send the logs to.</p>
    #[serde(rename = "S3KeyPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key_prefix: Option<String>,
}

/// <p>The default capacity provider strategy for the cluster. The default capacity provider strategy is used when services or tasks are run without a specified launch type or capacity provider strategy.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsEcsClusterDefaultCapacityProviderStrategyDetails {
    /// <p>The minimum number of tasks to run on the specified capacity provider.</p>
    #[serde(rename = "Base")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<i64>,
    /// <p>The name of the capacity provider.</p>
    #[serde(rename = "CapacityProvider")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_provider: Option<String>,
    /// <p>The relative percentage of the total number of tasks launched that should use the capacity provider.</p>
    #[serde(rename = "Weight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i64>,
}

/// <p>provides details about an ECS cluster.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsEcsClusterDetails {
    /// <p>The short name of one or more capacity providers to associate with the cluster.</p>
    #[serde(rename = "CapacityProviders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_providers: Option<Vec<String>>,
    /// <p>The setting to use to create the cluster. Specifically used to configure whether to enable CloudWatch Container Insights for the cluster.</p>
    #[serde(rename = "ClusterSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_settings: Option<Vec<AwsEcsClusterClusterSettingsDetails>>,
    /// <p>The run command configuration for the cluster.</p>
    #[serde(rename = "Configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<AwsEcsClusterConfigurationDetails>,
    /// <p>The default capacity provider strategy for the cluster. The default capacity provider strategy is used when services or tasks are run without a specified launch type or capacity provider strategy.</p>
    #[serde(rename = "DefaultCapacityProviderStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_capacity_provider_strategy:
        Option<Vec<AwsEcsClusterDefaultCapacityProviderStrategyDetails>>,
}

/// <p>A dependency that is defined for container startup and shutdown.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsEcsTaskDefinitionContainerDefinitionsDependsOnDetails {
    /// <p>The dependency condition of the dependent container. Indicates the required status of the dependent container before the current container can start.</p>
    #[serde(rename = "Condition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    /// <p>The name of the dependent container.</p>
    #[serde(rename = "ContainerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
}

/// <p>A container definition that describes a container in the task.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsEcsTaskDefinitionContainerDefinitionsDetails {
    /// <p>The command that is passed to the container.</p>
    #[serde(rename = "Command")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    /// <p>The number of CPU units reserved for the container.</p>
    #[serde(rename = "Cpu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<i64>,
    /// <p>The dependencies that are defined for container startup and shutdown.</p>
    #[serde(rename = "DependsOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depends_on: Option<Vec<AwsEcsTaskDefinitionContainerDefinitionsDependsOnDetails>>,
    /// <p>Whether to disable networking within the container.</p>
    #[serde(rename = "DisableNetworking")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_networking: Option<bool>,
    /// <p>A list of DNS search domains that are presented to the container.</p>
    #[serde(rename = "DnsSearchDomains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_search_domains: Option<Vec<String>>,
    /// <p>A list of DNS servers that are presented to the container.</p>
    #[serde(rename = "DnsServers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_servers: Option<Vec<String>>,
    /// <p>A key-value map of labels to add to the container.</p>
    #[serde(rename = "DockerLabels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docker_labels: Option<::std::collections::HashMap<String, String>>,
    /// <p>A list of strings to provide custom labels for SELinux and AppArmor multi-level security systems.</p>
    #[serde(rename = "DockerSecurityOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docker_security_options: Option<Vec<String>>,
    /// <p>The entry point that is passed to the container.</p>
    #[serde(rename = "EntryPoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_point: Option<Vec<String>>,
    /// <p>The environment variables to pass to a container.</p>
    #[serde(rename = "Environment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Vec<AwsEcsTaskDefinitionContainerDefinitionsEnvironmentDetails>>,
    /// <p>A list of files containing the environment variables to pass to a container.</p>
    #[serde(rename = "EnvironmentFiles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_files:
        Option<Vec<AwsEcsTaskDefinitionContainerDefinitionsEnvironmentFilesDetails>>,
    /// <p>Whether the container is essential. All tasks must have at least one essential container.</p>
    #[serde(rename = "Essential")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub essential: Option<bool>,
    /// <p>A list of hostnames and IP address mappings to append to the <b>/etc/hosts</b> file on the container.</p>
    #[serde(rename = "ExtraHosts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_hosts: Option<Vec<AwsEcsTaskDefinitionContainerDefinitionsExtraHostsDetails>>,
    /// <p>The FireLens configuration for the container. Specifies and configures a log router for container logs.</p>
    #[serde(rename = "FirelensConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firelens_configuration:
        Option<AwsEcsTaskDefinitionContainerDefinitionsFirelensConfigurationDetails>,
    /// <p>The container health check command and associated configuration parameters for the container.</p>
    #[serde(rename = "HealthCheck")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check: Option<AwsEcsTaskDefinitionContainerDefinitionsHealthCheckDetails>,
    /// <p>The hostname to use for the container.</p>
    #[serde(rename = "Hostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// <p>The image used to start the container.</p>
    #[serde(rename = "Image")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// <p>If set to true, then containerized applications can be deployed that require <code>stdin</code> or a <code>tty</code> to be allocated.</p>
    #[serde(rename = "Interactive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactive: Option<bool>,
    /// <p>A list of links for the container in the form <code> <i>container_name</i>:<i>alias</i> </code>. Allows containers to communicate with each other without the need for port mappings.</p>
    #[serde(rename = "Links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<String>>,
    /// <p>Linux-specific modifications that are applied to the container, such as Linux kernel capabilities.</p>
    #[serde(rename = "LinuxParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linux_parameters: Option<AwsEcsTaskDefinitionContainerDefinitionsLinuxParametersDetails>,
    /// <p>The log configuration specification for the container.</p>
    #[serde(rename = "LogConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_configuration: Option<AwsEcsTaskDefinitionContainerDefinitionsLogConfigurationDetails>,
    /// <p>The amount (in MiB) of memory to present to the container. If the container attempts to exceed the memory specified here, the container is shut down. The total amount of memory reserved for all containers within a task must be lower than the task memory value, if one is specified.</p>
    #[serde(rename = "Memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<i64>,
    /// <p>The soft limit (in MiB) of memory to reserve for the container.</p>
    #[serde(rename = "MemoryReservation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_reservation: Option<i64>,
    /// <p>The mount points for the data volumes in the container.</p>
    #[serde(rename = "MountPoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_points: Option<Vec<AwsEcsTaskDefinitionContainerDefinitionsMountPointsDetails>>,
    /// <p>The name of the container.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The list of port mappings for the container.</p>
    #[serde(rename = "PortMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_mappings: Option<Vec<AwsEcsTaskDefinitionContainerDefinitionsPortMappingsDetails>>,
    /// <p>Whether the container is given elevated privileges on the host container instance. The elevated privileges are similar to the root user.</p>
    #[serde(rename = "Privileged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
    /// <p>Whether to allocate a TTY to the container.</p>
    #[serde(rename = "PseudoTerminal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pseudo_terminal: Option<bool>,
    /// <p>Whether the container is given read-only access to its root file system.</p>
    #[serde(rename = "ReadonlyRootFilesystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readonly_root_filesystem: Option<bool>,
    /// <p>The private repository authentication credentials to use.</p>
    #[serde(rename = "RepositoryCredentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_credentials:
        Option<AwsEcsTaskDefinitionContainerDefinitionsRepositoryCredentialsDetails>,
    /// <p>The type and amount of a resource to assign to a container. The only supported resource is a GPU.</p>
    #[serde(rename = "ResourceRequirements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_requirements:
        Option<Vec<AwsEcsTaskDefinitionContainerDefinitionsResourceRequirementsDetails>>,
    /// <p>The secrets to pass to the container.</p>
    #[serde(rename = "Secrets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: Option<Vec<AwsEcsTaskDefinitionContainerDefinitionsSecretsDetails>>,
    /// <p>The number of seconds to wait before giving up on resolving dependencies for a container. </p>
    #[serde(rename = "StartTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_timeout: Option<i64>,
    /// <p>The number of seconds to wait before the container is stopped if it doesn't shut down normally on its own.</p>
    #[serde(rename = "StopTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_timeout: Option<i64>,
    /// <p>A list of namespaced kernel parameters to set in the container.</p>
    #[serde(rename = "SystemControls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_controls: Option<Vec<AwsEcsTaskDefinitionContainerDefinitionsSystemControlsDetails>>,
    /// <p>A list of ulimits to set in the container. </p>
    #[serde(rename = "Ulimits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ulimits: Option<Vec<AwsEcsTaskDefinitionContainerDefinitionsUlimitsDetails>>,
    /// <p><p>The user to use inside the container.</p> <p>The value can use one of the following formats.</p> <ul> <li> <p> <code> <i>user</i> </code> </p> </li> <li> <p> <code> <i>user</i> </code>:<code> <i>group</i> </code> </p> </li> <li> <p> <code> <i>uid</i> </code> </p> </li> <li> <p> <code> <i>uid</i> </code>:<code> <i>gid</i> </code> </p> </li> <li> <p> <code> <i>user</i> </code>:<code> <i>gid</i> </code> </p> </li> <li> <p> <code> <i>uid</i> </code>:<code> <i>group</i> </code> </p> </li> </ul></p>
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// <p>Data volumes to mount from another container.</p>
    #[serde(rename = "VolumesFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes_from: Option<Vec<AwsEcsTaskDefinitionContainerDefinitionsVolumesFromDetails>>,
    /// <p>The working directory in which to run commands inside the container.</p>
    #[serde(rename = "WorkingDirectory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_directory: Option<String>,
}

/// <p>An environment variable to pass to the container.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsEcsTaskDefinitionContainerDefinitionsEnvironmentDetails {
    /// <p>The name of the environment variable.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The value of the environment variable.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>A file that contain environment variables to pass to a container.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsEcsTaskDefinitionContainerDefinitionsEnvironmentFilesDetails {
    /// <p>The type of environment file.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The ARN of the S3 object that contains the environment variable file.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>A hostname and IP address mapping to append to the <b>/etc/hosts</b> file on the container.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsEcsTaskDefinitionContainerDefinitionsExtraHostsDetails {
    /// <p>The hostname to use in the <b>/etc/hosts</b> entry.</p>
    #[serde(rename = "Hostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// <p>The IP address to use in the <b>/etc/hosts</b> entry.</p>
    #[serde(rename = "IpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
}

/// <p>The FireLens configuration for the container. The configuration specifies and configures a log router for container logs.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsEcsTaskDefinitionContainerDefinitionsFirelensConfigurationDetails {
    /// <p><p>The options to use to configure the log router.</p> <p>The valid option keys are as follows:</p> <ul> <li> <p> <code>enable-ecs-log-metadata</code>. The value can be <code>true</code> or <code>false</code>.</p> </li> <li> <p> <code>config-file-type</code>. The value can be <code>s3</code> or <code>file</code>.</p> </li> <li> <p> <code>config-file-value</code>. The value is either an S3 ARN or a file path.</p> </li> </ul></p>
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<::std::collections::HashMap<String, String>>,
    /// <p>The log router to use. </p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>The container health check command and associated configuration parameters for the container.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsEcsTaskDefinitionContainerDefinitionsHealthCheckDetails {
    /// <p>The command that the container runs to determine whether it is healthy.</p>
    #[serde(rename = "Command")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    /// <p>The time period in seconds between each health check execution. The default value is 30 seconds.</p>
    #[serde(rename = "Interval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<i64>,
    /// <p>The number of times to retry a failed health check before the container is considered unhealthy. The default value is 3.</p>
    #[serde(rename = "Retries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retries: Option<i64>,
    /// <p>The optional grace period in seconds that allows containers time to bootstrap before failed health checks count towards the maximum number of retries.</p>
    #[serde(rename = "StartPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_period: Option<i64>,
    /// <p>The time period in seconds to wait for a health check to succeed before it is considered a failure. The default value is 5.</p>
    #[serde(rename = "Timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
}

/// <p>The Linux capabilities for the container that are added to or dropped from the default configuration provided by Docker.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsEcsTaskDefinitionContainerDefinitionsLinuxParametersCapabilitiesDetails {
    /// <p>The Linux capabilities for the container that are added to the default configuration provided by Docker.</p>
    #[serde(rename = "Add")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add: Option<Vec<String>>,
    /// <p>The Linux capabilities for the container that are dropped from the default configuration provided by Docker.</p>
    #[serde(rename = "Drop")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop: Option<Vec<String>>,
}

/// <p>&gt;Linux-specific modifications that are applied to the container, such as Linux kernel capabilities.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsEcsTaskDefinitionContainerDefinitionsLinuxParametersDetails {
    /// <p>The Linux capabilities for the container that are added to or dropped from the default configuration provided by Docker.</p>
    #[serde(rename = "Capabilities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities:
        Option<AwsEcsTaskDefinitionContainerDefinitionsLinuxParametersCapabilitiesDetails>,
    /// <p>The host devices to expose to the container.</p>
    #[serde(rename = "Devices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<AwsEcsTaskDefinitionContainerDefinitionsLinuxParametersDevicesDetails>>,
    /// <p>Whether to run an <code>init</code> process inside the container that forwards signals and reaps processes. </p>
    #[serde(rename = "InitProcessEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init_process_enabled: Option<bool>,
    /// <p>The total amount of swap memory (in MiB) that a container can use.</p>
    #[serde(rename = "MaxSwap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_swap: Option<i64>,
    /// <p>The value for the size (in MiB) of the <b>/dev/shm</b> volume.</p>
    #[serde(rename = "SharedMemorySize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_memory_size: Option<i64>,
    /// <p>Configures the container's memory swappiness behavior. Determines how aggressively pages are swapped. The higher the value, the more aggressive the swappiness. The default is 60.</p>
    #[serde(rename = "Swappiness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swappiness: Option<i64>,
    /// <p>The container path, mount options, and size (in MiB) of the tmpfs mount.</p>
    #[serde(rename = "Tmpfs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tmpfs: Option<Vec<AwsEcsTaskDefinitionContainerDefinitionsLinuxParametersTmpfsDetails>>,
}

/// <p>A host device to expose to the container.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsEcsTaskDefinitionContainerDefinitionsLinuxParametersDevicesDetails {
    /// <p>The path inside the container at which to expose the host device.</p>
    #[serde(rename = "ContainerPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_path: Option<String>,
    /// <p>The path for the device on the host container instance.</p>
    #[serde(rename = "HostPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_path: Option<String>,
    /// <p>The explicit permissions to provide to the container for the device. By default, the container has permissions for read, write, and <code>mknod</code> for the device.</p>
    #[serde(rename = "Permissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
}

/// <p>The container path, mount options, and size (in MiB) of a tmpfs mount.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsEcsTaskDefinitionContainerDefinitionsLinuxParametersTmpfsDetails {
    /// <p>The absolute file path where the tmpfs volume is to be mounted.</p>
    #[serde(rename = "ContainerPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_path: Option<String>,
    /// <p>The list of tmpfs volume mount options.</p>
    #[serde(rename = "MountOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_options: Option<Vec<String>>,
    /// <p>The maximum size (in MiB) of the tmpfs volume.</p>
    #[serde(rename = "Size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}

/// <p>The log configuration specification for the container.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsEcsTaskDefinitionContainerDefinitionsLogConfigurationDetails {
    /// <p>The log driver to use for the container.</p>
    #[serde(rename = "LogDriver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_driver: Option<String>,
    /// <p>The configuration options to send to the log driver. Requires version 1.19 of the Docker Remote API or greater on your container instance.</p>
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<::std::collections::HashMap<String, String>>,
    /// <p>The secrets to pass to the log configuration.</p>
    #[serde(rename = "SecretOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_options:
        Option<Vec<AwsEcsTaskDefinitionContainerDefinitionsLogConfigurationSecretOptionsDetails>>,
}

/// <p>A secret to pass to the log configuration.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsEcsTaskDefinitionContainerDefinitionsLogConfigurationSecretOptionsDetails {
    /// <p>The name of the secret.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The secret to expose to the container.</p> <p>The value is either the full ARN of the Secrets Manager secret or the full ARN of the parameter in the Systems Manager Parameter Store.</p>
    #[serde(rename = "ValueFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_from: Option<String>,
}

/// <p>A mount point for the data volumes in the container.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsEcsTaskDefinitionContainerDefinitionsMountPointsDetails {
    /// <p>The path on the container to mount the host volume at.</p>
    #[serde(rename = "ContainerPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_path: Option<String>,
    /// <p>Whether the container has read-only access to the volume.</p>
    #[serde(rename = "ReadOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// <p>The name of the volume to mount. Must match the name of a volume listed in <code>VolumeDetails</code> for the task definition.</p>
    #[serde(rename = "SourceVolume")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_volume: Option<String>,
}

/// <p>A port mapping for the container.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsEcsTaskDefinitionContainerDefinitionsPortMappingsDetails {
    /// <p>The port number on the container that is bound to the user-specified or automatically assigned host port.</p>
    #[serde(rename = "ContainerPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_port: Option<i64>,
    /// <p>The port number on the container instance to reserve for the container.</p>
    #[serde(rename = "HostPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_port: Option<i64>,
    /// <p>The protocol used for the port mapping. The default is <code>tcp</code>.</p>
    #[serde(rename = "Protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

/// <p>The private repository authentication credentials to use.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsEcsTaskDefinitionContainerDefinitionsRepositoryCredentialsDetails {
    /// <p>The ARN of the secret that contains the private repository credentials.</p>
    #[serde(rename = "CredentialsParameter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials_parameter: Option<String>,
}

/// <p>A resource to assign to a container.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsEcsTaskDefinitionContainerDefinitionsResourceRequirementsDetails {
    /// <p>The type of resource to assign to a container.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The value for the specified resource type.</p> <p>For <code>GPU</code>, the value is the number of physical GPUs the Amazon ECS container agent reserves for the container.</p> <p>For <code>InferenceAccelerator</code>, the value should match the <code>DeviceName</code> attribute of an entry in <code>InferenceAccelerators</code>.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>A secret to pass to the container.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsEcsTaskDefinitionContainerDefinitionsSecretsDetails {
    /// <p>The name of the secret.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The secret to expose to the container. The value is either the full ARN of the Secrets Manager secret or the full ARN of the parameter in the Systems Manager Parameter Store.</p>
    #[serde(rename = "ValueFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_from: Option<String>,
}

/// <p>A namespaced kernel parameter to set in the container.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsEcsTaskDefinitionContainerDefinitionsSystemControlsDetails {
    /// <p>The namespaced kernel parameter for which to set a value.</p>
    #[serde(rename = "Namespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// <p>The value of the parameter.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>A ulimit to set in the container.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsEcsTaskDefinitionContainerDefinitionsUlimitsDetails {
    /// <p>The hard limit for the ulimit type.</p>
    #[serde(rename = "HardLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hard_limit: Option<i64>,
    /// <p>The type of the ulimit.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The soft limit for the ulimit type.</p>
    #[serde(rename = "SoftLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub soft_limit: Option<i64>,
}

/// <p>A data volume to mount from another container.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsEcsTaskDefinitionContainerDefinitionsVolumesFromDetails {
    /// <p>Whether the container has read-only access to the volume.</p>
    #[serde(rename = "ReadOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// <p>The name of another container within the same task definition from which to mount volumes.</p>
    #[serde(rename = "SourceContainer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_container: Option<String>,
}

/// <p>details about a task definition. A task definition describes the container and volume definitions of an Amazon Elastic Container Service task.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsEcsTaskDefinitionDetails {
    /// <p>The container definitions that describe the containers that make up the task.</p>
    #[serde(rename = "ContainerDefinitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_definitions: Option<Vec<AwsEcsTaskDefinitionContainerDefinitionsDetails>>,
    /// <p>The number of CPU units used by the task.</p>
    #[serde(rename = "Cpu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<String>,
    /// <p>The ARN of the task execution role that grants the container agent permission to make API calls on behalf of the container user.</p>
    #[serde(rename = "ExecutionRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_arn: Option<String>,
    /// <p>The name of a family that this task definition is registered to.</p>
    #[serde(rename = "Family")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    /// <p>The Elastic Inference accelerators to use for the containers in the task.</p>
    #[serde(rename = "InferenceAccelerators")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_accelerators: Option<Vec<AwsEcsTaskDefinitionInferenceAcceleratorsDetails>>,
    /// <p>The IPC resource namespace to use for the containers in the task.</p>
    #[serde(rename = "IpcMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipc_mode: Option<String>,
    /// <p>The amount (in MiB) of memory used by the task.</p>
    #[serde(rename = "Memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<String>,
    /// <p>The Docker networking mode to use for the containers in the task.</p>
    #[serde(rename = "NetworkMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_mode: Option<String>,
    /// <p>The process namespace to use for the containers in the task.</p>
    #[serde(rename = "PidMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid_mode: Option<String>,
    /// <p>The placement constraint objects to use for tasks.</p>
    #[serde(rename = "PlacementConstraints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_constraints: Option<Vec<AwsEcsTaskDefinitionPlacementConstraintsDetails>>,
    /// <p>The configuration details for the App Mesh proxy.</p>
    #[serde(rename = "ProxyConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_configuration: Option<AwsEcsTaskDefinitionProxyConfigurationDetails>,
    /// <p>The task launch types that the task definition was validated against.</p>
    #[serde(rename = "RequiresCompatibilities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requires_compatibilities: Option<Vec<String>>,
    /// <p>The short name or ARN of the IAM role that grants containers in the task permission to call AWS API operations on your behalf.</p>
    #[serde(rename = "TaskRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_role_arn: Option<String>,
    /// <p>The data volume definitions for the task.</p>
    #[serde(rename = "Volumes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<AwsEcsTaskDefinitionVolumesDetails>>,
}

/// <p>An Elastic Inference accelerator to use for the containers in the task.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsEcsTaskDefinitionInferenceAcceleratorsDetails {
    /// <p>The Elastic Inference accelerator device name.</p>
    #[serde(rename = "DeviceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    /// <p>The Elastic Inference accelerator type to use.</p>
    #[serde(rename = "DeviceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,
}

/// <p>A placement constraint object to use for tasks.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsEcsTaskDefinitionPlacementConstraintsDetails {
    /// <p>A cluster query language expression to apply to the constraint.</p>
    #[serde(rename = "Expression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    /// <p>The type of constraint.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>The configuration details for the App Mesh proxy.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsEcsTaskDefinitionProxyConfigurationDetails {
    /// <p>The name of the container that will serve as the App Mesh proxy.</p>
    #[serde(rename = "ContainerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
    /// <p>The set of network configuration parameters to provide to the Container Network Interface (CNI) plugin, specified as key-value pairs.</p>
    #[serde(rename = "ProxyConfigurationProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_configuration_properties:
        Option<Vec<AwsEcsTaskDefinitionProxyConfigurationProxyConfigurationPropertiesDetails>>,
    /// <p>The proxy type.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>A network configuration parameter to provide to the Container Network Interface (CNI) plugin.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsEcsTaskDefinitionProxyConfigurationProxyConfigurationPropertiesDetails {
    /// <p>The name of the property.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The value of the property.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>A data volume to mount from another container.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsEcsTaskDefinitionVolumesDetails {
    /// <p>Information about a Docker volume.</p>
    #[serde(rename = "DockerVolumeConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docker_volume_configuration:
        Option<AwsEcsTaskDefinitionVolumesDockerVolumeConfigurationDetails>,
    /// <p>Information about the Amazon Elastic File System file system that is used for task storage.</p>
    #[serde(rename = "EfsVolumeConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub efs_volume_configuration: Option<AwsEcsTaskDefinitionVolumesEfsVolumeConfigurationDetails>,
    /// <p>Information about a bind mount host volume.</p>
    #[serde(rename = "Host")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<AwsEcsTaskDefinitionVolumesHostDetails>,
    /// <p>The name of the data volume.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Information about a Docker volume.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsEcsTaskDefinitionVolumesDockerVolumeConfigurationDetails {
    /// <p>Whether to create the Docker volume automatically if it does not already exist.</p>
    #[serde(rename = "Autoprovision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autoprovision: Option<bool>,
    /// <p>The Docker volume driver to use.</p>
    #[serde(rename = "Driver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    /// <p>A map of Docker driver-specific options that are passed through.</p>
    #[serde(rename = "DriverOpts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver_opts: Option<::std::collections::HashMap<String, String>>,
    /// <p>Custom metadata to add to the Docker volume.</p>
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
    /// <p>The scope for the Docker volume that determines its lifecycle. Docker volumes that are scoped to a task are provisioned automatically when the task starts and destroyed when the task stops. Docker volumes that are shared persist after the task stops.</p>
    #[serde(rename = "Scope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

/// <p><p/></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsEcsTaskDefinitionVolumesEfsVolumeConfigurationAuthorizationConfigDetails {
    /// <p>The Amazon EFS access point identifier to use.</p>
    #[serde(rename = "AccessPointId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point_id: Option<String>,
    /// <p>Whether to use the Amazon ECS task IAM role defined in a task definition when mounting the Amazon EFS file system.</p>
    #[serde(rename = "Iam")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam: Option<String>,
}

/// <p>Information about the Amazon Elastic File System file system that is used for task storage.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsEcsTaskDefinitionVolumesEfsVolumeConfigurationDetails {
    /// <p>The authorization configuration details for the Amazon EFS file system.</p>
    #[serde(rename = "AuthorizationConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_config:
        Option<AwsEcsTaskDefinitionVolumesEfsVolumeConfigurationAuthorizationConfigDetails>,
    /// <p>The Amazon EFS file system identifier to use.</p>
    #[serde(rename = "FilesystemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filesystem_id: Option<String>,
    /// <p>The directory within the Amazon EFS file system to mount as the root directory inside the host.</p>
    #[serde(rename = "RootDirectory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_directory: Option<String>,
    /// <p>Whether to enable encryption for Amazon EFS data in transit between the Amazon ECS host and the Amazon EFS server. </p>
    #[serde(rename = "TransitEncryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_encryption: Option<String>,
    /// <p>The port to use when sending encrypted data between the Amazon ECS host and the Amazon EFS server.</p>
    #[serde(rename = "TransitEncryptionPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_encryption_port: Option<i64>,
}

/// <p>Information about a bind mount host volume.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsEcsTaskDefinitionVolumesHostDetails {
    /// <p>The path on the host container instance that is presented to the container.</p>
    #[serde(rename = "SourcePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_path: Option<String>,
}

/// <p>Contains details about an Elastic Beanstalk environment.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsElasticBeanstalkEnvironmentDetails {
    /// <p>The name of the application that is associated with the environment.</p>
    #[serde(rename = "ApplicationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    /// <p>The URL to the CNAME for this environment.</p>
    #[serde(rename = "Cname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cname: Option<String>,
    /// <p>The creation date for this environment.</p>
    #[serde(rename = "DateCreated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// <p>The date when this environment was last modified.</p>
    #[serde(rename = "DateUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// <p>A description of the environment.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>For load-balanced, autoscaling environments, the URL to the load balancer. For single-instance environments, the IP address of the instance.</p>
    #[serde(rename = "EndpointUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_url: Option<String>,
    /// <p>The ARN of the environment.</p>
    #[serde(rename = "EnvironmentArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_arn: Option<String>,
    /// <p>The identifier of the environment.</p>
    #[serde(rename = "EnvironmentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_id: Option<String>,
    /// <p>Links to other environments in the same group.</p>
    #[serde(rename = "EnvironmentLinks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_links: Option<Vec<AwsElasticBeanstalkEnvironmentEnvironmentLink>>,
    /// <p>The name of the environment.</p>
    #[serde(rename = "EnvironmentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_name: Option<String>,
    /// <p>The configuration setting for the environment.</p>
    #[serde(rename = "OptionSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_settings: Option<Vec<AwsElasticBeanstalkEnvironmentOptionSetting>>,
    /// <p>The ARN of the platform version for the environment.</p>
    #[serde(rename = "PlatformArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_arn: Option<String>,
    /// <p>The name of the solution stack that is deployed with the environment.</p>
    #[serde(rename = "SolutionStackName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_stack_name: Option<String>,
    /// <p>The current operational status of the environment.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The tier of the environment.</p>
    #[serde(rename = "Tier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<AwsElasticBeanstalkEnvironmentTier>,
    /// <p>The application version of the environment.</p>
    #[serde(rename = "VersionLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_label: Option<String>,
}

/// <p>Contains information about a link to another environment that is in the same group.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsElasticBeanstalkEnvironmentEnvironmentLink {
    /// <p>The name of the linked environment.</p>
    #[serde(rename = "EnvironmentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_name: Option<String>,
    /// <p>The name of the environment link.</p>
    #[serde(rename = "LinkName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_name: Option<String>,
}

/// <p>A configuration option setting for the environment.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsElasticBeanstalkEnvironmentOptionSetting {
    /// <p>The type of resource that the configuration option is associated with.</p>
    #[serde(rename = "Namespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// <p>The name of the option.</p>
    #[serde(rename = "OptionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_name: Option<String>,
    /// <p>The name of the resource.</p>
    #[serde(rename = "ResourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    /// <p>The value of the configuration setting.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>Contains information about the tier of the environment.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsElasticBeanstalkEnvironmentTier {
    /// <p>The name of the environment tier.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The type of environment tier.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The version of the environment tier.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>Information about an Elasticsearch domain.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsElasticsearchDomainDetails {
    /// <p>IAM policy document specifying the access policies for the new Amazon ES domain.</p>
    #[serde(rename = "AccessPolicies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_policies: Option<String>,
    /// <p>Additional options for the domain endpoint.</p>
    #[serde(rename = "DomainEndpointOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_endpoint_options: Option<AwsElasticsearchDomainDomainEndpointOptions>,
    /// <p>Unique identifier for an Amazon ES domain.</p>
    #[serde(rename = "DomainId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_id: Option<String>,
    /// <p>Name of an Amazon ES domain.</p> <p>Domain names are unique across all domains owned by the same account within an AWS Region.</p> <p>Domain names must start with a lowercase letter and must be between 3 and 28 characters.</p> <p>Valid characters are a-z (lowercase only), 0-9, and – (hyphen). </p>
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// <p>Elasticsearch version.</p>
    #[serde(rename = "ElasticsearchVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elasticsearch_version: Option<String>,
    /// <p>Details about the configuration for encryption at rest.</p>
    #[serde(rename = "EncryptionAtRestOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_at_rest_options: Option<AwsElasticsearchDomainEncryptionAtRestOptions>,
    /// <p>Domain-specific endpoint used to submit index, search, and data upload requests to an Amazon ES domain.</p> <p>The endpoint is a service URL. </p>
    #[serde(rename = "Endpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    /// <p>The key-value pair that exists if the Amazon ES domain uses VPC endpoints.</p>
    #[serde(rename = "Endpoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<::std::collections::HashMap<String, String>>,
    /// <p>Configures the CloudWatch Logs to publish for the Elasticsearch domain.</p>
    #[serde(rename = "LogPublishingOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_publishing_options: Option<AwsElasticsearchDomainLogPublishingOptions>,
    /// <p>Details about the configuration for node-to-node encryption.</p>
    #[serde(rename = "NodeToNodeEncryptionOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_to_node_encryption_options: Option<AwsElasticsearchDomainNodeToNodeEncryptionOptions>,
    /// <p>Information about the status of a domain relative to the latest service software.</p>
    #[serde(rename = "ServiceSoftwareOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_software_options: Option<AwsElasticsearchDomainServiceSoftwareOptions>,
    /// <p>Information that Amazon ES derives based on <code>VPCOptions</code> for the domain.</p>
    #[serde(rename = "VPCOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_options: Option<AwsElasticsearchDomainVPCOptions>,
}

/// <p>Additional options for the domain endpoint, such as whether to require HTTPS for all traffic.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsElasticsearchDomainDomainEndpointOptions {
    /// <p>Whether to require that all traffic to the domain arrive over HTTPS.</p>
    #[serde(rename = "EnforceHTTPS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforce_https: Option<bool>,
    /// <p><p>The TLS security policy to apply to the HTTPS endpoint of the Elasticsearch domain.</p> <p>Valid values:</p> <ul> <li> <p> <code>Policy-Min-TLS-1-0-2019-07</code>, which supports TLSv1.0 and higher</p> </li> <li> <p> <code>Policy-Min-TLS-1-2-2019-07</code>, which only supports TLSv1.2</p> </li> </ul></p>
    #[serde(rename = "TLSSecurityPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_security_policy: Option<String>,
}

/// <p>Details about the configuration for encryption at rest.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsElasticsearchDomainEncryptionAtRestOptions {
    /// <p>Whether encryption at rest is enabled.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>The KMS key ID. Takes the form 1a2a3a4-1a2a-3a4a-5a6a-1a2a3a4a5a6a.</p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
}

/// <p>configures the CloudWatch Logs to publish for the Elasticsearch domain.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsElasticsearchDomainLogPublishingOptions {
    /// <p>Configures the Elasticsearch index logs publishing.</p>
    #[serde(rename = "IndexSlowLogs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_slow_logs: Option<AwsElasticsearchDomainLogPublishingOptionsLogConfig>,
    /// <p>Configures the Elasticsearch search slow log publishing.</p>
    #[serde(rename = "SearchSlowLogs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_slow_logs: Option<AwsElasticsearchDomainLogPublishingOptionsLogConfig>,
}

/// <p>The log configuration.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsElasticsearchDomainLogPublishingOptionsLogConfig {
    /// <p>The ARN of the CloudWatch Logs group to publish the logs to.</p>
    #[serde(rename = "CloudWatchLogsLogGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_log_group_arn: Option<String>,
    /// <p>Whether the log publishing is enabled.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// <p>Details about the configuration for node-to-node encryption.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsElasticsearchDomainNodeToNodeEncryptionOptions {
    /// <p>Whether node-to-node encryption is enabled.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// <p>Information about the state of the domain relative to the latest service software.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsElasticsearchDomainServiceSoftwareOptions {
    /// <p>The epoch time when the deployment window closes for required updates. After this time, Amazon Elasticsearch Service schedules the software upgrade automatically.</p>
    #[serde(rename = "AutomatedUpdateDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automated_update_date: Option<String>,
    /// <p>Whether a request to update the domain can be canceled.</p>
    #[serde(rename = "Cancellable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellable: Option<bool>,
    /// <p>The version of the service software that is currently installed on the domain.</p>
    #[serde(rename = "CurrentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_version: Option<String>,
    /// <p>A more detailed description of the service software status.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The most recent version of the service software.</p>
    #[serde(rename = "NewVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_version: Option<String>,
    /// <p>Whether a service software update is available for the domain.</p>
    #[serde(rename = "UpdateAvailable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_available: Option<bool>,
    /// <p>The status of the service software update.</p>
    #[serde(rename = "UpdateStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_status: Option<String>,
}

/// <p>Information that Amazon ES derives based on <code>VPCOptions</code> for the domain.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsElasticsearchDomainVPCOptions {
    /// <p>The list of Availability Zones associated with the VPC subnets.</p>
    #[serde(rename = "AvailabilityZones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<Vec<String>>,
    /// <p>The list of security group IDs associated with the VPC endpoints for the domain.</p>
    #[serde(rename = "SecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    /// <p>A list of subnet IDs associated with the VPC endpoints for the domain.</p>
    #[serde(rename = "SubnetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    /// <p>ID for the VPC.</p>
    #[serde(rename = "VPCId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

/// <p>Contains information about a stickiness policy that was created using <code>CreateAppCookieStickinessPolicy</code>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsElbAppCookieStickinessPolicy {
    /// <p>The name of the application cookie used for stickiness.</p>
    #[serde(rename = "CookieName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie_name: Option<String>,
    /// <p>The mnemonic name for the policy being created. The name must be unique within the set of policies for the load balancer.</p>
    #[serde(rename = "PolicyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
}

/// <p>Contains information about a stickiness policy that was created using <code>CreateLBCookieStickinessPolicy</code>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsElbLbCookieStickinessPolicy {
    /// <p>The amount of time, in seconds, after which the cookie is considered stale. If an expiration period is not specified, the stickiness session lasts for the duration of the browser session.</p>
    #[serde(rename = "CookieExpirationPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie_expiration_period: Option<i64>,
    /// <p>The name of the policy. The name must be unique within the set of policies for the load balancer.</p>
    #[serde(rename = "PolicyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
}

/// <p>Contains information about the access log configuration for the load balancer.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsElbLoadBalancerAccessLog {
    /// <p>The interval in minutes for publishing the access logs.</p> <p>You can publish access logs either every 5 minutes or every 60 minutes.</p>
    #[serde(rename = "EmitInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emit_interval: Option<i64>,
    /// <p>Indicates whether access logs are enabled for the load balancer.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>The name of the S3 bucket where the access logs are stored.</p>
    #[serde(rename = "S3BucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_name: Option<String>,
    /// <p>The logical hierarchy that was created for the S3 bucket.</p> <p>If a prefix is not provided, the log is placed at the root level of the bucket.</p>
    #[serde(rename = "S3BucketPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_prefix: Option<String>,
}

/// <p>Contains attributes for the load balancer.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsElbLoadBalancerAttributes {
    /// <p>Information about the access log configuration for the load balancer.</p> <p>If the access log is enabled, the load balancer captures detailed information about all requests. It delivers the information to a specified S3 bucket.</p>
    #[serde(rename = "AccessLog")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_log: Option<AwsElbLoadBalancerAccessLog>,
    /// <p>Information about the connection draining configuration for the load balancer.</p> <p>If connection draining is enabled, the load balancer allows existing requests to complete before it shifts traffic away from a deregistered or unhealthy instance.</p>
    #[serde(rename = "ConnectionDraining")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_draining: Option<AwsElbLoadBalancerConnectionDraining>,
    /// <p>Connection settings for the load balancer.</p> <p>If an idle timeout is configured, the load balancer allows connections to remain idle for the specified duration. When a connection is idle, no data is sent over the connection.</p>
    #[serde(rename = "ConnectionSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_settings: Option<AwsElbLoadBalancerConnectionSettings>,
    /// <p>Cross-zone load balancing settings for the load balancer.</p> <p>If cross-zone load balancing is enabled, the load balancer routes the request traffic evenly across all instances regardless of the Availability Zones.</p>
    #[serde(rename = "CrossZoneLoadBalancing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_zone_load_balancing: Option<AwsElbLoadBalancerCrossZoneLoadBalancing>,
}

/// <p>Provides information about the configuration of an EC2 instance for the load balancer.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsElbLoadBalancerBackendServerDescription {
    /// <p>The port on which the EC2 instance is listening.</p>
    #[serde(rename = "InstancePort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_port: Option<i64>,
    /// <p>The names of the policies that are enabled for the EC2 instance.</p>
    #[serde(rename = "PolicyNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_names: Option<Vec<String>>,
}

/// <p>Contains information about the connection draining configuration for the load balancer.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsElbLoadBalancerConnectionDraining {
    /// <p>Indicates whether connection draining is enabled for the load balancer.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>The maximum time, in seconds, to keep the existing connections open before deregistering the instances.</p>
    #[serde(rename = "Timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
}

/// <p>Contains connection settings for the load balancer.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsElbLoadBalancerConnectionSettings {
    /// <p>The time, in seconds, that the connection can be idle (no data is sent over the connection) before it is closed by the load balancer.</p>
    #[serde(rename = "IdleTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_timeout: Option<i64>,
}

/// <p>Contains cross-zone load balancing settings for the load balancer.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsElbLoadBalancerCrossZoneLoadBalancing {
    /// <p>Indicates whether cross-zone load balancing is enabled for the load balancer.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// <p>Contains details about a Classic Load Balancer.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsElbLoadBalancerDetails {
    /// <p>The list of Availability Zones for the load balancer.</p>
    #[serde(rename = "AvailabilityZones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<Vec<String>>,
    /// <p>Information about the configuration of the EC2 instances.</p>
    #[serde(rename = "BackendServerDescriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_server_descriptions: Option<Vec<AwsElbLoadBalancerBackendServerDescription>>,
    /// <p>The name of the Amazon Route 53 hosted zone for the load balancer.</p>
    #[serde(rename = "CanonicalHostedZoneName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canonical_hosted_zone_name: Option<String>,
    /// <p>The ID of the Amazon Route 53 hosted zone for the load balancer.</p>
    #[serde(rename = "CanonicalHostedZoneNameID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canonical_hosted_zone_name_id: Option<String>,
    /// <p>Indicates when the load balancer was created.</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    /// <p>The DNS name of the load balancer.</p>
    #[serde(rename = "DnsName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_name: Option<String>,
    /// <p>Information about the health checks that are conducted on the load balancer.</p>
    #[serde(rename = "HealthCheck")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check: Option<AwsElbLoadBalancerHealthCheck>,
    /// <p>List of EC2 instances for the load balancer.</p>
    #[serde(rename = "Instances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<Vec<AwsElbLoadBalancerInstance>>,
    /// <p>The policies that are enabled for the load balancer listeners.</p>
    #[serde(rename = "ListenerDescriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener_descriptions: Option<Vec<AwsElbLoadBalancerListenerDescription>>,
    /// <p>The attributes for a load balancer.</p>
    #[serde(rename = "LoadBalancerAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_attributes: Option<AwsElbLoadBalancerAttributes>,
    /// <p>The name of the load balancer.</p>
    #[serde(rename = "LoadBalancerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_name: Option<String>,
    /// <p>The policies for a load balancer.</p>
    #[serde(rename = "Policies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<AwsElbLoadBalancerPolicies>,
    /// <p>The type of load balancer. Only provided if the load balancer is in a VPC.</p> <p>If <code>Scheme</code> is <code>internet-facing</code>, the load balancer has a public DNS name that resolves to a public IP address.</p> <p>If <code>Scheme</code> is <code>internal</code>, the load balancer has a public DNS name that resolves to a private IP address.</p>
    #[serde(rename = "Scheme")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
    /// <p>The security groups for the load balancer. Only provided if the load balancer is in a VPC.</p>
    #[serde(rename = "SecurityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,
    /// <p>Information about the security group for the load balancer. This is the security group that is used for inbound rules.</p>
    #[serde(rename = "SourceSecurityGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_security_group: Option<AwsElbLoadBalancerSourceSecurityGroup>,
    /// <p>The list of subnet identifiers for the load balancer.</p>
    #[serde(rename = "Subnets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnets: Option<Vec<String>>,
    /// <p>The identifier of the VPC for the load balancer.</p>
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

/// <p>Contains information about the health checks that are conducted on the load balancer.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsElbLoadBalancerHealthCheck {
    /// <p>The number of consecutive health check successes required before the instance is moved to the Healthy state.</p>
    #[serde(rename = "HealthyThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub healthy_threshold: Option<i64>,
    /// <p>The approximate interval, in seconds, between health checks of an individual instance.</p>
    #[serde(rename = "Interval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<i64>,
    /// <p>The instance that is being checked. The target specifies the protocol and port. The available protocols are TCP, SSL, HTTP, and HTTPS. The range of valid ports is 1 through 65535.</p> <p>For the HTTP and HTTPS protocols, the target also specifies the ping path.</p> <p>For the TCP protocol, the target is specified as <code>TCP: <i>&lt;port&gt;</i> </code>.</p> <p>For the SSL protocol, the target is specified as <code>SSL.<i>&lt;port&gt;</i> </code>.</p> <p>For the HTTP and HTTPS protocols, the target is specified as <code> <i>&lt;protocol&gt;</i>:<i>&lt;port&gt;</i>/<i>&lt;path to ping&gt;</i> </code>.</p>
    #[serde(rename = "Target")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    /// <p>The amount of time, in seconds, during which no response means a failed health check.</p>
    #[serde(rename = "Timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
    /// <p>The number of consecutive health check failures that must occur before the instance is moved to the Unhealthy state.</p>
    #[serde(rename = "UnhealthyThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unhealthy_threshold: Option<i64>,
}

/// <p>Provides information about an EC2 instance for a load balancer.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsElbLoadBalancerInstance {
    /// <p>The instance identifier.</p>
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
}

/// <p>Information about a load balancer listener.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsElbLoadBalancerListener {
    /// <p>The port on which the instance is listening.</p>
    #[serde(rename = "InstancePort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_port: Option<i64>,
    /// <p>The protocol to use to route traffic to instances.</p> <p>Valid values: <code>HTTP</code> | <code>HTTPS</code> | <code>TCP</code> | <code>SSL</code> </p>
    #[serde(rename = "InstanceProtocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_protocol: Option<String>,
    /// <p>The port on which the load balancer is listening.</p> <p>On EC2-VPC, you can specify any port from the range 1-65535.</p> <p>On EC2-Classic, you can specify any port from the following list: 25, 80, 443, 465, 587, 1024-65535.</p>
    #[serde(rename = "LoadBalancerPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_port: Option<i64>,
    /// <p>The load balancer transport protocol to use for routing.</p> <p>Valid values: <code>HTTP</code> | <code>HTTPS</code> | <code>TCP</code> | <code>SSL</code> </p>
    #[serde(rename = "Protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// <p>The ARN of the server certificate.</p>
    #[serde(rename = "SslCertificateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_certificate_id: Option<String>,
}

/// <p>Lists the policies that are enabled for a load balancer listener.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsElbLoadBalancerListenerDescription {
    /// <p>Information about the listener.</p>
    #[serde(rename = "Listener")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener: Option<AwsElbLoadBalancerListener>,
    /// <p>The policies enabled for the listener.</p>
    #[serde(rename = "PolicyNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_names: Option<Vec<String>>,
}

/// <p>Contains information about the policies for a load balancer.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsElbLoadBalancerPolicies {
    /// <p>The stickiness policies that are created using <code>CreateAppCookieStickinessPolicy</code>.</p>
    #[serde(rename = "AppCookieStickinessPolicies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_cookie_stickiness_policies: Option<Vec<AwsElbAppCookieStickinessPolicy>>,
    /// <p>The stickiness policies that are created using <code>CreateLBCookieStickinessPolicy</code>.</p>
    #[serde(rename = "LbCookieStickinessPolicies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lb_cookie_stickiness_policies: Option<Vec<AwsElbLbCookieStickinessPolicy>>,
    /// <p>The policies other than the stickiness policies.</p>
    #[serde(rename = "OtherPolicies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_policies: Option<Vec<String>>,
}

/// <p>Contains information about the security group for the load balancer.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsElbLoadBalancerSourceSecurityGroup {
    /// <p>The name of the security group.</p>
    #[serde(rename = "GroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// <p>The owner of the security group.</p>
    #[serde(rename = "OwnerAlias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_alias: Option<String>,
}

/// <p>Information about a load balancer.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsElbv2LoadBalancerDetails {
    /// <p>The Availability Zones for the load balancer.</p>
    #[serde(rename = "AvailabilityZones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<Vec<AvailabilityZone>>,
    /// <p>The ID of the Amazon Route 53 hosted zone associated with the load balancer.</p>
    #[serde(rename = "CanonicalHostedZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canonical_hosted_zone_id: Option<String>,
    /// <p>Indicates when the load balancer was created.</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    /// <p>The public DNS name of the load balancer.</p>
    #[serde(rename = "DNSName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_name: Option<String>,
    /// <p>The type of IP addresses used by the subnets for your load balancer. The possible values are <code>ipv4</code> (for IPv4 addresses) and <code>dualstack</code> (for IPv4 and IPv6 addresses).</p>
    #[serde(rename = "IpAddressType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_type: Option<String>,
    /// <p>The nodes of an Internet-facing load balancer have public IP addresses.</p>
    #[serde(rename = "Scheme")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
    /// <p>The IDs of the security groups for the load balancer.</p>
    #[serde(rename = "SecurityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,
    /// <p>The state of the load balancer.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<LoadBalancerState>,
    /// <p>The type of load balancer.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The ID of the VPC for the load balancer.</p>
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

/// <p>IAM access key details related to a finding.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsIamAccessKeyDetails {
    /// <p>The identifier of the access key.</p>
    #[serde(rename = "AccessKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,
    /// <p>The AWS account ID of the account for the key.</p>
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>Indicates when the IAM access key was created.</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// <p>The ID of the principal associated with an access key.</p>
    #[serde(rename = "PrincipalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    /// <p>The name of the principal.</p>
    #[serde(rename = "PrincipalName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_name: Option<String>,
    /// <p>The type of principal associated with an access key.</p>
    #[serde(rename = "PrincipalType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_type: Option<String>,
    /// <p>Information about the session that the key was used for.</p>
    #[serde(rename = "SessionContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_context: Option<AwsIamAccessKeySessionContext>,
    /// <p>The status of the IAM access key related to a finding.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Provides information about the session that the key was used for.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsIamAccessKeySessionContext {
    /// <p>Attributes of the session that the key was used for.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<AwsIamAccessKeySessionContextAttributes>,
    /// <p>Information about the entity that created the session.</p>
    #[serde(rename = "SessionIssuer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_issuer: Option<AwsIamAccessKeySessionContextSessionIssuer>,
}

/// <p>Attributes of the session that the key was used for.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsIamAccessKeySessionContextAttributes {
    /// <p>Indicates when the session was created.</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// <p>Indicates whether the session used multi-factor authentication (MFA).</p>
    #[serde(rename = "MfaAuthenticated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_authenticated: Option<bool>,
}

/// <p>Information about the entity that created the session.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsIamAccessKeySessionContextSessionIssuer {
    /// <p>The identifier of the AWS account that created the session.</p>
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>The ARN of the session.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The principal ID of the principal (user, role, or group) that created the session.</p>
    #[serde(rename = "PrincipalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    /// <p>The type of principal (user, role, or group) that created the session.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The name of the principal that created the session.</p>
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

/// <p>A managed policy that is attached to an IAM principal.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsIamAttachedManagedPolicy {
    /// <p>The ARN of the policy.</p>
    #[serde(rename = "PolicyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arn: Option<String>,
    /// <p>The name of the policy.</p>
    #[serde(rename = "PolicyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
}

/// <p>Contains details about an IAM group.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsIamGroupDetails {
    /// <p>A list of the managed policies that are attached to the IAM group.</p>
    #[serde(rename = "AttachedManagedPolicies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_managed_policies: Option<Vec<AwsIamAttachedManagedPolicy>>,
    /// <p>Indicates when the IAM group was created.</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "CreateDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    /// <p>The identifier of the IAM group.</p>
    #[serde(rename = "GroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// <p>The name of the IAM group.</p>
    #[serde(rename = "GroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// <p>The list of inline policies that are embedded in the group.</p>
    #[serde(rename = "GroupPolicyList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_policy_list: Option<Vec<AwsIamGroupPolicy>>,
    /// <p>The path to the group.</p>
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

/// <p>A managed policy that is attached to the IAM group.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsIamGroupPolicy {
    /// <p>The name of the policy.</p>
    #[serde(rename = "PolicyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
}

/// <p>Information about an instance profile.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsIamInstanceProfile {
    /// <p>The ARN of the instance profile.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>Indicates when the instance profile was created.</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "CreateDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    /// <p>The identifier of the instance profile.</p>
    #[serde(rename = "InstanceProfileId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_profile_id: Option<String>,
    /// <p>The name of the instance profile.</p>
    #[serde(rename = "InstanceProfileName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_profile_name: Option<String>,
    /// <p>The path to the instance profile.</p>
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// <p>The roles associated with the instance profile.</p>
    #[serde(rename = "Roles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<AwsIamInstanceProfileRole>>,
}

/// <p>Information about a role associated with an instance profile.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsIamInstanceProfileRole {
    /// <p>The ARN of the role.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The policy that grants an entity permission to assume the role.</p>
    #[serde(rename = "AssumeRolePolicyDocument")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assume_role_policy_document: Option<String>,
    /// <p>Indicates when the role was created.</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "CreateDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    /// <p>The path to the role.</p>
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// <p>The identifier of the role.</p>
    #[serde(rename = "RoleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_id: Option<String>,
    /// <p>The name of the role.</p>
    #[serde(rename = "RoleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
}

/// <p>Information about the policy used to set the permissions boundary for an IAM principal.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsIamPermissionsBoundary {
    /// <p>The ARN of the policy used to set the permissions boundary.</p>
    #[serde(rename = "PermissionsBoundaryArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions_boundary_arn: Option<String>,
    /// <p>The usage type for the permissions boundary.</p>
    #[serde(rename = "PermissionsBoundaryType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions_boundary_type: Option<String>,
}

/// <p>Represents an IAM permissions policy.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsIamPolicyDetails {
    /// <p>The number of users, groups, and roles that the policy is attached to.</p>
    #[serde(rename = "AttachmentCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_count: Option<i64>,
    /// <p>When the policy was created.</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "CreateDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    /// <p>The identifier of the default version of the policy.</p>
    #[serde(rename = "DefaultVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_version_id: Option<String>,
    /// <p>A description of the policy.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Whether the policy can be attached to a user, group, or role.</p>
    #[serde(rename = "IsAttachable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_attachable: Option<bool>,
    /// <p>The path to the policy.</p>
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// <p>The number of users and roles that use the policy to set the permissions boundary.</p>
    #[serde(rename = "PermissionsBoundaryUsageCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions_boundary_usage_count: Option<i64>,
    /// <p>The unique identifier of the policy.</p>
    #[serde(rename = "PolicyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    /// <p>The name of the policy.</p>
    #[serde(rename = "PolicyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    /// <p>List of versions of the policy.</p>
    #[serde(rename = "PolicyVersionList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_version_list: Option<Vec<AwsIamPolicyVersion>>,
    /// <p>When the policy was most recently updated.</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "UpdateDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_date: Option<String>,
}

/// <p>A version of an IAM policy.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsIamPolicyVersion {
    /// <p>Indicates when the version was created.</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "CreateDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    /// <p>Whether the version is the default version.</p>
    #[serde(rename = "IsDefaultVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default_version: Option<bool>,
    /// <p>The identifier of the policy version.</p>
    #[serde(rename = "VersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

/// <p>Contains information about an IAM role, including all of the role's policies.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsIamRoleDetails {
    /// <p>The trust policy that grants permission to assume the role.</p>
    #[serde(rename = "AssumeRolePolicyDocument")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assume_role_policy_document: Option<String>,
    /// <p>The list of the managed policies that are attached to the role.</p>
    #[serde(rename = "AttachedManagedPolicies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_managed_policies: Option<Vec<AwsIamAttachedManagedPolicy>>,
    /// <p>Indicates when the role was created.</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "CreateDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    /// <p>The list of instance profiles that contain this role.</p>
    #[serde(rename = "InstanceProfileList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_profile_list: Option<Vec<AwsIamInstanceProfile>>,
    /// <p>The maximum session duration (in seconds) that you want to set for the specified role.</p>
    #[serde(rename = "MaxSessionDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_session_duration: Option<i64>,
    /// <p>The path to the role.</p>
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "PermissionsBoundary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions_boundary: Option<AwsIamPermissionsBoundary>,
    /// <p>The stable and unique string identifying the role.</p>
    #[serde(rename = "RoleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_id: Option<String>,
    /// <p>The friendly name that identifies the role.</p>
    #[serde(rename = "RoleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    /// <p>The list of inline policies that are embedded in the role.</p>
    #[serde(rename = "RolePolicyList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_policy_list: Option<Vec<AwsIamRolePolicy>>,
}

/// <p>An inline policy that is embedded in the role.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsIamRolePolicy {
    /// <p>The name of the policy.</p>
    #[serde(rename = "PolicyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
}

/// <p>Information about an IAM user.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsIamUserDetails {
    /// <p>A list of the managed policies that are attached to the user.</p>
    #[serde(rename = "AttachedManagedPolicies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_managed_policies: Option<Vec<AwsIamAttachedManagedPolicy>>,
    /// <p>Indicates when the user was created.</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "CreateDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    /// <p>A list of IAM groups that the user belongs to.</p>
    #[serde(rename = "GroupList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_list: Option<Vec<String>>,
    /// <p>The path to the user.</p>
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// <p>The permissions boundary for the user.</p>
    #[serde(rename = "PermissionsBoundary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions_boundary: Option<AwsIamPermissionsBoundary>,
    /// <p>The unique identifier for the user.</p>
    #[serde(rename = "UserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// <p>The name of the user.</p>
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    /// <p>The list of inline policies that are embedded in the user.</p>
    #[serde(rename = "UserPolicyList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_policy_list: Option<Vec<AwsIamUserPolicy>>,
}

/// <p>Information about an inline policy that is embedded in the user.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsIamUserPolicy {
    /// <p>The name of the policy.</p>
    #[serde(rename = "PolicyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
}

/// <p>Contains metadata about a customer master key (CMK).</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsKmsKeyDetails {
    /// <p>The twelve-digit account ID of the AWS account that owns the CMK.</p>
    #[serde(rename = "AWSAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_account_id: Option<String>,
    /// <p>Indicates when the CMK was created.</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>A description of the key.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The globally unique identifier for the CMK.</p>
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// <p>The manager of the CMK. CMKs in your AWS account are either customer managed or AWS managed.</p>
    #[serde(rename = "KeyManager")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_manager: Option<String>,
    /// <p>The state of the CMK.</p>
    #[serde(rename = "KeyState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_state: Option<String>,
    /// <p>The source of the CMK's key material.</p> <p>When this value is <code>AWS_KMS</code>, AWS KMS created the key material.</p> <p>When this value is <code>EXTERNAL</code>, the key material was imported from your existing key management infrastructure or the CMK lacks key material.</p> <p>When this value is <code>AWS_CLOUDHSM</code>, the key material was created in the AWS CloudHSM cluster associated with a custom key store.</p>
    #[serde(rename = "Origin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}

/// <p>The code for the Lambda function. You can specify either an object in Amazon S3, or upload a deployment package directly.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsLambdaFunctionCode {
    /// <p>An Amazon S3 bucket in the same AWS Region as your function. The bucket can be in a different AWS account.</p>
    #[serde(rename = "S3Bucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket: Option<String>,
    /// <p>The Amazon S3 key of the deployment package.</p>
    #[serde(rename = "S3Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key: Option<String>,
    /// <p>For versioned objects, the version of the deployment package object to use.</p>
    #[serde(rename = "S3ObjectVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_object_version: Option<String>,
    /// <p>The base64-encoded contents of the deployment package. AWS SDK and AWS CLI clients handle the encoding for you.</p>
    #[serde(rename = "ZipFile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_file: Option<String>,
}

/// <p>The dead-letter queue for failed asynchronous invocations.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsLambdaFunctionDeadLetterConfig {
    /// <p>The ARN of an Amazon SQS queue or Amazon SNS topic.</p>
    #[serde(rename = "TargetArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arn: Option<String>,
}

/// <p>Details about a function's configuration.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsLambdaFunctionDetails {
    /// <p>An <code>AwsLambdaFunctionCode</code> object.</p>
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<AwsLambdaFunctionCode>,
    /// <p>The SHA256 hash of the function's deployment package.</p>
    #[serde(rename = "CodeSha256")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_sha_256: Option<String>,
    /// <p>The function's dead letter queue.</p>
    #[serde(rename = "DeadLetterConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dead_letter_config: Option<AwsLambdaFunctionDeadLetterConfig>,
    /// <p>The function's environment variables.</p>
    #[serde(rename = "Environment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<AwsLambdaFunctionEnvironment>,
    /// <p>The name of the function.</p>
    #[serde(rename = "FunctionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_name: Option<String>,
    /// <p>The function that Lambda calls to begin executing your function.</p>
    #[serde(rename = "Handler")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handler: Option<String>,
    /// <p>The KMS key that's used to encrypt the function's environment variables. This key is only returned if you've configured a customer managed CMK.</p>
    #[serde(rename = "KmsKeyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    /// <p>Indicates when the function was last updated.</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "LastModified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    /// <p>The function's layers.</p>
    #[serde(rename = "Layers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layers: Option<Vec<AwsLambdaFunctionLayer>>,
    /// <p>For Lambda@Edge functions, the ARN of the master function.</p>
    #[serde(rename = "MasterArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_arn: Option<String>,
    /// <p>The memory that is allocated to the function.</p>
    #[serde(rename = "MemorySize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_size: Option<i64>,
    /// <p>The latest updated revision of the function or alias.</p>
    #[serde(rename = "RevisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    /// <p>The function's execution role.</p>
    #[serde(rename = "Role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// <p>The runtime environment for the Lambda function.</p>
    #[serde(rename = "Runtime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<String>,
    /// <p>The amount of time that Lambda allows a function to run before stopping it.</p>
    #[serde(rename = "Timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
    /// <p>The function's AWS X-Ray tracing configuration.</p>
    #[serde(rename = "TracingConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracing_config: Option<AwsLambdaFunctionTracingConfig>,
    /// <p>The version of the Lambda function.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// <p>The function's networking configuration.</p>
    #[serde(rename = "VpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<AwsLambdaFunctionVpcConfig>,
}

/// <p>A function's environment variable settings.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsLambdaFunctionEnvironment {
    /// <p>An <code>AwsLambdaFunctionEnvironmentError</code> object.</p>
    #[serde(rename = "Error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<AwsLambdaFunctionEnvironmentError>,
    /// <p>Environment variable key-value pairs.</p>
    #[serde(rename = "Variables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Error messages for environment variables that could not be applied.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsLambdaFunctionEnvironmentError {
    /// <p>The error code.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>The error message.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// <p>An AWS Lambda layer.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsLambdaFunctionLayer {
    /// <p>The ARN of the function layer.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The size of the layer archive in bytes.</p>
    #[serde(rename = "CodeSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_size: Option<i64>,
}

/// <p>The function's AWS X-Ray tracing configuration.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsLambdaFunctionTracingConfig {
    /// <p>The tracing mode.</p>
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
}

/// <p>The VPC security groups and subnets that are attached to a Lambda function.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsLambdaFunctionVpcConfig {
    /// <p>A list of VPC security groups IDs.</p>
    #[serde(rename = "SecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    /// <p>A list of VPC subnet IDs.</p>
    #[serde(rename = "SubnetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    /// <p>The ID of the VPC.</p>
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

/// <p>Details about a Lambda layer version.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsLambdaLayerVersionDetails {
    /// <p>The layer's compatible runtimes. Maximum number of five items.</p> <p>Valid values: <code>nodejs10.x</code> | <code>nodejs12.x</code> | <code>java8</code> | <code>java11</code> | <code>python2.7</code> | <code>python3.6</code> | <code>python3.7</code> | <code>python3.8</code> | <code>dotnetcore1.0</code> | <code>dotnetcore2.1</code> | <code>go1.x</code> | <code>ruby2.5</code> | <code>provided</code> </p>
    #[serde(rename = "CompatibleRuntimes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatible_runtimes: Option<Vec<String>>,
    /// <p>Indicates when the version was created.</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    /// <p>The version number.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>An IAM role that is associated with the Amazon RDS DB cluster.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsRdsDbClusterAssociatedRole {
    /// <p>The ARN of the IAM role.</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>The status of the association between the IAM role and the DB cluster.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Information about an Amazon RDS DB cluster.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsRdsDbClusterDetails {
    /// <p>The status of the database activity stream.</p>
    #[serde(rename = "ActivityStreamStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_stream_status: Option<String>,
    /// <p>For all database engines except Aurora, specifies the allocated storage size in gibibytes (GiB).</p>
    #[serde(rename = "AllocatedStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i64>,
    /// <p>A list of the IAM roles that are associated with the DB cluster.</p>
    #[serde(rename = "AssociatedRoles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_roles: Option<Vec<AwsRdsDbClusterAssociatedRole>>,
    /// <p>A list of Availability Zones (AZs) where instances in the DB cluster can be created.</p>
    #[serde(rename = "AvailabilityZones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<Vec<String>>,
    /// <p>The number of days for which automated backups are retained.</p>
    #[serde(rename = "BackupRetentionPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_retention_period: Option<i64>,
    /// <p>Indicates when the DB cluster was created, in Universal Coordinated Time (UTC).</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "ClusterCreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_create_time: Option<String>,
    /// <p>Whether tags are copied from the DB cluster to snapshots of the DB cluster.</p>
    #[serde(rename = "CopyTagsToSnapshot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_tags_to_snapshot: Option<bool>,
    /// <p>Whether the DB cluster is a clone of a DB cluster owned by a different AWS account.</p>
    #[serde(rename = "CrossAccountClone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_account_clone: Option<bool>,
    /// <p>A list of custom endpoints for the DB cluster.</p>
    #[serde(rename = "CustomEndpoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_endpoints: Option<Vec<String>>,
    /// <p>The name of the database.</p>
    #[serde(rename = "DatabaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    /// <p>The DB cluster identifier that the user assigned to the cluster. This identifier is the unique key that identifies a DB cluster.</p>
    #[serde(rename = "DbClusterIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_cluster_identifier: Option<String>,
    /// <p>The list of instances that make up the DB cluster.</p>
    #[serde(rename = "DbClusterMembers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_cluster_members: Option<Vec<AwsRdsDbClusterMember>>,
    /// <p>The list of option group memberships for this DB cluster.</p>
    #[serde(rename = "DbClusterOptionGroupMemberships")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_cluster_option_group_memberships: Option<Vec<AwsRdsDbClusterOptionGroupMembership>>,
    /// <p>The name of the DB cluster parameter group for the DB cluster.</p>
    #[serde(rename = "DbClusterParameterGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_cluster_parameter_group: Option<String>,
    /// <p>The identifier of the DB cluster. The identifier must be unique within each AWS Region and is immutable.</p>
    #[serde(rename = "DbClusterResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_cluster_resource_id: Option<String>,
    /// <p>The subnet group that is associated with the DB cluster, including the name, description, and subnets in the subnet group.</p>
    #[serde(rename = "DbSubnetGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_subnet_group: Option<String>,
    /// <p>Whether the DB cluster has deletion protection enabled.</p>
    #[serde(rename = "DeletionProtection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<bool>,
    /// <p>The Active Directory domain membership records that are associated with the DB cluster.</p>
    #[serde(rename = "DomainMemberships")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_memberships: Option<Vec<AwsRdsDbDomainMembership>>,
    /// <p>A list of log types that this DB cluster is configured to export to CloudWatch Logs.</p>
    #[serde(rename = "EnabledCloudWatchLogsExports")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_cloud_watch_logs_exports: Option<Vec<String>>,
    /// <p>The connection endpoint for the primary instance of the DB cluster.</p>
    #[serde(rename = "Endpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    /// <p>The name of the database engine to use for this DB cluster.</p>
    #[serde(rename = "Engine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    /// <p>The database engine mode of the DB cluster.</p>
    #[serde(rename = "EngineMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_mode: Option<String>,
    /// <p>The version number of the database engine to use.</p>
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// <p>Specifies the identifier that Amazon Route 53 assigns when you create a hosted zone.</p>
    #[serde(rename = "HostedZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_zone_id: Option<String>,
    /// <p>Whether the HTTP endpoint for an Aurora Serverless DB cluster is enabled.</p>
    #[serde(rename = "HttpEndpointEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_endpoint_enabled: Option<bool>,
    /// <p>Whether the mapping of IAM accounts to database accounts is enabled.</p>
    #[serde(rename = "IamDatabaseAuthenticationEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_database_authentication_enabled: Option<bool>,
    /// <p>The ARN of the AWS KMS master key that is used to encrypt the database instances in the DB cluster.</p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>The name of the master user for the DB cluster.</p>
    #[serde(rename = "MasterUsername")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_username: Option<String>,
    /// <p>Whether the DB cluster has instances in multiple Availability Zones.</p>
    #[serde(rename = "MultiAz")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_az: Option<bool>,
    /// <p>The port number on which the DB instances in the DB cluster accept connections.</p>
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    /// <p>The range of time each day when automated backups are created, if automated backups are enabled.</p> <p>Uses the format <code>HH:MM-HH:MM</code>. For example, <code>04:52-05:22</code>.</p>
    #[serde(rename = "PreferredBackupWindow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_backup_window: Option<String>,
    /// <p>The weekly time range during which system maintenance can occur, in Universal Coordinated Time (UTC).</p> <p>Uses the format <code>&lt;day&gt;:HH:MM-&lt;day&gt;:HH:MM</code>.</p> <p>For the day values, use <code>mon</code>|<code>tue</code>|<code>wed</code>|<code>thu</code>|<code>fri</code>|<code>sat</code>|<code>sun</code>.</p> <p>For example, <code>sun:09:32-sun:10:02</code>.</p>
    #[serde(rename = "PreferredMaintenanceWindow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    /// <p>The identifiers of the read replicas that are associated with this DB cluster.</p>
    #[serde(rename = "ReadReplicaIdentifiers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_replica_identifiers: Option<Vec<String>>,
    /// <p>The reader endpoint for the DB cluster.</p>
    #[serde(rename = "ReaderEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reader_endpoint: Option<String>,
    /// <p>The current status of this DB cluster.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>Whether the DB cluster is encrypted.</p>
    #[serde(rename = "StorageEncrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_encrypted: Option<bool>,
    /// <p>A list of VPC security groups that the DB cluster belongs to.</p>
    #[serde(rename = "VpcSecurityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_groups: Option<Vec<AwsRdsDbInstanceVpcSecurityGroup>>,
}

/// <p>Information about an instance in the DB cluster.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsRdsDbClusterMember {
    /// <p>The status of the DB cluster parameter group for this member of the DB cluster.</p>
    #[serde(rename = "DbClusterParameterGroupStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_cluster_parameter_group_status: Option<String>,
    /// <p>The instance identifier for this member of the DB cluster.</p>
    #[serde(rename = "DbInstanceIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_identifier: Option<String>,
    /// <p>Whether the cluster member is the primary instance for the DB cluster.</p>
    #[serde(rename = "IsClusterWriter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_cluster_writer: Option<bool>,
    /// <p>Specifies the order in which an Aurora replica is promoted to the primary instance when the existing primary instance fails.</p>
    #[serde(rename = "PromotionTier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_tier: Option<i64>,
}

/// <p>Information about an option group membership for a DB cluster.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsRdsDbClusterOptionGroupMembership {
    /// <p>The name of the DB cluster option group.</p>
    #[serde(rename = "DbClusterOptionGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_cluster_option_group_name: Option<String>,
    /// <p>The status of the DB cluster option group.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Information about an Amazon RDS DB cluster snapshot.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsRdsDbClusterSnapshotDetails {
    /// <p>Specifies the allocated storage size in gibibytes (GiB).</p>
    #[serde(rename = "AllocatedStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i64>,
    /// <p>A list of Availability Zones where instances in the DB cluster can be created.</p>
    #[serde(rename = "AvailabilityZones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<Vec<String>>,
    /// <p>Indicates when the DB cluster was created, in Universal Coordinated Time (UTC).</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "ClusterCreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_create_time: Option<String>,
    /// <p>The DB cluster identifier.</p>
    #[serde(rename = "DbClusterIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_cluster_identifier: Option<String>,
    /// <p>The identifier of the DB cluster snapshot.</p>
    #[serde(rename = "DbClusterSnapshotIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_cluster_snapshot_identifier: Option<String>,
    /// <p>The name of the database engine that you want to use for this DB instance.</p>
    #[serde(rename = "Engine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    /// <p>The version of the database engine to use.</p>
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// <p>Whether mapping of IAM accounts to database accounts is enabled.</p>
    #[serde(rename = "IamDatabaseAuthenticationEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_database_authentication_enabled: Option<bool>,
    /// <p>The ARN of the AWS KMS master key that is used to encrypt the database instances in the DB cluster.</p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>The license model information for this DB cluster snapshot.</p>
    #[serde(rename = "LicenseModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_model: Option<String>,
    /// <p>The name of the master user for the DB cluster.</p>
    #[serde(rename = "MasterUsername")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_username: Option<String>,
    /// <p>Specifies the percentage of the estimated data that has been transferred.</p>
    #[serde(rename = "PercentProgress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_progress: Option<i64>,
    /// <p>The port number on which the DB instances in the DB cluster accept connections.</p>
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    /// <p>Indicates when the snapshot was taken.</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "SnapshotCreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_create_time: Option<String>,
    /// <p>The type of DB cluster snapshot.</p>
    #[serde(rename = "SnapshotType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_type: Option<String>,
    /// <p>The status of this DB cluster snapshot.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>Whether the DB cluster is encrypted.</p>
    #[serde(rename = "StorageEncrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_encrypted: Option<bool>,
    /// <p>The VPC ID that is associated with the DB cluster snapshot.</p>
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

/// <p>Information about an Active Directory domain membership record associated with the DB instance.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsRdsDbDomainMembership {
    /// <p>The identifier of the Active Directory domain.</p>
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// <p>The fully qualified domain name of the Active Directory domain.</p>
    #[serde(rename = "Fqdn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    /// <p>The name of the IAM role to use when making API calls to the Directory Service.</p>
    #[serde(rename = "IamRoleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_name: Option<String>,
    /// <p>The status of the Active Directory Domain membership for the DB instance.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>An AWS Identity and Access Management (IAM) role associated with the DB instance.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsRdsDbInstanceAssociatedRole {
    /// <p>The name of the feature associated with the IAM)role.</p>
    #[serde(rename = "FeatureName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_name: Option<String>,
    /// <p>The ARN of the IAM role that is associated with the DB instance.</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p><p>Describes the state of the association between the IAM role and the DB instance. The <code>Status</code> property returns one of the following values:</p> <ul> <li> <p> <code>ACTIVE</code> - The IAM role ARN is associated with the DB instance and can be used to access other AWS services on your behalf.</p> </li> <li> <p> <code>PENDING</code> - The IAM role ARN is being associated with the DB instance.</p> </li> <li> <p> <code>INVALID</code> - The IAM role ARN is associated with the DB instance. But the DB instance is unable to assume the IAM role in order to access other AWS services on your behalf. </p> </li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Contains the details of an Amazon RDS DB instance.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsRdsDbInstanceDetails {
    /// <p>The amount of storage (in gigabytes) to initially allocate for the DB instance.</p>
    #[serde(rename = "AllocatedStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i64>,
    /// <p>The AWS Identity and Access Management (IAM) roles associated with the DB instance.</p>
    #[serde(rename = "AssociatedRoles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_roles: Option<Vec<AwsRdsDbInstanceAssociatedRole>>,
    /// <p>Indicates whether minor version patches are applied automatically.</p>
    #[serde(rename = "AutoMinorVersionUpgrade")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_minor_version_upgrade: Option<bool>,
    /// <p>The Availability Zone where the DB instance will be created.</p>
    #[serde(rename = "AvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    /// <p>The number of days for which to retain automated backups.</p>
    #[serde(rename = "BackupRetentionPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_retention_period: Option<i64>,
    /// <p>The identifier of the CA certificate for this DB instance.</p>
    #[serde(rename = "CACertificateIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca_certificate_identifier: Option<String>,
    /// <p>The name of the character set that this DB instance is associated with.</p>
    #[serde(rename = "CharacterSetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_set_name: Option<String>,
    /// <p>Whether to copy resource tags to snapshots of the DB instance.</p>
    #[serde(rename = "CopyTagsToSnapshot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_tags_to_snapshot: Option<bool>,
    /// <p>If the DB instance is a member of a DB cluster, contains the name of the DB cluster that the DB instance is a member of.</p>
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_cluster_identifier: Option<String>,
    /// <p>Contains the name of the compute and memory capacity class of the DB instance.</p>
    #[serde(rename = "DBInstanceClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_class: Option<String>,
    /// <p>Contains a user-supplied database identifier. This identifier is the unique key that identifies a DB instance.</p>
    #[serde(rename = "DBInstanceIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_identifier: Option<String>,
    /// <p>The meaning of this parameter differs according to the database engine you use.</p> <p> <b>MySQL, MariaDB, SQL Server, PostgreSQL</b> </p> <p>Contains the name of the initial database of this instance that was provided at create time, if one was specified when the DB instance was created. This same name is returned for the life of the DB instance.</p> <p> <b>Oracle</b> </p> <p>Contains the Oracle System ID (SID) of the created DB instance. Not shown when the returned parameters do not apply to an Oracle DB instance. </p>
    #[serde(rename = "DBName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_name: Option<String>,
    /// <p>Specifies the port that the DB instance listens on. If the DB instance is part of a DB cluster, this can be a different port than the DB cluster port.</p>
    #[serde(rename = "DbInstancePort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_port: Option<i64>,
    /// <p>The current status of the DB instance.</p>
    #[serde(rename = "DbInstanceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_status: Option<String>,
    /// <p>A list of the DB parameter groups to assign to the DB instance.</p>
    #[serde(rename = "DbParameterGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_parameter_groups: Option<Vec<AwsRdsDbParameterGroup>>,
    /// <p>A list of the DB security groups to assign to the DB instance.</p>
    #[serde(rename = "DbSecurityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_security_groups: Option<Vec<String>>,
    /// <p>Information about the subnet group that is associated with the DB instance.</p>
    #[serde(rename = "DbSubnetGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_subnet_group: Option<AwsRdsDbSubnetGroup>,
    /// <p>The AWS Region-unique, immutable identifier for the DB instance. This identifier is found in AWS CloudTrail log entries whenever the AWS KMS key for the DB instance is accessed. </p>
    #[serde(rename = "DbiResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbi_resource_id: Option<String>,
    /// <p>Indicates whether the DB instance has deletion protection enabled.</p> <p>When deletion protection is enabled, the database cannot be deleted.</p>
    #[serde(rename = "DeletionProtection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<bool>,
    /// <p>The Active Directory domain membership records associated with the DB instance.</p>
    #[serde(rename = "DomainMemberships")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_memberships: Option<Vec<AwsRdsDbDomainMembership>>,
    /// <p>A list of log types that this DB instance is configured to export to CloudWatch Logs.</p>
    #[serde(rename = "EnabledCloudWatchLogsExports")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_cloud_watch_logs_exports: Option<Vec<String>>,
    /// <p>Specifies the connection endpoint.</p>
    #[serde(rename = "Endpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<AwsRdsDbInstanceEndpoint>,
    /// <p>Provides the name of the database engine to use for this DB instance.</p>
    #[serde(rename = "Engine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    /// <p>Indicates the database engine version.</p>
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// <p>The ARN of the CloudWatch Logs log stream that receives the enhanced monitoring metrics data for the DB instance.</p>
    #[serde(rename = "EnhancedMonitoringResourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enhanced_monitoring_resource_arn: Option<String>,
    /// <p><p>True if mapping of AWS Identity and Access Management (IAM) accounts to database accounts is enabled, and otherwise false.</p> <p>IAM database authentication can be enabled for the following database engines.</p> <ul> <li> <p>For MySQL 5.6, minor version 5.6.34 or higher</p> </li> <li> <p>For MySQL 5.7, minor version 5.7.16 or higher</p> </li> <li> <p>Aurora 5.6 or higher</p> </li> </ul></p>
    #[serde(rename = "IAMDatabaseAuthenticationEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_database_authentication_enabled: Option<bool>,
    /// <p>Indicates when the DB instance was created.</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "InstanceCreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_create_time: Option<String>,
    /// <p>Specifies the provisioned IOPS (I/O operations per second) for this DB instance.</p>
    #[serde(rename = "Iops")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iops: Option<i64>,
    /// <p>If <code>StorageEncrypted</code> is true, the AWS KMS key identifier for the encrypted DB instance.</p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>Specifies the latest time to which a database can be restored with point-in-time restore.</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "LatestRestorableTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_restorable_time: Option<String>,
    /// <p>License model information for this DB instance.</p>
    #[serde(rename = "LicenseModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_model: Option<String>,
    #[serde(rename = "ListenerEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener_endpoint: Option<AwsRdsDbInstanceEndpoint>,
    /// <p>The master user name of the DB instance.</p>
    #[serde(rename = "MasterUsername")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_username: Option<String>,
    /// <p>The upper limit to which Amazon RDS can automatically scale the storage of the DB instance.</p>
    #[serde(rename = "MaxAllocatedStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_allocated_storage: Option<i64>,
    /// <p>The interval, in seconds, between points when enhanced monitoring metrics are collected for the DB instance.</p>
    #[serde(rename = "MonitoringInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_interval: Option<i64>,
    /// <p>The ARN for the IAM role that permits Amazon RDS to send enhanced monitoring metrics to CloudWatch Logs.</p>
    #[serde(rename = "MonitoringRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_role_arn: Option<String>,
    /// <p>Whether the DB instance is a multiple Availability Zone deployment.</p>
    #[serde(rename = "MultiAz")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_az: Option<bool>,
    /// <p>The list of option group memberships for this DB instance.</p>
    #[serde(rename = "OptionGroupMemberships")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_group_memberships: Option<Vec<AwsRdsDbOptionGroupMembership>>,
    /// <p>Changes to the DB instance that are currently pending.</p>
    #[serde(rename = "PendingModifiedValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_modified_values: Option<AwsRdsDbPendingModifiedValues>,
    /// <p>Indicates whether Performance Insights is enabled for the DB instance.</p>
    #[serde(rename = "PerformanceInsightsEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_insights_enabled: Option<bool>,
    /// <p>The identifier of the AWS KMS key used to encrypt the Performance Insights data.</p>
    #[serde(rename = "PerformanceInsightsKmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_insights_kms_key_id: Option<String>,
    /// <p>The number of days to retain Performance Insights data.</p>
    #[serde(rename = "PerformanceInsightsRetentionPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_insights_retention_period: Option<i64>,
    /// <p>The range of time each day when automated backups are created, if automated backups are enabled.</p> <p>Uses the format <code>HH:MM-HH:MM</code>. For example, <code>04:52-05:22</code>.</p>
    #[serde(rename = "PreferredBackupWindow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_backup_window: Option<String>,
    /// <p>The weekly time range during which system maintenance can occur, in Universal Coordinated Time (UTC).</p> <p>Uses the format <code>&lt;day&gt;:HH:MM-&lt;day&gt;:HH:MM</code>.</p> <p>For the day values, use <code>mon</code>|<code>tue</code>|<code>wed</code>|<code>thu</code>|<code>fri</code>|<code>sat</code>|<code>sun</code>.</p> <p>For example, <code>sun:09:32-sun:10:02</code>.</p>
    #[serde(rename = "PreferredMaintenanceWindow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    /// <p>The number of CPU cores and the number of threads per core for the DB instance class of the DB instance.</p>
    #[serde(rename = "ProcessorFeatures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor_features: Option<Vec<AwsRdsDbProcessorFeature>>,
    /// <p>The order in which to promote an Aurora replica to the primary instance after a failure of the existing primary instance.</p>
    #[serde(rename = "PromotionTier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_tier: Option<i64>,
    /// <p>Specifies the accessibility options for the DB instance.</p> <p>A value of true specifies an Internet-facing instance with a publicly resolvable DNS name, which resolves to a public IP address.</p> <p>A value of false specifies an internal instance with a DNS name that resolves to a private IP address. </p>
    #[serde(rename = "PubliclyAccessible")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,
    /// <p>List of identifiers of Aurora DB clusters to which the RDS DB instance is replicated as a read replica.</p>
    #[serde(rename = "ReadReplicaDBClusterIdentifiers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_replica_db_cluster_identifiers: Option<Vec<String>>,
    /// <p>List of identifiers of the read replicas associated with this DB instance.</p>
    #[serde(rename = "ReadReplicaDBInstanceIdentifiers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_replica_db_instance_identifiers: Option<Vec<String>>,
    /// <p>If this DB instance is a read replica, contains the identifier of the source DB instance.</p>
    #[serde(rename = "ReadReplicaSourceDBInstanceIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_replica_source_db_instance_identifier: Option<String>,
    /// <p>For a DB instance with multi-Availability Zone support, the name of the secondary Availability Zone.</p>
    #[serde(rename = "SecondaryAvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_availability_zone: Option<String>,
    /// <p>The status of a read replica. If the instance isn't a read replica, this is empty.</p>
    #[serde(rename = "StatusInfos")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_infos: Option<Vec<AwsRdsDbStatusInfo>>,
    /// <p>Specifies whether the DB instance is encrypted.</p>
    #[serde(rename = "StorageEncrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_encrypted: Option<bool>,
    /// <p>The storage type for the DB instance.</p>
    #[serde(rename = "StorageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    /// <p>The ARN from the key store with which the instance is associated for TDE encryption.</p>
    #[serde(rename = "TdeCredentialArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tde_credential_arn: Option<String>,
    /// <p>The time zone of the DB instance.</p>
    #[serde(rename = "Timezone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// <p>A list of VPC security groups that the DB instance belongs to.</p>
    #[serde(rename = "VpcSecurityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_groups: Option<Vec<AwsRdsDbInstanceVpcSecurityGroup>>,
}

/// <p>Specifies the connection endpoint.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsRdsDbInstanceEndpoint {
    /// <p>Specifies the DNS address of the DB instance.</p>
    #[serde(rename = "Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// <p>Specifies the ID that Amazon Route 53 assigns when you create a hosted zone.</p>
    #[serde(rename = "HostedZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_zone_id: Option<String>,
    /// <p>Specifies the port that the database engine is listening on.</p>
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
}

/// <p>A VPC security groups that the DB instance belongs to.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsRdsDbInstanceVpcSecurityGroup {
    /// <p>The status of the VPC security group.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The name of the VPC security group.</p>
    #[serde(rename = "VpcSecurityGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_id: Option<String>,
}

/// <p>An option group membership.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsRdsDbOptionGroupMembership {
    /// <p>The name of the option group.</p>
    #[serde(rename = "OptionGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_group_name: Option<String>,
    /// <p>The status of the option group membership.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Provides information about a parameter group for a DB instance.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsRdsDbParameterGroup {
    /// <p>The name of the parameter group.</p>
    #[serde(rename = "DbParameterGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_parameter_group_name: Option<String>,
    /// <p>The status of parameter updates.</p>
    #[serde(rename = "ParameterApplyStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_apply_status: Option<String>,
}

/// <p>Changes to a DB instance that are currently pending.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsRdsDbPendingModifiedValues {
    /// <p>The new value of the allocated storage for the DB instance.</p>
    #[serde(rename = "AllocatedStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i64>,
    /// <p>The new backup retention period for the DB instance.</p>
    #[serde(rename = "BackupRetentionPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_retention_period: Option<i64>,
    /// <p>The new CA certificate identifier for the DB instance.</p>
    #[serde(rename = "CaCertificateIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca_certificate_identifier: Option<String>,
    /// <p>The new DB instance class for the DB instance.</p>
    #[serde(rename = "DbInstanceClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_class: Option<String>,
    /// <p>The new DB instance identifier for the DB instance.</p>
    #[serde(rename = "DbInstanceIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_identifier: Option<String>,
    /// <p>The name of the new subnet group for the DB instance.</p>
    #[serde(rename = "DbSubnetGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_subnet_group_name: Option<String>,
    /// <p>The new engine version for the DB instance.</p>
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// <p>The new provisioned IOPS value for the DB instance.</p>
    #[serde(rename = "Iops")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iops: Option<i64>,
    /// <p>The new license model value for the DB instance.</p>
    #[serde(rename = "LicenseModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_model: Option<String>,
    /// <p>The new master user password for the DB instance.</p>
    #[serde(rename = "MasterUserPassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_password: Option<String>,
    /// <p>Indicates that a single Availability Zone DB instance is changing to a multiple Availability Zone deployment.</p>
    #[serde(rename = "MultiAZ")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_az: Option<bool>,
    /// <p>A list of log types that are being enabled or disabled.</p>
    #[serde(rename = "PendingCloudWatchLogsExports")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_cloud_watch_logs_exports: Option<AwsRdsPendingCloudWatchLogsExports>,
    /// <p>The new port for the DB instance.</p>
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    /// <p>Processor features that are being updated.</p>
    #[serde(rename = "ProcessorFeatures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor_features: Option<Vec<AwsRdsDbProcessorFeature>>,
    /// <p>The new storage type for the DB instance.</p>
    #[serde(rename = "StorageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
}

/// <p>A processor feature.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsRdsDbProcessorFeature {
    /// <p>The name of the processor feature.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The value of the processor feature.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>Provides details about an Amazon RDS DB cluster snapshot.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsRdsDbSnapshotDetails {
    /// <p>The amount of storage (in gigabytes) to be initially allocated for the database instance.</p>
    #[serde(rename = "AllocatedStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i64>,
    /// <p>Specifies the name of the Availability Zone in which the DB instance was located at the time of the DB snapshot.</p>
    #[serde(rename = "AvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    /// <p>A name for the DB instance.</p>
    #[serde(rename = "DbInstanceIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_identifier: Option<String>,
    /// <p>The name or ARN of the DB snapshot that is used to restore the DB instance.</p>
    #[serde(rename = "DbSnapshotIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_snapshot_identifier: Option<String>,
    /// <p>The identifier for the source DB instance.</p>
    #[serde(rename = "DbiResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbi_resource_id: Option<String>,
    /// <p>Whether the DB snapshot is encrypted.</p>
    #[serde(rename = "Encrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    /// <p>The name of the database engine to use for this DB instance.</p>
    #[serde(rename = "Engine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    /// <p>The version of the database engine.</p>
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// <p>Whether mapping of IAM accounts to database accounts is enabled.</p>
    #[serde(rename = "IamDatabaseAuthenticationEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_database_authentication_enabled: Option<bool>,
    /// <p>Specifies the time in Coordinated Universal Time (UTC) when the DB instance, from which the snapshot was taken, was created.</p>
    #[serde(rename = "InstanceCreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_create_time: Option<String>,
    /// <p>The provisioned IOPS (I/O operations per second) value of the DB instance at the time of the snapshot.</p>
    #[serde(rename = "Iops")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iops: Option<i64>,
    /// <p>If <code>Encrypted</code> is <code>true</code>, the AWS KMS key identifier for the encrypted DB snapshot.</p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>License model information for the restored DB instance.</p>
    #[serde(rename = "LicenseModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_model: Option<String>,
    /// <p>The master user name for the DB snapshot.</p>
    #[serde(rename = "MasterUsername")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_username: Option<String>,
    /// <p>The option group name for the DB snapshot.</p>
    #[serde(rename = "OptionGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_group_name: Option<String>,
    /// <p>The percentage of the estimated data that has been transferred.</p>
    #[serde(rename = "PercentProgress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_progress: Option<i64>,
    /// <p>The port that the database engine was listening on at the time of the snapshot.</p>
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    /// <p>The number of CPU cores and the number of threads per core for the DB instance class of the DB instance.</p>
    #[serde(rename = "ProcessorFeatures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor_features: Option<Vec<AwsRdsDbProcessorFeature>>,
    /// <p>When the snapshot was taken in Coordinated Universal Time (UTC).</p>
    #[serde(rename = "SnapshotCreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_create_time: Option<String>,
    /// <p>The type of the DB snapshot.</p>
    #[serde(rename = "SnapshotType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_type: Option<String>,
    /// <p>The DB snapshot ARN that the DB snapshot was copied from.</p>
    #[serde(rename = "SourceDbSnapshotIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_db_snapshot_identifier: Option<String>,
    /// <p>The AWS Region that the DB snapshot was created in or copied from.</p>
    #[serde(rename = "SourceRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_region: Option<String>,
    /// <p>The status of this DB snapshot.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The storage type associated with the DB snapshot.</p>
    #[serde(rename = "StorageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    /// <p>The ARN from the key store with which to associate the instance for TDE encryption.</p>
    #[serde(rename = "TdeCredentialArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tde_credential_arn: Option<String>,
    /// <p>The time zone of the DB snapshot.</p>
    #[serde(rename = "Timezone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// <p>The VPC ID associated with the DB snapshot.</p>
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

/// <p>Information about the status of a read replica.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsRdsDbStatusInfo {
    /// <p>If the read replica is currently in an error state, provides the error details.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>Whether the read replica instance is operating normally.</p>
    #[serde(rename = "Normal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normal: Option<bool>,
    /// <p>The status of the read replica instance.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The type of status. For a read replica, the status type is read replication.</p>
    #[serde(rename = "StatusType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_type: Option<String>,
}

/// <p>Information about the subnet group for the database instance.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsRdsDbSubnetGroup {
    /// <p>The ARN of the subnet group.</p>
    #[serde(rename = "DbSubnetGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_subnet_group_arn: Option<String>,
    /// <p>The description of the subnet group.</p>
    #[serde(rename = "DbSubnetGroupDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_subnet_group_description: Option<String>,
    /// <p>The name of the subnet group.</p>
    #[serde(rename = "DbSubnetGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_subnet_group_name: Option<String>,
    /// <p>The status of the subnet group.</p>
    #[serde(rename = "SubnetGroupStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_group_status: Option<String>,
    /// <p>A list of subnets in the subnet group.</p>
    #[serde(rename = "Subnets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnets: Option<Vec<AwsRdsDbSubnetGroupSubnet>>,
    /// <p>The VPC ID of the subnet group.</p>
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

/// <p>Information about a subnet in a subnet group.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsRdsDbSubnetGroupSubnet {
    /// <p>Information about the Availability Zone for a subnet in the subnet group.</p>
    #[serde(rename = "SubnetAvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_availability_zone: Option<AwsRdsDbSubnetGroupSubnetAvailabilityZone>,
    /// <p>The identifier of a subnet in the subnet group.</p>
    #[serde(rename = "SubnetIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_identifier: Option<String>,
    /// <p>The status of a subnet in the subnet group.</p>
    #[serde(rename = "SubnetStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_status: Option<String>,
}

/// <p>An Availability Zone for a subnet in a subnet group.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsRdsDbSubnetGroupSubnetAvailabilityZone {
    /// <p>The name of the Availability Zone for a subnet in the subnet group.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Identifies the log types to enable and disable.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsRdsPendingCloudWatchLogsExports {
    /// <p>A list of log types that are being disabled.</p>
    #[serde(rename = "LogTypesToDisable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_types_to_disable: Option<Vec<String>>,
    /// <p>A list of log types that are being enabled.</p>
    #[serde(rename = "LogTypesToEnable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_types_to_enable: Option<Vec<String>>,
}

/// <p>A node in an Amazon Redshift cluster.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsRedshiftClusterClusterNode {
    /// <p>The role of the node. A node might be a leader node or a compute node.</p>
    #[serde(rename = "NodeRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_role: Option<String>,
    /// <p>The private IP address of the node.</p>
    #[serde(rename = "PrivateIpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
    /// <p>The public IP address of the node.</p>
    #[serde(rename = "PublicIpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_ip_address: Option<String>,
}

/// <p>A cluster parameter group that is associated with an Amazon Redshift cluster.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsRedshiftClusterClusterParameterGroup {
    /// <p>The list of parameter statuses.</p>
    #[serde(rename = "ClusterParameterStatusList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_parameter_status_list: Option<Vec<AwsRedshiftClusterClusterParameterStatus>>,
    /// <p>The status of updates to the parameters.</p>
    #[serde(rename = "ParameterApplyStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_apply_status: Option<String>,
    /// <p>The name of the parameter group.</p>
    #[serde(rename = "ParameterGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_group_name: Option<String>,
}

/// <p>The status of a parameter in a cluster parameter group for an Amazon Redshift cluster.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsRedshiftClusterClusterParameterStatus {
    /// <p>The error that prevented the parameter from being applied to the database.</p>
    #[serde(rename = "ParameterApplyErrorDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_apply_error_description: Option<String>,
    /// <p>The status of the parameter. Indicates whether the parameter is in sync with the database, waiting for a cluster reboot, or encountered an error when it was applied.</p> <p>Valid values: <code>in-sync</code> | <code>pending-reboot</code> | <code>applying</code> | <code>invalid-parameter</code> | <code>apply-deferred</code> | <code>apply-error</code> | <code>unknown-error</code> </p>
    #[serde(rename = "ParameterApplyStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_apply_status: Option<String>,
    /// <p>The name of the parameter.</p>
    #[serde(rename = "ParameterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_name: Option<String>,
}

/// <p>A security group that is associated with the cluster.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsRedshiftClusterClusterSecurityGroup {
    /// <p>The name of the cluster security group.</p>
    #[serde(rename = "ClusterSecurityGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_security_group_name: Option<String>,
    /// <p>The status of the cluster security group.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Information about a cross-Region snapshot copy.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsRedshiftClusterClusterSnapshotCopyStatus {
    /// <p>The destination Region that snapshots are automatically copied to when cross-Region snapshot copy is enabled.</p>
    #[serde(rename = "DestinationRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_region: Option<String>,
    /// <p>The number of days that manual snapshots are retained in the destination region after they are copied from a source region.</p> <p>If the value is -1, then the manual snapshot is retained indefinitely.</p> <p>Valid values: Either -1 or an integer between 1 and 3,653</p>
    #[serde(rename = "ManualSnapshotRetentionPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_snapshot_retention_period: Option<i64>,
    /// <p>The number of days to retain automated snapshots in the destination Region after they are copied from a source Region.</p>
    #[serde(rename = "RetentionPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_period: Option<i64>,
    /// <p>The name of the snapshot copy grant.</p>
    #[serde(rename = "SnapshotCopyGrantName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_copy_grant_name: Option<String>,
}

/// <p>A time windows during which maintenance was deferred for an Amazon Redshift cluster.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsRedshiftClusterDeferredMaintenanceWindow {
    /// <p>The end of the time window for which maintenance was deferred.</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "DeferMaintenanceEndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defer_maintenance_end_time: Option<String>,
    /// <p>The identifier of the maintenance window.</p>
    #[serde(rename = "DeferMaintenanceIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defer_maintenance_identifier: Option<String>,
    /// <p>The start of the time window for which maintenance was deferred.</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "DeferMaintenanceStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defer_maintenance_start_time: Option<String>,
}

/// <p>Details about an Amazon Redshift cluster.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsRedshiftClusterDetails {
    /// <p>Indicates whether major version upgrades are applied automatically to the cluster during the maintenance window.</p>
    #[serde(rename = "AllowVersionUpgrade")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_version_upgrade: Option<bool>,
    /// <p>The number of days that automatic cluster snapshots are retained.</p>
    #[serde(rename = "AutomatedSnapshotRetentionPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automated_snapshot_retention_period: Option<i64>,
    /// <p>The name of the Availability Zone in which the cluster is located.</p>
    #[serde(rename = "AvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    /// <p><p>The availability status of the cluster for queries. Possible values are the following:</p> <ul> <li> <p> <code>Available</code> - The cluster is available for queries.</p> </li> <li> <p> <code>Unavailable</code> - The cluster is not available for queries.</p> </li> <li> <p> <code>Maintenance</code> - The cluster is intermittently available for queries due to maintenance activities.</p> </li> <li> <p> <code>Modifying</code> -The cluster is intermittently available for queries due to changes that modify the cluster.</p> </li> <li> <p> <code>Failed</code> - The cluster failed and is not available for queries.</p> </li> </ul></p>
    #[serde(rename = "ClusterAvailabilityStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_availability_status: Option<String>,
    /// <p>Indicates when the cluster was created.</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "ClusterCreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_create_time: Option<String>,
    /// <p>The unique identifier of the cluster.</p>
    #[serde(rename = "ClusterIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_identifier: Option<String>,
    /// <p>The nodes in the cluster.</p>
    #[serde(rename = "ClusterNodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_nodes: Option<Vec<AwsRedshiftClusterClusterNode>>,
    /// <p>The list of cluster parameter groups that are associated with this cluster.</p>
    #[serde(rename = "ClusterParameterGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_parameter_groups: Option<Vec<AwsRedshiftClusterClusterParameterGroup>>,
    /// <p>The public key for the cluster.</p>
    #[serde(rename = "ClusterPublicKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_public_key: Option<String>,
    /// <p>The specific revision number of the database in the cluster.</p>
    #[serde(rename = "ClusterRevisionNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_revision_number: Option<String>,
    /// <p>A list of cluster security groups that are associated with the cluster.</p>
    #[serde(rename = "ClusterSecurityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_security_groups: Option<Vec<AwsRedshiftClusterClusterSecurityGroup>>,
    /// <p>Information about the destination Region and retention period for the cross-Region snapshot copy.</p>
    #[serde(rename = "ClusterSnapshotCopyStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_snapshot_copy_status: Option<AwsRedshiftClusterClusterSnapshotCopyStatus>,
    /// <p>The current status of the cluster.</p> <p>Valid values: <code>available</code> | <code>available, prep-for-resize</code> | <code>available, resize-cleanup</code> |<code> cancelling-resize</code> | <code>creating</code> | <code>deleting</code> | <code>final-snapshot</code> | <code>hardware-failure</code> | <code>incompatible-hsm</code> |<code> incompatible-network</code> | <code>incompatible-parameters</code> | <code>incompatible-restore</code> | <code>modifying</code> | <code>paused</code> | <code>rebooting</code> | <code>renaming</code> | <code>resizing</code> | <code>rotating-keys</code> | <code>storage-full</code> | <code>updating-hsm</code> </p>
    #[serde(rename = "ClusterStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_status: Option<String>,
    /// <p>The name of the subnet group that is associated with the cluster. This parameter is valid only when the cluster is in a VPC.</p>
    #[serde(rename = "ClusterSubnetGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_subnet_group_name: Option<String>,
    /// <p>The version ID of the Amazon Redshift engine that runs on the cluster.</p>
    #[serde(rename = "ClusterVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_version: Option<String>,
    /// <p>The name of the initial database that was created when the cluster was created.</p> <p>The same name is returned for the life of the cluster.</p> <p>If an initial database is not specified, a database named <code>devdev</code> is created by default.</p>
    #[serde(rename = "DBName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_name: Option<String>,
    /// <p>List of time windows during which maintenance was deferred.</p>
    #[serde(rename = "DeferredMaintenanceWindows")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deferred_maintenance_windows: Option<Vec<AwsRedshiftClusterDeferredMaintenanceWindow>>,
    /// <p>Information about the status of the Elastic IP (EIP) address.</p>
    #[serde(rename = "ElasticIpStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elastic_ip_status: Option<AwsRedshiftClusterElasticIpStatus>,
    /// <p>The number of nodes that you can use the elastic resize method to resize the cluster to.</p>
    #[serde(rename = "ElasticResizeNumberOfNodeOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elastic_resize_number_of_node_options: Option<String>,
    /// <p>Indicates whether the data in the cluster is encrypted at rest.</p>
    #[serde(rename = "Encrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    /// <p>The connection endpoint.</p>
    #[serde(rename = "Endpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<AwsRedshiftClusterEndpoint>,
    /// <p>Indicates whether to create the cluster with enhanced VPC routing enabled.</p>
    #[serde(rename = "EnhancedVpcRouting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enhanced_vpc_routing: Option<bool>,
    /// <p>Indicates when the next snapshot is expected to be taken. The cluster must have a valid snapshot schedule and have backups enabled.</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "ExpectedNextSnapshotScheduleTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_next_snapshot_schedule_time: Option<String>,
    /// <p>The status of the next expected snapshot.</p> <p>Valid values: <code>OnTrack</code> | <code>Pending</code> </p>
    #[serde(rename = "ExpectedNextSnapshotScheduleTimeStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_next_snapshot_schedule_time_status: Option<String>,
    /// <p>Information about whether the Amazon Redshift cluster finished applying any changes to hardware security module (HSM) settings that were specified in a modify cluster command.</p>
    #[serde(rename = "HsmStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsm_status: Option<AwsRedshiftClusterHsmStatus>,
    /// <p>A list of IAM roles that the cluster can use to access other AWS services.</p>
    #[serde(rename = "IamRoles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_roles: Option<Vec<AwsRedshiftClusterIamRole>>,
    /// <p>The identifier of the AWS KMS encryption key that is used to encrypt data in the cluster.</p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>The name of the maintenance track for the cluster.</p>
    #[serde(rename = "MaintenanceTrackName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_track_name: Option<String>,
    /// <p>The default number of days to retain a manual snapshot.</p> <p>If the value is -1, the snapshot is retained indefinitely.</p> <p>This setting doesn't change the retention period of existing snapshots.</p> <p>Valid values: Either -1 or an integer between 1 and 3,653</p>
    #[serde(rename = "ManualSnapshotRetentionPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_snapshot_retention_period: Option<i64>,
    /// <p>The master user name for the cluster. This name is used to connect to the database that is specified in as the value of <code>DBName</code>.</p>
    #[serde(rename = "MasterUsername")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_username: Option<String>,
    /// <p>Indicates the start of the next maintenance window.</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "NextMaintenanceWindowStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_maintenance_window_start_time: Option<String>,
    /// <p>The node type for the nodes in the cluster.</p>
    #[serde(rename = "NodeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    /// <p>The number of compute nodes in the cluster.</p>
    #[serde(rename = "NumberOfNodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_nodes: Option<i64>,
    /// <p>A list of cluster operations that are waiting to start.</p>
    #[serde(rename = "PendingActions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_actions: Option<Vec<String>>,
    /// <p>A list of changes to the cluster that are currently pending.</p>
    #[serde(rename = "PendingModifiedValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_modified_values: Option<AwsRedshiftClusterPendingModifiedValues>,
    /// <p>The weekly time range, in Universal Coordinated Time (UTC), during which system maintenance can occur.</p> <p>Format: <code> <i>&lt;day&gt;</i>:HH:MM-<i>&lt;day&gt;</i>:HH:MM</code> </p> <p>For the day values, use <code>mon</code> | <code>tue</code> | <code>wed</code> | <code>thu</code> | <code>fri</code> | <code>sat</code> | <code>sun</code> </p> <p>For example, <code>sun:09:32-sun:10:02</code> </p>
    #[serde(rename = "PreferredMaintenanceWindow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    /// <p>Whether the cluster can be accessed from a public network.</p>
    #[serde(rename = "PubliclyAccessible")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,
    /// <p>Information about the resize operation for the cluster.</p>
    #[serde(rename = "ResizeInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resize_info: Option<AwsRedshiftClusterResizeInfo>,
    /// <p>Information about the status of a cluster restore action. Only applies to a cluster that was created by restoring a snapshot.</p>
    #[serde(rename = "RestoreStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_status: Option<AwsRedshiftClusterRestoreStatus>,
    /// <p>A unique identifier for the cluster snapshot schedule.</p>
    #[serde(rename = "SnapshotScheduleIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_schedule_identifier: Option<String>,
    /// <p>The current state of the cluster snapshot schedule.</p> <p>Valid values: <code>MODIFYING</code> | <code>ACTIVE</code> | <code>FAILED</code> </p>
    #[serde(rename = "SnapshotScheduleState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_schedule_state: Option<String>,
    /// <p>The identifier of the VPC that the cluster is in, if the cluster is in a VPC.</p>
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// <p>The list of VPC security groups that the cluster belongs to, if the cluster is in a VPC.</p>
    #[serde(rename = "VpcSecurityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_groups: Option<Vec<AwsRedshiftClusterVpcSecurityGroup>>,
}

/// <p>The status of the elastic IP (EIP) address for an Amazon Redshift cluster.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsRedshiftClusterElasticIpStatus {
    /// <p>The elastic IP address for the cluster.</p>
    #[serde(rename = "ElasticIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elastic_ip: Option<String>,
    /// <p>The status of the elastic IP address.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>The connection endpoint for an Amazon Redshift cluster.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsRedshiftClusterEndpoint {
    /// <p>The DNS address of the cluster.</p>
    #[serde(rename = "Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// <p>The port that the database engine listens on.</p>
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
}

/// <p>Information about whether an Amazon Redshift cluster finished applying any hardware changes to security module (HSM) settings that were specified in a modify cluster command.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsRedshiftClusterHsmStatus {
    /// <p>The name of the HSM client certificate that the Amazon Redshift cluster uses to retrieve the data encryption keys that are stored in an HSM.</p>
    #[serde(rename = "HsmClientCertificateIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsm_client_certificate_identifier: Option<String>,
    /// <p>The name of the HSM configuration that contains the information that the Amazon Redshift cluster can use to retrieve and store keys in an HSM.</p>
    #[serde(rename = "HsmConfigurationIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsm_configuration_identifier: Option<String>,
    /// <p>Indicates whether the Amazon Redshift cluster has finished applying any HSM settings changes specified in a modify cluster command.</p> <p>Type: String</p> <p>Valid values: <code>active</code> | <code>applying</code> </p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>An IAM role that the cluster can use to access other AWS services.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsRedshiftClusterIamRole {
    /// <p>The status of the IAM role's association with the cluster.</p> <p>Valid values: <code>in-sync</code> | <code>adding</code> | <code>removing</code> </p>
    #[serde(rename = "ApplyStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_status: Option<String>,
    /// <p>The ARN of the IAM role.</p>
    #[serde(rename = "IamRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<String>,
}

/// <p>Changes to the Amazon Redshift cluster that are currently pending.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsRedshiftClusterPendingModifiedValues {
    /// <p>The pending or in-progress change to the automated snapshot retention period.</p>
    #[serde(rename = "AutomatedSnapshotRetentionPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automated_snapshot_retention_period: Option<i64>,
    /// <p>The pending or in-progress change to the identifier for the cluster.</p>
    #[serde(rename = "ClusterIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_identifier: Option<String>,
    /// <p>The pending or in-progress change to the cluster type.</p>
    #[serde(rename = "ClusterType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_type: Option<String>,
    /// <p>The pending or in-progress change to the service version.</p>
    #[serde(rename = "ClusterVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_version: Option<String>,
    /// <p>The encryption type for a cluster.</p>
    #[serde(rename = "EncryptionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_type: Option<String>,
    /// <p>Indicates whether to create the cluster with enhanced VPC routing enabled.</p>
    #[serde(rename = "EnhancedVpcRouting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enhanced_vpc_routing: Option<bool>,
    /// <p>The name of the maintenance track that the cluster changes to during the next maintenance window.</p>
    #[serde(rename = "MaintenanceTrackName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_track_name: Option<String>,
    /// <p>The pending or in-progress change to the master user password for the cluster.</p>
    #[serde(rename = "MasterUserPassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_password: Option<String>,
    /// <p>The pending or in-progress change to the cluster's node type.</p>
    #[serde(rename = "NodeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    /// <p>The pending or in-progress change to the number of nodes in the cluster.</p>
    #[serde(rename = "NumberOfNodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_nodes: Option<i64>,
    /// <p>The pending or in-progress change to whether the cluster can be connected to from the public network.</p>
    #[serde(rename = "PubliclyAccessible")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,
}

/// <p>Information about the resize operation for the cluster.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsRedshiftClusterResizeInfo {
    /// <p>Indicates whether the resize operation can be canceled.</p>
    #[serde(rename = "AllowCancelResize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_cancel_resize: Option<bool>,
    /// <p>The type of resize operation.</p> <p>Valid values: <code>ClassicResize</code> </p>
    #[serde(rename = "ResizeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resize_type: Option<String>,
}

/// <p>Information about the status of a cluster restore action. It only applies if the cluster was created by restoring a snapshot.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsRedshiftClusterRestoreStatus {
    /// <p>The number of megabytes per second being transferred from the backup storage. Returns the average rate for a completed backup.</p> <p>This field is only updated when you restore to DC2 and DS2 node types.</p>
    #[serde(rename = "CurrentRestoreRateInMegaBytesPerSecond")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_restore_rate_in_mega_bytes_per_second: Option<f64>,
    /// <p>The amount of time an in-progress restore has been running, or the amount of time it took a completed restore to finish.</p> <p>This field is only updated when you restore to DC2 and DS2 node types.</p>
    #[serde(rename = "ElapsedTimeInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elapsed_time_in_seconds: Option<i64>,
    /// <p>The estimate of the time remaining before the restore is complete. Returns 0 for a completed restore.</p> <p>This field is only updated when you restore to DC2 and DS2 node types.</p>
    #[serde(rename = "EstimatedTimeToCompletionInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_time_to_completion_in_seconds: Option<i64>,
    /// <p>The number of megabytes that were transferred from snapshot storage.</p> <p>This field is only updated when you restore to DC2 and DS2 node types.</p>
    #[serde(rename = "ProgressInMegaBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_in_mega_bytes: Option<i64>,
    /// <p>The size of the set of snapshot data that was used to restore the cluster.</p> <p>This field is only updated when you restore to DC2 and DS2 node types.</p>
    #[serde(rename = "SnapshotSizeInMegaBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_size_in_mega_bytes: Option<i64>,
    /// <p>The status of the restore action.</p> <p>Valid values: <code>starting</code> | <code>restoring</code> | <code>completed</code> | <code>failed</code> </p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>A VPC security group that the cluster belongs to, if the cluster is in a VPC.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsRedshiftClusterVpcSecurityGroup {
    /// <p>The status of the VPC security group.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The identifier of the VPC security group.</p>
    #[serde(rename = "VpcSecurityGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_id: Option<String>,
}

/// <p>provides information about the Amazon S3 Public Access Block configuration for accounts.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsS3AccountPublicAccessBlockDetails {
    /// <p>Indicates whether to reject calls to update an S3 bucket if the calls include a public access control list (ACL).</p>
    #[serde(rename = "BlockPublicAcls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_public_acls: Option<bool>,
    /// <p>Indicates whether to reject calls to update the access policy for an S3 bucket or access point if the policy allows public access.</p>
    #[serde(rename = "BlockPublicPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_public_policy: Option<bool>,
    /// <p>Indicates whether Amazon S3 ignores public ACLs that are associated with an S3 bucket.</p>
    #[serde(rename = "IgnorePublicAcls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_public_acls: Option<bool>,
    /// <p>Indicates whether to restrict access to an access point or S3 bucket that has a public policy to only AWS service principals and authorized users within the S3 bucket owner's account.</p>
    #[serde(rename = "RestrictPublicBuckets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrict_public_buckets: Option<bool>,
}

/// <p>The lifecycle configuration for the objects in the S3 bucket.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsS3BucketBucketLifecycleConfigurationDetails {
    /// <p>The lifecycle rules.</p>
    #[serde(rename = "Rules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<AwsS3BucketBucketLifecycleConfigurationRulesDetails>>,
}

/// <p>Information about what Amazon S3 does when a multipart upload is incomplete.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsS3BucketBucketLifecycleConfigurationRulesAbortIncompleteMultipartUploadDetails {
    /// <p>The number of days after which Amazon S3 cancels an incomplete multipart upload.</p>
    #[serde(rename = "DaysAfterInitiation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_after_initiation: Option<i64>,
}

/// <p>Configuration for a lifecycle rule.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsS3BucketBucketLifecycleConfigurationRulesDetails {
    /// <p>How Amazon S3 responds when a multipart upload is incomplete. Specifically, provides a number of days before Amazon S3 cancels the entire upload.</p>
    #[serde(rename = "AbortIncompleteMultipartUpload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abort_incomplete_multipart_upload:
        Option<AwsS3BucketBucketLifecycleConfigurationRulesAbortIncompleteMultipartUploadDetails>,
    /// <p>The date when objects are moved or deleted.</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "ExpirationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    /// <p>The length in days of the lifetime for objects that are subject to the rule.</p>
    #[serde(rename = "ExpirationInDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_in_days: Option<i64>,
    /// <p>Whether Amazon S3 removes a delete marker that has no noncurrent versions. If set to <code>true</code>, the delete marker is expired. If set to <code>false</code>, the policy takes no action.</p> <p>If you provide <code>ExpiredObjectDeleteMarker</code>, you cannot provide <code>ExpirationInDays</code> or <code>ExpirationDate</code>.</p>
    #[serde(rename = "ExpiredObjectDeleteMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired_object_delete_marker: Option<bool>,
    /// <p>Identifies the objects that a rule applies to.</p>
    #[serde(rename = "Filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<AwsS3BucketBucketLifecycleConfigurationRulesFilterDetails>,
    /// <p>The unique identifier of the rule.</p>
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The number of days that an object is noncurrent before Amazon S3 can perform the associated action.</p>
    #[serde(rename = "NoncurrentVersionExpirationInDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub noncurrent_version_expiration_in_days: Option<i64>,
    /// <p>Transition rules that describe when noncurrent objects transition to a specified storage class.</p>
    #[serde(rename = "NoncurrentVersionTransitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub noncurrent_version_transitions: Option<
        Vec<AwsS3BucketBucketLifecycleConfigurationRulesNoncurrentVersionTransitionsDetails>,
    >,
    /// <p>A prefix that identifies one or more objects that the rule applies to.</p>
    #[serde(rename = "Prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// <p>The current status of the rule. Indicates whether the rule is currently being applied.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>Transition rules that indicate when objects transition to a specified storage class.</p>
    #[serde(rename = "Transitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transitions: Option<Vec<AwsS3BucketBucketLifecycleConfigurationRulesTransitionsDetails>>,
}

/// <p>Identifies the objects that a rule applies to.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsS3BucketBucketLifecycleConfigurationRulesFilterDetails {
    /// <p>The configuration for the filter.</p>
    #[serde(rename = "Predicate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicate: Option<AwsS3BucketBucketLifecycleConfigurationRulesFilterPredicateDetails>,
}

/// <p>The configuration for the filter.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsS3BucketBucketLifecycleConfigurationRulesFilterPredicateDetails {
    /// <p>The values to use for the filter.</p>
    #[serde(rename = "Operands")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operands:
        Option<Vec<AwsS3BucketBucketLifecycleConfigurationRulesFilterPredicateOperandsDetails>>,
    /// <p>A prefix filter.</p>
    #[serde(rename = "Prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// <p>A tag filter.</p>
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<AwsS3BucketBucketLifecycleConfigurationRulesFilterPredicateTagDetails>,
    /// <p>Whether to use <code>AND</code> or <code>OR</code> to join the operands.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>A value to use for the filter.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsS3BucketBucketLifecycleConfigurationRulesFilterPredicateOperandsDetails {
    /// <p>Prefix text for matching objects.</p>
    #[serde(rename = "Prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// <p>A tag that is assigned to matching objects.</p>
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<AwsS3BucketBucketLifecycleConfigurationRulesFilterPredicateOperandsTagDetails>,
    /// <p>The type of filter value.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>A tag that is assigned to matching objects.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsS3BucketBucketLifecycleConfigurationRulesFilterPredicateOperandsTagDetails {
    /// <p>The tag key.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The tag value.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>A tag filter.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsS3BucketBucketLifecycleConfigurationRulesFilterPredicateTagDetails {
    /// <p>The tag key.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The tag value</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>A transition rule that describes when noncurrent objects transition to a specified storage class.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsS3BucketBucketLifecycleConfigurationRulesNoncurrentVersionTransitionsDetails {
    /// <p>The number of days that an object is noncurrent before Amazon S3 can perform the associated action.</p>
    #[serde(rename = "Days")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<i64>,
    /// <p>The class of storage to change the object to after the object is noncurrent for the specified number of days.</p>
    #[serde(rename = "StorageClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_class: Option<String>,
}

/// <p>A rule for when objects transition to specific storage classes.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsS3BucketBucketLifecycleConfigurationRulesTransitionsDetails {
    /// <p>A date on which to transition objects to the specified storage class. If you provide <code>Date</code>, you cannot provide <code>Days</code>.</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "Date")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// <p>The number of days after which to transition the object to the specified storage class. If you provide <code>Days</code>, you cannot provide <code>Date</code>.</p>
    #[serde(rename = "Days")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<i64>,
    /// <p>The storage class to transition the object to.</p>
    #[serde(rename = "StorageClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_class: Option<String>,
}

/// <p>The details of an Amazon S3 bucket.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsS3BucketDetails {
    /// <p>The lifecycle configuration for objects in the S3 bucket.</p>
    #[serde(rename = "BucketLifecycleConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_lifecycle_configuration: Option<AwsS3BucketBucketLifecycleConfigurationDetails>,
    /// <p>Indicates when the S3 bucket was created.</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// <p>The canonical user ID of the owner of the S3 bucket.</p>
    #[serde(rename = "OwnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    /// <p>The display name of the owner of the S3 bucket.</p>
    #[serde(rename = "OwnerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_name: Option<String>,
    /// <p>Provides information about the Amazon S3 Public Access Block configuration for the S3 bucket.</p>
    #[serde(rename = "PublicAccessBlockConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_access_block_configuration: Option<AwsS3AccountPublicAccessBlockDetails>,
    /// <p>The encryption rules that are applied to the S3 bucket.</p>
    #[serde(rename = "ServerSideEncryptionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_side_encryption_configuration: Option<AwsS3BucketServerSideEncryptionConfiguration>,
}

/// <p>Specifies the default server-side encryption to apply to new objects in the bucket.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsS3BucketServerSideEncryptionByDefault {
    /// <p>AWS KMS customer master key (CMK) ID to use for the default encryption.</p>
    #[serde(rename = "KMSMasterKeyID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_master_key_id: Option<String>,
    /// <p>Server-side encryption algorithm to use for the default encryption.</p>
    #[serde(rename = "SSEAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sse_algorithm: Option<String>,
}

/// <p>The encryption configuration for the S3 bucket.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsS3BucketServerSideEncryptionConfiguration {
    /// <p>The encryption rules that are applied to the S3 bucket.</p>
    #[serde(rename = "Rules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<AwsS3BucketServerSideEncryptionRule>>,
}

/// <p>An encryption rule to apply to the S3 bucket.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsS3BucketServerSideEncryptionRule {
    /// <p>Specifies the default server-side encryption to apply to new objects in the bucket. If a <code>PUT</code> object request doesn't specify any server-side encryption, this default encryption is applied.</p>
    #[serde(rename = "ApplyServerSideEncryptionByDefault")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_server_side_encryption_by_default: Option<AwsS3BucketServerSideEncryptionByDefault>,
}

/// <p>Details about an Amazon S3 object.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsS3ObjectDetails {
    /// <p>A standard MIME type describing the format of the object data.</p>
    #[serde(rename = "ContentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// <p>The opaque identifier assigned by a web server to a specific version of a resource found at a URL.</p>
    #[serde(rename = "ETag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    /// <p>Indicates when the object was last modified.</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "LastModified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    /// <p>The identifier of the AWS Key Management Service (AWS KMS) symmetric customer managed customer master key (CMK) that was used for the object.</p>
    #[serde(rename = "SSEKMSKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssekms_key_id: Option<String>,
    /// <p>If the object is stored using server-side encryption, the value of the server-side encryption algorithm used when storing this object in Amazon S3.</p>
    #[serde(rename = "ServerSideEncryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_side_encryption: Option<String>,
    /// <p>The version of the object.</p>
    #[serde(rename = "VersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

/// <p>Details about an AWS Secrets Manager secret.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsSecretsManagerSecretDetails {
    /// <p>Whether the secret is deleted.</p>
    #[serde(rename = "Deleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// <p>The user-provided description of the secret.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ARN, Key ID, or alias of the AWS KMS customer master key (CMK) used to encrypt the <code>SecretString</code> or <code>SecretBinary</code> values for versions of this secret.</p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>The name of the secret.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Whether rotation is enabled.</p>
    #[serde(rename = "RotationEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_enabled: Option<bool>,
    /// <p>The ARN of the Lambda function that rotates the secret.</p>
    #[serde(rename = "RotationLambdaArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_lambda_arn: Option<String>,
    /// <p>Whether the rotation occurred within the specified rotation frequency.</p>
    #[serde(rename = "RotationOccurredWithinFrequency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_occurred_within_frequency: Option<bool>,
    /// <p>Defines the rotation schedule for the secret.</p>
    #[serde(rename = "RotationRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_rules: Option<AwsSecretsManagerSecretRotationRules>,
}

/// <p>Defines the rotation schedule for the secret.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsSecretsManagerSecretRotationRules {
    /// <p>The number of days after the previous rotation to rotate the secret.</p>
    #[serde(rename = "AutomaticallyAfterDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatically_after_days: Option<i64>,
}

/// <p><p>Provides consistent format for the contents of the Security Hub-aggregated findings. <code>AwsSecurityFinding</code> format enables you to share findings between AWS security services and third-party solutions, and security standards checks.</p> <note> <p>A finding is a potential security issue generated either by AWS services (Amazon GuardDuty, Amazon Inspector, and Amazon Macie) or by the integrated third-party solutions and standards checks.</p> </note></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsSecurityFinding {
    /// <p>Provides details about an action that affects or that was taken on a resource.</p>
    #[serde(rename = "Action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<Action>,
    /// <p>The AWS account ID that a finding is generated in.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>This data type is exclusive to findings that are generated as the result of a check run against a specific rule in a supported security standard, such as CIS AWS Foundations. Contains security standard-related finding details.</p>
    #[serde(rename = "Compliance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance: Option<Compliance>,
    /// <p>A finding's confidence. Confidence is defined as the likelihood that a finding accurately identifies the behavior or issue that it was intended to identify.</p> <p>Confidence is scored on a 0-100 basis using a ratio scale, where 0 means zero percent confidence and 100 means 100 percent confidence.</p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<i64>,
    /// <p>Indicates when the security-findings provider created the potential security issue that a finding captured.</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "CreatedAt")]
    pub created_at: String,
    /// <p>The level of importance assigned to the resources associated with the finding.</p> <p>A score of 0 means that the underlying resources have no criticality, and a score of 100 is reserved for the most critical resources.</p>
    #[serde(rename = "Criticality")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub criticality: Option<i64>,
    /// <p><p>A finding&#39;s description.</p> <note> <p>In this release, <code>Description</code> is a required property.</p> </note></p>
    #[serde(rename = "Description")]
    pub description: String,
    /// <p>In a <code>BatchImportFindings</code> request, finding providers use <code>FindingProviderFields</code> to provide and update their own values for confidence, criticality, related findings, severity, and types.</p>
    #[serde(rename = "FindingProviderFields")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_provider_fields: Option<FindingProviderFields>,
    /// <p>Indicates when the security-findings provider first observed the potential security issue that a finding captured.</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "FirstObservedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_observed_at: Option<String>,
    /// <p>The identifier for the solution-specific component (a discrete unit of logic) that generated a finding. In various security-findings providers' solutions, this generator can be called a rule, a check, a detector, a plugin, etc. </p>
    #[serde(rename = "GeneratorId")]
    pub generator_id: String,
    /// <p>The security findings provider-specific identifier for a finding.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>Indicates when the security-findings provider most recently observed the potential security issue that a finding captured.</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "LastObservedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_observed_at: Option<String>,
    /// <p>A list of malware related to a finding.</p>
    #[serde(rename = "Malware")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub malware: Option<Vec<Malware>>,
    /// <p>The details of network-related information about a finding.</p>
    #[serde(rename = "Network")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<Network>,
    /// <p>Provides information about a network path that is relevant to a finding. Each entry under <code>NetworkPath</code> represents a component of that path.</p>
    #[serde(rename = "NetworkPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_path: Option<Vec<NetworkPathComponent>>,
    /// <p>A user-defined note added to a finding.</p>
    #[serde(rename = "Note")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<Note>,
    /// <p>Provides an overview of the patch compliance status for an instance against a selected compliance standard.</p>
    #[serde(rename = "PatchSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_summary: Option<PatchSummary>,
    /// <p>The details of process-related information about a finding.</p>
    #[serde(rename = "Process")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process: Option<ProcessDetails>,
    /// <p>The ARN generated by Security Hub that uniquely identifies a product that generates findings. This can be the ARN for a third-party product that is integrated with Security Hub, or the ARN for a custom integration.</p>
    #[serde(rename = "ProductArn")]
    pub product_arn: String,
    /// <p>A data type where security-findings providers can include additional solution-specific details that aren't part of the defined <code>AwsSecurityFinding</code> format.</p>
    #[serde(rename = "ProductFields")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_fields: Option<::std::collections::HashMap<String, String>>,
    /// <p>The record state of a finding.</p>
    #[serde(rename = "RecordState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_state: Option<String>,
    /// <p>A list of related findings.</p>
    #[serde(rename = "RelatedFindings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_findings: Option<Vec<RelatedFinding>>,
    /// <p>A data type that describes the remediation options for a finding.</p>
    #[serde(rename = "Remediation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remediation: Option<Remediation>,
    /// <p>A set of resource data types that describe the resources that the finding refers to.</p>
    #[serde(rename = "Resources")]
    pub resources: Vec<Resource>,
    /// <p>The schema version that a finding is formatted for.</p>
    #[serde(rename = "SchemaVersion")]
    pub schema_version: String,
    /// <p>A finding's severity.</p>
    #[serde(rename = "Severity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<Severity>,
    /// <p>A URL that links to a page about the current finding in the security-findings provider's solution.</p>
    #[serde(rename = "SourceUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
    /// <p>Threat intelligence details related to a finding.</p>
    #[serde(rename = "ThreatIntelIndicators")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_intel_indicators: Option<Vec<ThreatIntelIndicator>>,
    /// <p><p>A finding&#39;s title.</p> <note> <p>In this release, <code>Title</code> is a required property.</p> </note></p>
    #[serde(rename = "Title")]
    pub title: String,
    /// <p>One or more finding types in the format of <code>namespace/category/classifier</code> that classify a finding.</p> <p>Valid namespace values are: Software and Configuration Checks | TTPs | Effects | Unusual Behaviors | Sensitive Data Identifications</p>
    #[serde(rename = "Types")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<String>>,
    /// <p>Indicates when the security-findings provider last updated the finding record.</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "UpdatedAt")]
    pub updated_at: String,
    /// <p>A list of name/value string pairs associated with the finding. These are custom, user-defined fields added to a finding. </p>
    #[serde(rename = "UserDefinedFields")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_defined_fields: Option<::std::collections::HashMap<String, String>>,
    /// <p>Indicates the veracity of a finding. </p>
    #[serde(rename = "VerificationState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_state: Option<String>,
    /// <p>Provides a list of vulnerabilities associated with the findings.</p>
    #[serde(rename = "Vulnerabilities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vulnerabilities: Option<Vec<Vulnerability>>,
    /// <p>Provides information about the status of the investigation into a finding.</p>
    #[serde(rename = "Workflow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow: Option<Workflow>,
    /// <p>The workflow state of a finding. </p>
    #[serde(rename = "WorkflowState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_state: Option<String>,
}

/// <p>A collection of attributes that are applied to all active Security Hub-aggregated findings and that result in a subset of findings that are included in this insight.</p> <p>You can filter by up to 10 finding attributes. For each attribute, you can provide up to 20 filter values.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsSecurityFindingFilters {
    /// <p>The AWS account ID that a finding is generated in.</p>
    #[serde(rename = "AwsAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_account_id: Option<Vec<StringFilter>>,
    /// <p>The name of the findings provider (company) that owns the solution (product) that generates findings.</p>
    #[serde(rename = "CompanyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<Vec<StringFilter>>,
    /// <p>Exclusive to findings that are generated as the result of a check run against a specific rule in a supported standard, such as CIS AWS Foundations. Contains security standard-related finding details.</p>
    #[serde(rename = "ComplianceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_status: Option<Vec<StringFilter>>,
    /// <p>A finding's confidence. Confidence is defined as the likelihood that a finding accurately identifies the behavior or issue that it was intended to identify.</p> <p>Confidence is scored on a 0-100 basis using a ratio scale, where 0 means zero percent confidence and 100 means 100 percent confidence.</p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<Vec<NumberFilter>>,
    /// <p>An ISO8601-formatted timestamp that indicates when the security-findings provider captured the potential security issue that a finding captured.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Vec<DateFilter>>,
    /// <p>The level of importance assigned to the resources associated with the finding.</p> <p>A score of 0 means that the underlying resources have no criticality, and a score of 100 is reserved for the most critical resources.</p>
    #[serde(rename = "Criticality")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub criticality: Option<Vec<NumberFilter>>,
    /// <p>A finding's description.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<StringFilter>>,
    /// <p>The finding provider value for the finding confidence. Confidence is defined as the likelihood that a finding accurately identifies the behavior or issue that it was intended to identify.</p> <p>Confidence is scored on a 0-100 basis using a ratio scale, where 0 means zero percent confidence and 100 means 100 percent confidence.</p>
    #[serde(rename = "FindingProviderFieldsConfidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_provider_fields_confidence: Option<Vec<NumberFilter>>,
    /// <p>The finding provider value for the level of importance assigned to the resources associated with the findings.</p> <p>A score of 0 means that the underlying resources have no criticality, and a score of 100 is reserved for the most critical resources. </p>
    #[serde(rename = "FindingProviderFieldsCriticality")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_provider_fields_criticality: Option<Vec<NumberFilter>>,
    /// <p>The finding identifier of a related finding that is identified by the finding provider.</p>
    #[serde(rename = "FindingProviderFieldsRelatedFindingsId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_provider_fields_related_findings_id: Option<Vec<StringFilter>>,
    /// <p>The ARN of the solution that generated a related finding that is identified by the finding provider.</p>
    #[serde(rename = "FindingProviderFieldsRelatedFindingsProductArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_provider_fields_related_findings_product_arn: Option<Vec<StringFilter>>,
    /// <p>The finding provider value for the severity label.</p>
    #[serde(rename = "FindingProviderFieldsSeverityLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_provider_fields_severity_label: Option<Vec<StringFilter>>,
    /// <p>The finding provider's original value for the severity.</p>
    #[serde(rename = "FindingProviderFieldsSeverityOriginal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_provider_fields_severity_original: Option<Vec<StringFilter>>,
    /// <p>One or more finding types that the finding provider assigned to the finding. Uses the format of <code>namespace/category/classifier</code> that classify a finding.</p> <p>Valid namespace values are: Software and Configuration Checks | TTPs | Effects | Unusual Behaviors | Sensitive Data Identifications</p>
    #[serde(rename = "FindingProviderFieldsTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_provider_fields_types: Option<Vec<StringFilter>>,
    /// <p>An ISO8601-formatted timestamp that indicates when the security-findings provider first observed the potential security issue that a finding captured.</p>
    #[serde(rename = "FirstObservedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_observed_at: Option<Vec<DateFilter>>,
    /// <p>The identifier for the solution-specific component (a discrete unit of logic) that generated a finding. In various security-findings providers' solutions, this generator can be called a rule, a check, a detector, a plugin, etc.</p>
    #[serde(rename = "GeneratorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generator_id: Option<Vec<StringFilter>>,
    /// <p>The security findings provider-specific identifier for a finding.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Vec<StringFilter>>,
    /// <p>An ISO8601-formatted timestamp that indicates when the security-findings provider most recently observed the potential security issue that a finding captured.</p>
    #[serde(rename = "LastObservedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_observed_at: Option<Vec<DateFilter>>,
    /// <p>The name of the malware that was observed.</p>
    #[serde(rename = "MalwareName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub malware_name: Option<Vec<StringFilter>>,
    /// <p>The filesystem path of the malware that was observed.</p>
    #[serde(rename = "MalwarePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub malware_path: Option<Vec<StringFilter>>,
    /// <p>The state of the malware that was observed.</p>
    #[serde(rename = "MalwareState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub malware_state: Option<Vec<StringFilter>>,
    /// <p>The type of the malware that was observed.</p>
    #[serde(rename = "MalwareType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub malware_type: Option<Vec<StringFilter>>,
    /// <p>The destination domain of network-related information about a finding.</p>
    #[serde(rename = "NetworkDestinationDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_destination_domain: Option<Vec<StringFilter>>,
    /// <p>The destination IPv4 address of network-related information about a finding.</p>
    #[serde(rename = "NetworkDestinationIpV4")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_destination_ip_v4: Option<Vec<IpFilter>>,
    /// <p>The destination IPv6 address of network-related information about a finding.</p>
    #[serde(rename = "NetworkDestinationIpV6")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_destination_ip_v6: Option<Vec<IpFilter>>,
    /// <p>The destination port of network-related information about a finding.</p>
    #[serde(rename = "NetworkDestinationPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_destination_port: Option<Vec<NumberFilter>>,
    /// <p>Indicates the direction of network traffic associated with a finding.</p>
    #[serde(rename = "NetworkDirection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_direction: Option<Vec<StringFilter>>,
    /// <p>The protocol of network-related information about a finding.</p>
    #[serde(rename = "NetworkProtocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_protocol: Option<Vec<StringFilter>>,
    /// <p>The source domain of network-related information about a finding.</p>
    #[serde(rename = "NetworkSourceDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_source_domain: Option<Vec<StringFilter>>,
    /// <p>The source IPv4 address of network-related information about a finding.</p>
    #[serde(rename = "NetworkSourceIpV4")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_source_ip_v4: Option<Vec<IpFilter>>,
    /// <p>The source IPv6 address of network-related information about a finding.</p>
    #[serde(rename = "NetworkSourceIpV6")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_source_ip_v6: Option<Vec<IpFilter>>,
    /// <p>The source media access control (MAC) address of network-related information about a finding.</p>
    #[serde(rename = "NetworkSourceMac")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_source_mac: Option<Vec<StringFilter>>,
    /// <p>The source port of network-related information about a finding.</p>
    #[serde(rename = "NetworkSourcePort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_source_port: Option<Vec<NumberFilter>>,
    /// <p>The text of a note.</p>
    #[serde(rename = "NoteText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note_text: Option<Vec<StringFilter>>,
    /// <p>The timestamp of when the note was updated.</p>
    #[serde(rename = "NoteUpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note_updated_at: Option<Vec<DateFilter>>,
    /// <p>The principal that created a note.</p>
    #[serde(rename = "NoteUpdatedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note_updated_by: Option<Vec<StringFilter>>,
    /// <p>The date/time that the process was launched.</p>
    #[serde(rename = "ProcessLaunchedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_launched_at: Option<Vec<DateFilter>>,
    /// <p>The name of the process.</p>
    #[serde(rename = "ProcessName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_name: Option<Vec<StringFilter>>,
    /// <p>The parent process ID.</p>
    #[serde(rename = "ProcessParentPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_parent_pid: Option<Vec<NumberFilter>>,
    /// <p>The path to the process executable.</p>
    #[serde(rename = "ProcessPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_path: Option<Vec<StringFilter>>,
    /// <p>The process ID.</p>
    #[serde(rename = "ProcessPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_pid: Option<Vec<NumberFilter>>,
    /// <p>The date/time that the process was terminated.</p>
    #[serde(rename = "ProcessTerminatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_terminated_at: Option<Vec<DateFilter>>,
    /// <p>The ARN generated by Security Hub that uniquely identifies a third-party company (security findings provider) after this provider's product (solution that generates findings) is registered with Security Hub.</p>
    #[serde(rename = "ProductArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_arn: Option<Vec<StringFilter>>,
    /// <p>A data type where security-findings providers can include additional solution-specific details that aren't part of the defined <code>AwsSecurityFinding</code> format.</p>
    #[serde(rename = "ProductFields")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_fields: Option<Vec<MapFilter>>,
    /// <p>The name of the solution (product) that generates findings.</p>
    #[serde(rename = "ProductName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_name: Option<Vec<StringFilter>>,
    /// <p>The recommendation of what to do about the issue described in a finding.</p>
    #[serde(rename = "RecommendationText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_text: Option<Vec<StringFilter>>,
    /// <p>The updated record state for the finding.</p>
    #[serde(rename = "RecordState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_state: Option<Vec<StringFilter>>,
    /// <p>The solution-generated identifier for a related finding.</p>
    #[serde(rename = "RelatedFindingsId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_findings_id: Option<Vec<StringFilter>>,
    /// <p>The ARN of the solution that generated a related finding.</p>
    #[serde(rename = "RelatedFindingsProductArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_findings_product_arn: Option<Vec<StringFilter>>,
    /// <p>The IAM profile ARN of the instance.</p>
    #[serde(rename = "ResourceAwsEc2InstanceIamInstanceProfileArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_aws_ec_2_instance_iam_instance_profile_arn: Option<Vec<StringFilter>>,
    /// <p>The Amazon Machine Image (AMI) ID of the instance.</p>
    #[serde(rename = "ResourceAwsEc2InstanceImageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_aws_ec_2_instance_image_id: Option<Vec<StringFilter>>,
    /// <p>The IPv4 addresses associated with the instance.</p>
    #[serde(rename = "ResourceAwsEc2InstanceIpV4Addresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_aws_ec_2_instance_ip_v4_addresses: Option<Vec<IpFilter>>,
    /// <p>The IPv6 addresses associated with the instance.</p>
    #[serde(rename = "ResourceAwsEc2InstanceIpV6Addresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_aws_ec_2_instance_ip_v6_addresses: Option<Vec<IpFilter>>,
    /// <p>The key name associated with the instance.</p>
    #[serde(rename = "ResourceAwsEc2InstanceKeyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_aws_ec_2_instance_key_name: Option<Vec<StringFilter>>,
    /// <p>The date and time the instance was launched.</p>
    #[serde(rename = "ResourceAwsEc2InstanceLaunchedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_aws_ec_2_instance_launched_at: Option<Vec<DateFilter>>,
    /// <p>The identifier of the subnet that the instance was launched in.</p>
    #[serde(rename = "ResourceAwsEc2InstanceSubnetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_aws_ec_2_instance_subnet_id: Option<Vec<StringFilter>>,
    /// <p>The instance type of the instance.</p>
    #[serde(rename = "ResourceAwsEc2InstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_aws_ec_2_instance_type: Option<Vec<StringFilter>>,
    /// <p>The identifier of the VPC that the instance was launched in.</p>
    #[serde(rename = "ResourceAwsEc2InstanceVpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_aws_ec_2_instance_vpc_id: Option<Vec<StringFilter>>,
    /// <p>The creation date/time of the IAM access key related to a finding.</p>
    #[serde(rename = "ResourceAwsIamAccessKeyCreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_aws_iam_access_key_created_at: Option<Vec<DateFilter>>,
    /// <p>The status of the IAM access key related to a finding.</p>
    #[serde(rename = "ResourceAwsIamAccessKeyStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_aws_iam_access_key_status: Option<Vec<StringFilter>>,
    /// <p>The user associated with the IAM access key related to a finding.</p>
    #[serde(rename = "ResourceAwsIamAccessKeyUserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_aws_iam_access_key_user_name: Option<Vec<StringFilter>>,
    /// <p>The canonical user ID of the owner of the S3 bucket.</p>
    #[serde(rename = "ResourceAwsS3BucketOwnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_aws_s3_bucket_owner_id: Option<Vec<StringFilter>>,
    /// <p>The display name of the owner of the S3 bucket.</p>
    #[serde(rename = "ResourceAwsS3BucketOwnerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_aws_s3_bucket_owner_name: Option<Vec<StringFilter>>,
    /// <p>The identifier of the image related to a finding.</p>
    #[serde(rename = "ResourceContainerImageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_container_image_id: Option<Vec<StringFilter>>,
    /// <p>The name of the image related to a finding.</p>
    #[serde(rename = "ResourceContainerImageName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_container_image_name: Option<Vec<StringFilter>>,
    /// <p>The date/time that the container was started.</p>
    #[serde(rename = "ResourceContainerLaunchedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_container_launched_at: Option<Vec<DateFilter>>,
    /// <p>The name of the container related to a finding.</p>
    #[serde(rename = "ResourceContainerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_container_name: Option<Vec<StringFilter>>,
    /// <p>The details of a resource that doesn't have a specific subfield for the resource type defined.</p>
    #[serde(rename = "ResourceDetailsOther")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_details_other: Option<Vec<MapFilter>>,
    /// <p>The canonical identifier for the given resource type.</p>
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<Vec<StringFilter>>,
    /// <p>The canonical AWS partition name that the Region is assigned to.</p>
    #[serde(rename = "ResourcePartition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_partition: Option<Vec<StringFilter>>,
    /// <p>The canonical AWS external Region name where this resource is located.</p>
    #[serde(rename = "ResourceRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_region: Option<Vec<StringFilter>>,
    /// <p>A list of AWS tags associated with a resource at the time the finding was processed.</p>
    #[serde(rename = "ResourceTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_tags: Option<Vec<MapFilter>>,
    /// <p>Specifies the type of the resource that details are provided for.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<Vec<StringFilter>>,
    /// <p>The label of a finding's severity.</p>
    #[serde(rename = "SeverityLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity_label: Option<Vec<StringFilter>>,
    /// <p>A URL that links to a page about the current finding in the security-findings provider's solution.</p>
    #[serde(rename = "SourceUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<StringFilter>>,
    /// <p>The category of a threat intelligence indicator.</p>
    #[serde(rename = "ThreatIntelIndicatorCategory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_intel_indicator_category: Option<Vec<StringFilter>>,
    /// <p>The date/time of the last observation of a threat intelligence indicator.</p>
    #[serde(rename = "ThreatIntelIndicatorLastObservedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_intel_indicator_last_observed_at: Option<Vec<DateFilter>>,
    /// <p>The source of the threat intelligence.</p>
    #[serde(rename = "ThreatIntelIndicatorSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_intel_indicator_source: Option<Vec<StringFilter>>,
    /// <p>The URL for more details from the source of the threat intelligence.</p>
    #[serde(rename = "ThreatIntelIndicatorSourceUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_intel_indicator_source_url: Option<Vec<StringFilter>>,
    /// <p>The type of a threat intelligence indicator.</p>
    #[serde(rename = "ThreatIntelIndicatorType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_intel_indicator_type: Option<Vec<StringFilter>>,
    /// <p>The value of a threat intelligence indicator.</p>
    #[serde(rename = "ThreatIntelIndicatorValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_intel_indicator_value: Option<Vec<StringFilter>>,
    /// <p>A finding's title.</p>
    #[serde(rename = "Title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Vec<StringFilter>>,
    /// <p>A finding type in the format of <code>namespace/category/classifier</code> that classifies a finding.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<Vec<StringFilter>>,
    /// <p>An ISO8601-formatted timestamp that indicates when the security-findings provider last updated the finding record. </p>
    #[serde(rename = "UpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Vec<DateFilter>>,
    /// <p>A list of name/value string pairs associated with the finding. These are custom, user-defined fields added to a finding. </p>
    #[serde(rename = "UserDefinedFields")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_defined_fields: Option<Vec<MapFilter>>,
    /// <p>The veracity of a finding.</p>
    #[serde(rename = "VerificationState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_state: Option<Vec<StringFilter>>,
    /// <p>The workflow state of a finding.</p> <p>Note that this field is deprecated. To search for a finding based on its workflow status, use <code>WorkflowStatus</code>.</p>
    #[serde(rename = "WorkflowState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_state: Option<Vec<StringFilter>>,
    /// <p><p>The status of the investigation into a finding. Allowed values are the following.</p> <ul> <li> <p> <code>NEW</code> - The initial state of a finding, before it is reviewed.</p> <p>Security Hub also resets the workflow status from <code>NOTIFIED</code> or <code>RESOLVED</code> to <code>NEW</code> in the following cases:</p> <ul> <li> <p>The record state changes from <code>ARCHIVED</code> to <code>ACTIVE</code>.</p> </li> <li> <p>The compliance status changes from <code>PASSED</code> to either <code>WARNING</code>, <code>FAILED</code>, or <code>NOT_AVAILABLE</code>.</p> </li> </ul> </li> <li> <p> <code>NOTIFIED</code> - Indicates that the resource owner has been notified about the security issue. Used when the initial reviewer is not the resource owner, and needs intervention from the resource owner.</p> </li> <li> <p> <code>SUPPRESSED</code> - The finding will not be reviewed again and will not be acted upon.</p> </li> <li> <p> <code>RESOLVED</code> - The finding was reviewed and remediated and is now considered resolved. </p> </li> </ul></p>
    #[serde(rename = "WorkflowStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_status: Option<Vec<StringFilter>>,
}

/// <p>Identifies a finding to update using <code>BatchUpdateFindings</code>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsSecurityFindingIdentifier {
    /// <p>The identifier of the finding that was specified by the finding provider.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The ARN generated by Security Hub that uniquely identifies a product that generates findings. This can be the ARN for a third-party product that is integrated with Security Hub, or the ARN for a custom integration.</p>
    #[serde(rename = "ProductArn")]
    pub product_arn: String,
}

/// <p>A wrapper type for the topic's ARN.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsSnsTopicDetails {
    /// <p>The ID of an AWS managed customer master key (CMK) for Amazon SNS or a custom CMK.</p>
    #[serde(rename = "KmsMasterKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_master_key_id: Option<String>,
    /// <p>The subscription's owner.</p>
    #[serde(rename = "Owner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// <p>Subscription is an embedded property that describes the subscription endpoints of an Amazon SNS topic.</p>
    #[serde(rename = "Subscription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<Vec<AwsSnsTopicSubscription>>,
    /// <p>The name of the topic.</p>
    #[serde(rename = "TopicName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_name: Option<String>,
}

/// <p>A wrapper type for the attributes of an Amazon SNS subscription.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsSnsTopicSubscription {
    /// <p>The subscription's endpoint (format depends on the protocol).</p>
    #[serde(rename = "Endpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    /// <p>The subscription's protocol.</p>
    #[serde(rename = "Protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

/// <p>Data about a queue.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsSqsQueueDetails {
    /// <p>The ARN of the dead-letter queue to which Amazon SQS moves messages after the value of <code>maxReceiveCount</code> is exceeded. </p>
    #[serde(rename = "DeadLetterTargetArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dead_letter_target_arn: Option<String>,
    /// <p>The length of time, in seconds, for which Amazon SQS can reuse a data key to encrypt or decrypt messages before calling AWS KMS again.</p>
    #[serde(rename = "KmsDataKeyReusePeriodSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_data_key_reuse_period_seconds: Option<i64>,
    /// <p>The ID of an AWS managed customer master key (CMK) for Amazon SQS or a custom CMK.</p>
    #[serde(rename = "KmsMasterKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_master_key_id: Option<String>,
    /// <p>The name of the new queue.</p>
    #[serde(rename = "QueueName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_name: Option<String>,
}

/// <p>Provides the details about the compliance status for a patch.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsSsmComplianceSummary {
    /// <p>The type of resource for which the compliance was determined. For <code>AwsSsmPatchCompliance</code>, <code>ComplianceType</code> is <code>Patch</code>. </p>
    #[serde(rename = "ComplianceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_type: Option<String>,
    /// <p>For the patches that are compliant, the number that have a severity of <code>CRITICAL</code>.</p>
    #[serde(rename = "CompliantCriticalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliant_critical_count: Option<i64>,
    /// <p>For the patches that are compliant, the number that have a severity of <code>HIGH</code>.</p>
    #[serde(rename = "CompliantHighCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliant_high_count: Option<i64>,
    /// <p>For the patches that are compliant, the number that have a severity of <code>INFORMATIONAL</code>.</p>
    #[serde(rename = "CompliantInformationalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliant_informational_count: Option<i64>,
    /// <p>For the patches that are compliant, the number that have a severity of <code>LOW</code>.</p>
    #[serde(rename = "CompliantLowCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliant_low_count: Option<i64>,
    /// <p>For the patches that are compliant, the number that have a severity of <code>MEDIUM</code>.</p>
    #[serde(rename = "CompliantMediumCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliant_medium_count: Option<i64>,
    /// <p>For the patches that are compliant, the number that have a severity of <code>UNSPECIFIED</code>.</p>
    #[serde(rename = "CompliantUnspecifiedCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliant_unspecified_count: Option<i64>,
    /// <p>The type of execution that was used determine compliance.</p>
    #[serde(rename = "ExecutionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_type: Option<String>,
    /// <p>For the patch items that are noncompliant, the number of items that have a severity of <code>CRITICAL</code>.</p>
    #[serde(rename = "NonCompliantCriticalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_compliant_critical_count: Option<i64>,
    /// <p>For the patches that are noncompliant, the number that have a severity of <code>HIGH</code>.</p>
    #[serde(rename = "NonCompliantHighCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_compliant_high_count: Option<i64>,
    /// <p>For the patches that are noncompliant, the number that have a severity of <code>INFORMATIONAL</code>.</p>
    #[serde(rename = "NonCompliantInformationalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_compliant_informational_count: Option<i64>,
    /// <p>For the patches that are noncompliant, the number that have a severity of <code>LOW</code>.</p>
    #[serde(rename = "NonCompliantLowCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_compliant_low_count: Option<i64>,
    /// <p>For the patches that are noncompliant, the number that have a severity of <code>MEDIUM</code>.</p>
    #[serde(rename = "NonCompliantMediumCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_compliant_medium_count: Option<i64>,
    /// <p>For the patches that are noncompliant, the number that have a severity of <code>UNSPECIFIED</code>.</p>
    #[serde(rename = "NonCompliantUnspecifiedCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_compliant_unspecified_count: Option<i64>,
    /// <p>The highest severity for the patches.</p>
    #[serde(rename = "OverallSeverity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall_severity: Option<String>,
    /// <p>The identifier of the patch baseline. The patch baseline lists the patches that are approved for installation.</p>
    #[serde(rename = "PatchBaselineId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_baseline_id: Option<String>,
    /// <p>The identifier of the patch group for which compliance was determined. A patch group uses tags to group EC2 instances that should have the same patch compliance.</p>
    #[serde(rename = "PatchGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_group: Option<String>,
    /// <p><p>The current patch compliance status.</p> <p>The possible status values are:</p> <ul> <li> <p> <code>COMPLIANT</code> </p> </li> <li> <p> <code>NON<em>COMPLIANT</code> </p> </li> <li> <p> <code>UNSPECIFIED</em>DATA</code> </p> </li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Provides details about the compliance for a patch.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsSsmPatch {
    /// <p>The compliance status details for the patch.</p>
    #[serde(rename = "ComplianceSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_summary: Option<AwsSsmComplianceSummary>,
}

/// <p>Provides information about the state of a patch on an instance based on the patch baseline that was used to patch the instance.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsSsmPatchComplianceDetails {
    /// <p>Information about the status of a patch.</p>
    #[serde(rename = "Patch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch: Option<AwsSsmPatch>,
}

/// <p>Details about a WAF WebACL.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsWafWebAclDetails {
    /// <p>The action to perform if none of the rules contained in the WebACL match.</p>
    #[serde(rename = "DefaultAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_action: Option<String>,
    /// <p>A friendly name or description of the WebACL. You can't change the name of a WebACL after you create it.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>An array that contains the action for each rule in a WebACL, the priority of the rule, and the ID of the rule.</p>
    #[serde(rename = "Rules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<AwsWafWebAclRule>>,
    /// <p>A unique identifier for a WebACL.</p>
    #[serde(rename = "WebAclId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_acl_id: Option<String>,
}

/// <p>Details for a rule in a WAF WebACL.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsWafWebAclRule {
    /// <p>Specifies the action that CloudFront or AWS WAF takes when a web request matches the conditions in the rule. </p>
    #[serde(rename = "Action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<WafAction>,
    /// <p>Rules to exclude from a rule group.</p>
    #[serde(rename = "ExcludedRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_rules: Option<Vec<WafExcludedRule>>,
    /// <p>Use the <code>OverrideAction</code> to test your RuleGroup.</p> <p>Any rule in a RuleGroup can potentially block a request. If you set the <code>OverrideAction</code> to <code>None</code>, the RuleGroup blocks a request if any individual rule in the RuleGroup matches the request and is configured to block that request.</p> <p>However, if you first want to test the RuleGroup, set the <code>OverrideAction</code> to <code>Count</code>. The RuleGroup then overrides any block action specified by individual rules contained within the group. Instead of blocking matching requests, those requests are counted.</p> <p> <code>ActivatedRule</code>|<code>OverrideAction</code> applies only when updating or adding a RuleGroup to a WebACL. In this case you do not use <code>ActivatedRule</code>|<code>Action</code>. For all other update requests, <code>ActivatedRule</code>|<code>Action</code> is used instead of <code>ActivatedRule</code>|<code>OverrideAction</code>. </p>
    #[serde(rename = "OverrideAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_action: Option<WafOverrideAction>,
    /// <p>Specifies the order in which the rules in a WebACL are evaluated. Rules with a lower value for <code>Priority</code> are evaluated before rules with a higher value. The value must be a unique integer. If you add multiple rules to a WebACL, the values do not need to be consecutive.</p>
    #[serde(rename = "Priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    /// <p>The identifier for a rule.</p>
    #[serde(rename = "RuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
    /// <p>The rule type.</p> <p>Valid values: <code>REGULAR</code> | <code>RATE_BASED</code> | <code>GROUP</code> </p> <p>The default is <code>REGULAR</code>.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchDisableStandardsRequest {
    /// <p>The ARNs of the standards subscriptions to disable.</p>
    #[serde(rename = "StandardsSubscriptionArns")]
    pub standards_subscription_arns: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchDisableStandardsResponse {
    /// <p>The details of the standards subscriptions that were disabled.</p>
    #[serde(rename = "StandardsSubscriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standards_subscriptions: Option<Vec<StandardsSubscription>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchEnableStandardsRequest {
    /// <p>The list of standards checks to enable.</p>
    #[serde(rename = "StandardsSubscriptionRequests")]
    pub standards_subscription_requests: Vec<StandardsSubscriptionRequest>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchEnableStandardsResponse {
    /// <p>The details of the standards subscriptions that were enabled.</p>
    #[serde(rename = "StandardsSubscriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standards_subscriptions: Option<Vec<StandardsSubscription>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchImportFindingsRequest {
    /// <p>A list of findings to import. To successfully import a finding, it must follow the <a href="https://docs.aws.amazon.com/securityhub/latest/userguide/securityhub-findings-format.html">AWS Security Finding Format</a>. Maximum of 100 findings per request.</p>
    #[serde(rename = "Findings")]
    pub findings: Vec<AwsSecurityFinding>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchImportFindingsResponse {
    /// <p>The number of findings that failed to import.</p>
    #[serde(rename = "FailedCount")]
    pub failed_count: i64,
    /// <p>The list of findings that failed to import.</p>
    #[serde(rename = "FailedFindings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_findings: Option<Vec<ImportFindingsError>>,
    /// <p>The number of findings that were successfully imported.</p>
    #[serde(rename = "SuccessCount")]
    pub success_count: i64,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchUpdateFindingsRequest {
    /// <p>The updated value for the finding confidence. Confidence is defined as the likelihood that a finding accurately identifies the behavior or issue that it was intended to identify.</p> <p>Confidence is scored on a 0-100 basis using a ratio scale, where 0 means zero percent confidence and 100 means 100 percent confidence.</p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<i64>,
    /// <p>The updated value for the level of importance assigned to the resources associated with the findings.</p> <p>A score of 0 means that the underlying resources have no criticality, and a score of 100 is reserved for the most critical resources. </p>
    #[serde(rename = "Criticality")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub criticality: Option<i64>,
    /// <p>The list of findings to update. <code>BatchUpdateFindings</code> can be used to update up to 100 findings at a time.</p> <p>For each finding, the list provides the finding identifier and the ARN of the finding provider.</p>
    #[serde(rename = "FindingIdentifiers")]
    pub finding_identifiers: Vec<AwsSecurityFindingIdentifier>,
    #[serde(rename = "Note")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<NoteUpdate>,
    /// <p>A list of findings that are related to the updated findings.</p>
    #[serde(rename = "RelatedFindings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_findings: Option<Vec<RelatedFinding>>,
    /// <p>Used to update the finding severity.</p>
    #[serde(rename = "Severity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<SeverityUpdate>,
    /// <p><p>One or more finding types in the format of namespace/category/classifier that classify a finding.</p> <p>Valid namespace values are as follows.</p> <ul> <li> <p>Software and Configuration Checks</p> </li> <li> <p>TTPs</p> </li> <li> <p>Effects</p> </li> <li> <p>Unusual Behaviors</p> </li> <li> <p>Sensitive Data Identifications </p> </li> </ul></p>
    #[serde(rename = "Types")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<String>>,
    /// <p>A list of name/value string pairs associated with the finding. These are custom, user-defined fields added to a finding.</p>
    #[serde(rename = "UserDefinedFields")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_defined_fields: Option<::std::collections::HashMap<String, String>>,
    /// <p><p>Indicates the veracity of a finding.</p> <p>The available values for <code>VerificationState</code> are as follows.</p> <ul> <li> <p> <code>UNKNOWN</code> – The default disposition of a security finding</p> </li> <li> <p> <code>TRUE<em>POSITIVE</code> – The security finding is confirmed</p> </li> <li> <p> <code>FALSE</em>POSITIVE</code> – The security finding was determined to be a false alarm</p> </li> <li> <p> <code>BENIGN<em>POSITIVE</code> – A special case of <code>TRUE</em>POSITIVE</code> where the finding doesn&#39;t pose any threat, is expected, or both</p> </li> </ul></p>
    #[serde(rename = "VerificationState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_state: Option<String>,
    /// <p>Used to update the workflow status of a finding.</p> <p>The workflow status indicates the progress of the investigation into the finding. </p>
    #[serde(rename = "Workflow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow: Option<WorkflowUpdate>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchUpdateFindingsResponse {
    /// <p>The list of findings that were updated successfully.</p>
    #[serde(rename = "ProcessedFindings")]
    pub processed_findings: Vec<AwsSecurityFindingIdentifier>,
    /// <p>The list of findings that were not updated.</p>
    #[serde(rename = "UnprocessedFindings")]
    pub unprocessed_findings: Vec<BatchUpdateFindingsUnprocessedFinding>,
}

/// <p>A finding from a <code>BatchUpdateFindings</code> request that Security Hub was unable to update.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchUpdateFindingsUnprocessedFinding {
    /// <p>The code associated with the error.</p>
    #[serde(rename = "ErrorCode")]
    pub error_code: String,
    /// <p>The message associated with the error.</p>
    #[serde(rename = "ErrorMessage")]
    pub error_message: String,
    /// <p>The identifier of the finding that was not updated.</p>
    #[serde(rename = "FindingIdentifier")]
    pub finding_identifier: AwsSecurityFindingIdentifier,
}

/// <p>An occurrence of sensitive data detected in a Microsoft Excel workbook, comma-separated value (CSV) file, or tab-separated value (TSV) file.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Cell {
    /// <p>For a Microsoft Excel workbook, provides the location of the cell, as an absolute cell reference, that contains the data. For example, Sheet2!C5 for cell C5 on Sheet2.</p>
    #[serde(rename = "CellReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cell_reference: Option<String>,
    /// <p>The column number of the column that contains the data. For a Microsoft Excel workbook, the column number corresponds to the alphabetical column identifiers. For example, a value of 1 for Column corresponds to the A column in the workbook.</p>
    #[serde(rename = "Column")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<i64>,
    /// <p>The name of the column that contains the data.</p>
    #[serde(rename = "ColumnName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_name: Option<String>,
    /// <p>The row number of the row that contains the data.</p>
    #[serde(rename = "Row")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row: Option<i64>,
}

/// <p>An IPv4 CIDR block association.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CidrBlockAssociation {
    /// <p>The association ID for the IPv4 CIDR block.</p>
    #[serde(rename = "AssociationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,
    /// <p>The IPv4 CIDR block.</p>
    #[serde(rename = "CidrBlock")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_block: Option<String>,
    /// <p>Information about the state of the IPv4 CIDR block.</p>
    #[serde(rename = "CidrBlockState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_block_state: Option<String>,
}

/// <p>Information about a city.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct City {
    /// <p>The name of the city.</p>
    #[serde(rename = "CityName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city_name: Option<String>,
}

/// <p>Details about the sensitive data that was detected on the resource.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ClassificationResult {
    /// <p>Indicates whether there are additional occurrences of sensitive data that are not included in the finding. This occurs when the number of occurrences exceeds the maximum that can be included.</p>
    #[serde(rename = "AdditionalOccurrences")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_occurrences: Option<bool>,
    /// <p>Provides details about sensitive data that was identified based on customer-defined configuration.</p>
    #[serde(rename = "CustomDataIdentifiers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data_identifiers: Option<CustomDataIdentifiersResult>,
    /// <p>The type of content that the finding applies to.</p>
    #[serde(rename = "MimeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// <p>Provides details about sensitive data that was identified based on built-in configuration.</p>
    #[serde(rename = "SensitiveData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensitive_data: Option<Vec<SensitiveDataResult>>,
    /// <p>The total size in bytes of the affected data.</p>
    #[serde(rename = "SizeClassified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_classified: Option<i64>,
    /// <p>The current status of the sensitive data detection.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ClassificationStatus>,
}

/// <p>Provides details about the current status of the sensitive data detection.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ClassificationStatus {
    /// <p>The code that represents the status of the sensitive data detection.</p>
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// <p>A longer description of the current status of the sensitive data detection.</p>
    #[serde(rename = "Reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// <p>Contains finding details that are specific to control-based findings. Only returned for findings generated from controls.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Compliance {
    /// <p>For a control, the industry or regulatory framework requirements that are related to the control. The check for that control is aligned with these requirements.</p>
    #[serde(rename = "RelatedRequirements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_requirements: Option<Vec<String>>,
    /// <p><p>The result of a standards check.</p> <p>The valid values for <code>Status</code> are as follows.</p> <ul> <li> <ul> <li> <p> <code>PASSED</code> - Standards check passed for all evaluated resources.</p> </li> <li> <p> <code>WARNING</code> - Some information is missing or this check is not supported for your configuration.</p> </li> <li> <p> <code>FAILED</code> - Standards check failed for at least one evaluated resource.</p> </li> <li> <p> <code>NOT<em>AVAILABLE</code> - Check could not be performed due to a service outage, API error, or because the result of the AWS Config evaluation was <code>NOT</em>APPLICABLE</code>. If the AWS Config evaluation result was <code>NOT_APPLICABLE</code>, then after 3 days, Security Hub automatically archives the finding.</p> </li> </ul> </li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>For findings generated from controls, a list of reasons behind the value of <code>Status</code>. For the list of status reason codes and their meanings, see <a href="https://docs.aws.amazon.com/securityhub/latest/userguide/securityhub-standards-results.html#securityhub-standards-results-asff">Standards-related information in the ASFF</a> in the <i>AWS Security Hub User Guide</i>. </p>
    #[serde(rename = "StatusReasons")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reasons: Option<Vec<StatusReason>>,
}

/// <p>Container details related to a finding.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ContainerDetails {
    /// <p>The identifier of the image related to a finding.</p>
    #[serde(rename = "ImageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    /// <p>The name of the image related to a finding.</p>
    #[serde(rename = "ImageName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_name: Option<String>,
    /// <p>Indicates when the container started.</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "LaunchedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launched_at: Option<String>,
    /// <p>The name of the container related to a finding.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Information about a country.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Country {
    /// <p>The 2-letter ISO 3166 country code for the country.</p>
    #[serde(rename = "CountryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    /// <p>The name of the country.</p>
    #[serde(rename = "CountryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_name: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateActionTargetRequest {
    /// <p>The description for the custom action target.</p>
    #[serde(rename = "Description")]
    pub description: String,
    /// <p>The ID for the custom action target. Can contain up to 20 alphanumeric characters.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The name of the custom action target. Can contain up to 20 characters.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateActionTargetResponse {
    /// <p>The ARN for the custom action target.</p>
    #[serde(rename = "ActionTargetArn")]
    pub action_target_arn: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateInsightRequest {
    /// <p>One or more attributes used to filter the findings included in the insight. The insight only includes findings that match the criteria defined in the filters.</p>
    #[serde(rename = "Filters")]
    pub filters: AwsSecurityFindingFilters,
    /// <p>The attribute used to group the findings for the insight. The grouping attribute identifies the type of item that the insight applies to. For example, if an insight is grouped by resource identifier, then the insight produces a list of resource identifiers.</p>
    #[serde(rename = "GroupByAttribute")]
    pub group_by_attribute: String,
    /// <p>The name of the custom insight to create.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateInsightResponse {
    /// <p>The ARN of the insight created.</p>
    #[serde(rename = "InsightArn")]
    pub insight_arn: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateMembersRequest {
    /// <p>The list of accounts to associate with the Security Hub administrator account. For each account, the list includes the account ID and optionally the email address.</p>
    #[serde(rename = "AccountDetails")]
    pub account_details: Vec<AccountDetails>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateMembersResponse {
    /// <p>The list of AWS accounts that were not processed. For each account, the list includes the account ID and the email address.</p>
    #[serde(rename = "UnprocessedAccounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_accounts: Option<Vec<SecurityHubResult>>,
}

/// <p>The list of detected instances of sensitive data.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CustomDataIdentifiersDetections {
    /// <p>The ARN of the custom identifier that was used to detect the sensitive data.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The total number of occurrences of sensitive data that were detected.</p>
    #[serde(rename = "Count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// <p>he name of the custom identifier that detected the sensitive data.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Details about the sensitive data that was detected.</p>
    #[serde(rename = "Occurrences")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub occurrences: Option<Occurrences>,
}

/// <p>Contains an instance of sensitive data that was detected by a customer-defined identifier.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CustomDataIdentifiersResult {
    /// <p>The list of detected instances of sensitive data.</p>
    #[serde(rename = "Detections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detections: Option<Vec<CustomDataIdentifiersDetections>>,
    /// <p>The total number of occurrences of sensitive data.</p>
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
}

/// <p>CVSS scores from the advisory related to the vulnerability.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Cvss {
    /// <p>The base CVSS score.</p>
    #[serde(rename = "BaseScore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_score: Option<f64>,
    /// <p>The base scoring vector for the CVSS score.</p>
    #[serde(rename = "BaseVector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_vector: Option<String>,
    /// <p>The version of CVSS for the CVSS score.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>Provides details about sensitive data that was detected on a resource.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DataClassificationDetails {
    /// <p>The path to the folder or file that contains the sensitive data.</p>
    #[serde(rename = "DetailedResultsLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detailed_results_location: Option<String>,
    /// <p>The details about the sensitive data that was detected on the resource.</p>
    #[serde(rename = "Result")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<ClassificationResult>,
}

/// <p>A date filter for querying findings.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DateFilter {
    /// <p>A date range for the date filter.</p>
    #[serde(rename = "DateRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_range: Option<DateRange>,
    /// <p>An end date for the date filter.</p>
    #[serde(rename = "End")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    /// <p>A start date for the date filter.</p>
    #[serde(rename = "Start")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
}

/// <p>A date range for the date filter.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DateRange {
    /// <p>A date range unit for the date filter.</p>
    #[serde(rename = "Unit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    /// <p>A date range value for the date filter.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeclineInvitationsRequest {
    /// <p>The list of account IDs for the accounts from which to decline the invitations to Security Hub.</p>
    #[serde(rename = "AccountIds")]
    pub account_ids: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeclineInvitationsResponse {
    /// <p>The list of AWS accounts that were not processed. For each account, the list includes the account ID and the email address.</p>
    #[serde(rename = "UnprocessedAccounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_accounts: Option<Vec<SecurityHubResult>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteActionTargetRequest {
    /// <p>The ARN of the custom action target to delete.</p>
    #[serde(rename = "ActionTargetArn")]
    pub action_target_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteActionTargetResponse {
    /// <p>The ARN of the custom action target that was deleted.</p>
    #[serde(rename = "ActionTargetArn")]
    pub action_target_arn: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteInsightRequest {
    /// <p>The ARN of the insight to delete.</p>
    #[serde(rename = "InsightArn")]
    pub insight_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteInsightResponse {
    /// <p>The ARN of the insight that was deleted.</p>
    #[serde(rename = "InsightArn")]
    pub insight_arn: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteInvitationsRequest {
    /// <p>The list of the account IDs that sent the invitations to delete.</p>
    #[serde(rename = "AccountIds")]
    pub account_ids: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteInvitationsResponse {
    /// <p>The list of AWS accounts for which the invitations were not deleted. For each account, the list includes the account ID and the email address.</p>
    #[serde(rename = "UnprocessedAccounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_accounts: Option<Vec<SecurityHubResult>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteMembersRequest {
    /// <p>The list of account IDs for the member accounts to delete.</p>
    #[serde(rename = "AccountIds")]
    pub account_ids: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteMembersResponse {
    /// <p>The list of AWS accounts that were not deleted. For each account, the list includes the account ID and the email address.</p>
    #[serde(rename = "UnprocessedAccounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_accounts: Option<Vec<SecurityHubResult>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeActionTargetsRequest {
    /// <p>A list of custom action target ARNs for the custom action targets to retrieve.</p>
    #[serde(rename = "ActionTargetArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_target_arns: Option<Vec<String>>,
    /// <p>The maximum number of results to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token that is required for pagination. On your first call to the <code>DescribeActionTargets</code> operation, set the value of this parameter to <code>NULL</code>.</p> <p>For subsequent calls to the operation, to continue listing data, set the value of this parameter to the value returned from the previous response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeActionTargetsResponse {
    /// <p>A list of <code>ActionTarget</code> objects. Each object includes the <code>ActionTargetArn</code>, <code>Description</code>, and <code>Name</code> of a custom action target available in Security Hub.</p>
    #[serde(rename = "ActionTargets")]
    pub action_targets: Vec<ActionTarget>,
    /// <p>The pagination token to use to request the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeHubRequest {
    /// <p>The ARN of the Hub resource to retrieve.</p>
    #[serde(rename = "HubArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hub_arn: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeHubResponse {
    /// <p>Whether to automatically enable new controls when they are added to standards that are enabled.</p> <p>If set to <code>true</code>, then new controls for enabled standards are enabled automatically. If set to <code>false</code>, then new controls are not enabled.</p>
    #[serde(rename = "AutoEnableControls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_enable_controls: Option<bool>,
    /// <p>The ARN of the Hub resource that was retrieved.</p>
    #[serde(rename = "HubArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hub_arn: Option<String>,
    /// <p>The date and time when Security Hub was enabled in the account.</p>
    #[serde(rename = "SubscribedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribed_at: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeOrganizationConfigurationRequest {}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeOrganizationConfigurationResponse {
    /// <p>Whether to automatically enable Security Hub for new accounts in the organization.</p> <p>If set to <code>true</code>, then Security Hub is enabled for new accounts. If set to false, then new accounts are not added automatically.</p>
    #[serde(rename = "AutoEnable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_enable: Option<bool>,
    /// <p>Whether the maximum number of allowed member accounts are already associated with the Security Hub administrator account.</p>
    #[serde(rename = "MemberAccountLimitReached")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_account_limit_reached: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeProductsRequest {
    /// <p>The maximum number of results to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token that is required for pagination. On your first call to the <code>DescribeProducts</code> operation, set the value of this parameter to <code>NULL</code>.</p> <p>For subsequent calls to the operation, to continue listing data, set the value of this parameter to the value returned from the previous response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ARN of the integration to return.</p>
    #[serde(rename = "ProductArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_arn: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeProductsResponse {
    /// <p>The pagination token to use to request the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of products, including details for each product.</p>
    #[serde(rename = "Products")]
    pub products: Vec<Product>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeStandardsControlsRequest {
    /// <p>The maximum number of security standard controls to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token that is required for pagination. On your first call to the <code>DescribeStandardsControls</code> operation, set the value of this parameter to <code>NULL</code>.</p> <p>For subsequent calls to the operation, to continue listing data, set the value of this parameter to the value returned from the previous response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ARN of a resource that represents your subscription to a supported standard. To get the subscription ARNs of the standards you have enabled, use the <code> <a>GetEnabledStandards</a> </code> operation.</p>
    #[serde(rename = "StandardsSubscriptionArn")]
    pub standards_subscription_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeStandardsControlsResponse {
    /// <p>A list of security standards controls.</p>
    #[serde(rename = "Controls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controls: Option<Vec<StandardsControl>>,
    /// <p>The pagination token to use to request the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeStandardsRequest {
    /// <p>The maximum number of standards to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token that is required for pagination. On your first call to the <code>DescribeStandards</code> operation, set the value of this parameter to <code>NULL</code>.</p> <p>For subsequent calls to the operation, to continue listing data, set the value of this parameter to the value returned from the previous response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeStandardsResponse {
    /// <p>The pagination token to use to request the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of available standards.</p>
    #[serde(rename = "Standards")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standards: Option<Vec<Standard>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisableImportFindingsForProductRequest {
    /// <p>The ARN of the integrated product to disable the integration for.</p>
    #[serde(rename = "ProductSubscriptionArn")]
    pub product_subscription_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisableImportFindingsForProductResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisableOrganizationAdminAccountRequest {
    /// <p>The AWS account identifier of the Security Hub administrator account.</p>
    #[serde(rename = "AdminAccountId")]
    pub admin_account_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisableOrganizationAdminAccountResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisableSecurityHubRequest {}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisableSecurityHubResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateFromAdministratorAccountRequest {}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisassociateFromAdministratorAccountResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateFromMasterAccountRequest {}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisassociateFromMasterAccountResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateMembersRequest {
    /// <p>The account IDs of the member accounts to disassociate from the administrator account.</p>
    #[serde(rename = "AccountIds")]
    pub account_ids: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisassociateMembersResponse {}

/// <p>Provided if <code>ActionType</code> is <code>DNS_REQUEST</code>. It provides details about the DNS request that was detected.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DnsRequestAction {
    /// <p>Indicates whether the DNS request was blocked.</p>
    #[serde(rename = "Blocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked: Option<bool>,
    /// <p>The DNS domain that is associated with the DNS request.</p>
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// <p>The protocol that was used for the DNS request.</p>
    #[serde(rename = "Protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EnableImportFindingsForProductRequest {
    /// <p>The ARN of the product to enable the integration for.</p>
    #[serde(rename = "ProductArn")]
    pub product_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EnableImportFindingsForProductResponse {
    /// <p>The ARN of your subscription to the product to enable integrations for.</p>
    #[serde(rename = "ProductSubscriptionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_subscription_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EnableOrganizationAdminAccountRequest {
    /// <p>The AWS account identifier of the account to designate as the Security Hub administrator account.</p>
    #[serde(rename = "AdminAccountId")]
    pub admin_account_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EnableOrganizationAdminAccountResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EnableSecurityHubRequest {
    /// <p>Whether to enable the security standards that Security Hub has designated as automatically enabled. If you do not provide a value for <code>EnableDefaultStandards</code>, it is set to <code>true</code>. To not enable the automatically enabled standards, set <code>EnableDefaultStandards</code> to <code>false</code>.</p>
    #[serde(rename = "EnableDefaultStandards")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_default_standards: Option<bool>,
    /// <p>The tags to add to the hub resource when you enable Security Hub.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EnableSecurityHubResponse {}

/// <p>In a <code>BatchImportFindings</code> request, finding providers use <code>FindingProviderFields</code> to provide and update values for confidence, criticality, related findings, severity, and types.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct FindingProviderFields {
    /// <p>A finding's confidence. Confidence is defined as the likelihood that a finding accurately identifies the behavior or issue that it was intended to identify.</p> <p>Confidence is scored on a 0-100 basis using a ratio scale, where 0 means zero percent confidence and 100 means 100 percent confidence.</p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<i64>,
    /// <p>The level of importance assigned to the resources associated with the finding.</p> <p>A score of 0 means that the underlying resources have no criticality, and a score of 100 is reserved for the most critical resources.</p>
    #[serde(rename = "Criticality")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub criticality: Option<i64>,
    /// <p>A list of findings that are related to the current finding.</p>
    #[serde(rename = "RelatedFindings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_findings: Option<Vec<RelatedFinding>>,
    /// <p>The severity of a finding.</p>
    #[serde(rename = "Severity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<FindingProviderSeverity>,
    /// <p>One or more finding types in the format of <code>namespace/category/classifier</code> that classify a finding.</p> <p>Valid namespace values are: Software and Configuration Checks | TTPs | Effects | Unusual Behaviors | Sensitive Data Identifications</p>
    #[serde(rename = "Types")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<String>>,
}

/// <p>The severity assigned to the finding by the finding provider.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct FindingProviderSeverity {
    /// <p>The severity label assigned to the finding by the finding provider.</p>
    #[serde(rename = "Label")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// <p>The finding provider's original value for the severity.</p>
    #[serde(rename = "Original")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original: Option<String>,
}

/// <p>Provides the latitude and longitude coordinates of a location.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct GeoLocation {
    /// <p>The latitude of the location.</p>
    #[serde(rename = "Lat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lat: Option<f64>,
    /// <p>The longitude of the location.</p>
    #[serde(rename = "Lon")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lon: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetAdministratorAccountRequest {}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetAdministratorAccountResponse {
    #[serde(rename = "Administrator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub administrator: Option<Invitation>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetEnabledStandardsRequest {
    /// <p>The maximum number of results to return in the response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token that is required for pagination. On your first call to the <code>GetEnabledStandards</code> operation, set the value of this parameter to <code>NULL</code>.</p> <p>For subsequent calls to the operation, to continue listing data, set the value of this parameter to the value returned from the previous response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of the standards subscription ARNs for the standards to retrieve.</p>
    #[serde(rename = "StandardsSubscriptionArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standards_subscription_arns: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetEnabledStandardsResponse {
    /// <p>The pagination token to use to request the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of <code>StandardsSubscriptions</code> objects that include information about the enabled standards.</p>
    #[serde(rename = "StandardsSubscriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standards_subscriptions: Option<Vec<StandardsSubscription>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetFindingsRequest {
    /// <p>The finding attributes used to define a condition to filter the returned findings.</p> <p>You can filter by up to 10 finding attributes. For each attribute, you can provide up to 20 filter values.</p> <p>Note that in the available filter fields, <code>WorkflowState</code> is deprecated. To search for a finding based on its workflow status, use <code>WorkflowStatus</code>.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<AwsSecurityFindingFilters>,
    /// <p>The maximum number of findings to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token that is required for pagination. On your first call to the <code>GetFindings</code> operation, set the value of this parameter to <code>NULL</code>.</p> <p>For subsequent calls to the operation, to continue listing data, set the value of this parameter to the value returned from the previous response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The finding attributes used to sort the list of returned findings.</p>
    #[serde(rename = "SortCriteria")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_criteria: Option<Vec<SortCriterion>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetFindingsResponse {
    /// <p>The findings that matched the filters specified in the request.</p>
    #[serde(rename = "Findings")]
    pub findings: Vec<AwsSecurityFinding>,
    /// <p>The pagination token to use to request the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetInsightResultsRequest {
    /// <p>The ARN of the insight for which to return results.</p>
    #[serde(rename = "InsightArn")]
    pub insight_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetInsightResultsResponse {
    /// <p>The insight results returned by the operation.</p>
    #[serde(rename = "InsightResults")]
    pub insight_results: InsightResults,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetInsightsRequest {
    /// <p>The ARNs of the insights to describe. If you do not provide any insight ARNs, then <code>GetInsights</code> returns all of your custom insights. It does not return any managed insights.</p>
    #[serde(rename = "InsightArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insight_arns: Option<Vec<String>>,
    /// <p>The maximum number of items to return in the response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token that is required for pagination. On your first call to the <code>GetInsights</code> operation, set the value of this parameter to <code>NULL</code>.</p> <p>For subsequent calls to the operation, to continue listing data, set the value of this parameter to the value returned from the previous response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetInsightsResponse {
    /// <p>The insights returned by the operation.</p>
    #[serde(rename = "Insights")]
    pub insights: Vec<Insight>,
    /// <p>The pagination token to use to request the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetInvitationsCountRequest {}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetInvitationsCountResponse {
    /// <p>The number of all membership invitations sent to this Security Hub member account, not including the currently accepted invitation.</p>
    #[serde(rename = "InvitationsCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitations_count: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetMasterAccountRequest {}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetMasterAccountResponse {
    /// <p>A list of details about the Security Hub administrator account for the current member account. </p>
    #[serde(rename = "Master")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master: Option<Invitation>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetMembersRequest {
    /// <p>The list of account IDs for the Security Hub member accounts to return the details for. </p>
    #[serde(rename = "AccountIds")]
    pub account_ids: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetMembersResponse {
    /// <p>The list of details about the Security Hub member accounts.</p>
    #[serde(rename = "Members")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<Member>>,
    /// <p>The list of AWS accounts that could not be processed. For each account, the list includes the account ID and the email address.</p>
    #[serde(rename = "UnprocessedAccounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_accounts: Option<Vec<SecurityHubResult>>,
}

/// <p>An Internet Control Message Protocol (ICMP) type and code.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct IcmpTypeCode {
    /// <p>The ICMP code for which to deny or allow access. To deny or allow all codes, use the value -1.</p>
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<i64>,
    /// <p>The ICMP type for which to deny or allow access. To deny or allow all types, use the value -1.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<i64>,
}

/// <p>The list of the findings that cannot be imported. For each finding, the list provides the error.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ImportFindingsError {
    /// <p>The code of the error returned by the <code>BatchImportFindings</code> operation.</p>
    #[serde(rename = "ErrorCode")]
    pub error_code: String,
    /// <p>The message of the error returned by the <code>BatchImportFindings</code> operation.</p>
    #[serde(rename = "ErrorMessage")]
    pub error_message: String,
    /// <p>The identifier of the finding that could not be updated.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

/// <p>Contains information about a Security Hub insight.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Insight {
    /// <p>One or more attributes used to filter the findings included in the insight. The insight only includes findings that match the criteria defined in the filters.</p>
    #[serde(rename = "Filters")]
    pub filters: AwsSecurityFindingFilters,
    /// <p>The grouping attribute for the insight's findings. Indicates how to group the matching findings, and identifies the type of item that the insight applies to. For example, if an insight is grouped by resource identifier, then the insight produces a list of resource identifiers.</p>
    #[serde(rename = "GroupByAttribute")]
    pub group_by_attribute: String,
    /// <p>The ARN of a Security Hub insight.</p>
    #[serde(rename = "InsightArn")]
    pub insight_arn: String,
    /// <p>The name of a Security Hub insight.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

/// <p>The insight result values returned by the <code>GetInsightResults</code> operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InsightResultValue {
    /// <p>The number of findings returned for each <code>GroupByAttributeValue</code>.</p>
    #[serde(rename = "Count")]
    pub count: i64,
    /// <p>The value of the attribute that the findings are grouped by for the insight whose results are returned by the <code>GetInsightResults</code> operation.</p>
    #[serde(rename = "GroupByAttributeValue")]
    pub group_by_attribute_value: String,
}

/// <p>The insight results returned by the <code>GetInsightResults</code> operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InsightResults {
    /// <p>The attribute that the findings are grouped by for the insight whose results are returned by the <code>GetInsightResults</code> operation.</p>
    #[serde(rename = "GroupByAttribute")]
    pub group_by_attribute: String,
    /// <p>The ARN of the insight whose results are returned by the <code>GetInsightResults</code> operation.</p>
    #[serde(rename = "InsightArn")]
    pub insight_arn: String,
    /// <p>The list of insight result values returned by the <code>GetInsightResults</code> operation.</p>
    #[serde(rename = "ResultValues")]
    pub result_values: Vec<InsightResultValue>,
}

/// <p>Details about an invitation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Invitation {
    /// <p>The account ID of the Security Hub administrator account that the invitation was sent from.</p>
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>The ID of the invitation sent to the member account.</p>
    #[serde(rename = "InvitationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitation_id: Option<String>,
    /// <p>The timestamp of when the invitation was sent.</p>
    #[serde(rename = "InvitedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invited_at: Option<f64>,
    /// <p>The current status of the association between the member and administrator accounts.</p>
    #[serde(rename = "MemberStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_status: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct InviteMembersRequest {
    /// <p>The list of account IDs of the AWS accounts to invite to Security Hub as members. </p>
    #[serde(rename = "AccountIds")]
    pub account_ids: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InviteMembersResponse {
    /// <p>The list of AWS accounts that could not be processed. For each account, the list includes the account ID and the email address.</p>
    #[serde(rename = "UnprocessedAccounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_accounts: Option<Vec<SecurityHubResult>>,
}

/// <p>The IP filter for querying findings.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct IpFilter {
    /// <p>A finding's CIDR value.</p>
    #[serde(rename = "Cidr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr: Option<String>,
}

/// <p>Provides information about an internet provider.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct IpOrganizationDetails {
    /// <p>The Autonomous System Number (ASN) of the internet provider</p>
    #[serde(rename = "Asn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn: Option<i64>,
    /// <p>The name of the organization that registered the ASN.</p>
    #[serde(rename = "AsnOrg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn_org: Option<String>,
    /// <p>The ISP information for the internet provider.</p>
    #[serde(rename = "Isp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isp: Option<String>,
    /// <p>The name of the internet provider.</p>
    #[serde(rename = "Org")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org: Option<String>,
}

/// <p>An IPV6 CIDR block association.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Ipv6CidrBlockAssociation {
    /// <p>The association ID for the IPv6 CIDR block.</p>
    #[serde(rename = "AssociationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,
    /// <p>Information about the state of the CIDR block.</p>
    #[serde(rename = "CidrBlockState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_block_state: Option<String>,
    /// <p>The IPv6 CIDR block.</p>
    #[serde(rename = "Ipv6CidrBlock")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv_6_cidr_block: Option<String>,
}

/// <p>A keyword filter for querying findings.</p>
#[derive(Clone, Debug, Default, PartialEq)]
pub struct KeywordFilter {
    /// <p>A value for the keyword.</p>
    pub value: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListEnabledProductsForImportRequest {
    /// <p>The maximum number of items to return in the response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token that is required for pagination. On your first call to the <code>ListEnabledProductsForImport</code> operation, set the value of this parameter to <code>NULL</code>.</p> <p>For subsequent calls to the operation, to continue listing data, set the value of this parameter to the value returned from the previous response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListEnabledProductsForImportResponse {
    /// <p>The pagination token to use to request the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of ARNs for the resources that represent your subscriptions to products. </p>
    #[serde(rename = "ProductSubscriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_subscriptions: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListInvitationsRequest {
    /// <p>The maximum number of items to return in the response. </p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token that is required for pagination. On your first call to the <code>ListInvitations</code> operation, set the value of this parameter to <code>NULL</code>.</p> <p>For subsequent calls to the operation, to continue listing data, set the value of this parameter to the value returned from the previous response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListInvitationsResponse {
    /// <p>The details of the invitations returned by the operation.</p>
    #[serde(rename = "Invitations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitations: Option<Vec<Invitation>>,
    /// <p>The pagination token to use to request the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListMembersRequest {
    /// <p>The maximum number of items to return in the response. </p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token that is required for pagination. On your first call to the <code>ListMembers</code> operation, set the value of this parameter to <code>NULL</code>.</p> <p>For subsequent calls to the operation, to continue listing data, set the value of this parameter to the value returned from the previous response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Specifies which member accounts to include in the response based on their relationship status with the administrator account. The default value is <code>TRUE</code>.</p> <p>If <code>OnlyAssociated</code> is set to <code>TRUE</code>, the response includes member accounts whose relationship status with the administrator account is set to <code>ENABLED</code>.</p> <p>If <code>OnlyAssociated</code> is set to <code>FALSE</code>, the response includes all existing member accounts. </p>
    #[serde(rename = "OnlyAssociated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_associated: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListMembersResponse {
    /// <p>Member details returned by the operation.</p>
    #[serde(rename = "Members")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<Member>>,
    /// <p>The pagination token to use to request the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListOrganizationAdminAccountsRequest {
    /// <p>The maximum number of items to return in the response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token that is required for pagination. On your first call to the <code>ListOrganizationAdminAccounts</code> operation, set the value of this parameter to <code>NULL</code>. For subsequent calls to the operation, to continue listing data, set the value of this parameter to the value returned from the previous response. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListOrganizationAdminAccountsResponse {
    /// <p>The list of Security Hub administrator accounts.</p>
    #[serde(rename = "AdminAccounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_accounts: Option<Vec<AdminAccount>>,
    /// <p>The pagination token to use to request the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>The ARN of the resource to retrieve tags for.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>The tags associated with a resource.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Information about the state of the load balancer.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct LoadBalancerState {
    /// <p>The state code. The initial state of the load balancer is provisioning.</p> <p>After the load balancer is fully set up and ready to route traffic, its state is active.</p> <p>If the load balancer could not be set up, its state is failed. </p>
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// <p>A description of the state.</p>
    #[serde(rename = "Reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// <p>A list of malware related to a finding.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Malware {
    /// <p>The name of the malware that was observed.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The file system path of the malware that was observed.</p>
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// <p>The state of the malware that was observed.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The type of the malware that was observed.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>A map filter for querying findings. Each map filter provides the field to check, the value to look for, and the comparison operator.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MapFilter {
    /// <p>The condition to apply to the key value when querying for findings with a map filter.</p> <p>To search for values that exactly match the filter value, use <code>EQUALS</code>. For example, for the <code>ResourceTags</code> field, the filter <code>Department EQUALS Security</code> matches findings that have the value <code>Security</code> for the tag <code>Department</code>.</p> <p>To search for values other than the filter value, use <code>NOT_EQUALS</code>. For example, for the <code>ResourceTags</code> field, the filter <code>Department NOT_EQUALS Finance</code> matches findings that do not have the value <code>Finance</code> for the tag <code>Department</code>.</p> <p> <code>EQUALS</code> filters on the same field are joined by <code>OR</code>. A finding matches if it matches any one of those filters.</p> <p> <code>NOT_EQUALS</code> filters on the same field are joined by <code>AND</code>. A finding matches only if it matches all of those filters.</p> <p>You cannot have both an <code>EQUALS</code> filter and a <code>NOT_EQUALS</code> filter on the same field.</p>
    #[serde(rename = "Comparison")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison: Option<String>,
    /// <p>The key of the map filter. For example, for <code>ResourceTags</code>, <code>Key</code> identifies the name of the tag. For <code>UserDefinedFields</code>, <code>Key</code> is the name of the field.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The value for the key in the map filter. Filter values are case sensitive. For example, one of the values for a tag called <code>Department</code> might be <code>Security</code>. If you provide <code>security</code> as the filter value, then there is no match.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>The details about a member account.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Member {
    /// <p>The AWS account ID of the member account.</p>
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>The AWS account ID of the Security Hub administrator account associated with this member account.</p>
    #[serde(rename = "AdministratorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub administrator_id: Option<String>,
    /// <p>The email address of the member account.</p>
    #[serde(rename = "Email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// <p>A timestamp for the date and time when the invitation was sent to the member account.</p>
    #[serde(rename = "InvitedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invited_at: Option<f64>,
    /// <p><p>The status of the relationship between the member account and its administrator account. </p> <p>The status can have one of the following values:</p> <ul> <li> <p> <code>CREATED</code> - Indicates that the administrator account added the member account, but has not yet invited the member account.</p> </li> <li> <p> <code>INVITED</code> - Indicates that the administrator account invited the member account. The member account has not yet responded to the invitation.</p> </li> <li> <p> <code>ENABLED</code> - Indicates that the member account is currently active. For manually invited member accounts, indicates that the member account accepted the invitation.</p> </li> <li> <p> <code>REMOVED</code> - Indicates that the administrator account disassociated the member account.</p> </li> <li> <p> <code>RESIGNED</code> - Indicates that the member account disassociated themselves from the administrator account.</p> </li> <li> <p> <code>DELETED</code> - Indicates that the administrator account deleted the member account.</p> </li> </ul></p>
    #[serde(rename = "MemberStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_status: Option<String>,
    /// <p>The timestamp for the date and time when the member account was updated.</p>
    #[serde(rename = "UpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

/// <p>The details of network-related information about a finding.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Network {
    /// <p>The destination domain of network-related information about a finding.</p>
    #[serde(rename = "DestinationDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_domain: Option<String>,
    /// <p>The destination IPv4 address of network-related information about a finding.</p>
    #[serde(rename = "DestinationIpV4")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_ip_v4: Option<String>,
    /// <p>The destination IPv6 address of network-related information about a finding.</p>
    #[serde(rename = "DestinationIpV6")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_ip_v6: Option<String>,
    /// <p>The destination port of network-related information about a finding.</p>
    #[serde(rename = "DestinationPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_port: Option<i64>,
    /// <p>The direction of network traffic associated with a finding.</p>
    #[serde(rename = "Direction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    /// <p>The range of open ports that is present on the network.</p>
    #[serde(rename = "OpenPortRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_port_range: Option<PortRange>,
    /// <p>The protocol of network-related information about a finding.</p>
    #[serde(rename = "Protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// <p>The source domain of network-related information about a finding.</p>
    #[serde(rename = "SourceDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_domain: Option<String>,
    /// <p>The source IPv4 address of network-related information about a finding.</p>
    #[serde(rename = "SourceIpV4")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ip_v4: Option<String>,
    /// <p>The source IPv6 address of network-related information about a finding.</p>
    #[serde(rename = "SourceIpV6")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ip_v6: Option<String>,
    /// <p>The source media access control (MAC) address of network-related information about a finding.</p>
    #[serde(rename = "SourceMac")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_mac: Option<String>,
    /// <p>The source port of network-related information about a finding.</p>
    #[serde(rename = "SourcePort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_port: Option<i64>,
}

/// <p>Provided if <code>ActionType</code> is <code>NETWORK_CONNECTION</code>. It provides details about the attempted network connection that was detected.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct NetworkConnectionAction {
    /// <p>Indicates whether the network connection attempt was blocked.</p>
    #[serde(rename = "Blocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked: Option<bool>,
    /// <p>The direction of the network connection request (<code>IN</code> or <code>OUT</code>).</p>
    #[serde(rename = "ConnectionDirection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_direction: Option<String>,
    /// <p>Information about the port on the EC2 instance.</p>
    #[serde(rename = "LocalPortDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_port_details: Option<ActionLocalPortDetails>,
    /// <p>The protocol used to make the network connection request.</p>
    #[serde(rename = "Protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// <p>Information about the remote IP address that issued the network connection request.</p>
    #[serde(rename = "RemoteIpDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_ip_details: Option<ActionRemoteIpDetails>,
    /// <p>Information about the port on the remote IP address.</p>
    #[serde(rename = "RemotePortDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_port_details: Option<ActionRemotePortDetails>,
}

/// <p>Details about a network path component that occurs before or after the current component.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct NetworkHeader {
    /// <p>Information about the destination of the component.</p>
    #[serde(rename = "Destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<NetworkPathComponentDetails>,
    /// <p>The protocol used for the component.</p>
    #[serde(rename = "Protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// <p>Information about the origin of the component.</p>
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<NetworkPathComponentDetails>,
}

/// <p>Information about a network path component.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct NetworkPathComponent {
    /// <p>The identifier of a component in the network path.</p>
    #[serde(rename = "ComponentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_id: Option<String>,
    /// <p>The type of component.</p>
    #[serde(rename = "ComponentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_type: Option<String>,
    /// <p>Information about the component that comes after the current component in the network path.</p>
    #[serde(rename = "Egress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress: Option<NetworkHeader>,
    /// <p>Information about the component that comes before the current node in the network path.</p>
    #[serde(rename = "Ingress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingress: Option<NetworkHeader>,
}

/// <p>Information about the destination of the next component in the network path.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct NetworkPathComponentDetails {
    /// <p>The IP addresses of the destination.</p>
    #[serde(rename = "Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Vec<String>>,
    /// <p>A list of port ranges for the destination.</p>
    #[serde(rename = "PortRanges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_ranges: Option<Vec<PortRange>>,
}

/// <p>A user-defined note added to a finding.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Note {
    /// <p>The text of a note.</p>
    #[serde(rename = "Text")]
    pub text: String,
    /// <p>The timestamp of when the note was updated.</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "UpdatedAt")]
    pub updated_at: String,
    /// <p>The principal that created a note.</p>
    #[serde(rename = "UpdatedBy")]
    pub updated_by: String,
}

/// <p>The updated note.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct NoteUpdate {
    /// <p>The updated note text.</p>
    #[serde(rename = "Text")]
    pub text: String,
    /// <p>The principal that updated the note.</p>
    #[serde(rename = "UpdatedBy")]
    pub updated_by: String,
}

/// <p>A number filter for querying findings.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct NumberFilter {
    /// <p>The equal-to condition to be applied to a single field when querying for findings.</p>
    #[serde(rename = "Eq")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eq: Option<f64>,
    /// <p>The greater-than-equal condition to be applied to a single field when querying for findings. </p>
    #[serde(rename = "Gte")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gte: Option<f64>,
    /// <p>The less-than-equal condition to be applied to a single field when querying for findings. </p>
    #[serde(rename = "Lte")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lte: Option<f64>,
}

/// <p>The detected occurrences of sensitive data.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Occurrences {
    /// <p>Occurrences of sensitive data detected in Microsoft Excel workbooks, comma-separated value (CSV) files, or tab-separated value (TSV) files.</p>
    #[serde(rename = "Cells")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cells: Option<Vec<Cell>>,
    /// <p>Occurrences of sensitive data detected in a non-binary text file or a Microsoft Word file. Non-binary text files include files such as HTML, XML, JSON, and TXT files.</p>
    #[serde(rename = "LineRanges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_ranges: Option<Vec<Range>>,
    /// <p>Occurrences of sensitive data detected in a binary text file.</p>
    #[serde(rename = "OffsetRanges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_ranges: Option<Vec<Range>>,
    /// <p>Occurrences of sensitive data in an Adobe Portable Document Format (PDF) file.</p>
    #[serde(rename = "Pages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pages: Option<Vec<Page>>,
    /// <p>Occurrences of sensitive data in an Apache Avro object container or an Apache Parquet file.</p>
    #[serde(rename = "Records")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<Record>>,
}

/// <p>An occurrence of sensitive data in an Adobe Portable Document Format (PDF) file.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Page {
    /// <p>An occurrence of sensitive data detected in a non-binary text file or a Microsoft Word file. Non-binary text files include files such as HTML, XML, JSON, and TXT files.</p>
    #[serde(rename = "LineRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_range: Option<Range>,
    /// <p>An occurrence of sensitive data detected in a binary text file.</p>
    #[serde(rename = "OffsetRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_range: Option<Range>,
    /// <p>The page number of the page that contains the sensitive data.</p>
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i64>,
}

/// <p>Provides an overview of the patch compliance status for an instance against a selected compliance standard.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PatchSummary {
    /// <p>The number of patches from the compliance standard that failed to install.</p>
    #[serde(rename = "FailedCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_count: Option<i64>,
    /// <p>The identifier of the compliance standard that was used to determine the patch compliance status.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The number of patches from the compliance standard that were installed successfully.</p>
    #[serde(rename = "InstalledCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installed_count: Option<i64>,
    /// <p>The number of installed patches that are not part of the compliance standard.</p>
    #[serde(rename = "InstalledOtherCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installed_other_count: Option<i64>,
    /// <p>The number of patches that were applied, but that require the instance to be rebooted in order to be marked as installed.</p>
    #[serde(rename = "InstalledPendingReboot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installed_pending_reboot: Option<i64>,
    /// <p>The number of patches that are installed but are also on a list of patches that the customer rejected.</p>
    #[serde(rename = "InstalledRejectedCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installed_rejected_count: Option<i64>,
    /// <p>The number of patches that are part of the compliance standard but are not installed. The count includes patches that failed to install.</p>
    #[serde(rename = "MissingCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_count: Option<i64>,
    /// <p>The type of patch operation performed. For Patch Manager, the values are <code>SCAN</code> and <code>INSTALL</code>. </p>
    #[serde(rename = "Operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    /// <p>Indicates when the operation completed.</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "OperationEndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_end_time: Option<String>,
    /// <p>Indicates when the operation started.</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "OperationStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_start_time: Option<String>,
    /// <p>The reboot option specified for the instance.</p>
    #[serde(rename = "RebootOption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reboot_option: Option<String>,
}

/// <p>Provided if <code>ActionType</code> is <code>PORT_PROBE</code>. It provides details about the attempted port probe that was detected.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PortProbeAction {
    /// <p>Indicates whether the port probe was blocked.</p>
    #[serde(rename = "Blocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked: Option<bool>,
    /// <p>Information about the ports affected by the port probe.</p>
    #[serde(rename = "PortProbeDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_probe_details: Option<Vec<PortProbeDetail>>,
}

/// <p>A port scan that was part of the port probe. For each scan, PortProbeDetails provides information about the local IP address and port that were scanned, and the remote IP address that the scan originated from.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PortProbeDetail {
    /// <p>Provides information about the IP address where the scanned port is located.</p>
    #[serde(rename = "LocalIpDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_ip_details: Option<ActionLocalIpDetails>,
    /// <p>Provides information about the port that was scanned.</p>
    #[serde(rename = "LocalPortDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_port_details: Option<ActionLocalPortDetails>,
    /// <p>Provides information about the remote IP address that performed the scan.</p>
    #[serde(rename = "RemoteIpDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_ip_details: Option<ActionRemoteIpDetails>,
}

/// <p>A range of ports.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PortRange {
    /// <p>The first port in the port range.</p>
    #[serde(rename = "Begin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin: Option<i64>,
    /// <p>The last port in the port range.</p>
    #[serde(rename = "End")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<i64>,
}

/// <p>A range of ports.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PortRangeFromTo {
    /// <p>The first port in the port range.</p>
    #[serde(rename = "From")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,
    /// <p>The last port in the port range.</p>
    #[serde(rename = "To")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,
}

/// <p>The details of process-related information about a finding.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ProcessDetails {
    /// <p>Indicates when the process was launched.</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "LaunchedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launched_at: Option<String>,
    /// <p>The name of the process.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The parent process ID.</p>
    #[serde(rename = "ParentPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_pid: Option<i64>,
    /// <p>The path to the process executable.</p>
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// <p>The process ID.</p>
    #[serde(rename = "Pid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<i64>,
    /// <p>Indicates when the process was terminated.</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "TerminatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminated_at: Option<String>,
}

/// <p>Contains details about a product.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Product {
    /// <p>The URL to the service or product documentation about the integration with Security Hub, including how to activate the integration.</p>
    #[serde(rename = "ActivationUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation_url: Option<String>,
    /// <p>The categories assigned to the product.</p>
    #[serde(rename = "Categories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,
    /// <p>The name of the company that provides the product.</p>
    #[serde(rename = "CompanyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    /// <p>A description of the product.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p><p>The types of integration that the product supports. Available values are the following.</p> <ul> <li> <p> <code>SEND<em>FINDINGS</em>TO<em>SECURITY</em>HUB</code> - The integration sends findings to Security Hub.</p> </li> <li> <p> <code>RECEIVE<em>FINDINGS</em>FROM<em>SECURITY</em>HUB</code> - The integration receives findings from Security Hub.</p> </li> <li> <p> <code>UPDATE<em>FINDINGS</em>IN<em>SECURITY</em>HUB</code> - The integration does not send new findings to Security Hub, but does make updates to the findings that it receives from Security Hub.</p> </li> </ul></p>
    #[serde(rename = "IntegrationTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_types: Option<Vec<String>>,
    /// <p>For integrations with AWS services, the AWS Console URL from which to activate the service.</p> <p>For integrations with third-party products, the AWS Marketplace URL from which to subscribe to or purchase the product.</p>
    #[serde(rename = "MarketplaceUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketplace_url: Option<String>,
    /// <p>The ARN assigned to the product.</p>
    #[serde(rename = "ProductArn")]
    pub product_arn: String,
    /// <p>The name of the product.</p>
    #[serde(rename = "ProductName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    /// <p>The resource policy associated with the product.</p>
    #[serde(rename = "ProductSubscriptionResourcePolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_subscription_resource_policy: Option<String>,
}

/// <p>Identifies where the sensitive data begins and ends.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Range {
    /// <p>The number of lines (for a line range) or characters (for an offset range) from the beginning of the file to the end of the sensitive data.</p>
    #[serde(rename = "End")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<i64>,
    /// <p>The number of lines (for a line range) or characters (for an offset range) from the beginning of the file to the end of the sensitive data.</p>
    #[serde(rename = "Start")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<i64>,
    /// <p>In the line where the sensitive data starts, the column within the line where the sensitive data starts.</p>
    #[serde(rename = "StartColumn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_column: Option<i64>,
}

/// <p>A recommendation on how to remediate the issue identified in a finding.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Recommendation {
    /// <p>Describes the recommended steps to take to remediate an issue identified in a finding.</p>
    #[serde(rename = "Text")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// <p>A URL to a page or site that contains information about how to remediate a finding.</p>
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// <p>An occurrence of sensitive data in an Apache Avro object container or an Apache Parquet file.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Record {
    /// <p>The path, as a JSONPath expression, to the field in the record that contains the data. If the field name is longer than 20 characters, it is truncated. If the path is longer than 250 characters, it is truncated.</p>
    #[serde(rename = "JsonPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_path: Option<String>,
    /// <p>The record index, starting from 0, for the record that contains the data.</p>
    #[serde(rename = "RecordIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_index: Option<i64>,
}

/// <p>Details about a related finding.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct RelatedFinding {
    /// <p>The product-generated identifier for a related finding.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The ARN of the product that generated a related finding.</p>
    #[serde(rename = "ProductArn")]
    pub product_arn: String,
}

/// <p>Details about the remediation steps for a finding.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Remediation {
    /// <p>A recommendation on the steps to take to remediate the issue identified by a finding.</p>
    #[serde(rename = "Recommendation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation: Option<Recommendation>,
}

/// <p>A resource related to a finding.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Resource {
    /// <p>Contains information about sensitive data that was detected on the resource.</p>
    #[serde(rename = "DataClassification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_classification: Option<DataClassificationDetails>,
    /// <p>Additional details about the resource related to a finding.</p>
    #[serde(rename = "Details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<ResourceDetails>,
    /// <p>The canonical identifier for the given resource type.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The canonical AWS partition name that the Region is assigned to.</p>
    #[serde(rename = "Partition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition: Option<String>,
    /// <p>The canonical AWS external Region name where this resource is located.</p>
    #[serde(rename = "Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// <p>Identifies the role of the resource in the finding. A resource is either the actor or target of the finding activity,</p>
    #[serde(rename = "ResourceRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_role: Option<String>,
    /// <p>A list of AWS tags associated with a resource at the time the finding was processed.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The type of the resource that details are provided for. If possible, set <code>Type</code> to one of the supported resource types. For example, if the resource is an EC2 instance, then set <code>Type</code> to <code>AwsEc2Instance</code>.</p> <p>If the resource does not match any of the provided types, then set <code>Type</code> to <code>Other</code>. </p>
    #[serde(rename = "Type")]
    pub type_: String,
}

/// <p>Additional details about a resource related to a finding.</p> <p>To provide the details, use the object that corresponds to the resource type. For example, if the resource type is <code>AwsEc2Instance</code>, then you use the <code>AwsEc2Instance</code> object to provide the details.</p> <p>If the type-specific object does not contain all of the fields you want to populate, then you use the <code>Other</code> object to populate those additional fields.</p> <p>You also use the <code>Other</code> object to populate the details when the selected type does not have a corresponding object.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ResourceDetails {
    /// <p>Provides information about a REST API in version 1 of Amazon API Gateway.</p>
    #[serde(rename = "AwsApiGatewayRestApi")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_api_gateway_rest_api: Option<AwsApiGatewayRestApiDetails>,
    /// <p>Provides information about a version 1 Amazon API Gateway stage.</p>
    #[serde(rename = "AwsApiGatewayStage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_api_gateway_stage: Option<AwsApiGatewayStageDetails>,
    /// <p>Provides information about a version 2 API in Amazon API Gateway.</p>
    #[serde(rename = "AwsApiGatewayV2Api")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_api_gateway_v2_api: Option<AwsApiGatewayV2ApiDetails>,
    /// <p>Provides information about a version 2 stage for Amazon API Gateway.</p>
    #[serde(rename = "AwsApiGatewayV2Stage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_api_gateway_v2_stage: Option<AwsApiGatewayV2StageDetails>,
    /// <p>Details for an autoscaling group.</p>
    #[serde(rename = "AwsAutoScalingAutoScalingGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_auto_scaling_auto_scaling_group: Option<AwsAutoScalingAutoScalingGroupDetails>,
    /// <p>Provides details about an AWS Certificate Manager (ACM) certificate.</p>
    #[serde(rename = "AwsCertificateManagerCertificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_certificate_manager_certificate: Option<AwsCertificateManagerCertificateDetails>,
    /// <p>Details about a CloudFront distribution.</p>
    #[serde(rename = "AwsCloudFrontDistribution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_cloud_front_distribution: Option<AwsCloudFrontDistributionDetails>,
    /// <p>Provides details about a CloudTrail trail.</p>
    #[serde(rename = "AwsCloudTrailTrail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_cloud_trail_trail: Option<AwsCloudTrailTrailDetails>,
    /// <p>Details for an AWS CodeBuild project.</p>
    #[serde(rename = "AwsCodeBuildProject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_code_build_project: Option<AwsCodeBuildProjectDetails>,
    /// <p>Details about a DynamoDB table.</p>
    #[serde(rename = "AwsDynamoDbTable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_dynamo_db_table: Option<AwsDynamoDbTableDetails>,
    /// <p>Details about an Elastic IP address.</p>
    #[serde(rename = "AwsEc2Eip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_ec_2_eip: Option<AwsEc2EipDetails>,
    /// <p>Details about an EC2 instance related to a finding.</p>
    #[serde(rename = "AwsEc2Instance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_ec_2_instance: Option<AwsEc2InstanceDetails>,
    /// <p>Details about an EC2 network access control list (ACL).</p>
    #[serde(rename = "AwsEc2NetworkAcl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_ec_2_network_acl: Option<AwsEc2NetworkAclDetails>,
    /// <p>Details for an Amazon EC2 network interface.</p>
    #[serde(rename = "AwsEc2NetworkInterface")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_ec_2_network_interface: Option<AwsEc2NetworkInterfaceDetails>,
    /// <p>Details for an EC2 security group.</p>
    #[serde(rename = "AwsEc2SecurityGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_ec_2_security_group: Option<AwsEc2SecurityGroupDetails>,
    /// <p>Details about a subnet in EC2.</p>
    #[serde(rename = "AwsEc2Subnet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_ec_2_subnet: Option<AwsEc2SubnetDetails>,
    /// <p>Details for an EC2 volume.</p>
    #[serde(rename = "AwsEc2Volume")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_ec_2_volume: Option<AwsEc2VolumeDetails>,
    /// <p>Details for an EC2 VPC.</p>
    #[serde(rename = "AwsEc2Vpc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_ec_2_vpc: Option<AwsEc2VpcDetails>,
    /// <p>Details about an ECS cluster.</p>
    #[serde(rename = "AwsEcsCluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_ecs_cluster: Option<AwsEcsClusterDetails>,
    /// <p>Details about a task definition. A task definition describes the container and volume definitions of an Amazon Elastic Container Service task.</p>
    #[serde(rename = "AwsEcsTaskDefinition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_ecs_task_definition: Option<AwsEcsTaskDefinitionDetails>,
    /// <p>Details about an Elastic Beanstalk environment.</p>
    #[serde(rename = "AwsElasticBeanstalkEnvironment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_elastic_beanstalk_environment: Option<AwsElasticBeanstalkEnvironmentDetails>,
    /// <p>Details for an Elasticsearch domain.</p>
    #[serde(rename = "AwsElasticsearchDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_elasticsearch_domain: Option<AwsElasticsearchDomainDetails>,
    /// <p>contains details about a Classic Load Balancer.</p>
    #[serde(rename = "AwsElbLoadBalancer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_elb_load_balancer: Option<AwsElbLoadBalancerDetails>,
    /// <p>Details about a load balancer.</p>
    #[serde(rename = "AwsElbv2LoadBalancer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_elbv_2_load_balancer: Option<AwsElbv2LoadBalancerDetails>,
    /// <p>Details about an IAM access key related to a finding.</p>
    #[serde(rename = "AwsIamAccessKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_iam_access_key: Option<AwsIamAccessKeyDetails>,
    /// <p>Contains details about an IAM group.</p>
    #[serde(rename = "AwsIamGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_iam_group: Option<AwsIamGroupDetails>,
    /// <p>Details about an IAM permissions policy.</p>
    #[serde(rename = "AwsIamPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_iam_policy: Option<AwsIamPolicyDetails>,
    /// <p>Details about an IAM role.</p>
    #[serde(rename = "AwsIamRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_iam_role: Option<AwsIamRoleDetails>,
    /// <p>Details about an IAM user.</p>
    #[serde(rename = "AwsIamUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_iam_user: Option<AwsIamUserDetails>,
    /// <p>Details about a KMS key.</p>
    #[serde(rename = "AwsKmsKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_kms_key: Option<AwsKmsKeyDetails>,
    /// <p>Details about a Lambda function.</p>
    #[serde(rename = "AwsLambdaFunction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_lambda_function: Option<AwsLambdaFunctionDetails>,
    /// <p>Details for a Lambda layer version.</p>
    #[serde(rename = "AwsLambdaLayerVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_lambda_layer_version: Option<AwsLambdaLayerVersionDetails>,
    /// <p>Details about an Amazon RDS database cluster.</p>
    #[serde(rename = "AwsRdsDbCluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_rds_db_cluster: Option<AwsRdsDbClusterDetails>,
    /// <p>Details about an Amazon RDS database cluster snapshot.</p>
    #[serde(rename = "AwsRdsDbClusterSnapshot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_rds_db_cluster_snapshot: Option<AwsRdsDbClusterSnapshotDetails>,
    /// <p>Details about an Amazon RDS database instance.</p>
    #[serde(rename = "AwsRdsDbInstance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_rds_db_instance: Option<AwsRdsDbInstanceDetails>,
    /// <p>Details about an Amazon RDS database snapshot.</p>
    #[serde(rename = "AwsRdsDbSnapshot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_rds_db_snapshot: Option<AwsRdsDbSnapshotDetails>,
    /// <p>Contains details about an Amazon Redshift cluster.</p>
    #[serde(rename = "AwsRedshiftCluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_redshift_cluster: Option<AwsRedshiftClusterDetails>,
    /// <p>Details about the Amazon S3 Public Access Block configuration for an account.</p>
    #[serde(rename = "AwsS3AccountPublicAccessBlock")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_s3_account_public_access_block: Option<AwsS3AccountPublicAccessBlockDetails>,
    /// <p>Details about an Amazon S3 bucket related to a finding.</p>
    #[serde(rename = "AwsS3Bucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_s3_bucket: Option<AwsS3BucketDetails>,
    /// <p>Details about an Amazon S3 object related to a finding.</p>
    #[serde(rename = "AwsS3Object")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_s3_object: Option<AwsS3ObjectDetails>,
    /// <p>Details about a Secrets Manager secret.</p>
    #[serde(rename = "AwsSecretsManagerSecret")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_secrets_manager_secret: Option<AwsSecretsManagerSecretDetails>,
    /// <p>Details about an SNS topic.</p>
    #[serde(rename = "AwsSnsTopic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_sns_topic: Option<AwsSnsTopicDetails>,
    /// <p>Details about an SQS queue.</p>
    #[serde(rename = "AwsSqsQueue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_sqs_queue: Option<AwsSqsQueueDetails>,
    /// <p>Provides information about the state of a patch on an instance based on the patch baseline that was used to patch the instance.</p>
    #[serde(rename = "AwsSsmPatchCompliance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_ssm_patch_compliance: Option<AwsSsmPatchComplianceDetails>,
    /// <p>Details for a WAF WebACL.</p>
    #[serde(rename = "AwsWafWebAcl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_waf_web_acl: Option<AwsWafWebAclDetails>,
    /// <p>Details about a container resource related to a finding.</p>
    #[serde(rename = "Container")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<ContainerDetails>,
    /// <p><p>Details about a resource that are not available in a type-specific details object. Use the <code>Other</code> object in the following cases.</p> <ul> <li> <p>The type-specific object does not contain all of the fields that you want to populate. In this case, first use the type-specific object to populate those fields. Use the <code>Other</code> object to populate the fields that are missing from the type-specific object.</p> </li> <li> <p>The resource type does not have a corresponding object. This includes resources for which the type is <code>Other</code>. </p> </li> </ul></p>
    #[serde(rename = "Other")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Details about the account that was not processed.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SecurityHubResult {
    /// <p>An AWS account ID of the account that was not processed.</p>
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>The reason that the account was not processed.</p>
    #[serde(rename = "ProcessingResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_result: Option<String>,
}

/// <p>The list of detected instances of sensitive data.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SensitiveDataDetections {
    /// <p>The total number of occurrences of sensitive data that were detected.</p>
    #[serde(rename = "Count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// <p>Details about the sensitive data that was detected.</p>
    #[serde(rename = "Occurrences")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub occurrences: Option<Occurrences>,
    /// <p>The type of sensitive data that was detected. For example, the type might indicate that the data is an email address.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Contains a detected instance of sensitive data that are based on built-in identifiers.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SensitiveDataResult {
    /// <p>The category of sensitive data that was detected. For example, the category can indicate that the sensitive data involved credentials, financial information, or personal information.</p>
    #[serde(rename = "Category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// <p>The list of detected instances of sensitive data.</p>
    #[serde(rename = "Detections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detections: Option<Vec<SensitiveDataDetections>>,
    /// <p>The total number of occurrences of sensitive data.</p>
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
}

/// <p>The severity of the finding.</p> <p>The finding provider can provide the initial severity. The finding provider can only update the severity if it has not been updated using <code>BatchUpdateFindings</code>.</p> <p>The finding must have either <code>Label</code> or <code>Normalized</code> populated. If only one of these attributes is populated, then Security Hub automatically populates the other one. If neither attribute is populated, then the finding is invalid. <code>Label</code> is the preferred attribute.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Severity {
    /// <p><p>The severity value of the finding. The allowed values are the following.</p> <ul> <li> <p> <code>INFORMATIONAL</code> - No issue was found.</p> </li> <li> <p> <code>LOW</code> - The issue does not require action on its own.</p> </li> <li> <p> <code>MEDIUM</code> - The issue must be addressed but not urgently.</p> </li> <li> <p> <code>HIGH</code> - The issue must be addressed as a priority.</p> </li> <li> <p> <code>CRITICAL</code> - The issue must be remediated immediately to avoid it escalating.</p> </li> </ul> <p>If you provide <code>Normalized</code> and do not provide <code>Label</code>, then <code>Label</code> is set automatically as follows. </p> <ul> <li> <p>0 - <code>INFORMATIONAL</code> </p> </li> <li> <p>1–39 - <code>LOW</code> </p> </li> <li> <p>40–69 - <code>MEDIUM</code> </p> </li> <li> <p>70–89 - <code>HIGH</code> </p> </li> <li> <p>90–100 - <code>CRITICAL</code> </p> </li> </ul></p>
    #[serde(rename = "Label")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// <p><p>Deprecated. The normalized severity of a finding. This attribute is being deprecated. Instead of providing <code>Normalized</code>, provide <code>Label</code>.</p> <p>If you provide <code>Label</code> and do not provide <code>Normalized</code>, then <code>Normalized</code> is set automatically as follows.</p> <ul> <li> <p> <code>INFORMATIONAL</code> - 0</p> </li> <li> <p> <code>LOW</code> - 1</p> </li> <li> <p> <code>MEDIUM</code> - 40</p> </li> <li> <p> <code>HIGH</code> - 70</p> </li> <li> <p> <code>CRITICAL</code> - 90</p> </li> </ul></p>
    #[serde(rename = "Normalized")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normalized: Option<i64>,
    /// <p>The native severity from the finding product that generated the finding.</p>
    #[serde(rename = "Original")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original: Option<String>,
    /// <p>Deprecated. This attribute is being deprecated. Instead of providing <code>Product</code>, provide <code>Original</code>.</p> <p>The native severity as defined by the AWS service or integrated partner product that generated the finding.</p>
    #[serde(rename = "Product")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<f64>,
}

/// <p>Updates to the severity information for a finding.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SeverityUpdate {
    /// <p><p>The severity value of the finding. The allowed values are the following.</p> <ul> <li> <p> <code>INFORMATIONAL</code> - No issue was found.</p> </li> <li> <p> <code>LOW</code> - The issue does not require action on its own.</p> </li> <li> <p> <code>MEDIUM</code> - The issue must be addressed but not urgently.</p> </li> <li> <p> <code>HIGH</code> - The issue must be addressed as a priority.</p> </li> <li> <p> <code>CRITICAL</code> - The issue must be remediated immediately to avoid it escalating.</p> </li> </ul></p>
    #[serde(rename = "Label")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// <p><p>The normalized severity for the finding. This attribute is to be deprecated in favor of <code>Label</code>.</p> <p>If you provide <code>Normalized</code> and do not provide <code>Label</code>, <code>Label</code> is set automatically as follows.</p> <ul> <li> <p>0 - <code>INFORMATIONAL</code> </p> </li> <li> <p>1–39 - <code>LOW</code> </p> </li> <li> <p>40–69 - <code>MEDIUM</code> </p> </li> <li> <p>70–89 - <code>HIGH</code> </p> </li> <li> <p>90–100 - <code>CRITICAL</code> </p> </li> </ul></p>
    #[serde(rename = "Normalized")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normalized: Option<i64>,
    /// <p>The native severity as defined by the AWS service or integrated partner product that generated the finding.</p>
    #[serde(rename = "Product")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<f64>,
}

/// <p>Information about a software package.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SoftwarePackage {
    /// <p>The architecture used for the software package.</p>
    #[serde(rename = "Architecture")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    /// <p>The epoch of the software package.</p>
    #[serde(rename = "Epoch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epoch: Option<String>,
    /// <p>The name of the software package.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The release of the software package.</p>
    #[serde(rename = "Release")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release: Option<String>,
    /// <p>The version of the software package.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>A collection of finding attributes used to sort findings.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SortCriterion {
    /// <p>The finding attribute used to sort findings.</p>
    #[serde(rename = "Field")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    /// <p>The order used to sort findings.</p>
    #[serde(rename = "SortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

/// <p>Provides information about a specific standard.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Standard {
    /// <p>A description of the standard.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Whether the standard is enabled by default. When Security Hub is enabled from the console, if a standard is enabled by default, the check box for that standard is selected by default.</p> <p>When Security Hub is enabled using the <code>EnableSecurityHub</code> API operation, the standard is enabled by default unless <code>EnableDefaultStandards</code> is set to <code>false</code>.</p>
    #[serde(rename = "EnabledByDefault")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_by_default: Option<bool>,
    /// <p>The name of the standard.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The ARN of a standard.</p>
    #[serde(rename = "StandardsArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standards_arn: Option<String>,
}

/// <p>Details for an individual security standard control.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StandardsControl {
    /// <p>The identifier of the security standard control.</p>
    #[serde(rename = "ControlId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_id: Option<String>,
    /// <p>The current status of the security standard control. Indicates whether the control is enabled or disabled. Security Hub does not check against disabled controls.</p>
    #[serde(rename = "ControlStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_status: Option<String>,
    /// <p>The date and time that the status of the security standard control was most recently updated.</p>
    #[serde(rename = "ControlStatusUpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_status_updated_at: Option<f64>,
    /// <p>The longer description of the security standard control. Provides information about what the control is checking for.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The reason provided for the most recent change in status for the control.</p>
    #[serde(rename = "DisabledReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_reason: Option<String>,
    /// <p>The list of requirements that are related to this control.</p>
    #[serde(rename = "RelatedRequirements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_requirements: Option<Vec<String>>,
    /// <p>A link to remediation information for the control in the Security Hub user documentation.</p>
    #[serde(rename = "RemediationUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remediation_url: Option<String>,
    /// <p>The severity of findings generated from this security standard control.</p> <p>The finding severity is based on an assessment of how easy it would be to compromise AWS resources if the issue is detected.</p>
    #[serde(rename = "SeverityRating")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity_rating: Option<String>,
    /// <p>The ARN of the security standard control.</p>
    #[serde(rename = "StandardsControlArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standards_control_arn: Option<String>,
    /// <p>The title of the security standard control.</p>
    #[serde(rename = "Title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// <p>A resource that represents your subscription to a supported standard.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StandardsSubscription {
    /// <p>The ARN of a standard.</p>
    #[serde(rename = "StandardsArn")]
    pub standards_arn: String,
    /// <p>A key-value pair of input for the standard.</p>
    #[serde(rename = "StandardsInput")]
    pub standards_input: ::std::collections::HashMap<String, String>,
    /// <p><p>The status of the standard subscription.</p> <p>The status values are as follows:</p> <ul> <li> <p> <code>PENDING</code> - Standard is in the process of being enabled.</p> </li> <li> <p> <code>READY</code> - Standard is enabled.</p> </li> <li> <p> <code>INCOMPLETE</code> - Standard could not be enabled completely. Some controls may not be available.</p> </li> <li> <p> <code>DELETING</code> - Standard is in the process of being disabled.</p> </li> <li> <p> <code>FAILED</code> - Standard could not be disabled.</p> </li> </ul></p>
    #[serde(rename = "StandardsStatus")]
    pub standards_status: String,
    /// <p>The ARN of a resource that represents your subscription to a supported standard.</p>
    #[serde(rename = "StandardsSubscriptionArn")]
    pub standards_subscription_arn: String,
}

/// <p>The standard that you want to enable.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StandardsSubscriptionRequest {
    /// <p>The ARN of the standard that you want to enable. To view the list of available standards and their ARNs, use the <code> <a>DescribeStandards</a> </code> operation.</p>
    #[serde(rename = "StandardsArn")]
    pub standards_arn: String,
    /// <p>A key-value pair of input for the standard.</p>
    #[serde(rename = "StandardsInput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standards_input: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Provides additional context for the value of <code>Compliance.Status</code>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct StatusReason {
    /// <p>The corresponding description for the status reason code.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A code that represents a reason for the control status. For the list of status reason codes and their meanings, see <a href="https://docs.aws.amazon.com/securityhub/latest/userguide/securityhub-standards-results.html#securityhub-standards-results-asff">Standards-related information in the ASFF</a> in the <i>AWS Security Hub User Guide</i>. </p>
    #[serde(rename = "ReasonCode")]
    pub reason_code: String,
}

/// <p>A string filter for querying findings.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct StringFilter {
    /// <p><p>The condition to apply to a string value when querying for findings. To search for values that contain the filter criteria value, use one of the following comparison operators:</p> <ul> <li> <p>To search for values that exactly match the filter value, use <code>EQUALS</code>.</p> <p>For example, the filter <code>ResourceType EQUALS AwsEc2SecurityGroup</code> only matches findings that have a resource type of <code>AwsEc2SecurityGroup</code>.</p> </li> <li> <p>To search for values that start with the filter value, use <code>PREFIX</code>.</p> <p>For example, the filter <code>ResourceType PREFIX AwsIam</code> matches findings that have a resource type that starts with <code>AwsIam</code>. Findings with a resource type of <code>AwsIamPolicy</code>, <code>AwsIamRole</code>, or <code>AwsIamUser</code> would all match.</p> </li> </ul> <p> <code>EQUALS</code> and <code>PREFIX</code> filters on the same field are joined by <code>OR</code>. A finding matches if it matches any one of those filters.</p> <p>To search for values that do not contain the filter criteria value, use one of the following comparison operators:</p> <ul> <li> <p>To search for values that do not exactly match the filter value, use <code>NOT<em>EQUALS</code>.</p> <p>For example, the filter <code>ResourceType NOT</em>EQUALS AwsIamPolicy</code> matches findings that have a resource type other than <code>AwsIamPolicy</code>.</p> </li> <li> <p>To search for values that do not start with the filter value, use <code>PREFIX<em>NOT</em>EQUALS</code>.</p> <p>For example, the filter <code>ResourceType PREFIX<em>NOT</em>EQUALS AwsIam</code> matches findings that have a resource type that does not start with <code>AwsIam</code>. Findings with a resource type of <code>AwsIamPolicy</code>, <code>AwsIamRole</code>, or <code>AwsIamUser</code> would all be excluded from the results.</p> </li> </ul> <p> <code>NOT<em>EQUALS</code> and <code>PREFIX</em>NOT<em>EQUALS</code> filters on the same field are joined by <code>AND</code>. A finding matches only if it matches all of those filters.</p> <p>For filters on the same field, you cannot provide both an <code>EQUALS</code> filter and a <code>NOT</em>EQUALS</code> or <code>PREFIX<em>NOT</em>EQUALS</code> filter. Combining filters in this way always returns an error, even if the provided filter values would return valid results.</p> <p>You can combine <code>PREFIX</code> filters with <code>NOT<em>EQUALS</code> or <code>PREFIX</em>NOT<em>EQUALS</code> filters for the same field. Security Hub first processes the <code>PREFIX</code> filters, then the <code>NOT</em>EQUALS</code> or <code>PREFIX<em>NOT</em>EQUALS</code> filters.</p> <p> For example, for the following filter, Security Hub first identifies findings that have resource types that start with either <code>AwsIAM</code> or <code>AwsEc2</code>. It then excludes findings that have a resource type of <code>AwsIamPolicy</code> and findings that have a resource type of <code>AwsEc2NetworkInterface</code>.</p> <ul> <li> <p> <code>ResourceType PREFIX AwsIam</code> </p> </li> <li> <p> <code>ResourceType PREFIX AwsEc2</code> </p> </li> <li> <p> <code>ResourceType NOT<em>EQUALS AwsIamPolicy</code> </p> </li> <li> <p> <code>ResourceType NOT</em>EQUALS AwsEc2NetworkInterface</code> </p> </li> </ul></p>
    #[serde(rename = "Comparison")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison: Option<String>,
    /// <p>The string filter value. Filter values are case sensitive. For example, the product name for control-based findings is <code>Security Hub</code>. If you provide <code>security hub</code> as the filter text, then there is no match.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The ARN of the resource to apply the tags to.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>The tags to add to the resource. You can add up to 50 tags at a time. The tag keys can be no longer than 128 characters. The tag values can be no longer than 256 characters.</p>
    #[serde(rename = "Tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

/// <p>Details about the threat intelligence related to a finding.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ThreatIntelIndicator {
    /// <p>The category of a threat intelligence indicator.</p>
    #[serde(rename = "Category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// <p>Indicates when the most recent instance of a threat intelligence indicator was observed.</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "LastObservedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_observed_at: Option<String>,
    /// <p>The source of the threat intelligence indicator.</p>
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// <p>The URL to the page or site where you can get more information about the threat intelligence indicator.</p>
    #[serde(rename = "SourceUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
    /// <p>The type of threat intelligence indicator.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The value of a threat intelligence indicator.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The ARN of the resource to remove the tags from.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>The tag keys associated with the tags to remove from the resource. You can remove up to 50 tags at a time.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateActionTargetRequest {
    /// <p>The ARN of the custom action target to update.</p>
    #[serde(rename = "ActionTargetArn")]
    pub action_target_arn: String,
    /// <p>The updated description for the custom action target.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The updated name of the custom action target.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateActionTargetResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateFindingsRequest {
    /// <p>A collection of attributes that specify which findings you want to update.</p>
    #[serde(rename = "Filters")]
    pub filters: AwsSecurityFindingFilters,
    /// <p>The updated note for the finding.</p>
    #[serde(rename = "Note")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<NoteUpdate>,
    /// <p>The updated record state for the finding.</p>
    #[serde(rename = "RecordState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_state: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateFindingsResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateInsightRequest {
    /// <p>The updated filters that define this insight.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<AwsSecurityFindingFilters>,
    /// <p>The updated <code>GroupBy</code> attribute that defines this insight.</p>
    #[serde(rename = "GroupByAttribute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by_attribute: Option<String>,
    /// <p>The ARN of the insight that you want to update.</p>
    #[serde(rename = "InsightArn")]
    pub insight_arn: String,
    /// <p>The updated name for the insight.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateInsightResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateOrganizationConfigurationRequest {
    /// <p>Whether to automatically enable Security Hub for new accounts in the organization.</p> <p>By default, this is <code>false</code>, and new accounts are not added automatically.</p> <p>To automatically enable Security Hub for new accounts, set this to <code>true</code>.</p>
    #[serde(rename = "AutoEnable")]
    pub auto_enable: bool,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateOrganizationConfigurationResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateSecurityHubConfigurationRequest {
    /// <p>Whether to automatically enable new controls when they are added to standards that are enabled.</p> <p>By default, this is set to <code>true</code>, and new controls are enabled automatically. To not automatically enable new controls, set this to <code>false</code>. </p>
    #[serde(rename = "AutoEnableControls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_enable_controls: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateSecurityHubConfigurationResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateStandardsControlRequest {
    /// <p>The updated status of the security standard control.</p>
    #[serde(rename = "ControlStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_status: Option<String>,
    /// <p>A description of the reason why you are disabling a security standard control. If you are disabling a control, then this is required.</p>
    #[serde(rename = "DisabledReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_reason: Option<String>,
    /// <p>The ARN of the security standard control to enable or disable.</p>
    #[serde(rename = "StandardsControlArn")]
    pub standards_control_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateStandardsControlResponse {}

/// <p>A vulnerability associated with a finding.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Vulnerability {
    /// <p>CVSS scores from the advisory related to the vulnerability.</p>
    #[serde(rename = "Cvss")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvss: Option<Vec<Cvss>>,
    /// <p>The identifier of the vulnerability.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>A list of URLs that provide additional information about the vulnerability.</p>
    #[serde(rename = "ReferenceUrls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_urls: Option<Vec<String>>,
    /// <p>List of vulnerabilities that are related to this vulnerability.</p>
    #[serde(rename = "RelatedVulnerabilities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_vulnerabilities: Option<Vec<String>>,
    /// <p>Information about the vendor that generates the vulnerability report.</p>
    #[serde(rename = "Vendor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<VulnerabilityVendor>,
    /// <p>List of software packages that have the vulnerability.</p>
    #[serde(rename = "VulnerablePackages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vulnerable_packages: Option<Vec<SoftwarePackage>>,
}

/// <p>A vendor that generates a vulnerability report.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VulnerabilityVendor {
    /// <p>The name of the vendor.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The URL of the vulnerability advisory.</p>
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// <p>Indicates when the vulnerability advisory was created.</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "VendorCreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_created_at: Option<String>,
    /// <p>The severity that the vendor assigned to the vulnerability.</p>
    #[serde(rename = "VendorSeverity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_severity: Option<String>,
    /// <p>Indicates when the vulnerability advisory was last updated.</p> <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[serde(rename = "VendorUpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_updated_at: Option<String>,
}

/// <p>Details about the action that CloudFront or AWS WAF takes when a web request matches the conditions in the rule. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct WafAction {
    /// <p><p>Specifies how you want AWS WAF to respond to requests that match the settings in a rule.</p> <p>Valid settings include the following:</p> <ul> <li> <p> <code>ALLOW</code> - AWS WAF allows requests</p> </li> <li> <p> <code>BLOCK</code> - AWS WAF blocks requests</p> </li> <li> <p> <code>COUNT</code> - AWS WAF increments a counter of the requests that match all of the conditions in the rule. AWS WAF then continues to inspect the web request based on the remaining rules in the web ACL. You can&#39;t specify <code>COUNT</code> for the default action for a WebACL.</p> </li> </ul></p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Details about a rule to exclude from a rule group.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct WafExcludedRule {
    /// <p>The unique identifier for the rule to exclude from the rule group.</p>
    #[serde(rename = "RuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
}

/// <p>Details about an override action for a rule.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct WafOverrideAction {
    /// <p> <code>COUNT</code> overrides the action specified by the individual rule within a <code>RuleGroup</code> .</p> <p>If set to <code>NONE</code>, the rule's action takes place.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Provides information about the status of the investigation into a finding.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Workflow {
    /// <p><p>The status of the investigation into the finding. The allowed values are the following.</p> <ul> <li> <p> <code>NEW</code> - The initial state of a finding, before it is reviewed.</p> <p>Security Hub also resets the workflow status from <code>NOTIFIED</code> or <code>RESOLVED</code> to <code>NEW</code> in the following cases:</p> <ul> <li> <p> <code>RecordState</code> changes from <code>ARCHIVED</code> to <code>ACTIVE</code>.</p> </li> <li> <p> <code>ComplianceStatus</code> changes from <code>PASSED</code> to either <code>WARNING</code>, <code>FAILED</code>, or <code>NOT_AVAILABLE</code>.</p> </li> </ul> </li> <li> <p> <code>NOTIFIED</code> - Indicates that you notified the resource owner about the security issue. Used when the initial reviewer is not the resource owner, and needs intervention from the resource owner.</p> </li> <li> <p> <code>SUPPRESSED</code> - The finding will not be reviewed again and will not be acted upon.</p> </li> <li> <p> <code>RESOLVED</code> - The finding was reviewed and remediated and is now considered resolved. </p> </li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Used to update information about the investigation into the finding.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct WorkflowUpdate {
    /// <p><p>The status of the investigation into the finding. The allowed values are the following.</p> <ul> <li> <p> <code>NEW</code> - The initial state of a finding, before it is reviewed.</p> <p>Security Hub also resets <code>WorkFlowStatus</code> from <code>NOTIFIED</code> or <code>RESOLVED</code> to <code>NEW</code> in the following cases:</p> <ul> <li> <p>The record state changes from <code>ARCHIVED</code> to <code>ACTIVE</code>.</p> </li> <li> <p>The compliance status changes from <code>PASSED</code> to either <code>WARNING</code>, <code>FAILED</code>, or <code>NOT_AVAILABLE</code>.</p> </li> </ul> </li> <li> <p> <code>NOTIFIED</code> - Indicates that you notified the resource owner about the security issue. Used when the initial reviewer is not the resource owner, and needs intervention from the resource owner.</p> </li> <li> <p> <code>RESOLVED</code> - The finding was reviewed and remediated and is now considered resolved.</p> </li> <li> <p> <code>SUPPRESSED</code> - The finding will not be reviewed again and will not be acted upon.</p> </li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// Errors returned by AcceptAdministratorInvitation
#[derive(Debug, PartialEq)]
pub enum AcceptAdministratorInvitationError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>There is an issue with the account used to make the request. Either Security Hub is not enabled for the account, or the account does not have permission to perform this action.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account or throttling limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because we can't find the specified resource.</p>
    ResourceNotFound(String),
}

impl AcceptAdministratorInvitationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AcceptAdministratorInvitationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(AcceptAdministratorInvitationError::Internal(
                        err.msg,
                    ))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(AcceptAdministratorInvitationError::InvalidAccess(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(AcceptAdministratorInvitationError::InvalidInput(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(AcceptAdministratorInvitationError::LimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        AcceptAdministratorInvitationError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AcceptAdministratorInvitationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AcceptAdministratorInvitationError::Internal(ref cause) => write!(f, "{}", cause),
            AcceptAdministratorInvitationError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            AcceptAdministratorInvitationError::InvalidInput(ref cause) => write!(f, "{}", cause),
            AcceptAdministratorInvitationError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            AcceptAdministratorInvitationError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for AcceptAdministratorInvitationError {}
/// Errors returned by AcceptInvitation
#[derive(Debug, PartialEq)]
pub enum AcceptInvitationError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>There is an issue with the account used to make the request. Either Security Hub is not enabled for the account, or the account does not have permission to perform this action.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account or throttling limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because we can't find the specified resource.</p>
    ResourceNotFound(String),
}

impl AcceptInvitationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AcceptInvitationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(AcceptInvitationError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(AcceptInvitationError::InvalidAccess(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(AcceptInvitationError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(AcceptInvitationError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AcceptInvitationError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AcceptInvitationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AcceptInvitationError::Internal(ref cause) => write!(f, "{}", cause),
            AcceptInvitationError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            AcceptInvitationError::InvalidInput(ref cause) => write!(f, "{}", cause),
            AcceptInvitationError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            AcceptInvitationError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AcceptInvitationError {}
/// Errors returned by BatchDisableStandards
#[derive(Debug, PartialEq)]
pub enum BatchDisableStandardsError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>There is an issue with the account used to make the request. Either Security Hub is not enabled for the account, or the account does not have permission to perform this action.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account or throttling limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
}

impl BatchDisableStandardsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchDisableStandardsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(BatchDisableStandardsError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(BatchDisableStandardsError::InvalidAccess(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(BatchDisableStandardsError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(BatchDisableStandardsError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for BatchDisableStandardsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchDisableStandardsError::Internal(ref cause) => write!(f, "{}", cause),
            BatchDisableStandardsError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            BatchDisableStandardsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            BatchDisableStandardsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchDisableStandardsError {}
/// Errors returned by BatchEnableStandards
#[derive(Debug, PartialEq)]
pub enum BatchEnableStandardsError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>There is an issue with the account used to make the request. Either Security Hub is not enabled for the account, or the account does not have permission to perform this action.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account or throttling limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
}

impl BatchEnableStandardsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchEnableStandardsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(BatchEnableStandardsError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(BatchEnableStandardsError::InvalidAccess(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(BatchEnableStandardsError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(BatchEnableStandardsError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for BatchEnableStandardsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchEnableStandardsError::Internal(ref cause) => write!(f, "{}", cause),
            BatchEnableStandardsError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            BatchEnableStandardsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            BatchEnableStandardsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchEnableStandardsError {}
/// Errors returned by BatchImportFindings
#[derive(Debug, PartialEq)]
pub enum BatchImportFindingsError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>There is an issue with the account used to make the request. Either Security Hub is not enabled for the account, or the account does not have permission to perform this action.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account or throttling limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
}

impl BatchImportFindingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchImportFindingsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(BatchImportFindingsError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(BatchImportFindingsError::InvalidAccess(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(BatchImportFindingsError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(BatchImportFindingsError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for BatchImportFindingsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchImportFindingsError::Internal(ref cause) => write!(f, "{}", cause),
            BatchImportFindingsError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            BatchImportFindingsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            BatchImportFindingsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchImportFindingsError {}
/// Errors returned by BatchUpdateFindings
#[derive(Debug, PartialEq)]
pub enum BatchUpdateFindingsError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>There is an issue with the account used to make the request. Either Security Hub is not enabled for the account, or the account does not have permission to perform this action.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account or throttling limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
}

impl BatchUpdateFindingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchUpdateFindingsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(BatchUpdateFindingsError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(BatchUpdateFindingsError::InvalidAccess(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(BatchUpdateFindingsError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(BatchUpdateFindingsError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for BatchUpdateFindingsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchUpdateFindingsError::Internal(ref cause) => write!(f, "{}", cause),
            BatchUpdateFindingsError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            BatchUpdateFindingsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            BatchUpdateFindingsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchUpdateFindingsError {}
/// Errors returned by CreateActionTarget
#[derive(Debug, PartialEq)]
pub enum CreateActionTargetError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>There is an issue with the account used to make the request. Either Security Hub is not enabled for the account, or the account does not have permission to perform this action.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account or throttling limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
    /// <p>The resource specified in the request conflicts with an existing resource.</p>
    ResourceConflict(String),
}

impl CreateActionTargetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateActionTargetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(CreateActionTargetError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(CreateActionTargetError::InvalidAccess(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateActionTargetError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateActionTargetError::LimitExceeded(err.msg))
                }
                "ResourceConflictException" => {
                    return RusotoError::Service(CreateActionTargetError::ResourceConflict(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateActionTargetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateActionTargetError::Internal(ref cause) => write!(f, "{}", cause),
            CreateActionTargetError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            CreateActionTargetError::InvalidInput(ref cause) => write!(f, "{}", cause),
            CreateActionTargetError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateActionTargetError::ResourceConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateActionTargetError {}
/// Errors returned by CreateInsight
#[derive(Debug, PartialEq)]
pub enum CreateInsightError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>There is an issue with the account used to make the request. Either Security Hub is not enabled for the account, or the account does not have permission to perform this action.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account or throttling limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
    /// <p>The resource specified in the request conflicts with an existing resource.</p>
    ResourceConflict(String),
}

impl CreateInsightError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateInsightError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(CreateInsightError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(CreateInsightError::InvalidAccess(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateInsightError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateInsightError::LimitExceeded(err.msg))
                }
                "ResourceConflictException" => {
                    return RusotoError::Service(CreateInsightError::ResourceConflict(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateInsightError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateInsightError::Internal(ref cause) => write!(f, "{}", cause),
            CreateInsightError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            CreateInsightError::InvalidInput(ref cause) => write!(f, "{}", cause),
            CreateInsightError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateInsightError::ResourceConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateInsightError {}
/// Errors returned by CreateMembers
#[derive(Debug, PartialEq)]
pub enum CreateMembersError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>There is an issue with the account used to make the request. Either Security Hub is not enabled for the account, or the account does not have permission to perform this action.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account or throttling limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
    /// <p>The resource specified in the request conflicts with an existing resource.</p>
    ResourceConflict(String),
}

impl CreateMembersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateMembersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(CreateMembersError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(CreateMembersError::InvalidAccess(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateMembersError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateMembersError::LimitExceeded(err.msg))
                }
                "ResourceConflictException" => {
                    return RusotoError::Service(CreateMembersError::ResourceConflict(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateMembersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateMembersError::Internal(ref cause) => write!(f, "{}", cause),
            CreateMembersError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            CreateMembersError::InvalidInput(ref cause) => write!(f, "{}", cause),
            CreateMembersError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateMembersError::ResourceConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateMembersError {}
/// Errors returned by DeclineInvitations
#[derive(Debug, PartialEq)]
pub enum DeclineInvitationsError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>There is an issue with the account used to make the request. Either Security Hub is not enabled for the account, or the account does not have permission to perform this action.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because we can't find the specified resource.</p>
    ResourceNotFound(String),
}

impl DeclineInvitationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeclineInvitationsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DeclineInvitationsError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(DeclineInvitationsError::InvalidAccess(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeclineInvitationsError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeclineInvitationsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeclineInvitationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeclineInvitationsError::Internal(ref cause) => write!(f, "{}", cause),
            DeclineInvitationsError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            DeclineInvitationsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DeclineInvitationsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeclineInvitationsError {}
/// Errors returned by DeleteActionTarget
#[derive(Debug, PartialEq)]
pub enum DeleteActionTargetError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>There is an issue with the account used to make the request. Either Security Hub is not enabled for the account, or the account does not have permission to perform this action.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because we can't find the specified resource.</p>
    ResourceNotFound(String),
}

impl DeleteActionTargetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteActionTargetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DeleteActionTargetError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(DeleteActionTargetError::InvalidAccess(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteActionTargetError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteActionTargetError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteActionTargetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteActionTargetError::Internal(ref cause) => write!(f, "{}", cause),
            DeleteActionTargetError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            DeleteActionTargetError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DeleteActionTargetError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteActionTargetError {}
/// Errors returned by DeleteInsight
#[derive(Debug, PartialEq)]
pub enum DeleteInsightError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>There is an issue with the account used to make the request. Either Security Hub is not enabled for the account, or the account does not have permission to perform this action.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account or throttling limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because we can't find the specified resource.</p>
    ResourceNotFound(String),
}

impl DeleteInsightError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteInsightError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DeleteInsightError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(DeleteInsightError::InvalidAccess(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteInsightError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DeleteInsightError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteInsightError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteInsightError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteInsightError::Internal(ref cause) => write!(f, "{}", cause),
            DeleteInsightError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            DeleteInsightError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DeleteInsightError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DeleteInsightError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteInsightError {}
/// Errors returned by DeleteInvitations
#[derive(Debug, PartialEq)]
pub enum DeleteInvitationsError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>There is an issue with the account used to make the request. Either Security Hub is not enabled for the account, or the account does not have permission to perform this action.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account or throttling limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because we can't find the specified resource.</p>
    ResourceNotFound(String),
}

impl DeleteInvitationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteInvitationsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DeleteInvitationsError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(DeleteInvitationsError::InvalidAccess(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteInvitationsError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DeleteInvitationsError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteInvitationsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteInvitationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteInvitationsError::Internal(ref cause) => write!(f, "{}", cause),
            DeleteInvitationsError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            DeleteInvitationsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DeleteInvitationsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DeleteInvitationsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteInvitationsError {}
/// Errors returned by DeleteMembers
#[derive(Debug, PartialEq)]
pub enum DeleteMembersError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>There is an issue with the account used to make the request. Either Security Hub is not enabled for the account, or the account does not have permission to perform this action.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account or throttling limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because we can't find the specified resource.</p>
    ResourceNotFound(String),
}

impl DeleteMembersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteMembersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DeleteMembersError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(DeleteMembersError::InvalidAccess(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteMembersError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DeleteMembersError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteMembersError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteMembersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteMembersError::Internal(ref cause) => write!(f, "{}", cause),
            DeleteMembersError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            DeleteMembersError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DeleteMembersError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DeleteMembersError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteMembersError {}
/// Errors returned by DescribeActionTargets
#[derive(Debug, PartialEq)]
pub enum DescribeActionTargetsError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>There is an issue with the account used to make the request. Either Security Hub is not enabled for the account, or the account does not have permission to perform this action.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because we can't find the specified resource.</p>
    ResourceNotFound(String),
}

impl DescribeActionTargetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeActionTargetsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DescribeActionTargetsError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(DescribeActionTargetsError::InvalidAccess(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DescribeActionTargetsError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeActionTargetsError::ResourceNotFound(
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
impl fmt::Display for DescribeActionTargetsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeActionTargetsError::Internal(ref cause) => write!(f, "{}", cause),
            DescribeActionTargetsError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            DescribeActionTargetsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DescribeActionTargetsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeActionTargetsError {}
/// Errors returned by DescribeHub
#[derive(Debug, PartialEq)]
pub enum DescribeHubError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>There is an issue with the account used to make the request. Either Security Hub is not enabled for the account, or the account does not have permission to perform this action.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account or throttling limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because we can't find the specified resource.</p>
    ResourceNotFound(String),
}

impl DescribeHubError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeHubError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DescribeHubError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(DescribeHubError::InvalidAccess(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DescribeHubError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DescribeHubError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeHubError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeHubError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeHubError::Internal(ref cause) => write!(f, "{}", cause),
            DescribeHubError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            DescribeHubError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DescribeHubError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DescribeHubError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeHubError {}
/// Errors returned by DescribeOrganizationConfiguration
#[derive(Debug, PartialEq)]
pub enum DescribeOrganizationConfigurationError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>There is an issue with the account used to make the request. Either Security Hub is not enabled for the account, or the account does not have permission to perform this action.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account or throttling limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
}

impl DescribeOrganizationConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeOrganizationConfigurationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DescribeOrganizationConfigurationError::Internal(
                        err.msg,
                    ))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(
                        DescribeOrganizationConfigurationError::InvalidAccess(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(
                        DescribeOrganizationConfigurationError::InvalidInput(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(
                        DescribeOrganizationConfigurationError::LimitExceeded(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeOrganizationConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeOrganizationConfigurationError::Internal(ref cause) => write!(f, "{}", cause),
            DescribeOrganizationConfigurationError::InvalidAccess(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeOrganizationConfigurationError::InvalidInput(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeOrganizationConfigurationError::LimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeOrganizationConfigurationError {}
/// Errors returned by DescribeProducts
#[derive(Debug, PartialEq)]
pub enum DescribeProductsError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>There is an issue with the account used to make the request. Either Security Hub is not enabled for the account, or the account does not have permission to perform this action.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account or throttling limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
}

impl DescribeProductsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeProductsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DescribeProductsError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(DescribeProductsError::InvalidAccess(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DescribeProductsError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DescribeProductsError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeProductsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeProductsError::Internal(ref cause) => write!(f, "{}", cause),
            DescribeProductsError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            DescribeProductsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DescribeProductsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeProductsError {}
/// Errors returned by DescribeStandards
#[derive(Debug, PartialEq)]
pub enum DescribeStandardsError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>There is an issue with the account used to make the request. Either Security Hub is not enabled for the account, or the account does not have permission to perform this action.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
}

impl DescribeStandardsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeStandardsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DescribeStandardsError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(DescribeStandardsError::InvalidAccess(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DescribeStandardsError::InvalidInput(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeStandardsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeStandardsError::Internal(ref cause) => write!(f, "{}", cause),
            DescribeStandardsError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            DescribeStandardsError::InvalidInput(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeStandardsError {}
/// Errors returned by DescribeStandardsControls
#[derive(Debug, PartialEq)]
pub enum DescribeStandardsControlsError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>There is an issue with the account used to make the request. Either Security Hub is not enabled for the account, or the account does not have permission to perform this action.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because we can't find the specified resource.</p>
    ResourceNotFound(String),
}

impl DescribeStandardsControlsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeStandardsControlsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DescribeStandardsControlsError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(DescribeStandardsControlsError::InvalidAccess(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DescribeStandardsControlsError::InvalidInput(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeStandardsControlsError::ResourceNotFound(
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
impl fmt::Display for DescribeStandardsControlsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeStandardsControlsError::Internal(ref cause) => write!(f, "{}", cause),
            DescribeStandardsControlsError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            DescribeStandardsControlsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DescribeStandardsControlsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeStandardsControlsError {}
/// Errors returned by DisableImportFindingsForProduct
#[derive(Debug, PartialEq)]
pub enum DisableImportFindingsForProductError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>There is an issue with the account used to make the request. Either Security Hub is not enabled for the account, or the account does not have permission to perform this action.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account or throttling limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because we can't find the specified resource.</p>
    ResourceNotFound(String),
}

impl DisableImportFindingsForProductError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisableImportFindingsForProductError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DisableImportFindingsForProductError::Internal(
                        err.msg,
                    ))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(
                        DisableImportFindingsForProductError::InvalidAccess(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(
                        DisableImportFindingsForProductError::InvalidInput(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(
                        DisableImportFindingsForProductError::LimitExceeded(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DisableImportFindingsForProductError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DisableImportFindingsForProductError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisableImportFindingsForProductError::Internal(ref cause) => write!(f, "{}", cause),
            DisableImportFindingsForProductError::InvalidAccess(ref cause) => {
                write!(f, "{}", cause)
            }
            DisableImportFindingsForProductError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DisableImportFindingsForProductError::LimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            DisableImportFindingsForProductError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DisableImportFindingsForProductError {}
/// Errors returned by DisableOrganizationAdminAccount
#[derive(Debug, PartialEq)]
pub enum DisableOrganizationAdminAccountError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>There is an issue with the account used to make the request. Either Security Hub is not enabled for the account, or the account does not have permission to perform this action.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account or throttling limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
}

impl DisableOrganizationAdminAccountError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisableOrganizationAdminAccountError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DisableOrganizationAdminAccountError::Internal(
                        err.msg,
                    ))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(
                        DisableOrganizationAdminAccountError::InvalidAccess(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(
                        DisableOrganizationAdminAccountError::InvalidInput(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(
                        DisableOrganizationAdminAccountError::LimitExceeded(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DisableOrganizationAdminAccountError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisableOrganizationAdminAccountError::Internal(ref cause) => write!(f, "{}", cause),
            DisableOrganizationAdminAccountError::InvalidAccess(ref cause) => {
                write!(f, "{}", cause)
            }
            DisableOrganizationAdminAccountError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DisableOrganizationAdminAccountError::LimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DisableOrganizationAdminAccountError {}
/// Errors returned by DisableSecurityHub
#[derive(Debug, PartialEq)]
pub enum DisableSecurityHubError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>There is an issue with the account used to make the request. Either Security Hub is not enabled for the account, or the account does not have permission to perform this action.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account or throttling limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because we can't find the specified resource.</p>
    ResourceNotFound(String),
}

impl DisableSecurityHubError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisableSecurityHubError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DisableSecurityHubError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(DisableSecurityHubError::InvalidAccess(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DisableSecurityHubError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DisableSecurityHubError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DisableSecurityHubError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisableSecurityHubError::Internal(ref cause) => write!(f, "{}", cause),
            DisableSecurityHubError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            DisableSecurityHubError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DisableSecurityHubError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DisableSecurityHubError {}
/// Errors returned by DisassociateFromAdministratorAccount
#[derive(Debug, PartialEq)]
pub enum DisassociateFromAdministratorAccountError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>There is an issue with the account used to make the request. Either Security Hub is not enabled for the account, or the account does not have permission to perform this action.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account or throttling limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because we can't find the specified resource.</p>
    ResourceNotFound(String),
}

impl DisassociateFromAdministratorAccountError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisassociateFromAdministratorAccountError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(
                        DisassociateFromAdministratorAccountError::Internal(err.msg),
                    )
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(
                        DisassociateFromAdministratorAccountError::InvalidAccess(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(
                        DisassociateFromAdministratorAccountError::InvalidInput(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(
                        DisassociateFromAdministratorAccountError::LimitExceeded(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DisassociateFromAdministratorAccountError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DisassociateFromAdministratorAccountError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateFromAdministratorAccountError::Internal(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateFromAdministratorAccountError::InvalidAccess(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateFromAdministratorAccountError::InvalidInput(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateFromAdministratorAccountError::LimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateFromAdministratorAccountError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DisassociateFromAdministratorAccountError {}
/// Errors returned by DisassociateFromMasterAccount
#[derive(Debug, PartialEq)]
pub enum DisassociateFromMasterAccountError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>There is an issue with the account used to make the request. Either Security Hub is not enabled for the account, or the account does not have permission to perform this action.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account or throttling limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because we can't find the specified resource.</p>
    ResourceNotFound(String),
}

impl DisassociateFromMasterAccountError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisassociateFromMasterAccountError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DisassociateFromMasterAccountError::Internal(
                        err.msg,
                    ))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(DisassociateFromMasterAccountError::InvalidAccess(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DisassociateFromMasterAccountError::InvalidInput(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DisassociateFromMasterAccountError::LimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DisassociateFromMasterAccountError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DisassociateFromMasterAccountError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateFromMasterAccountError::Internal(ref cause) => write!(f, "{}", cause),
            DisassociateFromMasterAccountError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            DisassociateFromMasterAccountError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DisassociateFromMasterAccountError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DisassociateFromMasterAccountError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DisassociateFromMasterAccountError {}
/// Errors returned by DisassociateMembers
#[derive(Debug, PartialEq)]
pub enum DisassociateMembersError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>There is an issue with the account used to make the request. Either Security Hub is not enabled for the account, or the account does not have permission to perform this action.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account or throttling limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because we can't find the specified resource.</p>
    ResourceNotFound(String),
}

impl DisassociateMembersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisassociateMembersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DisassociateMembersError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(DisassociateMembersError::InvalidAccess(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DisassociateMembersError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DisassociateMembersError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DisassociateMembersError::ResourceNotFound(
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
impl fmt::Display for DisassociateMembersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateMembersError::Internal(ref cause) => write!(f, "{}", cause),
            DisassociateMembersError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            DisassociateMembersError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DisassociateMembersError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DisassociateMembersError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DisassociateMembersError {}
/// Errors returned by EnableImportFindingsForProduct
#[derive(Debug, PartialEq)]
pub enum EnableImportFindingsForProductError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>There is an issue with the account used to make the request. Either Security Hub is not enabled for the account, or the account does not have permission to perform this action.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account or throttling limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
    /// <p>The resource specified in the request conflicts with an existing resource.</p>
    ResourceConflict(String),
}

impl EnableImportFindingsForProductError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<EnableImportFindingsForProductError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(EnableImportFindingsForProductError::Internal(
                        err.msg,
                    ))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(
                        EnableImportFindingsForProductError::InvalidAccess(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(EnableImportFindingsForProductError::InvalidInput(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(
                        EnableImportFindingsForProductError::LimitExceeded(err.msg),
                    )
                }
                "ResourceConflictException" => {
                    return RusotoError::Service(
                        EnableImportFindingsForProductError::ResourceConflict(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for EnableImportFindingsForProductError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EnableImportFindingsForProductError::Internal(ref cause) => write!(f, "{}", cause),
            EnableImportFindingsForProductError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            EnableImportFindingsForProductError::InvalidInput(ref cause) => write!(f, "{}", cause),
            EnableImportFindingsForProductError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            EnableImportFindingsForProductError::ResourceConflict(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for EnableImportFindingsForProductError {}
/// Errors returned by EnableOrganizationAdminAccount
#[derive(Debug, PartialEq)]
pub enum EnableOrganizationAdminAccountError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>There is an issue with the account used to make the request. Either Security Hub is not enabled for the account, or the account does not have permission to perform this action.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account or throttling limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
}

impl EnableOrganizationAdminAccountError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<EnableOrganizationAdminAccountError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(EnableOrganizationAdminAccountError::Internal(
                        err.msg,
                    ))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(
                        EnableOrganizationAdminAccountError::InvalidAccess(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(EnableOrganizationAdminAccountError::InvalidInput(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(
                        EnableOrganizationAdminAccountError::LimitExceeded(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for EnableOrganizationAdminAccountError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EnableOrganizationAdminAccountError::Internal(ref cause) => write!(f, "{}", cause),
            EnableOrganizationAdminAccountError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            EnableOrganizationAdminAccountError::InvalidInput(ref cause) => write!(f, "{}", cause),
            EnableOrganizationAdminAccountError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for EnableOrganizationAdminAccountError {}
/// Errors returned by EnableSecurityHub
#[derive(Debug, PartialEq)]
pub enum EnableSecurityHubError {
    /// <p>You don't have permission to perform the action specified in the request.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>There is an issue with the account used to make the request. Either Security Hub is not enabled for the account, or the account does not have permission to perform this action.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account or throttling limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
    /// <p>The resource specified in the request conflicts with an existing resource.</p>
    ResourceConflict(String),
}

impl EnableSecurityHubError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<EnableSecurityHubError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(EnableSecurityHubError::AccessDenied(err.msg))
                }
                "InternalException" => {
                    return RusotoError::Service(EnableSecurityHubError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(EnableSecurityHubError::InvalidAccess(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(EnableSecurityHubError::LimitExceeded(err.msg))
                }
                "ResourceConflictException" => {
                    return RusotoError::Service(EnableSecurityHubError::ResourceConflict(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for EnableSecurityHubError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EnableSecurityHubError::AccessDenied(ref cause) => write!(f, "{}", cause),
            EnableSecurityHubError::Internal(ref cause) => write!(f, "{}", cause),
            EnableSecurityHubError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            EnableSecurityHubError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            EnableSecurityHubError::ResourceConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for EnableSecurityHubError {}
/// Errors returned by GetAdministratorAccount
#[derive(Debug, PartialEq)]
pub enum GetAdministratorAccountError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>There is an issue with the account used to make the request. Either Security Hub is not enabled for the account, or the account does not have permission to perform this action.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account or throttling limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because we can't find the specified resource.</p>
    ResourceNotFound(String),
}

impl GetAdministratorAccountError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetAdministratorAccountError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(GetAdministratorAccountError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(GetAdministratorAccountError::InvalidAccess(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetAdministratorAccountError::InvalidInput(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetAdministratorAccountError::LimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetAdministratorAccountError::ResourceNotFound(
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
impl fmt::Display for GetAdministratorAccountError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetAdministratorAccountError::Internal(ref cause) => write!(f, "{}", cause),
            GetAdministratorAccountError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            GetAdministratorAccountError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetAdministratorAccountError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetAdministratorAccountError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetAdministratorAccountError {}
/// Errors returned by GetEnabledStandards
#[derive(Debug, PartialEq)]
pub enum GetEnabledStandardsError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>There is an issue with the account used to make the request. Either Security Hub is not enabled for the account, or the account does not have permission to perform this action.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account or throttling limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
}

impl GetEnabledStandardsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetEnabledStandardsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(GetEnabledStandardsError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(GetEnabledStandardsError::InvalidAccess(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetEnabledStandardsError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetEnabledStandardsError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetEnabledStandardsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetEnabledStandardsError::Internal(ref cause) => write!(f, "{}", cause),
            GetEnabledStandardsError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            GetEnabledStandardsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetEnabledStandardsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetEnabledStandardsError {}
/// Errors returned by GetFindings
#[derive(Debug, PartialEq)]
pub enum GetFindingsError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>There is an issue with the account used to make the request. Either Security Hub is not enabled for the account, or the account does not have permission to perform this action.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account or throttling limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
}

impl GetFindingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetFindingsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(GetFindingsError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(GetFindingsError::InvalidAccess(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetFindingsError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetFindingsError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetFindingsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetFindingsError::Internal(ref cause) => write!(f, "{}", cause),
            GetFindingsError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            GetFindingsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetFindingsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetFindingsError {}
/// Errors returned by GetInsightResults
#[derive(Debug, PartialEq)]
pub enum GetInsightResultsError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>There is an issue with the account used to make the request. Either Security Hub is not enabled for the account, or the account does not have permission to perform this action.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account or throttling limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because we can't find the specified resource.</p>
    ResourceNotFound(String),
}

impl GetInsightResultsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetInsightResultsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(GetInsightResultsError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(GetInsightResultsError::InvalidAccess(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetInsightResultsError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetInsightResultsError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetInsightResultsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetInsightResultsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetInsightResultsError::Internal(ref cause) => write!(f, "{}", cause),
            GetInsightResultsError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            GetInsightResultsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetInsightResultsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetInsightResultsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetInsightResultsError {}
/// Errors returned by GetInsights
#[derive(Debug, PartialEq)]
pub enum GetInsightsError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>There is an issue with the account used to make the request. Either Security Hub is not enabled for the account, or the account does not have permission to perform this action.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account or throttling limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because we can't find the specified resource.</p>
    ResourceNotFound(String),
}

impl GetInsightsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetInsightsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(GetInsightsError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(GetInsightsError::InvalidAccess(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetInsightsError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetInsightsError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetInsightsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetInsightsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetInsightsError::Internal(ref cause) => write!(f, "{}", cause),
            GetInsightsError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            GetInsightsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetInsightsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetInsightsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetInsightsError {}
/// Errors returned by GetInvitationsCount
#[derive(Debug, PartialEq)]
pub enum GetInvitationsCountError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>There is an issue with the account used to make the request. Either Security Hub is not enabled for the account, or the account does not have permission to perform this action.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account or throttling limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
}

impl GetInvitationsCountError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetInvitationsCountError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(GetInvitationsCountError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(GetInvitationsCountError::InvalidAccess(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetInvitationsCountError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetInvitationsCountError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetInvitationsCountError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetInvitationsCountError::Internal(ref cause) => write!(f, "{}", cause),
            GetInvitationsCountError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            GetInvitationsCountError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetInvitationsCountError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetInvitationsCountError {}
/// Errors returned by GetMasterAccount
#[derive(Debug, PartialEq)]
pub enum GetMasterAccountError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>There is an issue with the account used to make the request. Either Security Hub is not enabled for the account, or the account does not have permission to perform this action.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account or throttling limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because we can't find the specified resource.</p>
    ResourceNotFound(String),
}

impl GetMasterAccountError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetMasterAccountError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(GetMasterAccountError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(GetMasterAccountError::InvalidAccess(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetMasterAccountError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetMasterAccountError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetMasterAccountError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetMasterAccountError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetMasterAccountError::Internal(ref cause) => write!(f, "{}", cause),
            GetMasterAccountError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            GetMasterAccountError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetMasterAccountError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetMasterAccountError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetMasterAccountError {}
/// Errors returned by GetMembers
#[derive(Debug, PartialEq)]
pub enum GetMembersError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>There is an issue with the account used to make the request. Either Security Hub is not enabled for the account, or the account does not have permission to perform this action.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account or throttling limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because we can't find the specified resource.</p>
    ResourceNotFound(String),
}

impl GetMembersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetMembersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(GetMembersError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(GetMembersError::InvalidAccess(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetMembersError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetMembersError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetMembersError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetMembersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetMembersError::Internal(ref cause) => write!(f, "{}", cause),
            GetMembersError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            GetMembersError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetMembersError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetMembersError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetMembersError {}
/// Errors returned by InviteMembers
#[derive(Debug, PartialEq)]
pub enum InviteMembersError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>There is an issue with the account used to make the request. Either Security Hub is not enabled for the account, or the account does not have permission to perform this action.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account or throttling limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because we can't find the specified resource.</p>
    ResourceNotFound(String),
}

impl InviteMembersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<InviteMembersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(InviteMembersError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(InviteMembersError::InvalidAccess(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(InviteMembersError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(InviteMembersError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(InviteMembersError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for InviteMembersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InviteMembersError::Internal(ref cause) => write!(f, "{}", cause),
            InviteMembersError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            InviteMembersError::InvalidInput(ref cause) => write!(f, "{}", cause),
            InviteMembersError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            InviteMembersError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for InviteMembersError {}
/// Errors returned by ListEnabledProductsForImport
#[derive(Debug, PartialEq)]
pub enum ListEnabledProductsForImportError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>There is an issue with the account used to make the request. Either Security Hub is not enabled for the account, or the account does not have permission to perform this action.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account or throttling limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
}

impl ListEnabledProductsForImportError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListEnabledProductsForImportError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(ListEnabledProductsForImportError::Internal(
                        err.msg,
                    ))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(ListEnabledProductsForImportError::InvalidAccess(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListEnabledProductsForImportError::LimitExceeded(
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
impl fmt::Display for ListEnabledProductsForImportError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListEnabledProductsForImportError::Internal(ref cause) => write!(f, "{}", cause),
            ListEnabledProductsForImportError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            ListEnabledProductsForImportError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListEnabledProductsForImportError {}
/// Errors returned by ListInvitations
#[derive(Debug, PartialEq)]
pub enum ListInvitationsError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>There is an issue with the account used to make the request. Either Security Hub is not enabled for the account, or the account does not have permission to perform this action.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account or throttling limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
}

impl ListInvitationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListInvitationsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(ListInvitationsError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(ListInvitationsError::InvalidAccess(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ListInvitationsError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListInvitationsError::LimitExceeded(err.msg))
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
            ListInvitationsError::Internal(ref cause) => write!(f, "{}", cause),
            ListInvitationsError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            ListInvitationsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            ListInvitationsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListInvitationsError {}
/// Errors returned by ListMembers
#[derive(Debug, PartialEq)]
pub enum ListMembersError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>There is an issue with the account used to make the request. Either Security Hub is not enabled for the account, or the account does not have permission to perform this action.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account or throttling limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
}

impl ListMembersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListMembersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(ListMembersError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(ListMembersError::InvalidAccess(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ListMembersError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListMembersError::LimitExceeded(err.msg))
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
            ListMembersError::Internal(ref cause) => write!(f, "{}", cause),
            ListMembersError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            ListMembersError::InvalidInput(ref cause) => write!(f, "{}", cause),
            ListMembersError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListMembersError {}
/// Errors returned by ListOrganizationAdminAccounts
#[derive(Debug, PartialEq)]
pub enum ListOrganizationAdminAccountsError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>There is an issue with the account used to make the request. Either Security Hub is not enabled for the account, or the account does not have permission to perform this action.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account or throttling limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
}

impl ListOrganizationAdminAccountsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListOrganizationAdminAccountsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(ListOrganizationAdminAccountsError::Internal(
                        err.msg,
                    ))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(ListOrganizationAdminAccountsError::InvalidAccess(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ListOrganizationAdminAccountsError::InvalidInput(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListOrganizationAdminAccountsError::LimitExceeded(
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
impl fmt::Display for ListOrganizationAdminAccountsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListOrganizationAdminAccountsError::Internal(ref cause) => write!(f, "{}", cause),
            ListOrganizationAdminAccountsError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            ListOrganizationAdminAccountsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            ListOrganizationAdminAccountsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListOrganizationAdminAccountsError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because we can't find the specified resource.</p>
    ResourceNotFound(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(ListTagsForResourceError::Internal(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ListTagsForResourceError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListTagsForResourceError::ResourceNotFound(
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
impl fmt::Display for ListTagsForResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTagsForResourceError::Internal(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::InvalidInput(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because we can't find the specified resource.</p>
    ResourceNotFound(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(TagResourceError::Internal(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(TagResourceError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(TagResourceError::ResourceNotFound(err.msg))
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
            TagResourceError::Internal(ref cause) => write!(f, "{}", cause),
            TagResourceError::InvalidInput(ref cause) => write!(f, "{}", cause),
            TagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because we can't find the specified resource.</p>
    ResourceNotFound(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(UntagResourceError::Internal(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(UntagResourceError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UntagResourceError::ResourceNotFound(err.msg))
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
            UntagResourceError::Internal(ref cause) => write!(f, "{}", cause),
            UntagResourceError::InvalidInput(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateActionTarget
#[derive(Debug, PartialEq)]
pub enum UpdateActionTargetError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>There is an issue with the account used to make the request. Either Security Hub is not enabled for the account, or the account does not have permission to perform this action.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because we can't find the specified resource.</p>
    ResourceNotFound(String),
}

impl UpdateActionTargetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateActionTargetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(UpdateActionTargetError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(UpdateActionTargetError::InvalidAccess(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(UpdateActionTargetError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateActionTargetError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateActionTargetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateActionTargetError::Internal(ref cause) => write!(f, "{}", cause),
            UpdateActionTargetError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            UpdateActionTargetError::InvalidInput(ref cause) => write!(f, "{}", cause),
            UpdateActionTargetError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateActionTargetError {}
/// Errors returned by UpdateFindings
#[derive(Debug, PartialEq)]
pub enum UpdateFindingsError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>There is an issue with the account used to make the request. Either Security Hub is not enabled for the account, or the account does not have permission to perform this action.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account or throttling limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because we can't find the specified resource.</p>
    ResourceNotFound(String),
}

impl UpdateFindingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateFindingsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(UpdateFindingsError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(UpdateFindingsError::InvalidAccess(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(UpdateFindingsError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateFindingsError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateFindingsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateFindingsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateFindingsError::Internal(ref cause) => write!(f, "{}", cause),
            UpdateFindingsError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            UpdateFindingsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            UpdateFindingsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateFindingsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateFindingsError {}
/// Errors returned by UpdateInsight
#[derive(Debug, PartialEq)]
pub enum UpdateInsightError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>There is an issue with the account used to make the request. Either Security Hub is not enabled for the account, or the account does not have permission to perform this action.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account or throttling limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because we can't find the specified resource.</p>
    ResourceNotFound(String),
}

impl UpdateInsightError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateInsightError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(UpdateInsightError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(UpdateInsightError::InvalidAccess(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(UpdateInsightError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateInsightError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateInsightError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateInsightError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateInsightError::Internal(ref cause) => write!(f, "{}", cause),
            UpdateInsightError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            UpdateInsightError::InvalidInput(ref cause) => write!(f, "{}", cause),
            UpdateInsightError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateInsightError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateInsightError {}
/// Errors returned by UpdateOrganizationConfiguration
#[derive(Debug, PartialEq)]
pub enum UpdateOrganizationConfigurationError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>There is an issue with the account used to make the request. Either Security Hub is not enabled for the account, or the account does not have permission to perform this action.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account or throttling limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
}

impl UpdateOrganizationConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateOrganizationConfigurationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(UpdateOrganizationConfigurationError::Internal(
                        err.msg,
                    ))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(
                        UpdateOrganizationConfigurationError::InvalidAccess(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(
                        UpdateOrganizationConfigurationError::InvalidInput(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(
                        UpdateOrganizationConfigurationError::LimitExceeded(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateOrganizationConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateOrganizationConfigurationError::Internal(ref cause) => write!(f, "{}", cause),
            UpdateOrganizationConfigurationError::InvalidAccess(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateOrganizationConfigurationError::InvalidInput(ref cause) => write!(f, "{}", cause),
            UpdateOrganizationConfigurationError::LimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateOrganizationConfigurationError {}
/// Errors returned by UpdateSecurityHubConfiguration
#[derive(Debug, PartialEq)]
pub enum UpdateSecurityHubConfigurationError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>There is an issue with the account used to make the request. Either Security Hub is not enabled for the account, or the account does not have permission to perform this action.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account or throttling limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because we can't find the specified resource.</p>
    ResourceNotFound(String),
}

impl UpdateSecurityHubConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateSecurityHubConfigurationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(UpdateSecurityHubConfigurationError::Internal(
                        err.msg,
                    ))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(
                        UpdateSecurityHubConfigurationError::InvalidAccess(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(UpdateSecurityHubConfigurationError::InvalidInput(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(
                        UpdateSecurityHubConfigurationError::LimitExceeded(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        UpdateSecurityHubConfigurationError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateSecurityHubConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateSecurityHubConfigurationError::Internal(ref cause) => write!(f, "{}", cause),
            UpdateSecurityHubConfigurationError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            UpdateSecurityHubConfigurationError::InvalidInput(ref cause) => write!(f, "{}", cause),
            UpdateSecurityHubConfigurationError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateSecurityHubConfigurationError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateSecurityHubConfigurationError {}
/// Errors returned by UpdateStandardsControl
#[derive(Debug, PartialEq)]
pub enum UpdateStandardsControlError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>There is an issue with the account used to make the request. Either Security Hub is not enabled for the account, or the account does not have permission to perform this action.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because we can't find the specified resource.</p>
    ResourceNotFound(String),
}

impl UpdateStandardsControlError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateStandardsControlError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(UpdateStandardsControlError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(UpdateStandardsControlError::InvalidAccess(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(UpdateStandardsControlError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateStandardsControlError::ResourceNotFound(
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
impl fmt::Display for UpdateStandardsControlError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateStandardsControlError::Internal(ref cause) => write!(f, "{}", cause),
            UpdateStandardsControlError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            UpdateStandardsControlError::InvalidInput(ref cause) => write!(f, "{}", cause),
            UpdateStandardsControlError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateStandardsControlError {}
/// Trait representing the capabilities of the AWS SecurityHub API. AWS SecurityHub clients implement this trait.
#[async_trait]
pub trait SecurityHub {
    /// <p>Accepts the invitation to be a member account and be monitored by the Security Hub administrator account that the invitation was sent from.</p> <p>This operation is only used by member accounts that are not added through Organizations.</p> <p>When the member account accepts the invitation, permission is granted to the administrator account to view findings generated in the member account.</p>
    async fn accept_administrator_invitation(
        &self,
        input: AcceptAdministratorInvitationRequest,
    ) -> Result<
        AcceptAdministratorInvitationResponse,
        RusotoError<AcceptAdministratorInvitationError>,
    >;

    /// <p>This method is deprecated. Instead, use <code>AcceptAdministratorInvitation</code>.</p> <p>The Security Hub console continues to use <code>AcceptInvitation</code>. It will eventually change to use <code>AcceptAdministratorInvitation</code>. Any IAM policies that specifically control access to this function must continue to use <code>AcceptInvitation</code>. You should also add <code>AcceptAdministratorInvitation</code> to your policies to ensure that the correct permissions are in place after the console begins to use <code>AcceptAdministratorInvitation</code>.</p> <p>Accepts the invitation to be a member account and be monitored by the Security Hub administrator account that the invitation was sent from.</p> <p>This operation is only used by member accounts that are not added through Organizations.</p> <p>When the member account accepts the invitation, permission is granted to the administrator account to view findings generated in the member account.</p>
    async fn accept_invitation(
        &self,
        input: AcceptInvitationRequest,
    ) -> Result<AcceptInvitationResponse, RusotoError<AcceptInvitationError>>;

    /// <p>Disables the standards specified by the provided <code>StandardsSubscriptionArns</code>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/securityhub/latest/userguide/securityhub-standards.html">Security Standards</a> section of the <i>AWS Security Hub User Guide</i>.</p>
    async fn batch_disable_standards(
        &self,
        input: BatchDisableStandardsRequest,
    ) -> Result<BatchDisableStandardsResponse, RusotoError<BatchDisableStandardsError>>;

    /// <p>Enables the standards specified by the provided <code>StandardsArn</code>. To obtain the ARN for a standard, use the <code> <a>DescribeStandards</a> </code> operation.</p> <p>For more information, see the <a href="https://docs.aws.amazon.com/securityhub/latest/userguide/securityhub-standards.html">Security Standards</a> section of the <i>AWS Security Hub User Guide</i>.</p>
    async fn batch_enable_standards(
        &self,
        input: BatchEnableStandardsRequest,
    ) -> Result<BatchEnableStandardsResponse, RusotoError<BatchEnableStandardsError>>;

    /// <p>Imports security findings generated from an integrated product into Security Hub. This action is requested by the integrated product to import its findings into Security Hub.</p> <p>The maximum allowed size for a finding is 240 Kb. An error is returned for any finding larger than 240 Kb.</p> <p>After a finding is created, <code>BatchImportFindings</code> cannot be used to update the following finding fields and objects, which Security Hub customers use to manage their investigation workflow.</p> <ul> <li> <p> <code>Note</code> </p> </li> <li> <p> <code>UserDefinedFields</code> </p> </li> <li> <p> <code>VerificationState</code> </p> </li> <li> <p> <code>Workflow</code> </p> </li> </ul> <p>Finding providers also should not use <code>BatchImportFindings</code> to update the following attributes.</p> <ul> <li> <p> <code>Confidence</code> </p> </li> <li> <p> <code>Criticality</code> </p> </li> <li> <p> <code>RelatedFindings</code> </p> </li> <li> <p> <code>Severity</code> </p> </li> <li> <p> <code>Types</code> </p> </li> </ul> <p>Instead, finding providers use <code>FindingProviderFields</code> to provide values for these attributes.</p>
    async fn batch_import_findings(
        &self,
        input: BatchImportFindingsRequest,
    ) -> Result<BatchImportFindingsResponse, RusotoError<BatchImportFindingsError>>;

    /// <p>Used by Security Hub customers to update information about their investigation into a finding. Requested by administrator accounts or member accounts. Administrator accounts can update findings for their account and their member accounts. Member accounts can update findings for their account.</p> <p>Updates from <code>BatchUpdateFindings</code> do not affect the value of <code>UpdatedAt</code> for a finding.</p> <p>Administrator and member accounts can use <code>BatchUpdateFindings</code> to update the following finding fields and objects.</p> <ul> <li> <p> <code>Confidence</code> </p> </li> <li> <p> <code>Criticality</code> </p> </li> <li> <p> <code>Note</code> </p> </li> <li> <p> <code>RelatedFindings</code> </p> </li> <li> <p> <code>Severity</code> </p> </li> <li> <p> <code>Types</code> </p> </li> <li> <p> <code>UserDefinedFields</code> </p> </li> <li> <p> <code>VerificationState</code> </p> </li> <li> <p> <code>Workflow</code> </p> </li> </ul> <p>You can configure IAM policies to restrict access to fields and field values. For example, you might not want member accounts to be able to suppress findings or change the finding severity. See <a href="https://docs.aws.amazon.com/securityhub/latest/userguide/finding-update-batchupdatefindings.html#batchupdatefindings-configure-access">Configuring access to BatchUpdateFindings</a> in the <i>AWS Security Hub User Guide</i>.</p>
    async fn batch_update_findings(
        &self,
        input: BatchUpdateFindingsRequest,
    ) -> Result<BatchUpdateFindingsResponse, RusotoError<BatchUpdateFindingsError>>;

    /// <p>Creates a custom action target in Security Hub.</p> <p>You can use custom actions on findings and insights in Security Hub to trigger target actions in Amazon CloudWatch Events.</p>
    async fn create_action_target(
        &self,
        input: CreateActionTargetRequest,
    ) -> Result<CreateActionTargetResponse, RusotoError<CreateActionTargetError>>;

    /// <p>Creates a custom insight in Security Hub. An insight is a consolidation of findings that relate to a security issue that requires attention or remediation.</p> <p>To group the related findings in the insight, use the <code>GroupByAttribute</code>.</p>
    async fn create_insight(
        &self,
        input: CreateInsightRequest,
    ) -> Result<CreateInsightResponse, RusotoError<CreateInsightError>>;

    /// <p>Creates a member association in Security Hub between the specified accounts and the account used to make the request, which is the administrator account. If you are integrated with Organizations, then the administrator account is designated by the organization management account.</p> <p> <code>CreateMembers</code> is always used to add accounts that are not organization members.</p> <p>For accounts that are part of an organization, <code>CreateMembers</code> is only used in the following cases:</p> <ul> <li> <p>Security Hub is not configured to automatically add new accounts in an organization.</p> </li> <li> <p>The account was disassociated or deleted in Security Hub.</p> </li> </ul> <p>This action can only be used by an account that has Security Hub enabled. To enable Security Hub, you can use the <code> <a>EnableSecurityHub</a> </code> operation.</p> <p>For accounts that are not organization members, you create the account association and then send an invitation to the member account. To send the invitation, you use the <code> <a>InviteMembers</a> </code> operation. If the account owner accepts the invitation, the account becomes a member account in Security Hub.</p> <p>Accounts that are part of an organization do not receive an invitation. They automatically become a member account in Security Hub.</p> <p>A permissions policy is added that permits the administrator account to view the findings generated in the member account. When Security Hub is enabled in a member account, the member account findings are also visible to the administrator account. </p> <p>To remove the association between the administrator and member accounts, use the <code> <a>DisassociateFromMasterAccount</a> </code> or <code> <a>DisassociateMembers</a> </code> operation.</p>
    async fn create_members(
        &self,
        input: CreateMembersRequest,
    ) -> Result<CreateMembersResponse, RusotoError<CreateMembersError>>;

    /// <p>Declines invitations to become a member account.</p> <p>This operation is only used by accounts that are not part of an organization. Organization accounts do not receive invitations.</p>
    async fn decline_invitations(
        &self,
        input: DeclineInvitationsRequest,
    ) -> Result<DeclineInvitationsResponse, RusotoError<DeclineInvitationsError>>;

    /// <p>Deletes a custom action target from Security Hub.</p> <p>Deleting a custom action target does not affect any findings or insights that were already sent to Amazon CloudWatch Events using the custom action.</p>
    async fn delete_action_target(
        &self,
        input: DeleteActionTargetRequest,
    ) -> Result<DeleteActionTargetResponse, RusotoError<DeleteActionTargetError>>;

    /// <p>Deletes the insight specified by the <code>InsightArn</code>.</p>
    async fn delete_insight(
        &self,
        input: DeleteInsightRequest,
    ) -> Result<DeleteInsightResponse, RusotoError<DeleteInsightError>>;

    /// <p>Deletes invitations received by the AWS account to become a member account.</p> <p>This operation is only used by accounts that are not part of an organization. Organization accounts do not receive invitations.</p>
    async fn delete_invitations(
        &self,
        input: DeleteInvitationsRequest,
    ) -> Result<DeleteInvitationsResponse, RusotoError<DeleteInvitationsError>>;

    /// <p>Deletes the specified member accounts from Security Hub.</p> <p>Can be used to delete member accounts that belong to an organization as well as member accounts that were invited manually.</p>
    async fn delete_members(
        &self,
        input: DeleteMembersRequest,
    ) -> Result<DeleteMembersResponse, RusotoError<DeleteMembersError>>;

    /// <p>Returns a list of the custom action targets in Security Hub in your account.</p>
    async fn describe_action_targets(
        &self,
        input: DescribeActionTargetsRequest,
    ) -> Result<DescribeActionTargetsResponse, RusotoError<DescribeActionTargetsError>>;

    /// <p>Returns details about the Hub resource in your account, including the <code>HubArn</code> and the time when you enabled Security Hub.</p>
    async fn describe_hub(
        &self,
        input: DescribeHubRequest,
    ) -> Result<DescribeHubResponse, RusotoError<DescribeHubError>>;

    /// <p>Returns information about the Organizations configuration for Security Hub. Can only be called from a Security Hub administrator account.</p>
    async fn describe_organization_configuration(
        &self,
    ) -> Result<
        DescribeOrganizationConfigurationResponse,
        RusotoError<DescribeOrganizationConfigurationError>,
    >;

    /// <p>Returns information about product integrations in Security Hub.</p> <p>You can optionally provide an integration ARN. If you provide an integration ARN, then the results only include that integration.</p> <p>If you do not provide an integration ARN, then the results include all of the available product integrations. </p>
    async fn describe_products(
        &self,
        input: DescribeProductsRequest,
    ) -> Result<DescribeProductsResponse, RusotoError<DescribeProductsError>>;

    /// <p>Returns a list of the available standards in Security Hub.</p> <p>For each standard, the results include the standard ARN, the name, and a description. </p>
    async fn describe_standards(
        &self,
        input: DescribeStandardsRequest,
    ) -> Result<DescribeStandardsResponse, RusotoError<DescribeStandardsError>>;

    /// <p>Returns a list of security standards controls.</p> <p>For each control, the results include information about whether it is currently enabled, the severity, and a link to remediation information.</p>
    async fn describe_standards_controls(
        &self,
        input: DescribeStandardsControlsRequest,
    ) -> Result<DescribeStandardsControlsResponse, RusotoError<DescribeStandardsControlsError>>;

    /// <p>Disables the integration of the specified product with Security Hub. After the integration is disabled, findings from that product are no longer sent to Security Hub.</p>
    async fn disable_import_findings_for_product(
        &self,
        input: DisableImportFindingsForProductRequest,
    ) -> Result<
        DisableImportFindingsForProductResponse,
        RusotoError<DisableImportFindingsForProductError>,
    >;

    /// <p>Disables a Security Hub administrator account. Can only be called by the organization management account.</p>
    async fn disable_organization_admin_account(
        &self,
        input: DisableOrganizationAdminAccountRequest,
    ) -> Result<
        DisableOrganizationAdminAccountResponse,
        RusotoError<DisableOrganizationAdminAccountError>,
    >;

    /// <p>Disables Security Hub in your account only in the current Region. To disable Security Hub in all Regions, you must submit one request per Region where you have enabled Security Hub.</p> <p>When you disable Security Hub for an administrator account, it doesn't disable Security Hub for any associated member accounts.</p> <p>When you disable Security Hub, your existing findings and insights and any Security Hub configuration settings are deleted after 90 days and cannot be recovered. Any standards that were enabled are disabled, and your administrator and member account associations are removed.</p> <p>If you want to save your existing findings, you must export them before you disable Security Hub.</p>
    async fn disable_security_hub(
        &self,
    ) -> Result<DisableSecurityHubResponse, RusotoError<DisableSecurityHubError>>;

    /// <p>Disassociates the current Security Hub member account from the associated administrator account.</p> <p>This operation is only used by accounts that are not part of an organization. For organization accounts, only the administrator account can disassociate a member account.</p>
    async fn disassociate_from_administrator_account(
        &self,
    ) -> Result<
        DisassociateFromAdministratorAccountResponse,
        RusotoError<DisassociateFromAdministratorAccountError>,
    >;

    /// <p>This method is deprecated. Instead, use <code>DisassociateFromAdministratorAccount</code>.</p> <p>The Security Hub console continues to use <code>DisassociateFromMasterAccount</code>. It will eventually change to use <code>DisassociateFromAdministratorAccount</code>. Any IAM policies that specifically control access to this function must continue to use <code>DisassociateFromMasterAccount</code>. You should also add <code>DisassociateFromAdministratorAccount</code> to your policies to ensure that the correct permissions are in place after the console begins to use <code>DisassociateFromAdministratorAccount</code>.</p> <p>Disassociates the current Security Hub member account from the associated administrator account.</p> <p>This operation is only used by accounts that are not part of an organization. For organization accounts, only the administrator account can disassociate a member account.</p>
    async fn disassociate_from_master_account(
        &self,
    ) -> Result<
        DisassociateFromMasterAccountResponse,
        RusotoError<DisassociateFromMasterAccountError>,
    >;

    /// <p>Disassociates the specified member accounts from the associated administrator account.</p> <p>Can be used to disassociate both accounts that are managed using Organizations and accounts that were invited manually.</p>
    async fn disassociate_members(
        &self,
        input: DisassociateMembersRequest,
    ) -> Result<DisassociateMembersResponse, RusotoError<DisassociateMembersError>>;

    /// <p>Enables the integration of a partner product with Security Hub. Integrated products send findings to Security Hub.</p> <p>When you enable a product integration, a permissions policy that grants permission for the product to send findings to Security Hub is applied.</p>
    async fn enable_import_findings_for_product(
        &self,
        input: EnableImportFindingsForProductRequest,
    ) -> Result<
        EnableImportFindingsForProductResponse,
        RusotoError<EnableImportFindingsForProductError>,
    >;

    /// <p>Designates the Security Hub administrator account for an organization. Can only be called by the organization management account.</p>
    async fn enable_organization_admin_account(
        &self,
        input: EnableOrganizationAdminAccountRequest,
    ) -> Result<
        EnableOrganizationAdminAccountResponse,
        RusotoError<EnableOrganizationAdminAccountError>,
    >;

    /// <p>Enables Security Hub for your account in the current Region or the Region you specify in the request.</p> <p>When you enable Security Hub, you grant to Security Hub the permissions necessary to gather findings from other services that are integrated with Security Hub.</p> <p>When you use the <code>EnableSecurityHub</code> operation to enable Security Hub, you also automatically enable the following standards.</p> <ul> <li> <p>CIS AWS Foundations</p> </li> <li> <p>AWS Foundational Security Best Practices</p> </li> </ul> <p>You do not enable the Payment Card Industry Data Security Standard (PCI DSS) standard. </p> <p>To not enable the automatically enabled standards, set <code>EnableDefaultStandards</code> to <code>false</code>.</p> <p>After you enable Security Hub, to enable a standard, use the <code> <a>BatchEnableStandards</a> </code> operation. To disable a standard, use the <code> <a>BatchDisableStandards</a> </code> operation.</p> <p>To learn more, see <a href="https://docs.aws.amazon.com/securityhub/latest/userguide/securityhub-settingup.html">Setting Up AWS Security Hub</a> in the <i>AWS Security Hub User Guide</i>.</p>
    async fn enable_security_hub(
        &self,
        input: EnableSecurityHubRequest,
    ) -> Result<EnableSecurityHubResponse, RusotoError<EnableSecurityHubError>>;

    /// <p>Provides the details for the Security Hub administrator account for the current member account.</p> <p>Can be used by both member accounts that are managed using Organizations and accounts that were invited manually.</p>
    async fn get_administrator_account(
        &self,
    ) -> Result<GetAdministratorAccountResponse, RusotoError<GetAdministratorAccountError>>;

    /// <p>Returns a list of the standards that are currently enabled.</p>
    async fn get_enabled_standards(
        &self,
        input: GetEnabledStandardsRequest,
    ) -> Result<GetEnabledStandardsResponse, RusotoError<GetEnabledStandardsError>>;

    /// <p>Returns a list of findings that match the specified criteria.</p>
    async fn get_findings(
        &self,
        input: GetFindingsRequest,
    ) -> Result<GetFindingsResponse, RusotoError<GetFindingsError>>;

    /// <p>Lists the results of the Security Hub insight specified by the insight ARN.</p>
    async fn get_insight_results(
        &self,
        input: GetInsightResultsRequest,
    ) -> Result<GetInsightResultsResponse, RusotoError<GetInsightResultsError>>;

    /// <p>Lists and describes insights for the specified insight ARNs.</p>
    async fn get_insights(
        &self,
        input: GetInsightsRequest,
    ) -> Result<GetInsightsResponse, RusotoError<GetInsightsError>>;

    /// <p>Returns the count of all Security Hub membership invitations that were sent to the current member account, not including the currently accepted invitation. </p>
    async fn get_invitations_count(
        &self,
    ) -> Result<GetInvitationsCountResponse, RusotoError<GetInvitationsCountError>>;

    /// <p>This method is deprecated. Instead, use <code>GetAdministratorAccount</code>.</p> <p>The Security Hub console continues to use <code>GetMasterAccount</code>. It will eventually change to use <code>GetAdministratorAccount</code>. Any IAM policies that specifically control access to this function must continue to use <code>GetMasterAccount</code>. You should also add <code>GetAdministratorAccount</code> to your policies to ensure that the correct permissions are in place after the console begins to use <code>GetAdministratorAccount</code>.</p> <p>Provides the details for the Security Hub administrator account for the current member account.</p> <p>Can be used by both member accounts that are managed using Organizations and accounts that were invited manually.</p>
    async fn get_master_account(
        &self,
    ) -> Result<GetMasterAccountResponse, RusotoError<GetMasterAccountError>>;

    /// <p>Returns the details for the Security Hub member accounts for the specified account IDs.</p> <p>An administrator account can be either the delegated Security Hub administrator account for an organization or an administrator account that enabled Security Hub manually.</p> <p>The results include both member accounts that are managed using Organizations and accounts that were invited manually.</p>
    async fn get_members(
        &self,
        input: GetMembersRequest,
    ) -> Result<GetMembersResponse, RusotoError<GetMembersError>>;

    /// <p>Invites other AWS accounts to become member accounts for the Security Hub administrator account that the invitation is sent from.</p> <p>This operation is only used to invite accounts that do not belong to an organization. Organization accounts do not receive invitations.</p> <p>Before you can use this action to invite a member, you must first use the <code> <a>CreateMembers</a> </code> action to create the member account in Security Hub.</p> <p>When the account owner enables Security Hub and accepts the invitation to become a member account, the administrator account can view the findings generated from the member account.</p>
    async fn invite_members(
        &self,
        input: InviteMembersRequest,
    ) -> Result<InviteMembersResponse, RusotoError<InviteMembersError>>;

    /// <p>Lists all findings-generating solutions (products) that you are subscribed to receive findings from in Security Hub.</p>
    async fn list_enabled_products_for_import(
        &self,
        input: ListEnabledProductsForImportRequest,
    ) -> Result<ListEnabledProductsForImportResponse, RusotoError<ListEnabledProductsForImportError>>;

    /// <p>Lists all Security Hub membership invitations that were sent to the current AWS account.</p> <p>This operation is only used by accounts that are managed by invitation. Accounts that are managed using the integration with AWS Organizations do not receive invitations.</p>
    async fn list_invitations(
        &self,
        input: ListInvitationsRequest,
    ) -> Result<ListInvitationsResponse, RusotoError<ListInvitationsError>>;

    /// <p>Lists details about all member accounts for the current Security Hub administrator account.</p> <p>The results include both member accounts that belong to an organization and member accounts that were invited manually.</p>
    async fn list_members(
        &self,
        input: ListMembersRequest,
    ) -> Result<ListMembersResponse, RusotoError<ListMembersError>>;

    /// <p>Lists the Security Hub administrator accounts. Can only be called by the organization management account.</p>
    async fn list_organization_admin_accounts(
        &self,
        input: ListOrganizationAdminAccountsRequest,
    ) -> Result<
        ListOrganizationAdminAccountsResponse,
        RusotoError<ListOrganizationAdminAccountsError>,
    >;

    /// <p>Returns a list of tags associated with a resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>Adds one or more tags to a resource.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>>;

    /// <p>Removes one or more tags from a resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>>;

    /// <p>Updates the name and description of a custom action target in Security Hub.</p>
    async fn update_action_target(
        &self,
        input: UpdateActionTargetRequest,
    ) -> Result<UpdateActionTargetResponse, RusotoError<UpdateActionTargetError>>;

    /// <p> <code>UpdateFindings</code> is deprecated. Instead of <code>UpdateFindings</code>, use <code>BatchUpdateFindings</code>.</p> <p>Updates the <code>Note</code> and <code>RecordState</code> of the Security Hub-aggregated findings that the filter attributes specify. Any member account that can view the finding also sees the update to the finding.</p>
    async fn update_findings(
        &self,
        input: UpdateFindingsRequest,
    ) -> Result<UpdateFindingsResponse, RusotoError<UpdateFindingsError>>;

    /// <p>Updates the Security Hub insight identified by the specified insight ARN.</p>
    async fn update_insight(
        &self,
        input: UpdateInsightRequest,
    ) -> Result<UpdateInsightResponse, RusotoError<UpdateInsightError>>;

    /// <p>Used to update the configuration related to Organizations. Can only be called from a Security Hub administrator account.</p>
    async fn update_organization_configuration(
        &self,
        input: UpdateOrganizationConfigurationRequest,
    ) -> Result<
        UpdateOrganizationConfigurationResponse,
        RusotoError<UpdateOrganizationConfigurationError>,
    >;

    /// <p>Updates configuration options for Security Hub.</p>
    async fn update_security_hub_configuration(
        &self,
        input: UpdateSecurityHubConfigurationRequest,
    ) -> Result<
        UpdateSecurityHubConfigurationResponse,
        RusotoError<UpdateSecurityHubConfigurationError>,
    >;

    /// <p>Used to control whether an individual security standard control is enabled or disabled.</p>
    async fn update_standards_control(
        &self,
        input: UpdateStandardsControlRequest,
    ) -> Result<UpdateStandardsControlResponse, RusotoError<UpdateStandardsControlError>>;
}
/// A client for the AWS SecurityHub API.
#[derive(Clone)]
pub struct SecurityHubClient {
    client: Client,
    region: region::Region,
}

impl SecurityHubClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> SecurityHubClient {
        SecurityHubClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> SecurityHubClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        SecurityHubClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> SecurityHubClient {
        SecurityHubClient { client, region }
    }
}

#[async_trait]
impl SecurityHub for SecurityHubClient {
    /// <p>Accepts the invitation to be a member account and be monitored by the Security Hub administrator account that the invitation was sent from.</p> <p>This operation is only used by member accounts that are not added through Organizations.</p> <p>When the member account accepts the invitation, permission is granted to the administrator account to view findings generated in the member account.</p>
    #[allow(unused_mut)]
    async fn accept_administrator_invitation(
        &self,
        input: AcceptAdministratorInvitationRequest,
    ) -> Result<
        AcceptAdministratorInvitationResponse,
        RusotoError<AcceptAdministratorInvitationError>,
    > {
        let request_uri = "/administrator";

        let mut request = SignedRequest::new("POST", "securityhub", &self.region, &request_uri);
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
                .deserialize::<AcceptAdministratorInvitationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(AcceptAdministratorInvitationError::from_response(response))
        }
    }

    /// <p>This method is deprecated. Instead, use <code>AcceptAdministratorInvitation</code>.</p> <p>The Security Hub console continues to use <code>AcceptInvitation</code>. It will eventually change to use <code>AcceptAdministratorInvitation</code>. Any IAM policies that specifically control access to this function must continue to use <code>AcceptInvitation</code>. You should also add <code>AcceptAdministratorInvitation</code> to your policies to ensure that the correct permissions are in place after the console begins to use <code>AcceptAdministratorInvitation</code>.</p> <p>Accepts the invitation to be a member account and be monitored by the Security Hub administrator account that the invitation was sent from.</p> <p>This operation is only used by member accounts that are not added through Organizations.</p> <p>When the member account accepts the invitation, permission is granted to the administrator account to view findings generated in the member account.</p>
    #[allow(unused_mut)]
    async fn accept_invitation(
        &self,
        input: AcceptInvitationRequest,
    ) -> Result<AcceptInvitationResponse, RusotoError<AcceptInvitationError>> {
        let request_uri = "/master";

        let mut request = SignedRequest::new("POST", "securityhub", &self.region, &request_uri);
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
                .deserialize::<AcceptInvitationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(AcceptInvitationError::from_response(response))
        }
    }

    /// <p>Disables the standards specified by the provided <code>StandardsSubscriptionArns</code>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/securityhub/latest/userguide/securityhub-standards.html">Security Standards</a> section of the <i>AWS Security Hub User Guide</i>.</p>
    #[allow(unused_mut)]
    async fn batch_disable_standards(
        &self,
        input: BatchDisableStandardsRequest,
    ) -> Result<BatchDisableStandardsResponse, RusotoError<BatchDisableStandardsError>> {
        let request_uri = "/standards/deregister";

        let mut request = SignedRequest::new("POST", "securityhub", &self.region, &request_uri);
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
                .deserialize::<BatchDisableStandardsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(BatchDisableStandardsError::from_response(response))
        }
    }

    /// <p>Enables the standards specified by the provided <code>StandardsArn</code>. To obtain the ARN for a standard, use the <code> <a>DescribeStandards</a> </code> operation.</p> <p>For more information, see the <a href="https://docs.aws.amazon.com/securityhub/latest/userguide/securityhub-standards.html">Security Standards</a> section of the <i>AWS Security Hub User Guide</i>.</p>
    #[allow(unused_mut)]
    async fn batch_enable_standards(
        &self,
        input: BatchEnableStandardsRequest,
    ) -> Result<BatchEnableStandardsResponse, RusotoError<BatchEnableStandardsError>> {
        let request_uri = "/standards/register";

        let mut request = SignedRequest::new("POST", "securityhub", &self.region, &request_uri);
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
                .deserialize::<BatchEnableStandardsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(BatchEnableStandardsError::from_response(response))
        }
    }

    /// <p>Imports security findings generated from an integrated product into Security Hub. This action is requested by the integrated product to import its findings into Security Hub.</p> <p>The maximum allowed size for a finding is 240 Kb. An error is returned for any finding larger than 240 Kb.</p> <p>After a finding is created, <code>BatchImportFindings</code> cannot be used to update the following finding fields and objects, which Security Hub customers use to manage their investigation workflow.</p> <ul> <li> <p> <code>Note</code> </p> </li> <li> <p> <code>UserDefinedFields</code> </p> </li> <li> <p> <code>VerificationState</code> </p> </li> <li> <p> <code>Workflow</code> </p> </li> </ul> <p>Finding providers also should not use <code>BatchImportFindings</code> to update the following attributes.</p> <ul> <li> <p> <code>Confidence</code> </p> </li> <li> <p> <code>Criticality</code> </p> </li> <li> <p> <code>RelatedFindings</code> </p> </li> <li> <p> <code>Severity</code> </p> </li> <li> <p> <code>Types</code> </p> </li> </ul> <p>Instead, finding providers use <code>FindingProviderFields</code> to provide values for these attributes.</p>
    #[allow(unused_mut)]
    async fn batch_import_findings(
        &self,
        input: BatchImportFindingsRequest,
    ) -> Result<BatchImportFindingsResponse, RusotoError<BatchImportFindingsError>> {
        let request_uri = "/findings/import";

        let mut request = SignedRequest::new("POST", "securityhub", &self.region, &request_uri);
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
                .deserialize::<BatchImportFindingsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(BatchImportFindingsError::from_response(response))
        }
    }

    /// <p>Used by Security Hub customers to update information about their investigation into a finding. Requested by administrator accounts or member accounts. Administrator accounts can update findings for their account and their member accounts. Member accounts can update findings for their account.</p> <p>Updates from <code>BatchUpdateFindings</code> do not affect the value of <code>UpdatedAt</code> for a finding.</p> <p>Administrator and member accounts can use <code>BatchUpdateFindings</code> to update the following finding fields and objects.</p> <ul> <li> <p> <code>Confidence</code> </p> </li> <li> <p> <code>Criticality</code> </p> </li> <li> <p> <code>Note</code> </p> </li> <li> <p> <code>RelatedFindings</code> </p> </li> <li> <p> <code>Severity</code> </p> </li> <li> <p> <code>Types</code> </p> </li> <li> <p> <code>UserDefinedFields</code> </p> </li> <li> <p> <code>VerificationState</code> </p> </li> <li> <p> <code>Workflow</code> </p> </li> </ul> <p>You can configure IAM policies to restrict access to fields and field values. For example, you might not want member accounts to be able to suppress findings or change the finding severity. See <a href="https://docs.aws.amazon.com/securityhub/latest/userguide/finding-update-batchupdatefindings.html#batchupdatefindings-configure-access">Configuring access to BatchUpdateFindings</a> in the <i>AWS Security Hub User Guide</i>.</p>
    #[allow(unused_mut)]
    async fn batch_update_findings(
        &self,
        input: BatchUpdateFindingsRequest,
    ) -> Result<BatchUpdateFindingsResponse, RusotoError<BatchUpdateFindingsError>> {
        let request_uri = "/findings/batchupdate";

        let mut request = SignedRequest::new("PATCH", "securityhub", &self.region, &request_uri);
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
                .deserialize::<BatchUpdateFindingsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(BatchUpdateFindingsError::from_response(response))
        }
    }

    /// <p>Creates a custom action target in Security Hub.</p> <p>You can use custom actions on findings and insights in Security Hub to trigger target actions in Amazon CloudWatch Events.</p>
    #[allow(unused_mut)]
    async fn create_action_target(
        &self,
        input: CreateActionTargetRequest,
    ) -> Result<CreateActionTargetResponse, RusotoError<CreateActionTargetError>> {
        let request_uri = "/actionTargets";

        let mut request = SignedRequest::new("POST", "securityhub", &self.region, &request_uri);
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
                .deserialize::<CreateActionTargetResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateActionTargetError::from_response(response))
        }
    }

    /// <p>Creates a custom insight in Security Hub. An insight is a consolidation of findings that relate to a security issue that requires attention or remediation.</p> <p>To group the related findings in the insight, use the <code>GroupByAttribute</code>.</p>
    #[allow(unused_mut)]
    async fn create_insight(
        &self,
        input: CreateInsightRequest,
    ) -> Result<CreateInsightResponse, RusotoError<CreateInsightError>> {
        let request_uri = "/insights";

        let mut request = SignedRequest::new("POST", "securityhub", &self.region, &request_uri);
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
                .deserialize::<CreateInsightResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateInsightError::from_response(response))
        }
    }

    /// <p>Creates a member association in Security Hub between the specified accounts and the account used to make the request, which is the administrator account. If you are integrated with Organizations, then the administrator account is designated by the organization management account.</p> <p> <code>CreateMembers</code> is always used to add accounts that are not organization members.</p> <p>For accounts that are part of an organization, <code>CreateMembers</code> is only used in the following cases:</p> <ul> <li> <p>Security Hub is not configured to automatically add new accounts in an organization.</p> </li> <li> <p>The account was disassociated or deleted in Security Hub.</p> </li> </ul> <p>This action can only be used by an account that has Security Hub enabled. To enable Security Hub, you can use the <code> <a>EnableSecurityHub</a> </code> operation.</p> <p>For accounts that are not organization members, you create the account association and then send an invitation to the member account. To send the invitation, you use the <code> <a>InviteMembers</a> </code> operation. If the account owner accepts the invitation, the account becomes a member account in Security Hub.</p> <p>Accounts that are part of an organization do not receive an invitation. They automatically become a member account in Security Hub.</p> <p>A permissions policy is added that permits the administrator account to view the findings generated in the member account. When Security Hub is enabled in a member account, the member account findings are also visible to the administrator account. </p> <p>To remove the association between the administrator and member accounts, use the <code> <a>DisassociateFromMasterAccount</a> </code> or <code> <a>DisassociateMembers</a> </code> operation.</p>
    #[allow(unused_mut)]
    async fn create_members(
        &self,
        input: CreateMembersRequest,
    ) -> Result<CreateMembersResponse, RusotoError<CreateMembersError>> {
        let request_uri = "/members";

        let mut request = SignedRequest::new("POST", "securityhub", &self.region, &request_uri);
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
                .deserialize::<CreateMembersResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateMembersError::from_response(response))
        }
    }

    /// <p>Declines invitations to become a member account.</p> <p>This operation is only used by accounts that are not part of an organization. Organization accounts do not receive invitations.</p>
    #[allow(unused_mut)]
    async fn decline_invitations(
        &self,
        input: DeclineInvitationsRequest,
    ) -> Result<DeclineInvitationsResponse, RusotoError<DeclineInvitationsError>> {
        let request_uri = "/invitations/decline";

        let mut request = SignedRequest::new("POST", "securityhub", &self.region, &request_uri);
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
                .deserialize::<DeclineInvitationsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeclineInvitationsError::from_response(response))
        }
    }

    /// <p>Deletes a custom action target from Security Hub.</p> <p>Deleting a custom action target does not affect any findings or insights that were already sent to Amazon CloudWatch Events using the custom action.</p>
    #[allow(unused_mut)]
    async fn delete_action_target(
        &self,
        input: DeleteActionTargetRequest,
    ) -> Result<DeleteActionTargetResponse, RusotoError<DeleteActionTargetError>> {
        let request_uri = format!(
            "/actionTargets/{action_target_arn}",
            action_target_arn = input.action_target_arn
        );

        let mut request = SignedRequest::new("DELETE", "securityhub", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteActionTargetResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteActionTargetError::from_response(response))
        }
    }

    /// <p>Deletes the insight specified by the <code>InsightArn</code>.</p>
    #[allow(unused_mut)]
    async fn delete_insight(
        &self,
        input: DeleteInsightRequest,
    ) -> Result<DeleteInsightResponse, RusotoError<DeleteInsightError>> {
        let request_uri = format!("/insights/{insight_arn}", insight_arn = input.insight_arn);

        let mut request = SignedRequest::new("DELETE", "securityhub", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteInsightResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteInsightError::from_response(response))
        }
    }

    /// <p>Deletes invitations received by the AWS account to become a member account.</p> <p>This operation is only used by accounts that are not part of an organization. Organization accounts do not receive invitations.</p>
    #[allow(unused_mut)]
    async fn delete_invitations(
        &self,
        input: DeleteInvitationsRequest,
    ) -> Result<DeleteInvitationsResponse, RusotoError<DeleteInvitationsError>> {
        let request_uri = "/invitations/delete";

        let mut request = SignedRequest::new("POST", "securityhub", &self.region, &request_uri);
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
                .deserialize::<DeleteInvitationsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteInvitationsError::from_response(response))
        }
    }

    /// <p>Deletes the specified member accounts from Security Hub.</p> <p>Can be used to delete member accounts that belong to an organization as well as member accounts that were invited manually.</p>
    #[allow(unused_mut)]
    async fn delete_members(
        &self,
        input: DeleteMembersRequest,
    ) -> Result<DeleteMembersResponse, RusotoError<DeleteMembersError>> {
        let request_uri = "/members/delete";

        let mut request = SignedRequest::new("POST", "securityhub", &self.region, &request_uri);
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
                .deserialize::<DeleteMembersResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteMembersError::from_response(response))
        }
    }

    /// <p>Returns a list of the custom action targets in Security Hub in your account.</p>
    #[allow(unused_mut)]
    async fn describe_action_targets(
        &self,
        input: DescribeActionTargetsRequest,
    ) -> Result<DescribeActionTargetsResponse, RusotoError<DescribeActionTargetsError>> {
        let request_uri = "/actionTargets/get";

        let mut request = SignedRequest::new("POST", "securityhub", &self.region, &request_uri);
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
                .deserialize::<DescribeActionTargetsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeActionTargetsError::from_response(response))
        }
    }

    /// <p>Returns details about the Hub resource in your account, including the <code>HubArn</code> and the time when you enabled Security Hub.</p>
    #[allow(unused_mut)]
    async fn describe_hub(
        &self,
        input: DescribeHubRequest,
    ) -> Result<DescribeHubResponse, RusotoError<DescribeHubError>> {
        let request_uri = "/accounts";

        let mut request = SignedRequest::new("GET", "securityhub", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.hub_arn {
            params.put("HubArn", x);
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
                .deserialize::<DescribeHubResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeHubError::from_response(response))
        }
    }

    /// <p>Returns information about the Organizations configuration for Security Hub. Can only be called from a Security Hub administrator account.</p>
    #[allow(unused_mut)]
    async fn describe_organization_configuration(
        &self,
    ) -> Result<
        DescribeOrganizationConfigurationResponse,
        RusotoError<DescribeOrganizationConfigurationError>,
    > {
        let request_uri = "/organization/configuration";

        let mut request = SignedRequest::new("GET", "securityhub", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeOrganizationConfigurationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeOrganizationConfigurationError::from_response(
                response,
            ))
        }
    }

    /// <p>Returns information about product integrations in Security Hub.</p> <p>You can optionally provide an integration ARN. If you provide an integration ARN, then the results only include that integration.</p> <p>If you do not provide an integration ARN, then the results include all of the available product integrations. </p>
    #[allow(unused_mut)]
    async fn describe_products(
        &self,
        input: DescribeProductsRequest,
    ) -> Result<DescribeProductsResponse, RusotoError<DescribeProductsError>> {
        let request_uri = "/products";

        let mut request = SignedRequest::new("GET", "securityhub", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("MaxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        if let Some(ref x) = input.product_arn {
            params.put("ProductArn", x);
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
                .deserialize::<DescribeProductsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeProductsError::from_response(response))
        }
    }

    /// <p>Returns a list of the available standards in Security Hub.</p> <p>For each standard, the results include the standard ARN, the name, and a description. </p>
    #[allow(unused_mut)]
    async fn describe_standards(
        &self,
        input: DescribeStandardsRequest,
    ) -> Result<DescribeStandardsResponse, RusotoError<DescribeStandardsError>> {
        let request_uri = "/standards";

        let mut request = SignedRequest::new("GET", "securityhub", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("MaxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
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
                .deserialize::<DescribeStandardsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeStandardsError::from_response(response))
        }
    }

    /// <p>Returns a list of security standards controls.</p> <p>For each control, the results include information about whether it is currently enabled, the severity, and a link to remediation information.</p>
    #[allow(unused_mut)]
    async fn describe_standards_controls(
        &self,
        input: DescribeStandardsControlsRequest,
    ) -> Result<DescribeStandardsControlsResponse, RusotoError<DescribeStandardsControlsError>>
    {
        let request_uri = format!(
            "/standards/controls/{standards_subscription_arn}",
            standards_subscription_arn = input.standards_subscription_arn
        );

        let mut request = SignedRequest::new("GET", "securityhub", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("MaxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
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
                .deserialize::<DescribeStandardsControlsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeStandardsControlsError::from_response(response))
        }
    }

    /// <p>Disables the integration of the specified product with Security Hub. After the integration is disabled, findings from that product are no longer sent to Security Hub.</p>
    #[allow(unused_mut)]
    async fn disable_import_findings_for_product(
        &self,
        input: DisableImportFindingsForProductRequest,
    ) -> Result<
        DisableImportFindingsForProductResponse,
        RusotoError<DisableImportFindingsForProductError>,
    > {
        let request_uri = format!(
            "/productSubscriptions/{product_subscription_arn}",
            product_subscription_arn = input.product_subscription_arn
        );

        let mut request = SignedRequest::new("DELETE", "securityhub", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DisableImportFindingsForProductResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DisableImportFindingsForProductError::from_response(
                response,
            ))
        }
    }

    /// <p>Disables a Security Hub administrator account. Can only be called by the organization management account.</p>
    #[allow(unused_mut)]
    async fn disable_organization_admin_account(
        &self,
        input: DisableOrganizationAdminAccountRequest,
    ) -> Result<
        DisableOrganizationAdminAccountResponse,
        RusotoError<DisableOrganizationAdminAccountError>,
    > {
        let request_uri = "/organization/admin/disable";

        let mut request = SignedRequest::new("POST", "securityhub", &self.region, &request_uri);
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
                .deserialize::<DisableOrganizationAdminAccountResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DisableOrganizationAdminAccountError::from_response(
                response,
            ))
        }
    }

    /// <p>Disables Security Hub in your account only in the current Region. To disable Security Hub in all Regions, you must submit one request per Region where you have enabled Security Hub.</p> <p>When you disable Security Hub for an administrator account, it doesn't disable Security Hub for any associated member accounts.</p> <p>When you disable Security Hub, your existing findings and insights and any Security Hub configuration settings are deleted after 90 days and cannot be recovered. Any standards that were enabled are disabled, and your administrator and member account associations are removed.</p> <p>If you want to save your existing findings, you must export them before you disable Security Hub.</p>
    #[allow(unused_mut)]
    async fn disable_security_hub(
        &self,
    ) -> Result<DisableSecurityHubResponse, RusotoError<DisableSecurityHubError>> {
        let request_uri = "/accounts";

        let mut request = SignedRequest::new("DELETE", "securityhub", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DisableSecurityHubResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DisableSecurityHubError::from_response(response))
        }
    }

    /// <p>Disassociates the current Security Hub member account from the associated administrator account.</p> <p>This operation is only used by accounts that are not part of an organization. For organization accounts, only the administrator account can disassociate a member account.</p>
    #[allow(unused_mut)]
    async fn disassociate_from_administrator_account(
        &self,
    ) -> Result<
        DisassociateFromAdministratorAccountResponse,
        RusotoError<DisassociateFromAdministratorAccountError>,
    > {
        let request_uri = "/administrator/disassociate";

        let mut request = SignedRequest::new("POST", "securityhub", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DisassociateFromAdministratorAccountResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DisassociateFromAdministratorAccountError::from_response(
                response,
            ))
        }
    }

    /// <p>This method is deprecated. Instead, use <code>DisassociateFromAdministratorAccount</code>.</p> <p>The Security Hub console continues to use <code>DisassociateFromMasterAccount</code>. It will eventually change to use <code>DisassociateFromAdministratorAccount</code>. Any IAM policies that specifically control access to this function must continue to use <code>DisassociateFromMasterAccount</code>. You should also add <code>DisassociateFromAdministratorAccount</code> to your policies to ensure that the correct permissions are in place after the console begins to use <code>DisassociateFromAdministratorAccount</code>.</p> <p>Disassociates the current Security Hub member account from the associated administrator account.</p> <p>This operation is only used by accounts that are not part of an organization. For organization accounts, only the administrator account can disassociate a member account.</p>
    #[allow(unused_mut)]
    async fn disassociate_from_master_account(
        &self,
    ) -> Result<
        DisassociateFromMasterAccountResponse,
        RusotoError<DisassociateFromMasterAccountError>,
    > {
        let request_uri = "/master/disassociate";

        let mut request = SignedRequest::new("POST", "securityhub", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DisassociateFromMasterAccountResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DisassociateFromMasterAccountError::from_response(response))
        }
    }

    /// <p>Disassociates the specified member accounts from the associated administrator account.</p> <p>Can be used to disassociate both accounts that are managed using Organizations and accounts that were invited manually.</p>
    #[allow(unused_mut)]
    async fn disassociate_members(
        &self,
        input: DisassociateMembersRequest,
    ) -> Result<DisassociateMembersResponse, RusotoError<DisassociateMembersError>> {
        let request_uri = "/members/disassociate";

        let mut request = SignedRequest::new("POST", "securityhub", &self.region, &request_uri);
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
                .deserialize::<DisassociateMembersResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DisassociateMembersError::from_response(response))
        }
    }

    /// <p>Enables the integration of a partner product with Security Hub. Integrated products send findings to Security Hub.</p> <p>When you enable a product integration, a permissions policy that grants permission for the product to send findings to Security Hub is applied.</p>
    #[allow(unused_mut)]
    async fn enable_import_findings_for_product(
        &self,
        input: EnableImportFindingsForProductRequest,
    ) -> Result<
        EnableImportFindingsForProductResponse,
        RusotoError<EnableImportFindingsForProductError>,
    > {
        let request_uri = "/productSubscriptions";

        let mut request = SignedRequest::new("POST", "securityhub", &self.region, &request_uri);
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
                .deserialize::<EnableImportFindingsForProductResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(EnableImportFindingsForProductError::from_response(response))
        }
    }

    /// <p>Designates the Security Hub administrator account for an organization. Can only be called by the organization management account.</p>
    #[allow(unused_mut)]
    async fn enable_organization_admin_account(
        &self,
        input: EnableOrganizationAdminAccountRequest,
    ) -> Result<
        EnableOrganizationAdminAccountResponse,
        RusotoError<EnableOrganizationAdminAccountError>,
    > {
        let request_uri = "/organization/admin/enable";

        let mut request = SignedRequest::new("POST", "securityhub", &self.region, &request_uri);
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
                .deserialize::<EnableOrganizationAdminAccountResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(EnableOrganizationAdminAccountError::from_response(response))
        }
    }

    /// <p>Enables Security Hub for your account in the current Region or the Region you specify in the request.</p> <p>When you enable Security Hub, you grant to Security Hub the permissions necessary to gather findings from other services that are integrated with Security Hub.</p> <p>When you use the <code>EnableSecurityHub</code> operation to enable Security Hub, you also automatically enable the following standards.</p> <ul> <li> <p>CIS AWS Foundations</p> </li> <li> <p>AWS Foundational Security Best Practices</p> </li> </ul> <p>You do not enable the Payment Card Industry Data Security Standard (PCI DSS) standard. </p> <p>To not enable the automatically enabled standards, set <code>EnableDefaultStandards</code> to <code>false</code>.</p> <p>After you enable Security Hub, to enable a standard, use the <code> <a>BatchEnableStandards</a> </code> operation. To disable a standard, use the <code> <a>BatchDisableStandards</a> </code> operation.</p> <p>To learn more, see <a href="https://docs.aws.amazon.com/securityhub/latest/userguide/securityhub-settingup.html">Setting Up AWS Security Hub</a> in the <i>AWS Security Hub User Guide</i>.</p>
    #[allow(unused_mut)]
    async fn enable_security_hub(
        &self,
        input: EnableSecurityHubRequest,
    ) -> Result<EnableSecurityHubResponse, RusotoError<EnableSecurityHubError>> {
        let request_uri = "/accounts";

        let mut request = SignedRequest::new("POST", "securityhub", &self.region, &request_uri);
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
                .deserialize::<EnableSecurityHubResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(EnableSecurityHubError::from_response(response))
        }
    }

    /// <p>Provides the details for the Security Hub administrator account for the current member account.</p> <p>Can be used by both member accounts that are managed using Organizations and accounts that were invited manually.</p>
    #[allow(unused_mut)]
    async fn get_administrator_account(
        &self,
    ) -> Result<GetAdministratorAccountResponse, RusotoError<GetAdministratorAccountError>> {
        let request_uri = "/administrator";

        let mut request = SignedRequest::new("GET", "securityhub", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetAdministratorAccountResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetAdministratorAccountError::from_response(response))
        }
    }

    /// <p>Returns a list of the standards that are currently enabled.</p>
    #[allow(unused_mut)]
    async fn get_enabled_standards(
        &self,
        input: GetEnabledStandardsRequest,
    ) -> Result<GetEnabledStandardsResponse, RusotoError<GetEnabledStandardsError>> {
        let request_uri = "/standards/get";

        let mut request = SignedRequest::new("POST", "securityhub", &self.region, &request_uri);
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
                .deserialize::<GetEnabledStandardsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetEnabledStandardsError::from_response(response))
        }
    }

    /// <p>Returns a list of findings that match the specified criteria.</p>
    #[allow(unused_mut)]
    async fn get_findings(
        &self,
        input: GetFindingsRequest,
    ) -> Result<GetFindingsResponse, RusotoError<GetFindingsError>> {
        let request_uri = "/findings";

        let mut request = SignedRequest::new("POST", "securityhub", &self.region, &request_uri);
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
                .deserialize::<GetFindingsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetFindingsError::from_response(response))
        }
    }

    /// <p>Lists the results of the Security Hub insight specified by the insight ARN.</p>
    #[allow(unused_mut)]
    async fn get_insight_results(
        &self,
        input: GetInsightResultsRequest,
    ) -> Result<GetInsightResultsResponse, RusotoError<GetInsightResultsError>> {
        let request_uri = format!(
            "/insights/results/{insight_arn}",
            insight_arn = input.insight_arn
        );

        let mut request = SignedRequest::new("GET", "securityhub", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetInsightResultsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetInsightResultsError::from_response(response))
        }
    }

    /// <p>Lists and describes insights for the specified insight ARNs.</p>
    #[allow(unused_mut)]
    async fn get_insights(
        &self,
        input: GetInsightsRequest,
    ) -> Result<GetInsightsResponse, RusotoError<GetInsightsError>> {
        let request_uri = "/insights/get";

        let mut request = SignedRequest::new("POST", "securityhub", &self.region, &request_uri);
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
                .deserialize::<GetInsightsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetInsightsError::from_response(response))
        }
    }

    /// <p>Returns the count of all Security Hub membership invitations that were sent to the current member account, not including the currently accepted invitation. </p>
    #[allow(unused_mut)]
    async fn get_invitations_count(
        &self,
    ) -> Result<GetInvitationsCountResponse, RusotoError<GetInvitationsCountError>> {
        let request_uri = "/invitations/count";

        let mut request = SignedRequest::new("GET", "securityhub", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetInvitationsCountResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetInvitationsCountError::from_response(response))
        }
    }

    /// <p>This method is deprecated. Instead, use <code>GetAdministratorAccount</code>.</p> <p>The Security Hub console continues to use <code>GetMasterAccount</code>. It will eventually change to use <code>GetAdministratorAccount</code>. Any IAM policies that specifically control access to this function must continue to use <code>GetMasterAccount</code>. You should also add <code>GetAdministratorAccount</code> to your policies to ensure that the correct permissions are in place after the console begins to use <code>GetAdministratorAccount</code>.</p> <p>Provides the details for the Security Hub administrator account for the current member account.</p> <p>Can be used by both member accounts that are managed using Organizations and accounts that were invited manually.</p>
    #[allow(unused_mut)]
    async fn get_master_account(
        &self,
    ) -> Result<GetMasterAccountResponse, RusotoError<GetMasterAccountError>> {
        let request_uri = "/master";

        let mut request = SignedRequest::new("GET", "securityhub", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetMasterAccountResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetMasterAccountError::from_response(response))
        }
    }

    /// <p>Returns the details for the Security Hub member accounts for the specified account IDs.</p> <p>An administrator account can be either the delegated Security Hub administrator account for an organization or an administrator account that enabled Security Hub manually.</p> <p>The results include both member accounts that are managed using Organizations and accounts that were invited manually.</p>
    #[allow(unused_mut)]
    async fn get_members(
        &self,
        input: GetMembersRequest,
    ) -> Result<GetMembersResponse, RusotoError<GetMembersError>> {
        let request_uri = "/members/get";

        let mut request = SignedRequest::new("POST", "securityhub", &self.region, &request_uri);
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
                .deserialize::<GetMembersResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetMembersError::from_response(response))
        }
    }

    /// <p>Invites other AWS accounts to become member accounts for the Security Hub administrator account that the invitation is sent from.</p> <p>This operation is only used to invite accounts that do not belong to an organization. Organization accounts do not receive invitations.</p> <p>Before you can use this action to invite a member, you must first use the <code> <a>CreateMembers</a> </code> action to create the member account in Security Hub.</p> <p>When the account owner enables Security Hub and accepts the invitation to become a member account, the administrator account can view the findings generated from the member account.</p>
    #[allow(unused_mut)]
    async fn invite_members(
        &self,
        input: InviteMembersRequest,
    ) -> Result<InviteMembersResponse, RusotoError<InviteMembersError>> {
        let request_uri = "/members/invite";

        let mut request = SignedRequest::new("POST", "securityhub", &self.region, &request_uri);
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
                .deserialize::<InviteMembersResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(InviteMembersError::from_response(response))
        }
    }

    /// <p>Lists all findings-generating solutions (products) that you are subscribed to receive findings from in Security Hub.</p>
    #[allow(unused_mut)]
    async fn list_enabled_products_for_import(
        &self,
        input: ListEnabledProductsForImportRequest,
    ) -> Result<ListEnabledProductsForImportResponse, RusotoError<ListEnabledProductsForImportError>>
    {
        let request_uri = "/productSubscriptions";

        let mut request = SignedRequest::new("GET", "securityhub", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("MaxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
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
                .deserialize::<ListEnabledProductsForImportResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListEnabledProductsForImportError::from_response(response))
        }
    }

    /// <p>Lists all Security Hub membership invitations that were sent to the current AWS account.</p> <p>This operation is only used by accounts that are managed by invitation. Accounts that are managed using the integration with AWS Organizations do not receive invitations.</p>
    #[allow(unused_mut)]
    async fn list_invitations(
        &self,
        input: ListInvitationsRequest,
    ) -> Result<ListInvitationsResponse, RusotoError<ListInvitationsError>> {
        let request_uri = "/invitations";

        let mut request = SignedRequest::new("GET", "securityhub", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("MaxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
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
                .deserialize::<ListInvitationsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListInvitationsError::from_response(response))
        }
    }

    /// <p>Lists details about all member accounts for the current Security Hub administrator account.</p> <p>The results include both member accounts that belong to an organization and member accounts that were invited manually.</p>
    #[allow(unused_mut)]
    async fn list_members(
        &self,
        input: ListMembersRequest,
    ) -> Result<ListMembersResponse, RusotoError<ListMembersError>> {
        let request_uri = "/members";

        let mut request = SignedRequest::new("GET", "securityhub", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("MaxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        if let Some(ref x) = input.only_associated {
            params.put("OnlyAssociated", x);
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
                .deserialize::<ListMembersResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListMembersError::from_response(response))
        }
    }

    /// <p>Lists the Security Hub administrator accounts. Can only be called by the organization management account.</p>
    #[allow(unused_mut)]
    async fn list_organization_admin_accounts(
        &self,
        input: ListOrganizationAdminAccountsRequest,
    ) -> Result<
        ListOrganizationAdminAccountsResponse,
        RusotoError<ListOrganizationAdminAccountsError>,
    > {
        let request_uri = "/organization/admin";

        let mut request = SignedRequest::new("GET", "securityhub", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("MaxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
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
                .deserialize::<ListOrganizationAdminAccountsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListOrganizationAdminAccountsError::from_response(response))
        }
    }

    /// <p>Returns a list of tags associated with a resource.</p>
    #[allow(unused_mut)]
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("GET", "securityhub", &self.region, &request_uri);
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

    /// <p>Adds one or more tags to a resource.</p>
    #[allow(unused_mut)]
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("POST", "securityhub", &self.region, &request_uri);
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

    /// <p>Removes one or more tags from a resource.</p>
    #[allow(unused_mut)]
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("DELETE", "securityhub", &self.region, &request_uri);
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

    /// <p>Updates the name and description of a custom action target in Security Hub.</p>
    #[allow(unused_mut)]
    async fn update_action_target(
        &self,
        input: UpdateActionTargetRequest,
    ) -> Result<UpdateActionTargetResponse, RusotoError<UpdateActionTargetError>> {
        let request_uri = format!(
            "/actionTargets/{action_target_arn}",
            action_target_arn = input.action_target_arn
        );

        let mut request = SignedRequest::new("PATCH", "securityhub", &self.region, &request_uri);
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
                .deserialize::<UpdateActionTargetResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateActionTargetError::from_response(response))
        }
    }

    /// <p> <code>UpdateFindings</code> is deprecated. Instead of <code>UpdateFindings</code>, use <code>BatchUpdateFindings</code>.</p> <p>Updates the <code>Note</code> and <code>RecordState</code> of the Security Hub-aggregated findings that the filter attributes specify. Any member account that can view the finding also sees the update to the finding.</p>
    #[allow(unused_mut)]
    async fn update_findings(
        &self,
        input: UpdateFindingsRequest,
    ) -> Result<UpdateFindingsResponse, RusotoError<UpdateFindingsError>> {
        let request_uri = "/findings";

        let mut request = SignedRequest::new("PATCH", "securityhub", &self.region, &request_uri);
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
                .deserialize::<UpdateFindingsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateFindingsError::from_response(response))
        }
    }

    /// <p>Updates the Security Hub insight identified by the specified insight ARN.</p>
    #[allow(unused_mut)]
    async fn update_insight(
        &self,
        input: UpdateInsightRequest,
    ) -> Result<UpdateInsightResponse, RusotoError<UpdateInsightError>> {
        let request_uri = format!("/insights/{insight_arn}", insight_arn = input.insight_arn);

        let mut request = SignedRequest::new("PATCH", "securityhub", &self.region, &request_uri);
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
                .deserialize::<UpdateInsightResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateInsightError::from_response(response))
        }
    }

    /// <p>Used to update the configuration related to Organizations. Can only be called from a Security Hub administrator account.</p>
    #[allow(unused_mut)]
    async fn update_organization_configuration(
        &self,
        input: UpdateOrganizationConfigurationRequest,
    ) -> Result<
        UpdateOrganizationConfigurationResponse,
        RusotoError<UpdateOrganizationConfigurationError>,
    > {
        let request_uri = "/organization/configuration";

        let mut request = SignedRequest::new("POST", "securityhub", &self.region, &request_uri);
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
                .deserialize::<UpdateOrganizationConfigurationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateOrganizationConfigurationError::from_response(
                response,
            ))
        }
    }

    /// <p>Updates configuration options for Security Hub.</p>
    #[allow(unused_mut)]
    async fn update_security_hub_configuration(
        &self,
        input: UpdateSecurityHubConfigurationRequest,
    ) -> Result<
        UpdateSecurityHubConfigurationResponse,
        RusotoError<UpdateSecurityHubConfigurationError>,
    > {
        let request_uri = "/accounts";

        let mut request = SignedRequest::new("PATCH", "securityhub", &self.region, &request_uri);
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
                .deserialize::<UpdateSecurityHubConfigurationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateSecurityHubConfigurationError::from_response(response))
        }
    }

    /// <p>Used to control whether an individual security standard control is enabled or disabled.</p>
    #[allow(unused_mut)]
    async fn update_standards_control(
        &self,
        input: UpdateStandardsControlRequest,
    ) -> Result<UpdateStandardsControlResponse, RusotoError<UpdateStandardsControlError>> {
        let request_uri = format!(
            "/standards/control/{standards_control_arn}",
            standards_control_arn = input.standards_control_arn
        );

        let mut request = SignedRequest::new("PATCH", "securityhub", &self.region, &request_uri);
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
                .deserialize::<UpdateStandardsControlResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateStandardsControlError::from_response(response))
        }
    }
}
