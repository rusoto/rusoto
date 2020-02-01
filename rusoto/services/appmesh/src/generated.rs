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
/// <p>An object that represents the access logging information for a virtual node.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccessLog {
    /// <p>The file object to send virtual node access logs to.</p>
    #[serde(rename = "file")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<FileAccessLog>,
}

/// <p>An object that represents the AWS Cloud Map attribute information for your virtual
/// node.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsCloudMapInstanceAttribute {
    /// <p>The name of an AWS Cloud Map service instance attribute key. Any AWS Cloud Map service
    /// instance that contains the specified key and value is returned.</p>
    #[serde(rename = "key")]
    pub key: String,
    /// <p>The value of an AWS Cloud Map service instance attribute key. Any AWS Cloud Map service
    /// instance that contains the specified key and value is returned.</p>
    #[serde(rename = "value")]
    pub value: String,
}

/// <p>An object that represents the AWS Cloud Map service discovery information for your virtual
/// node.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsCloudMapServiceDiscovery {
    /// <p>A string map that contains attributes with values that you can use to filter instances
    /// by any custom attribute that you specified when you registered the instance. Only instances
    /// that match all of the specified key/value pairs will be returned.</p>
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<AwsCloudMapInstanceAttribute>>,
    /// <p>The name of the AWS Cloud Map namespace to use.</p>
    #[serde(rename = "namespaceName")]
    pub namespace_name: String,
    /// <p>The name of the AWS Cloud Map service to use.</p>
    #[serde(rename = "serviceName")]
    pub service_name: String,
}

/// <p>An object that represents the backends that a virtual node is expected to send outbound
/// traffic to. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Backend {
    /// <p>Specifies a virtual service to use as a backend for a virtual node. </p>
    #[serde(rename = "virtualService")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_service: Option<VirtualServiceBackend>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateMeshInput {
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the
    /// request. Up to 36 letters, numbers, hyphens, and underscores are allowed.</p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The name to use for the service mesh.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The service mesh specification to apply.</p>
    #[serde(rename = "spec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<MeshSpec>,
    /// <p>Optional metadata that you can apply to the service mesh to assist with categorization
    /// and organization. Each tag consists of a key and an optional value, both of which you
    /// define. Tag keys can have a maximum character length of 128 characters, and tag values can have
    /// a maximum length of 256 characters.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagRef>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateMeshOutput {
    /// <p>The full description of your service mesh following the create call.</p>
    #[serde(rename = "mesh")]
    pub mesh: MeshData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateRouteInput {
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the
    /// request. Up to 36 letters, numbers, hyphens, and underscores are allowed.</p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The name of the service mesh to create the route in.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The name to use for the route.</p>
    #[serde(rename = "routeName")]
    pub route_name: String,
    /// <p>The route specification to apply.</p>
    #[serde(rename = "spec")]
    pub spec: RouteSpec,
    /// <p>Optional metadata that you can apply to the route to assist with categorization and
    /// organization. Each tag consists of a key and an optional value, both of which you define.
    /// Tag keys can have a maximum character length of 128 characters, and tag values can have
    /// a maximum length of 256 characters.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagRef>>,
    /// <p>The name of the virtual router in which to create the route.</p>
    #[serde(rename = "virtualRouterName")]
    pub virtual_router_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateRouteOutput {
    /// <p>The full description of your mesh following the create call.</p>
    #[serde(rename = "route")]
    pub route: RouteData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateVirtualNodeInput {
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the
    /// request. Up to 36 letters, numbers, hyphens, and underscores are allowed.</p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The name of the service mesh to create the virtual node in.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The virtual node specification to apply.</p>
    #[serde(rename = "spec")]
    pub spec: VirtualNodeSpec,
    /// <p>Optional metadata that you can apply to the virtual node to assist with categorization
    /// and organization. Each tag consists of a key and an optional value, both of which you
    /// define. Tag keys can have a maximum character length of 128 characters, and tag values can have
    /// a maximum length of 256 characters.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagRef>>,
    /// <p>The name to use for the virtual node.</p>
    #[serde(rename = "virtualNodeName")]
    pub virtual_node_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateVirtualNodeOutput {
    /// <p>The full description of your virtual node following the create call.</p>
    #[serde(rename = "virtualNode")]
    pub virtual_node: VirtualNodeData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateVirtualRouterInput {
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the
    /// request. Up to 36 letters, numbers, hyphens, and underscores are allowed.</p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The name of the service mesh to create the virtual router in.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The virtual router specification to apply.</p>
    #[serde(rename = "spec")]
    pub spec: VirtualRouterSpec,
    /// <p>Optional metadata that you can apply to the virtual router to assist with categorization
    /// and organization. Each tag consists of a key and an optional value, both of which you
    /// define. Tag keys can have a maximum character length of 128 characters, and tag values can have
    /// a maximum length of 256 characters.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagRef>>,
    /// <p>The name to use for the virtual router.</p>
    #[serde(rename = "virtualRouterName")]
    pub virtual_router_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateVirtualRouterOutput {
    /// <p>The full description of your virtual router following the create call.</p>
    #[serde(rename = "virtualRouter")]
    pub virtual_router: VirtualRouterData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateVirtualServiceInput {
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the
    /// request. Up to 36 letters, numbers, hyphens, and underscores are allowed.</p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The name of the service mesh to create the virtual service in.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The virtual service specification to apply.</p>
    #[serde(rename = "spec")]
    pub spec: VirtualServiceSpec,
    /// <p>Optional metadata that you can apply to the virtual service to assist with
    /// categorization and organization. Each tag consists of a key and an optional value, both of
    /// which you define. Tag keys can have a maximum character length of 128 characters, and tag values can have
    /// a maximum length of 256 characters.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagRef>>,
    /// <p>The name to use for the virtual service.</p>
    #[serde(rename = "virtualServiceName")]
    pub virtual_service_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateVirtualServiceOutput {
    /// <p>The full description of your virtual service following the create call.</p>
    #[serde(rename = "virtualService")]
    pub virtual_service: VirtualServiceData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteMeshInput {
    /// <p>The name of the service mesh to delete.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteMeshOutput {
    /// <p>The service mesh that was deleted.</p>
    #[serde(rename = "mesh")]
    pub mesh: MeshData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteRouteInput {
    /// <p>The name of the service mesh to delete the route in.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The name of the route to delete.</p>
    #[serde(rename = "routeName")]
    pub route_name: String,
    /// <p>The name of the virtual router to delete the route in.</p>
    #[serde(rename = "virtualRouterName")]
    pub virtual_router_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteRouteOutput {
    /// <p>The route that was deleted.</p>
    #[serde(rename = "route")]
    pub route: RouteData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteVirtualNodeInput {
    /// <p>The name of the service mesh to delete the virtual node in.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The name of the virtual node to delete.</p>
    #[serde(rename = "virtualNodeName")]
    pub virtual_node_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteVirtualNodeOutput {
    /// <p>The virtual node that was deleted.</p>
    #[serde(rename = "virtualNode")]
    pub virtual_node: VirtualNodeData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteVirtualRouterInput {
    /// <p>The name of the service mesh to delete the virtual router in.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The name of the virtual router to delete.</p>
    #[serde(rename = "virtualRouterName")]
    pub virtual_router_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteVirtualRouterOutput {
    /// <p>The virtual router that was deleted.</p>
    #[serde(rename = "virtualRouter")]
    pub virtual_router: VirtualRouterData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteVirtualServiceInput {
    /// <p>The name of the service mesh to delete the virtual service in.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The name of the virtual service to delete.</p>
    #[serde(rename = "virtualServiceName")]
    pub virtual_service_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteVirtualServiceOutput {
    /// <p>The virtual service that was deleted.</p>
    #[serde(rename = "virtualService")]
    pub virtual_service: VirtualServiceData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeMeshInput {
    /// <p>The name of the service mesh to describe.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeMeshOutput {
    /// <p>The full description of your service mesh.</p>
    #[serde(rename = "mesh")]
    pub mesh: MeshData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeRouteInput {
    /// <p>The name of the service mesh that the route resides in.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The name of the route to describe.</p>
    #[serde(rename = "routeName")]
    pub route_name: String,
    /// <p>The name of the virtual router that the route is associated with.</p>
    #[serde(rename = "virtualRouterName")]
    pub virtual_router_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeRouteOutput {
    /// <p>The full description of your route.</p>
    #[serde(rename = "route")]
    pub route: RouteData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeVirtualNodeInput {
    /// <p>The name of the service mesh that the virtual node resides in.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The name of the virtual node to describe.</p>
    #[serde(rename = "virtualNodeName")]
    pub virtual_node_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeVirtualNodeOutput {
    /// <p>The full description of your virtual node.</p>
    #[serde(rename = "virtualNode")]
    pub virtual_node: VirtualNodeData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeVirtualRouterInput {
    /// <p>The name of the service mesh that the virtual router resides in.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The name of the virtual router to describe.</p>
    #[serde(rename = "virtualRouterName")]
    pub virtual_router_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeVirtualRouterOutput {
    /// <p>The full description of your virtual router.</p>
    #[serde(rename = "virtualRouter")]
    pub virtual_router: VirtualRouterData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeVirtualServiceInput {
    /// <p>The name of the service mesh that the virtual service resides in.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The name of the virtual service to describe.</p>
    #[serde(rename = "virtualServiceName")]
    pub virtual_service_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeVirtualServiceOutput {
    /// <p>The full description of your virtual service.</p>
    #[serde(rename = "virtualService")]
    pub virtual_service: VirtualServiceData,
}

/// <p>An object that represents the DNS service discovery information for your virtual
/// node.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DnsServiceDiscovery {
    /// <p>Specifies the DNS service discovery hostname for the virtual node. </p>
    #[serde(rename = "hostname")]
    pub hostname: String,
}

/// <p>An object that represents a duration of time.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Duration {
    /// <p>A unit of time.</p>
    #[serde(rename = "unit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    /// <p>A number of time units.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
}

/// <p>An object that represents the egress filter rules for a service mesh.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EgressFilter {
    /// <p>The egress filter type. By default, the type is <code>DROP_ALL</code>, which allows
    /// egress only from virtual nodes to other defined resources in the service mesh (and any
    /// traffic to <code>*.amazonaws.com</code> for AWS API calls). You can set the egress filter
    /// type to <code>ALLOW_ALL</code> to allow egress to any endpoint inside or outside of the
    /// service mesh.</p>
    #[serde(rename = "type")]
    pub type_: String,
}

/// <p>An object that represents an access log file.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileAccessLog {
    /// <p>The file path to write access logs to. You can use <code>/dev/stdout</code> to send
    /// access logs to standard out and configure your Envoy container to use a log driver, such as
    /// <code>awslogs</code>, to export the access logs to a log storage service such as Amazon
    /// CloudWatch Logs. You can also specify a path in the Envoy container's file system to write
    /// the files to disk.</p>
    ///
    /// <pre><code>     &lt;note&gt;
    /// &lt;p&gt;The Envoy process must have write permissions to the path that you specify here.
    /// Otherwise, Envoy fails to bootstrap properly.&lt;/p&gt;
    /// &lt;/note&gt;
    /// </code></pre>
    #[serde(rename = "path")]
    pub path: String,
}

/// <p>An object that represents a retry policy. Specify at least one value for at least one of the types of <code>RetryEvents</code>, a value for <code>maxRetries</code>, and a value for <code>perRetryTimeout</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GrpcRetryPolicy {
    /// <p>Specify at least one of the valid values.</p>
    #[serde(rename = "grpcRetryEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grpc_retry_events: Option<Vec<String>>,
    /// <p>Specify at least one of the following values.</p>
    ///
    /// <pre><code>     &lt;ul&gt;
    /// &lt;li&gt;
    /// &lt;p&gt;
    /// &lt;b&gt;server-error&lt;/b&gt; – HTTP status codes 500, 501,
    /// 502, 503, 504, 505, 506, 507, 508, 510, and 511&lt;/p&gt;
    /// &lt;/li&gt;
    /// &lt;li&gt;
    /// &lt;p&gt;
    /// &lt;b&gt;gateway-error&lt;/b&gt; – HTTP status codes 502,
    /// 503, and 504&lt;/p&gt;
    /// &lt;/li&gt;
    /// &lt;li&gt;
    /// &lt;p&gt;
    /// &lt;b&gt;client-error&lt;/b&gt; – HTTP status code 409&lt;/p&gt;
    /// &lt;/li&gt;
    /// &lt;li&gt;
    /// &lt;p&gt;
    /// &lt;b&gt;stream-error&lt;/b&gt; – Retry on refused
    /// stream&lt;/p&gt;
    /// &lt;/li&gt;
    /// &lt;/ul&gt;
    /// </code></pre>
    #[serde(rename = "httpRetryEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_retry_events: Option<Vec<String>>,
    /// <p>The maximum number of retry attempts.</p>
    #[serde(rename = "maxRetries")]
    pub max_retries: i64,
    /// <p>An object that represents a duration of time.</p>
    #[serde(rename = "perRetryTimeout")]
    pub per_retry_timeout: Duration,
    /// <p>Specify a valid value.</p>
    #[serde(rename = "tcpRetryEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_retry_events: Option<Vec<String>>,
}

/// <p>An object that represents a GRPC route type.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GrpcRoute {
    /// <p>An object that represents the action to take if a match is determined.</p>
    #[serde(rename = "action")]
    pub action: GrpcRouteAction,
    /// <p>An object that represents the criteria for determining a request match.</p>
    #[serde(rename = "match")]
    pub route_match: Option<GrpcRouteMatch>,
    /// <p>An object that represents a retry policy.</p>
    #[serde(rename = "retryPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_policy: Option<GrpcRetryPolicy>,
}

/// <p>An object that represents the action to take if a match is determined.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GrpcRouteAction {
    /// <p>An object that represents the targets that traffic is routed to when a request matches the route.</p>
    #[serde(rename = "weightedTargets")]
    pub weighted_targets: Vec<WeightedTarget>,
}

/// <p>An object that represents the criteria for determining a request match.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GrpcRouteMatch {
    /// <p>An object that represents the data to match from the request.</p>
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Vec<GrpcRouteMetadata>>,
    /// <p>The method name to match from the request. If you specify a name, you must also specify a <code>serviceName</code>.</p>
    #[serde(rename = "methodName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method_name: Option<String>,
    /// <p>The fully qualified domain name for the service to match from the request.</p>
    #[serde(rename = "serviceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
}

/// <p>An object that represents the match metadata for the route.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GrpcRouteMetadata {
    /// <p>Specify <code>True</code> to match anything except the match criteria. The default value is <code>False</code>.</p>
    #[serde(rename = "invert")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invert: Option<bool>,
    /// <p>An object that represents the data to match from the request.</p>
    #[serde(rename = "match")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_match: Option<GrpcRouteMetadataMatchMethod>,
    /// <p>The name of the route.</p>
    #[serde(rename = "name")]
    pub name: String,
}

/// <p>An object that represents the match method. Specify one of the match values.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GrpcRouteMetadataMatchMethod {
    /// <p>The value sent by the client must match the specified value exactly.</p>
    #[serde(rename = "exact")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exact: Option<String>,
    /// <p>The value sent by the client must begin with the specified characters.</p>
    #[serde(rename = "prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// <p>An object that represents the range of values to match on.</p>
    #[serde(rename = "range")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<MatchRange>,
    /// <p>The value sent by the client must include the specified characters.</p>
    #[serde(rename = "regex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,
    /// <p>The value sent by the client must end with the specified characters.</p>
    #[serde(rename = "suffix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
}

/// <p>An object that represents the method and value to match with the header value sent in a
/// request. Specify one match method.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HeaderMatchMethod {
    /// <p>The value sent by the client must match the specified value exactly.</p>
    #[serde(rename = "exact")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exact: Option<String>,
    /// <p>The value sent by the client must begin with the specified characters.</p>
    #[serde(rename = "prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// <p>An object that represents the range of values to match on.</p>
    #[serde(rename = "range")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<MatchRange>,
    /// <p>The value sent by the client must include the specified characters.</p>
    #[serde(rename = "regex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,
    /// <p>The value sent by the client must end with the specified characters.</p>
    #[serde(rename = "suffix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
}

/// <p>An object that represents the health check policy for a virtual node's listener.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HealthCheckPolicy {
    /// <p>The number of consecutive successful health checks that must occur before declaring
    /// listener healthy.</p>
    #[serde(rename = "healthyThreshold")]
    pub healthy_threshold: i64,
    /// <p>The time period in milliseconds between each health check execution.</p>
    #[serde(rename = "intervalMillis")]
    pub interval_millis: i64,
    /// <p>The destination path for the health check request. This is required only if the
    /// specified protocol is HTTP. If the protocol is TCP, this parameter is ignored.</p>
    #[serde(rename = "path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// <p>The destination port for the health check request. This port must match the port defined
    /// in the <a>PortMapping</a> for the listener.</p>
    #[serde(rename = "port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    /// <p>The protocol for the health check request.</p>
    #[serde(rename = "protocol")]
    pub protocol: String,
    /// <p>The amount of time to wait when receiving a response from the health check, in
    /// milliseconds.</p>
    #[serde(rename = "timeoutMillis")]
    pub timeout_millis: i64,
    /// <p>The number of consecutive failed health checks that must occur before declaring a
    /// virtual node unhealthy. </p>
    #[serde(rename = "unhealthyThreshold")]
    pub unhealthy_threshold: i64,
}

/// <p>An object that represents a retry policy. Specify at least one value for at least one of the types of <code>RetryEvents</code>, a value for <code>maxRetries</code>, and a value for <code>perRetryTimeout</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HttpRetryPolicy {
    /// <p>Specify at least one of the following values.</p>
    ///
    /// <pre><code>     &lt;ul&gt;
    /// &lt;li&gt;
    /// &lt;p&gt;
    /// &lt;b&gt;server-error&lt;/b&gt; – HTTP status codes 500, 501,
    /// 502, 503, 504, 505, 506, 507, 508, 510, and 511&lt;/p&gt;
    /// &lt;/li&gt;
    /// &lt;li&gt;
    /// &lt;p&gt;
    /// &lt;b&gt;gateway-error&lt;/b&gt; – HTTP status codes 502,
    /// 503, and 504&lt;/p&gt;
    /// &lt;/li&gt;
    /// &lt;li&gt;
    /// &lt;p&gt;
    /// &lt;b&gt;client-error&lt;/b&gt; – HTTP status code 409&lt;/p&gt;
    /// &lt;/li&gt;
    /// &lt;li&gt;
    /// &lt;p&gt;
    /// &lt;b&gt;stream-error&lt;/b&gt; – Retry on refused
    /// stream&lt;/p&gt;
    /// &lt;/li&gt;
    /// &lt;/ul&gt;
    /// </code></pre>
    #[serde(rename = "httpRetryEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_retry_events: Option<Vec<String>>,
    /// <p>The maximum number of retry attempts.</p>
    #[serde(rename = "maxRetries")]
    pub max_retries: i64,
    /// <p>An object that represents a duration of time.</p>
    #[serde(rename = "perRetryTimeout")]
    pub per_retry_timeout: Duration,
    /// <p>Specify a valid value.</p>
    #[serde(rename = "tcpRetryEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_retry_events: Option<Vec<String>>,
}

/// <p>An object that represents an HTTP or HTTP2 route type.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HttpRoute {
    /// <p>An object that represents the action to take if a match is determined.</p>
    #[serde(rename = "action")]
    pub action: HttpRouteAction,
    /// <p>An object that represents the criteria for determining a request match.</p>
    #[serde(rename = "match")]
    pub route_match: Option<HttpRouteMatch>,
    /// <p>An object that represents a retry policy.</p>
    #[serde(rename = "retryPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_policy: Option<HttpRetryPolicy>,
}

/// <p>An object that represents the action to take if a match is determined.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HttpRouteAction {
    /// <p>An object that represents the targets that traffic is routed to when a request matches the route.</p>
    #[serde(rename = "weightedTargets")]
    pub weighted_targets: Vec<WeightedTarget>,
}

/// <p>An object that represents the HTTP header in the request.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HttpRouteHeader {
    /// <p>Specify <code>True</code> to match anything except the match criteria. The default value is <code>False</code>.</p>
    #[serde(rename = "invert")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invert: Option<bool>,
    /// <p>The <code>HeaderMatchMethod</code> object.</p>
    #[serde(rename = "match")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_match: Option<HeaderMatchMethod>,
    /// <p>A name for the HTTP header in the client request that will be matched on.</p>
    #[serde(rename = "name")]
    pub name: String,
}

/// <p>An object that represents the requirements for a route to match HTTP requests for a virtual
/// router.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HttpRouteMatch {
    /// <p>An object that represents the client request headers to match on.</p>
    #[serde(rename = "headers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<HttpRouteHeader>>,
    /// <p>The client request method to match on. Specify only one.</p>
    #[serde(rename = "method")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    /// <p>Specifies the path to match requests with. This parameter must always start with
    /// <code>/</code>, which by itself matches all requests to the virtual service name. You
    /// can also match for path-based routing of requests. For example, if your virtual service
    /// name is <code>my-service.local</code> and you want the route to match requests to
    /// <code>my-service.local/metrics</code>, your prefix should be
    /// <code>/metrics</code>.</p>
    #[serde(rename = "prefix")]
    pub prefix: String,
    /// <p>The client request scheme to match on. Specify only one.</p>
    #[serde(rename = "scheme")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListMeshesInput {
    /// <p>The maximum number of results returned by <code>ListMeshes</code> in paginated output.
    /// When you use this parameter, <code>ListMeshes</code> returns only <code>limit</code>
    /// results in a single page along with a <code>nextToken</code> response element. You can see
    /// the remaining results of the initial request by sending another <code>ListMeshes</code>
    /// request with the returned <code>nextToken</code> value. This value can be between
    /// 1 and 100. If you don't use this parameter,
    /// <code>ListMeshes</code> returns up to 100 results and a
    /// <code>nextToken</code> value if applicable.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The <code>nextToken</code> value returned from a previous paginated
    /// <code>ListMeshes</code> request where <code>limit</code> was used and the results
    /// exceeded the value of that parameter. Pagination continues from the end of the previous
    /// results that returned the <code>nextToken</code> value.</p>
    ///
    /// <pre><code>     &lt;note&gt;
    /// &lt;p&gt;This token should be treated as an opaque identifier that is used only to
    /// retrieve the next items in a list and not for other programmatic purposes.&lt;/p&gt;
    /// &lt;/note&gt;
    /// </code></pre>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListMeshesOutput {
    /// <p>The list of existing service meshes.</p>
    #[serde(rename = "meshes")]
    pub meshes: Vec<MeshRef>,
    /// <p>The <code>nextToken</code> value to include in a future <code>ListMeshes</code> request.
    /// When the results of a <code>ListMeshes</code> request exceed <code>limit</code>, you can
    /// use this value to retrieve the next page of results. This value is <code>null</code> when
    /// there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListRoutesInput {
    /// <p>The maximum number of results returned by <code>ListRoutes</code> in paginated output.
    /// When you use this parameter, <code>ListRoutes</code> returns only <code>limit</code>
    /// results in a single page along with a <code>nextToken</code> response element. You can see
    /// the remaining results of the initial request by sending another <code>ListRoutes</code>
    /// request with the returned <code>nextToken</code> value. This value can be between
    /// 1 and 100. If you don't use this parameter,
    /// <code>ListRoutes</code> returns up to 100 results and a
    /// <code>nextToken</code> value if applicable.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The name of the service mesh to list routes in.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The <code>nextToken</code> value returned from a previous paginated
    /// <code>ListRoutes</code> request where <code>limit</code> was used and the results
    /// exceeded the value of that parameter. Pagination continues from the end of the previous
    /// results that returned the <code>nextToken</code> value.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the virtual router to list routes in.</p>
    #[serde(rename = "virtualRouterName")]
    pub virtual_router_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListRoutesOutput {
    /// <p>The <code>nextToken</code> value to include in a future <code>ListRoutes</code> request.
    /// When the results of a <code>ListRoutes</code> request exceed <code>limit</code>, you can
    /// use this value to retrieve the next page of results. This value is <code>null</code> when
    /// there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of existing routes for the specified service mesh and virtual router.</p>
    #[serde(rename = "routes")]
    pub routes: Vec<RouteRef>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceInput {
    /// <p>The maximum number of tag results returned by <code>ListTagsForResource</code> in
    /// paginated output. When this parameter is used, <code>ListTagsForResource</code> returns
    /// only <code>limit</code> results in a single page along with a <code>nextToken</code>
    /// response element. You can see the remaining results of the initial request by sending
    /// another <code>ListTagsForResource</code> request with the returned <code>nextToken</code>
    /// value. This value can be between 1 and 100. If you don't use
    /// this parameter, <code>ListTagsForResource</code> returns up to 100
    /// results and a <code>nextToken</code> value if applicable.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The <code>nextToken</code> value returned from a previous paginated
    /// <code>ListTagsForResource</code> request where <code>limit</code> was used and the
    /// results exceeded the value of that parameter. Pagination continues from the end of the
    /// previous results that returned the <code>nextToken</code> value.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The Amazon Resource Name (ARN) that identifies the resource to list the tags for.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceOutput {
    /// <p>The <code>nextToken</code> value to include in a future <code>ListTagsForResource</code>
    /// request. When the results of a <code>ListTagsForResource</code> request exceed
    /// <code>limit</code>, you can use this value to retrieve the next page of results. This
    /// value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The tags for the resource.</p>
    #[serde(rename = "tags")]
    pub tags: Vec<TagRef>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListVirtualNodesInput {
    /// <p>The maximum number of results returned by <code>ListVirtualNodes</code> in paginated
    /// output. When you use this parameter, <code>ListVirtualNodes</code> returns only
    /// <code>limit</code> results in a single page along with a <code>nextToken</code> response
    /// element. You can see the remaining results of the initial request by sending another
    /// <code>ListVirtualNodes</code> request with the returned <code>nextToken</code> value.
    /// This value can be between 1 and 100. If you don't use this
    /// parameter, <code>ListVirtualNodes</code> returns up to 100 results and a
    /// <code>nextToken</code> value if applicable.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The name of the service mesh to list virtual nodes in.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The <code>nextToken</code> value returned from a previous paginated
    /// <code>ListVirtualNodes</code> request where <code>limit</code> was used and the results
    /// exceeded the value of that parameter. Pagination continues from the end of the previous
    /// results that returned the <code>nextToken</code> value.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListVirtualNodesOutput {
    /// <p>The <code>nextToken</code> value to include in a future <code>ListVirtualNodes</code>
    /// request. When the results of a <code>ListVirtualNodes</code> request exceed
    /// <code>limit</code>, you can use this value to retrieve the next page of results. This
    /// value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of existing virtual nodes for the specified service mesh.</p>
    #[serde(rename = "virtualNodes")]
    pub virtual_nodes: Vec<VirtualNodeRef>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListVirtualRoutersInput {
    /// <p>The maximum number of results returned by <code>ListVirtualRouters</code> in paginated
    /// output. When you use this parameter, <code>ListVirtualRouters</code> returns only
    /// <code>limit</code> results in a single page along with a <code>nextToken</code> response
    /// element. You can see the remaining results of the initial request by sending another
    /// <code>ListVirtualRouters</code> request with the returned <code>nextToken</code> value.
    /// This value can be between 1 and 100. If you don't use this
    /// parameter, <code>ListVirtualRouters</code> returns up to 100 results and
    /// a <code>nextToken</code> value if applicable.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The name of the service mesh to list virtual routers in.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The <code>nextToken</code> value returned from a previous paginated
    /// <code>ListVirtualRouters</code> request where <code>limit</code> was used and the
    /// results exceeded the value of that parameter. Pagination continues from the end of the
    /// previous results that returned the <code>nextToken</code> value.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListVirtualRoutersOutput {
    /// <p>The <code>nextToken</code> value to include in a future <code>ListVirtualRouters</code>
    /// request. When the results of a <code>ListVirtualRouters</code> request exceed
    /// <code>limit</code>, you can use this value to retrieve the next page of results. This
    /// value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of existing virtual routers for the specified service mesh.</p>
    #[serde(rename = "virtualRouters")]
    pub virtual_routers: Vec<VirtualRouterRef>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListVirtualServicesInput {
    /// <p>The maximum number of results returned by <code>ListVirtualServices</code> in paginated
    /// output. When you use this parameter, <code>ListVirtualServices</code> returns only
    /// <code>limit</code> results in a single page along with a <code>nextToken</code> response
    /// element. You can see the remaining results of the initial request by sending another
    /// <code>ListVirtualServices</code> request with the returned <code>nextToken</code> value.
    /// This value can be between 1 and 100. If you don't use this
    /// parameter, <code>ListVirtualServices</code> returns up to 100 results and
    /// a <code>nextToken</code> value if applicable.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The name of the service mesh to list virtual services in.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The <code>nextToken</code> value returned from a previous paginated
    /// <code>ListVirtualServices</code> request where <code>limit</code> was used and the
    /// results exceeded the value of that parameter. Pagination continues from the end of the
    /// previous results that returned the <code>nextToken</code> value.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListVirtualServicesOutput {
    /// <p>The <code>nextToken</code> value to include in a future <code>ListVirtualServices</code>
    /// request. When the results of a <code>ListVirtualServices</code> request exceed
    /// <code>limit</code>, you can use this value to retrieve the next page of results. This
    /// value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of existing virtual services for the specified service mesh.</p>
    #[serde(rename = "virtualServices")]
    pub virtual_services: Vec<VirtualServiceRef>,
}

/// <p>An object that represents a listener for a virtual node.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Listener {
    /// <p>The health check information for the listener.</p>
    #[serde(rename = "healthCheck")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check: Option<HealthCheckPolicy>,
    /// <p>The port mapping information for the listener.</p>
    #[serde(rename = "portMapping")]
    pub port_mapping: PortMapping,
}

/// <p>An object that represents the logging information for a virtual node.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Logging {
    /// <p>The access log configuration for a virtual node.</p>
    #[serde(rename = "accessLog")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_log: Option<AccessLog>,
}

/// <p>An object that represents the range of values to match on. The first character of the range is included in the range, though the last character is not. For example, if the range specified were 1-100, only values 1-99 would be matched.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MatchRange {
    /// <p>The end of the range.</p>
    #[serde(rename = "end")]
    pub end: i64,
    /// <p>The start of the range.</p>
    #[serde(rename = "start")]
    pub start: i64,
}

/// <p>An object that represents a service mesh returned by a describe operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MeshData {
    /// <p>The name of the service mesh.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The associated metadata for the service mesh.</p>
    #[serde(rename = "metadata")]
    pub metadata: ResourceMetadata,
    /// <p>The associated specification for the service mesh.</p>
    #[serde(rename = "spec")]
    pub spec: MeshSpec,
    /// <p>The status of the service mesh.</p>
    #[serde(rename = "status")]
    pub status: MeshStatus,
}

/// <p>An object that represents a service mesh returned by a list operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MeshRef {
    /// <p>The full Amazon Resource Name (ARN) of the service mesh.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>The name of the service mesh.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
}

/// <p>An object that represents the specification of a service mesh.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeshSpec {
    /// <p>The egress filter rules for the service mesh.</p>
    #[serde(rename = "egressFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_filter: Option<EgressFilter>,
}

/// <p>An object that represents the status of a service mesh.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MeshStatus {
    /// <p>The current mesh status.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>An object that represents a port mapping.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PortMapping {
    /// <p>The port used for the port mapping.</p>
    #[serde(rename = "port")]
    pub port: i64,
    /// <p>The protocol used for the port mapping. Specify one protocol.</p>
    #[serde(rename = "protocol")]
    pub protocol: String,
}

/// <p>An object that represents metadata for a resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResourceMetadata {
    /// <p>The full Amazon Resource Name (ARN) for the resource.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>The Unix epoch timestamp in seconds for when the resource was created.</p>
    #[serde(rename = "createdAt")]
    pub created_at: f64,
    /// <p>The Unix epoch timestamp in seconds for when the resource was last updated.</p>
    #[serde(rename = "lastUpdatedAt")]
    pub last_updated_at: f64,
    /// <p>The unique identifier for the resource.</p>
    #[serde(rename = "uid")]
    pub uid: String,
    /// <p>The version of the resource. Resources are created at version 1, and this version is
    /// incremented each time that they're updated.</p>
    #[serde(rename = "version")]
    pub version: i64,
}

/// <p>An object that represents a route returned by a describe operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RouteData {
    /// <p>The name of the service mesh that the route resides in.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The associated metadata for the route.</p>
    #[serde(rename = "metadata")]
    pub metadata: ResourceMetadata,
    /// <p>The name of the route.</p>
    #[serde(rename = "routeName")]
    pub route_name: String,
    /// <p>The specifications of the route.</p>
    #[serde(rename = "spec")]
    pub spec: RouteSpec,
    /// <p>The status of the route.</p>
    #[serde(rename = "status")]
    pub status: RouteStatus,
    /// <p>The virtual router that the route is associated with.</p>
    #[serde(rename = "virtualRouterName")]
    pub virtual_router_name: String,
}

/// <p>An object that represents a route returned by a list operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RouteRef {
    /// <p>The full Amazon Resource Name (ARN) for the route.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>The name of the service mesh that the route resides in.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The name of the route.</p>
    #[serde(rename = "routeName")]
    pub route_name: String,
    /// <p>The virtual router that the route is associated with.</p>
    #[serde(rename = "virtualRouterName")]
    pub virtual_router_name: String,
}

/// <p>An object that represents a route specification. Specify one route type.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RouteSpec {
    /// <p>An object that represents the specification of a GRPC route.</p>
    #[serde(rename = "grpcRoute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grpc_route: Option<GrpcRoute>,
    /// <p>An object that represents the specification of an HTTP2 route.</p>
    #[serde(rename = "http2Route")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_2_route: Option<HttpRoute>,
    /// <p>An object that represents the specification of an HTTP route.</p>
    #[serde(rename = "httpRoute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_route: Option<HttpRoute>,
    /// <p>The priority for the route. Routes are matched based on the specified value, where 0 is
    /// the highest priority.</p>
    #[serde(rename = "priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    /// <p>An object that represents the specification of a TCP route.</p>
    #[serde(rename = "tcpRoute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_route: Option<TcpRoute>,
}

/// <p>An object that represents the current status of a route.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RouteStatus {
    /// <p>The current status for the route.</p>
    #[serde(rename = "status")]
    pub status: String,
}

/// <p>An object that represents the service discovery information for a virtual node.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceDiscovery {
    /// <p>Specifies any AWS Cloud Map information for the virtual node.</p>
    #[serde(rename = "awsCloudMap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_cloud_map: Option<AwsCloudMapServiceDiscovery>,
    /// <p>Specifies the DNS information for the virtual node.</p>
    #[serde(rename = "dns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns: Option<DnsServiceDiscovery>,
}

/// <p>Optional metadata that you apply to a resource to assist with categorization and
/// organization. Each tag consists of a key and an optional value, both of which you define.
/// Tag keys can have a maximum character length of 128 characters, and tag values can have
/// a maximum length of 256 characters.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TagRef {
    /// <p>One part of a key-value pair that make up a tag. A <code>key</code> is a general label
    /// that acts like a category for more specific tag values.</p>
    #[serde(rename = "key")]
    pub key: String,
    /// <p>The optional part of a key-value pair that make up a tag. A <code>value</code> acts as a
    /// descriptor within a tag category (key).</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceInput {
    /// <p>The Amazon Resource Name (ARN) of the resource to add tags to.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The tags to add to the resource. A tag is an array of key-value pairs.
    /// Tag keys can have a maximum character length of 128 characters, and tag values can have
    /// a maximum length of 256 characters.</p>
    #[serde(rename = "tags")]
    pub tags: Vec<TagRef>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceOutput {}

/// <p>An object that represents a TCP route type.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TcpRoute {
    /// <p>The action to take if a match is determined.</p>
    #[serde(rename = "action")]
    pub action: TcpRouteAction,
}

/// <p>An object that represents the action to take if a match is determined.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TcpRouteAction {
    /// <p>An object that represents the targets that traffic is routed to when a request matches the route.</p>
    #[serde(rename = "weightedTargets")]
    pub weighted_targets: Vec<WeightedTarget>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceInput {
    /// <p>The Amazon Resource Name (ARN) of the resource to delete tags from.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The keys of the tags to be removed.</p>
    #[serde(rename = "tagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateMeshInput {
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the
    /// request. Up to 36 letters, numbers, hyphens, and underscores are allowed.</p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The name of the service mesh to update.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The service mesh specification to apply.</p>
    #[serde(rename = "spec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<MeshSpec>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateMeshOutput {
    #[serde(rename = "mesh")]
    pub mesh: MeshData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateRouteInput {
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the
    /// request. Up to 36 letters, numbers, hyphens, and underscores are allowed.</p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The name of the service mesh that the route resides in.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The name of the route to update.</p>
    #[serde(rename = "routeName")]
    pub route_name: String,
    /// <p>The new route specification to apply. This overwrites the existing data.</p>
    #[serde(rename = "spec")]
    pub spec: RouteSpec,
    /// <p>The name of the virtual router that the route is associated with.</p>
    #[serde(rename = "virtualRouterName")]
    pub virtual_router_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateRouteOutput {
    /// <p>A full description of the route that was updated.</p>
    #[serde(rename = "route")]
    pub route: RouteData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateVirtualNodeInput {
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the
    /// request. Up to 36 letters, numbers, hyphens, and underscores are allowed.</p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The name of the service mesh that the virtual node resides in.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The new virtual node specification to apply. This overwrites the existing data.</p>
    #[serde(rename = "spec")]
    pub spec: VirtualNodeSpec,
    /// <p>The name of the virtual node to update.</p>
    #[serde(rename = "virtualNodeName")]
    pub virtual_node_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateVirtualNodeOutput {
    /// <p>A full description of the virtual node that was updated.</p>
    #[serde(rename = "virtualNode")]
    pub virtual_node: VirtualNodeData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateVirtualRouterInput {
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the
    /// request. Up to 36 letters, numbers, hyphens, and underscores are allowed.</p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The name of the service mesh that the virtual router resides in.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The new virtual router specification to apply. This overwrites the existing data.</p>
    #[serde(rename = "spec")]
    pub spec: VirtualRouterSpec,
    /// <p>The name of the virtual router to update.</p>
    #[serde(rename = "virtualRouterName")]
    pub virtual_router_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateVirtualRouterOutput {
    /// <p>A full description of the virtual router that was updated.</p>
    #[serde(rename = "virtualRouter")]
    pub virtual_router: VirtualRouterData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateVirtualServiceInput {
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the
    /// request. Up to 36 letters, numbers, hyphens, and underscores are allowed.</p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The name of the service mesh that the virtual service resides in.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The new virtual service specification to apply. This overwrites the existing
    /// data.</p>
    #[serde(rename = "spec")]
    pub spec: VirtualServiceSpec,
    /// <p>The name of the virtual service to update.</p>
    #[serde(rename = "virtualServiceName")]
    pub virtual_service_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateVirtualServiceOutput {
    /// <p>A full description of the virtual service that was updated.</p>
    #[serde(rename = "virtualService")]
    pub virtual_service: VirtualServiceData,
}

/// <p>An object that represents a virtual node returned by a describe operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct VirtualNodeData {
    /// <p>The name of the service mesh that the virtual node resides in.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The associated metadata for the virtual node.</p>
    #[serde(rename = "metadata")]
    pub metadata: ResourceMetadata,
    /// <p>The specifications of the virtual node.</p>
    #[serde(rename = "spec")]
    pub spec: VirtualNodeSpec,
    /// <p>The current status for the virtual node.</p>
    #[serde(rename = "status")]
    pub status: VirtualNodeStatus,
    /// <p>The name of the virtual node.</p>
    #[serde(rename = "virtualNodeName")]
    pub virtual_node_name: String,
}

/// <p>An object that represents a virtual node returned by a list operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct VirtualNodeRef {
    /// <p>The full Amazon Resource Name (ARN) for the virtual node.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>The name of the service mesh that the virtual node resides in.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The name of the virtual node.</p>
    #[serde(rename = "virtualNodeName")]
    pub virtual_node_name: String,
}

/// <p>An object that represents a virtual node service provider.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VirtualNodeServiceProvider {
    /// <p>The name of the virtual node that is acting as a service provider.</p>
    #[serde(rename = "virtualNodeName")]
    pub virtual_node_name: String,
}

/// <p>An object that represents the specification of a virtual node.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VirtualNodeSpec {
    /// <p>The backends that the virtual node is expected to send outbound traffic to.</p>
    #[serde(rename = "backends")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backends: Option<Vec<Backend>>,
    /// <p>The listeners that the virtual node is expected to receive inbound traffic from.
    /// You can specify one listener.</p>
    #[serde(rename = "listeners")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listeners: Option<Vec<Listener>>,
    /// <p>The inbound and outbound access logging information for the virtual node.</p>
    #[serde(rename = "logging")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<Logging>,
    /// <p>The service discovery information for the virtual node. If your virtual node does not
    /// expect ingress traffic, you can omit this parameter.</p>
    #[serde(rename = "serviceDiscovery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_discovery: Option<ServiceDiscovery>,
}

/// <p>An object that represents the current status of the virtual node.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct VirtualNodeStatus {
    /// <p>The current status of the virtual node.</p>
    #[serde(rename = "status")]
    pub status: String,
}

/// <p>An object that represents a virtual router returned by a describe operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct VirtualRouterData {
    /// <p>The name of the service mesh that the virtual router resides in.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The associated metadata for the virtual router.</p>
    #[serde(rename = "metadata")]
    pub metadata: ResourceMetadata,
    /// <p>The specifications of the virtual router.</p>
    #[serde(rename = "spec")]
    pub spec: VirtualRouterSpec,
    /// <p>The current status of the virtual router.</p>
    #[serde(rename = "status")]
    pub status: VirtualRouterStatus,
    /// <p>The name of the virtual router.</p>
    #[serde(rename = "virtualRouterName")]
    pub virtual_router_name: String,
}

/// <p>An object that represents a virtual router listener.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VirtualRouterListener {
    #[serde(rename = "portMapping")]
    pub port_mapping: PortMapping,
}

/// <p>An object that represents a virtual router returned by a list operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct VirtualRouterRef {
    /// <p>The full Amazon Resource Name (ARN) for the virtual router.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>The name of the service mesh that the virtual router resides in.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The name of the virtual router.</p>
    #[serde(rename = "virtualRouterName")]
    pub virtual_router_name: String,
}

/// <p>An object that represents a virtual node service provider.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VirtualRouterServiceProvider {
    /// <p>The name of the virtual router that is acting as a service provider.</p>
    #[serde(rename = "virtualRouterName")]
    pub virtual_router_name: String,
}

/// <p>An object that represents the specification of a virtual router.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VirtualRouterSpec {
    /// <p>The listeners that the virtual router is expected to receive inbound traffic from.
    /// You can specify one listener.</p>
    #[serde(rename = "listeners")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listeners: Option<Vec<VirtualRouterListener>>,
}

/// <p>An object that represents the status of a virtual router. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct VirtualRouterStatus {
    /// <p>The current status of the virtual router.</p>
    #[serde(rename = "status")]
    pub status: String,
}

/// <p>An object that represents a virtual service backend for a virtual node.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VirtualServiceBackend {
    /// <p>The name of the virtual service that is acting as a virtual node backend.</p>
    #[serde(rename = "virtualServiceName")]
    pub virtual_service_name: String,
}

/// <p>An object that represents a virtual service returned by a describe operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct VirtualServiceData {
    /// <p>The name of the service mesh that the virtual service resides in.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    #[serde(rename = "metadata")]
    pub metadata: ResourceMetadata,
    /// <p>The specifications of the virtual service.</p>
    #[serde(rename = "spec")]
    pub spec: VirtualServiceSpec,
    /// <p>The current status of the virtual service.</p>
    #[serde(rename = "status")]
    pub status: VirtualServiceStatus,
    /// <p>The name of the virtual service.</p>
    #[serde(rename = "virtualServiceName")]
    pub virtual_service_name: String,
}

/// <p>An object that represents the provider for a virtual service.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VirtualServiceProvider {
    /// <p>The virtual node associated with a virtual service.</p>
    #[serde(rename = "virtualNode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_node: Option<VirtualNodeServiceProvider>,
    /// <p>The virtual router associated with a virtual service.</p>
    #[serde(rename = "virtualRouter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_router: Option<VirtualRouterServiceProvider>,
}

/// <p>An object that represents a virtual service returned by a list operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct VirtualServiceRef {
    /// <p>The full Amazon Resource Name (ARN) for the virtual service.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>The name of the service mesh that the virtual service resides in.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The name of the virtual service.</p>
    #[serde(rename = "virtualServiceName")]
    pub virtual_service_name: String,
}

/// <p>An object that represents the specification of a virtual service.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VirtualServiceSpec {
    /// <p>The App Mesh object that is acting as the provider for a virtual service. You can specify
    /// a single virtual node or virtual router.</p>
    #[serde(rename = "provider")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<VirtualServiceProvider>,
}

/// <p>An object that represents the status of a virtual service.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct VirtualServiceStatus {
    /// <p>The current status of the virtual service.</p>
    #[serde(rename = "status")]
    pub status: String,
}

/// <p>An object that represents a target and its relative weight. Traffic is distributed across
/// targets according to their relative weight. For example, a weighted target with a relative
/// weight of 50 receives five times as much traffic as one with a relative weight of
/// 10. The total weight for all targets combined must be less than or equal to 100.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WeightedTarget {
    /// <p>The virtual node to associate with the weighted target.</p>
    #[serde(rename = "virtualNode")]
    pub virtual_node: String,
    /// <p>The relative weight of the weighted target.</p>
    #[serde(rename = "weight")]
    pub weight: i64,
}

/// Errors returned by CreateMesh
#[derive(Debug, PartialEq)]
pub enum CreateMeshError {
    /// <p>The request syntax was malformed. Check your request syntax and try again.</p>
    BadRequest(String),
    /// <p>The request contains a client token that was used for a previous update resource call
    /// with different specifications. Try the request again with a new client token.</p>
    Conflict(String),
    /// <p>You don't have permissions to perform this action.</p>
    Forbidden(String),
    /// <p>The request processing has failed because of an unknown error, exception, or
    /// failure.</p>
    InternalServerError(String),
    /// <p>You have exceeded a service limit for your account. For more information, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/service_limits.html">Service
    /// Limits</a> in the <i>AWS App Mesh User Guide</i>.</p>
    LimitExceeded(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your
    /// account. For best results, use an increasing or variable sleep interval between
    /// requests.</p>
    TooManyRequests(String),
}

impl CreateMeshError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateMeshError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateMeshError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateMeshError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateMeshError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateMeshError::InternalServerError(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateMeshError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateMeshError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateMeshError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateMeshError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateMeshError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateMeshError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateMeshError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateMeshError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreateMeshError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreateMeshError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateMeshError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateMeshError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            CreateMeshError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateMeshError {}
/// Errors returned by CreateRoute
#[derive(Debug, PartialEq)]
pub enum CreateRouteError {
    /// <p>The request syntax was malformed. Check your request syntax and try again.</p>
    BadRequest(String),
    /// <p>The request contains a client token that was used for a previous update resource call
    /// with different specifications. Try the request again with a new client token.</p>
    Conflict(String),
    /// <p>You don't have permissions to perform this action.</p>
    Forbidden(String),
    /// <p>The request processing has failed because of an unknown error, exception, or
    /// failure.</p>
    InternalServerError(String),
    /// <p>You have exceeded a service limit for your account. For more information, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/service_limits.html">Service
    /// Limits</a> in the <i>AWS App Mesh User Guide</i>.</p>
    LimitExceeded(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your
    /// account. For best results, use an increasing or variable sleep interval between
    /// requests.</p>
    TooManyRequests(String),
}

impl CreateRouteError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateRouteError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateRouteError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateRouteError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateRouteError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateRouteError::InternalServerError(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateRouteError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateRouteError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateRouteError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateRouteError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateRouteError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateRouteError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateRouteError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateRouteError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreateRouteError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreateRouteError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateRouteError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateRouteError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            CreateRouteError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateRouteError {}
/// Errors returned by CreateVirtualNode
#[derive(Debug, PartialEq)]
pub enum CreateVirtualNodeError {
    /// <p>The request syntax was malformed. Check your request syntax and try again.</p>
    BadRequest(String),
    /// <p>The request contains a client token that was used for a previous update resource call
    /// with different specifications. Try the request again with a new client token.</p>
    Conflict(String),
    /// <p>You don't have permissions to perform this action.</p>
    Forbidden(String),
    /// <p>The request processing has failed because of an unknown error, exception, or
    /// failure.</p>
    InternalServerError(String),
    /// <p>You have exceeded a service limit for your account. For more information, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/service_limits.html">Service
    /// Limits</a> in the <i>AWS App Mesh User Guide</i>.</p>
    LimitExceeded(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your
    /// account. For best results, use an increasing or variable sleep interval between
    /// requests.</p>
    TooManyRequests(String),
}

impl CreateVirtualNodeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateVirtualNodeError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateVirtualNodeError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateVirtualNodeError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateVirtualNodeError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateVirtualNodeError::InternalServerError(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateVirtualNodeError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateVirtualNodeError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateVirtualNodeError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateVirtualNodeError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateVirtualNodeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateVirtualNodeError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateVirtualNodeError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateVirtualNodeError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreateVirtualNodeError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreateVirtualNodeError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateVirtualNodeError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateVirtualNodeError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            CreateVirtualNodeError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateVirtualNodeError {}
/// Errors returned by CreateVirtualRouter
#[derive(Debug, PartialEq)]
pub enum CreateVirtualRouterError {
    /// <p>The request syntax was malformed. Check your request syntax and try again.</p>
    BadRequest(String),
    /// <p>The request contains a client token that was used for a previous update resource call
    /// with different specifications. Try the request again with a new client token.</p>
    Conflict(String),
    /// <p>You don't have permissions to perform this action.</p>
    Forbidden(String),
    /// <p>The request processing has failed because of an unknown error, exception, or
    /// failure.</p>
    InternalServerError(String),
    /// <p>You have exceeded a service limit for your account. For more information, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/service_limits.html">Service
    /// Limits</a> in the <i>AWS App Mesh User Guide</i>.</p>
    LimitExceeded(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your
    /// account. For best results, use an increasing or variable sleep interval between
    /// requests.</p>
    TooManyRequests(String),
}

impl CreateVirtualRouterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateVirtualRouterError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateVirtualRouterError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateVirtualRouterError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateVirtualRouterError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateVirtualRouterError::InternalServerError(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateVirtualRouterError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateVirtualRouterError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateVirtualRouterError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateVirtualRouterError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateVirtualRouterError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateVirtualRouterError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateVirtualRouterError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateVirtualRouterError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreateVirtualRouterError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreateVirtualRouterError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateVirtualRouterError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateVirtualRouterError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            CreateVirtualRouterError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateVirtualRouterError {}
/// Errors returned by CreateVirtualService
#[derive(Debug, PartialEq)]
pub enum CreateVirtualServiceError {
    /// <p>The request syntax was malformed. Check your request syntax and try again.</p>
    BadRequest(String),
    /// <p>The request contains a client token that was used for a previous update resource call
    /// with different specifications. Try the request again with a new client token.</p>
    Conflict(String),
    /// <p>You don't have permissions to perform this action.</p>
    Forbidden(String),
    /// <p>The request processing has failed because of an unknown error, exception, or
    /// failure.</p>
    InternalServerError(String),
    /// <p>You have exceeded a service limit for your account. For more information, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/service_limits.html">Service
    /// Limits</a> in the <i>AWS App Mesh User Guide</i>.</p>
    LimitExceeded(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your
    /// account. For best results, use an increasing or variable sleep interval between
    /// requests.</p>
    TooManyRequests(String),
}

impl CreateVirtualServiceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateVirtualServiceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateVirtualServiceError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateVirtualServiceError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateVirtualServiceError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateVirtualServiceError::InternalServerError(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateVirtualServiceError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateVirtualServiceError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateVirtualServiceError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateVirtualServiceError::TooManyRequests(
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
impl fmt::Display for CreateVirtualServiceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateVirtualServiceError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateVirtualServiceError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateVirtualServiceError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreateVirtualServiceError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreateVirtualServiceError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateVirtualServiceError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateVirtualServiceError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            CreateVirtualServiceError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateVirtualServiceError {}
/// Errors returned by DeleteMesh
#[derive(Debug, PartialEq)]
pub enum DeleteMeshError {
    /// <p>The request syntax was malformed. Check your request syntax and try again.</p>
    BadRequest(String),
    /// <p>You don't have permissions to perform this action.</p>
    Forbidden(String),
    /// <p>The request processing has failed because of an unknown error, exception, or
    /// failure.</p>
    InternalServerError(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>You can't delete the specified resource because it's in use or required by another
    /// resource.</p>
    ResourceInUse(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your
    /// account. For best results, use an increasing or variable sleep interval between
    /// requests.</p>
    TooManyRequests(String),
}

impl DeleteMeshError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteMeshError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteMeshError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteMeshError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteMeshError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteMeshError::NotFound(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteMeshError::ResourceInUse(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteMeshError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteMeshError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteMeshError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteMeshError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteMeshError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteMeshError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteMeshError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteMeshError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            DeleteMeshError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DeleteMeshError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteMeshError {}
/// Errors returned by DeleteRoute
#[derive(Debug, PartialEq)]
pub enum DeleteRouteError {
    /// <p>The request syntax was malformed. Check your request syntax and try again.</p>
    BadRequest(String),
    /// <p>You don't have permissions to perform this action.</p>
    Forbidden(String),
    /// <p>The request processing has failed because of an unknown error, exception, or
    /// failure.</p>
    InternalServerError(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>You can't delete the specified resource because it's in use or required by another
    /// resource.</p>
    ResourceInUse(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your
    /// account. For best results, use an increasing or variable sleep interval between
    /// requests.</p>
    TooManyRequests(String),
}

impl DeleteRouteError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteRouteError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteRouteError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteRouteError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteRouteError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteRouteError::NotFound(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteRouteError::ResourceInUse(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteRouteError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteRouteError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteRouteError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteRouteError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteRouteError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteRouteError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteRouteError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteRouteError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            DeleteRouteError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DeleteRouteError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteRouteError {}
/// Errors returned by DeleteVirtualNode
#[derive(Debug, PartialEq)]
pub enum DeleteVirtualNodeError {
    /// <p>The request syntax was malformed. Check your request syntax and try again.</p>
    BadRequest(String),
    /// <p>You don't have permissions to perform this action.</p>
    Forbidden(String),
    /// <p>The request processing has failed because of an unknown error, exception, or
    /// failure.</p>
    InternalServerError(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>You can't delete the specified resource because it's in use or required by another
    /// resource.</p>
    ResourceInUse(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your
    /// account. For best results, use an increasing or variable sleep interval between
    /// requests.</p>
    TooManyRequests(String),
}

impl DeleteVirtualNodeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteVirtualNodeError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteVirtualNodeError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteVirtualNodeError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteVirtualNodeError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteVirtualNodeError::NotFound(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteVirtualNodeError::ResourceInUse(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteVirtualNodeError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteVirtualNodeError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteVirtualNodeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteVirtualNodeError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteVirtualNodeError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteVirtualNodeError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteVirtualNodeError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteVirtualNodeError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            DeleteVirtualNodeError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DeleteVirtualNodeError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteVirtualNodeError {}
/// Errors returned by DeleteVirtualRouter
#[derive(Debug, PartialEq)]
pub enum DeleteVirtualRouterError {
    /// <p>The request syntax was malformed. Check your request syntax and try again.</p>
    BadRequest(String),
    /// <p>You don't have permissions to perform this action.</p>
    Forbidden(String),
    /// <p>The request processing has failed because of an unknown error, exception, or
    /// failure.</p>
    InternalServerError(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>You can't delete the specified resource because it's in use or required by another
    /// resource.</p>
    ResourceInUse(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your
    /// account. For best results, use an increasing or variable sleep interval between
    /// requests.</p>
    TooManyRequests(String),
}

impl DeleteVirtualRouterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteVirtualRouterError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteVirtualRouterError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteVirtualRouterError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteVirtualRouterError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteVirtualRouterError::NotFound(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteVirtualRouterError::ResourceInUse(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteVirtualRouterError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteVirtualRouterError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteVirtualRouterError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteVirtualRouterError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteVirtualRouterError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteVirtualRouterError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteVirtualRouterError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteVirtualRouterError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            DeleteVirtualRouterError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DeleteVirtualRouterError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteVirtualRouterError {}
/// Errors returned by DeleteVirtualService
#[derive(Debug, PartialEq)]
pub enum DeleteVirtualServiceError {
    /// <p>The request syntax was malformed. Check your request syntax and try again.</p>
    BadRequest(String),
    /// <p>You don't have permissions to perform this action.</p>
    Forbidden(String),
    /// <p>The request processing has failed because of an unknown error, exception, or
    /// failure.</p>
    InternalServerError(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your
    /// account. For best results, use an increasing or variable sleep interval between
    /// requests.</p>
    TooManyRequests(String),
}

impl DeleteVirtualServiceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteVirtualServiceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteVirtualServiceError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteVirtualServiceError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteVirtualServiceError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteVirtualServiceError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteVirtualServiceError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteVirtualServiceError::TooManyRequests(
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
impl fmt::Display for DeleteVirtualServiceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteVirtualServiceError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteVirtualServiceError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteVirtualServiceError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteVirtualServiceError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteVirtualServiceError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DeleteVirtualServiceError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteVirtualServiceError {}
/// Errors returned by DescribeMesh
#[derive(Debug, PartialEq)]
pub enum DescribeMeshError {
    /// <p>The request syntax was malformed. Check your request syntax and try again.</p>
    BadRequest(String),
    /// <p>You don't have permissions to perform this action.</p>
    Forbidden(String),
    /// <p>The request processing has failed because of an unknown error, exception, or
    /// failure.</p>
    InternalServerError(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your
    /// account. For best results, use an increasing or variable sleep interval between
    /// requests.</p>
    TooManyRequests(String),
}

impl DescribeMeshError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeMeshError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DescribeMeshError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DescribeMeshError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DescribeMeshError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeMeshError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeMeshError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeMeshError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeMeshError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeMeshError::BadRequest(ref cause) => write!(f, "{}", cause),
            DescribeMeshError::Forbidden(ref cause) => write!(f, "{}", cause),
            DescribeMeshError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DescribeMeshError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribeMeshError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DescribeMeshError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeMeshError {}
/// Errors returned by DescribeRoute
#[derive(Debug, PartialEq)]
pub enum DescribeRouteError {
    /// <p>The request syntax was malformed. Check your request syntax and try again.</p>
    BadRequest(String),
    /// <p>You don't have permissions to perform this action.</p>
    Forbidden(String),
    /// <p>The request processing has failed because of an unknown error, exception, or
    /// failure.</p>
    InternalServerError(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your
    /// account. For best results, use an increasing or variable sleep interval between
    /// requests.</p>
    TooManyRequests(String),
}

impl DescribeRouteError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeRouteError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DescribeRouteError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DescribeRouteError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DescribeRouteError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeRouteError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeRouteError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeRouteError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeRouteError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeRouteError::BadRequest(ref cause) => write!(f, "{}", cause),
            DescribeRouteError::Forbidden(ref cause) => write!(f, "{}", cause),
            DescribeRouteError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DescribeRouteError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribeRouteError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DescribeRouteError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeRouteError {}
/// Errors returned by DescribeVirtualNode
#[derive(Debug, PartialEq)]
pub enum DescribeVirtualNodeError {
    /// <p>The request syntax was malformed. Check your request syntax and try again.</p>
    BadRequest(String),
    /// <p>You don't have permissions to perform this action.</p>
    Forbidden(String),
    /// <p>The request processing has failed because of an unknown error, exception, or
    /// failure.</p>
    InternalServerError(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your
    /// account. For best results, use an increasing or variable sleep interval between
    /// requests.</p>
    TooManyRequests(String),
}

impl DescribeVirtualNodeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeVirtualNodeError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DescribeVirtualNodeError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DescribeVirtualNodeError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DescribeVirtualNodeError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeVirtualNodeError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeVirtualNodeError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeVirtualNodeError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeVirtualNodeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeVirtualNodeError::BadRequest(ref cause) => write!(f, "{}", cause),
            DescribeVirtualNodeError::Forbidden(ref cause) => write!(f, "{}", cause),
            DescribeVirtualNodeError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DescribeVirtualNodeError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribeVirtualNodeError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DescribeVirtualNodeError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeVirtualNodeError {}
/// Errors returned by DescribeVirtualRouter
#[derive(Debug, PartialEq)]
pub enum DescribeVirtualRouterError {
    /// <p>The request syntax was malformed. Check your request syntax and try again.</p>
    BadRequest(String),
    /// <p>You don't have permissions to perform this action.</p>
    Forbidden(String),
    /// <p>The request processing has failed because of an unknown error, exception, or
    /// failure.</p>
    InternalServerError(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your
    /// account. For best results, use an increasing or variable sleep interval between
    /// requests.</p>
    TooManyRequests(String),
}

impl DescribeVirtualRouterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeVirtualRouterError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DescribeVirtualRouterError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DescribeVirtualRouterError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DescribeVirtualRouterError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeVirtualRouterError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeVirtualRouterError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeVirtualRouterError::TooManyRequests(
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
impl fmt::Display for DescribeVirtualRouterError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeVirtualRouterError::BadRequest(ref cause) => write!(f, "{}", cause),
            DescribeVirtualRouterError::Forbidden(ref cause) => write!(f, "{}", cause),
            DescribeVirtualRouterError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DescribeVirtualRouterError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribeVirtualRouterError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DescribeVirtualRouterError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeVirtualRouterError {}
/// Errors returned by DescribeVirtualService
#[derive(Debug, PartialEq)]
pub enum DescribeVirtualServiceError {
    /// <p>The request syntax was malformed. Check your request syntax and try again.</p>
    BadRequest(String),
    /// <p>You don't have permissions to perform this action.</p>
    Forbidden(String),
    /// <p>The request processing has failed because of an unknown error, exception, or
    /// failure.</p>
    InternalServerError(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your
    /// account. For best results, use an increasing or variable sleep interval between
    /// requests.</p>
    TooManyRequests(String),
}

impl DescribeVirtualServiceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeVirtualServiceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DescribeVirtualServiceError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DescribeVirtualServiceError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DescribeVirtualServiceError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeVirtualServiceError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeVirtualServiceError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeVirtualServiceError::TooManyRequests(
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
impl fmt::Display for DescribeVirtualServiceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeVirtualServiceError::BadRequest(ref cause) => write!(f, "{}", cause),
            DescribeVirtualServiceError::Forbidden(ref cause) => write!(f, "{}", cause),
            DescribeVirtualServiceError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DescribeVirtualServiceError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribeVirtualServiceError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DescribeVirtualServiceError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeVirtualServiceError {}
/// Errors returned by ListMeshes
#[derive(Debug, PartialEq)]
pub enum ListMeshesError {
    /// <p>The request syntax was malformed. Check your request syntax and try again.</p>
    BadRequest(String),
    /// <p>You don't have permissions to perform this action.</p>
    Forbidden(String),
    /// <p>The request processing has failed because of an unknown error, exception, or
    /// failure.</p>
    InternalServerError(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your
    /// account. For best results, use an increasing or variable sleep interval between
    /// requests.</p>
    TooManyRequests(String),
}

impl ListMeshesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListMeshesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListMeshesError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListMeshesError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListMeshesError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListMeshesError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListMeshesError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListMeshesError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListMeshesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListMeshesError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListMeshesError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListMeshesError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListMeshesError::NotFound(ref cause) => write!(f, "{}", cause),
            ListMeshesError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            ListMeshesError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListMeshesError {}
/// Errors returned by ListRoutes
#[derive(Debug, PartialEq)]
pub enum ListRoutesError {
    /// <p>The request syntax was malformed. Check your request syntax and try again.</p>
    BadRequest(String),
    /// <p>You don't have permissions to perform this action.</p>
    Forbidden(String),
    /// <p>The request processing has failed because of an unknown error, exception, or
    /// failure.</p>
    InternalServerError(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your
    /// account. For best results, use an increasing or variable sleep interval between
    /// requests.</p>
    TooManyRequests(String),
}

impl ListRoutesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListRoutesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListRoutesError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListRoutesError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListRoutesError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListRoutesError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListRoutesError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListRoutesError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListRoutesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListRoutesError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListRoutesError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListRoutesError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListRoutesError::NotFound(ref cause) => write!(f, "{}", cause),
            ListRoutesError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            ListRoutesError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListRoutesError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>The request syntax was malformed. Check your request syntax and try again.</p>
    BadRequest(String),
    /// <p>You don't have permissions to perform this action.</p>
    Forbidden(String),
    /// <p>The request processing has failed because of an unknown error, exception, or
    /// failure.</p>
    InternalServerError(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your
    /// account. For best results, use an increasing or variable sleep interval between
    /// requests.</p>
    TooManyRequests(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListTagsForResourceError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListTagsForResourceError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListTagsForResourceError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListTagsForResourceError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListTagsForResourceError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListTagsForResourceError::TooManyRequests(err.msg))
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
            ListTagsForResourceError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::NotFound(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by ListVirtualNodes
#[derive(Debug, PartialEq)]
pub enum ListVirtualNodesError {
    /// <p>The request syntax was malformed. Check your request syntax and try again.</p>
    BadRequest(String),
    /// <p>You don't have permissions to perform this action.</p>
    Forbidden(String),
    /// <p>The request processing has failed because of an unknown error, exception, or
    /// failure.</p>
    InternalServerError(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your
    /// account. For best results, use an increasing or variable sleep interval between
    /// requests.</p>
    TooManyRequests(String),
}

impl ListVirtualNodesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListVirtualNodesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListVirtualNodesError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListVirtualNodesError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListVirtualNodesError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListVirtualNodesError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListVirtualNodesError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListVirtualNodesError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListVirtualNodesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListVirtualNodesError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListVirtualNodesError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListVirtualNodesError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListVirtualNodesError::NotFound(ref cause) => write!(f, "{}", cause),
            ListVirtualNodesError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            ListVirtualNodesError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListVirtualNodesError {}
/// Errors returned by ListVirtualRouters
#[derive(Debug, PartialEq)]
pub enum ListVirtualRoutersError {
    /// <p>The request syntax was malformed. Check your request syntax and try again.</p>
    BadRequest(String),
    /// <p>You don't have permissions to perform this action.</p>
    Forbidden(String),
    /// <p>The request processing has failed because of an unknown error, exception, or
    /// failure.</p>
    InternalServerError(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your
    /// account. For best results, use an increasing or variable sleep interval between
    /// requests.</p>
    TooManyRequests(String),
}

impl ListVirtualRoutersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListVirtualRoutersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListVirtualRoutersError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListVirtualRoutersError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListVirtualRoutersError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListVirtualRoutersError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListVirtualRoutersError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListVirtualRoutersError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListVirtualRoutersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListVirtualRoutersError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListVirtualRoutersError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListVirtualRoutersError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListVirtualRoutersError::NotFound(ref cause) => write!(f, "{}", cause),
            ListVirtualRoutersError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            ListVirtualRoutersError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListVirtualRoutersError {}
/// Errors returned by ListVirtualServices
#[derive(Debug, PartialEq)]
pub enum ListVirtualServicesError {
    /// <p>The request syntax was malformed. Check your request syntax and try again.</p>
    BadRequest(String),
    /// <p>You don't have permissions to perform this action.</p>
    Forbidden(String),
    /// <p>The request processing has failed because of an unknown error, exception, or
    /// failure.</p>
    InternalServerError(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your
    /// account. For best results, use an increasing or variable sleep interval between
    /// requests.</p>
    TooManyRequests(String),
}

impl ListVirtualServicesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListVirtualServicesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListVirtualServicesError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListVirtualServicesError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListVirtualServicesError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListVirtualServicesError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListVirtualServicesError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListVirtualServicesError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListVirtualServicesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListVirtualServicesError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListVirtualServicesError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListVirtualServicesError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListVirtualServicesError::NotFound(ref cause) => write!(f, "{}", cause),
            ListVirtualServicesError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            ListVirtualServicesError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListVirtualServicesError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>The request syntax was malformed. Check your request syntax and try again.</p>
    BadRequest(String),
    /// <p>You don't have permissions to perform this action.</p>
    Forbidden(String),
    /// <p>The request processing has failed because of an unknown error, exception, or
    /// failure.</p>
    InternalServerError(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your
    /// account. For best results, use an increasing or variable sleep interval between
    /// requests.</p>
    TooManyRequests(String),
    /// <p>The request exceeds the maximum allowed number of tags allowed per resource. The current
    /// limit is 50 user tags per resource. You must reduce the number of tags in the request. None
    /// of the tags in this request were applied.</p>
    TooManyTags(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(TagResourceError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(TagResourceError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(TagResourceError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(TagResourceError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(TagResourceError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(TagResourceError::TooManyRequests(err.msg))
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
            TagResourceError::BadRequest(ref cause) => write!(f, "{}", cause),
            TagResourceError::Forbidden(ref cause) => write!(f, "{}", cause),
            TagResourceError::InternalServerError(ref cause) => write!(f, "{}", cause),
            TagResourceError::NotFound(ref cause) => write!(f, "{}", cause),
            TagResourceError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            TagResourceError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            TagResourceError::TooManyTags(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>The request syntax was malformed. Check your request syntax and try again.</p>
    BadRequest(String),
    /// <p>You don't have permissions to perform this action.</p>
    Forbidden(String),
    /// <p>The request processing has failed because of an unknown error, exception, or
    /// failure.</p>
    InternalServerError(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your
    /// account. For best results, use an increasing or variable sleep interval between
    /// requests.</p>
    TooManyRequests(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UntagResourceError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UntagResourceError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UntagResourceError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UntagResourceError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UntagResourceError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UntagResourceError::TooManyRequests(err.msg))
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
            UntagResourceError::BadRequest(ref cause) => write!(f, "{}", cause),
            UntagResourceError::Forbidden(ref cause) => write!(f, "{}", cause),
            UntagResourceError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UntagResourceError::NotFound(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            UntagResourceError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateMesh
#[derive(Debug, PartialEq)]
pub enum UpdateMeshError {
    /// <p>The request syntax was malformed. Check your request syntax and try again.</p>
    BadRequest(String),
    /// <p>The request contains a client token that was used for a previous update resource call
    /// with different specifications. Try the request again with a new client token.</p>
    Conflict(String),
    /// <p>You don't have permissions to perform this action.</p>
    Forbidden(String),
    /// <p>The request processing has failed because of an unknown error, exception, or
    /// failure.</p>
    InternalServerError(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your
    /// account. For best results, use an increasing or variable sleep interval between
    /// requests.</p>
    TooManyRequests(String),
}

impl UpdateMeshError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateMeshError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateMeshError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateMeshError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateMeshError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateMeshError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateMeshError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdateMeshError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateMeshError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateMeshError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateMeshError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateMeshError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateMeshError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateMeshError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateMeshError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateMeshError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            UpdateMeshError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateMeshError {}
/// Errors returned by UpdateRoute
#[derive(Debug, PartialEq)]
pub enum UpdateRouteError {
    /// <p>The request syntax was malformed. Check your request syntax and try again.</p>
    BadRequest(String),
    /// <p>The request contains a client token that was used for a previous update resource call
    /// with different specifications. Try the request again with a new client token.</p>
    Conflict(String),
    /// <p>You don't have permissions to perform this action.</p>
    Forbidden(String),
    /// <p>The request processing has failed because of an unknown error, exception, or
    /// failure.</p>
    InternalServerError(String),
    /// <p>You have exceeded a service limit for your account. For more information, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/service_limits.html">Service
    /// Limits</a> in the <i>AWS App Mesh User Guide</i>.</p>
    LimitExceeded(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your
    /// account. For best results, use an increasing or variable sleep interval between
    /// requests.</p>
    TooManyRequests(String),
}

impl UpdateRouteError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateRouteError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateRouteError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateRouteError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateRouteError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateRouteError::InternalServerError(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateRouteError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateRouteError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdateRouteError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateRouteError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateRouteError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateRouteError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateRouteError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateRouteError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateRouteError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateRouteError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateRouteError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateRouteError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            UpdateRouteError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateRouteError {}
/// Errors returned by UpdateVirtualNode
#[derive(Debug, PartialEq)]
pub enum UpdateVirtualNodeError {
    /// <p>The request syntax was malformed. Check your request syntax and try again.</p>
    BadRequest(String),
    /// <p>The request contains a client token that was used for a previous update resource call
    /// with different specifications. Try the request again with a new client token.</p>
    Conflict(String),
    /// <p>You don't have permissions to perform this action.</p>
    Forbidden(String),
    /// <p>The request processing has failed because of an unknown error, exception, or
    /// failure.</p>
    InternalServerError(String),
    /// <p>You have exceeded a service limit for your account. For more information, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/service_limits.html">Service
    /// Limits</a> in the <i>AWS App Mesh User Guide</i>.</p>
    LimitExceeded(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your
    /// account. For best results, use an increasing or variable sleep interval between
    /// requests.</p>
    TooManyRequests(String),
}

impl UpdateVirtualNodeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateVirtualNodeError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateVirtualNodeError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateVirtualNodeError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateVirtualNodeError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateVirtualNodeError::InternalServerError(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateVirtualNodeError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateVirtualNodeError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdateVirtualNodeError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateVirtualNodeError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateVirtualNodeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateVirtualNodeError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateVirtualNodeError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateVirtualNodeError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateVirtualNodeError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateVirtualNodeError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateVirtualNodeError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateVirtualNodeError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            UpdateVirtualNodeError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateVirtualNodeError {}
/// Errors returned by UpdateVirtualRouter
#[derive(Debug, PartialEq)]
pub enum UpdateVirtualRouterError {
    /// <p>The request syntax was malformed. Check your request syntax and try again.</p>
    BadRequest(String),
    /// <p>The request contains a client token that was used for a previous update resource call
    /// with different specifications. Try the request again with a new client token.</p>
    Conflict(String),
    /// <p>You don't have permissions to perform this action.</p>
    Forbidden(String),
    /// <p>The request processing has failed because of an unknown error, exception, or
    /// failure.</p>
    InternalServerError(String),
    /// <p>You have exceeded a service limit for your account. For more information, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/service_limits.html">Service
    /// Limits</a> in the <i>AWS App Mesh User Guide</i>.</p>
    LimitExceeded(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your
    /// account. For best results, use an increasing or variable sleep interval between
    /// requests.</p>
    TooManyRequests(String),
}

impl UpdateVirtualRouterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateVirtualRouterError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateVirtualRouterError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateVirtualRouterError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateVirtualRouterError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateVirtualRouterError::InternalServerError(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateVirtualRouterError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateVirtualRouterError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdateVirtualRouterError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateVirtualRouterError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateVirtualRouterError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateVirtualRouterError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateVirtualRouterError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateVirtualRouterError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateVirtualRouterError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateVirtualRouterError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateVirtualRouterError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateVirtualRouterError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            UpdateVirtualRouterError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateVirtualRouterError {}
/// Errors returned by UpdateVirtualService
#[derive(Debug, PartialEq)]
pub enum UpdateVirtualServiceError {
    /// <p>The request syntax was malformed. Check your request syntax and try again.</p>
    BadRequest(String),
    /// <p>The request contains a client token that was used for a previous update resource call
    /// with different specifications. Try the request again with a new client token.</p>
    Conflict(String),
    /// <p>You don't have permissions to perform this action.</p>
    Forbidden(String),
    /// <p>The request processing has failed because of an unknown error, exception, or
    /// failure.</p>
    InternalServerError(String),
    /// <p>You have exceeded a service limit for your account. For more information, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/service_limits.html">Service
    /// Limits</a> in the <i>AWS App Mesh User Guide</i>.</p>
    LimitExceeded(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your
    /// account. For best results, use an increasing or variable sleep interval between
    /// requests.</p>
    TooManyRequests(String),
}

impl UpdateVirtualServiceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateVirtualServiceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateVirtualServiceError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateVirtualServiceError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateVirtualServiceError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateVirtualServiceError::InternalServerError(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateVirtualServiceError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateVirtualServiceError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdateVirtualServiceError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateVirtualServiceError::TooManyRequests(
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
impl fmt::Display for UpdateVirtualServiceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateVirtualServiceError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateVirtualServiceError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateVirtualServiceError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateVirtualServiceError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateVirtualServiceError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateVirtualServiceError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateVirtualServiceError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            UpdateVirtualServiceError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateVirtualServiceError {}
/// Trait representing the capabilities of the AWS App Mesh API. AWS App Mesh clients implement this trait.
#[async_trait]
pub trait AppMesh {
    /// <p>Creates a service mesh. A service mesh is a logical boundary for network traffic between
    /// the services that reside within it.</p>
    ///
    /// <pre><code>     &lt;p&gt;After you create your service mesh, you can create virtual services, virtual nodes,
    /// virtual routers, and routes to distribute traffic between the applications in your
    /// mesh.&lt;/p&gt;
    /// </code></pre>
    async fn create_mesh(
        &self,
        input: CreateMeshInput,
    ) -> Result<CreateMeshOutput, RusotoError<CreateMeshError>>;

    /// <p>Creates a route that is associated with a virtual router.</p>
    ///
    /// <pre><code>     &lt;p&gt;You can use the &lt;code&gt;prefix&lt;/code&gt; parameter in your route specification for path-based
    /// routing of requests. For example, if your virtual service name is
    /// &lt;code&gt;my-service.local&lt;/code&gt; and you want the route to match requests to
    /// &lt;code&gt;my-service.local/metrics&lt;/code&gt;, your prefix should be
    /// &lt;code&gt;/metrics&lt;/code&gt;.&lt;/p&gt;
    /// &lt;p&gt;If your route matches a request, you can distribute traffic to one or more target
    /// virtual nodes with relative weighting.&lt;/p&gt;
    /// </code></pre>
    async fn create_route(
        &self,
        input: CreateRouteInput,
    ) -> Result<CreateRouteOutput, RusotoError<CreateRouteError>>;

    /// <p>Creates a virtual node within a service mesh.</p>
    ///
    /// <pre><code>     &lt;p&gt;A virtual node acts as a logical pointer to a particular task group, such as an Amazon ECS
    /// service or a Kubernetes deployment. When you create a virtual node, you can specify the
    /// service discovery information for your task group.&lt;/p&gt;
    /// &lt;p&gt;Any inbound traffic that your virtual node expects should be specified as a
    /// &lt;code&gt;listener&lt;/code&gt;. Any outbound traffic that your virtual node expects to reach
    /// should be specified as a &lt;code&gt;backend&lt;/code&gt;.&lt;/p&gt;
    /// &lt;p&gt;The response metadata for your new virtual node contains the &lt;code&gt;arn&lt;/code&gt; that is
    /// associated with the virtual node. Set this value (either the full ARN or the truncated
    /// resource name: for example, &lt;code&gt;mesh/default/virtualNode/simpleapp&lt;/code&gt;) as the
    /// &lt;code&gt;APPMESH_VIRTUAL_NODE_NAME&lt;/code&gt; environment variable for your task group&#39;s Envoy
    /// proxy container in your task definition or pod spec. This is then mapped to the
    /// &lt;code&gt;node.id&lt;/code&gt; and &lt;code&gt;node.cluster&lt;/code&gt; Envoy parameters.&lt;/p&gt;
    /// &lt;note&gt;
    /// &lt;p&gt;If you require your Envoy stats or tracing to use a different name, you can override
    /// the &lt;code&gt;node.cluster&lt;/code&gt; value that is set by
    /// &lt;code&gt;APPMESH_VIRTUAL_NODE_NAME&lt;/code&gt; with the
    /// &lt;code&gt;APPMESH_VIRTUAL_NODE_CLUSTER&lt;/code&gt; environment variable.&lt;/p&gt;
    /// &lt;/note&gt;
    /// </code></pre>
    async fn create_virtual_node(
        &self,
        input: CreateVirtualNodeInput,
    ) -> Result<CreateVirtualNodeOutput, RusotoError<CreateVirtualNodeError>>;

    /// <p>Creates a virtual router within a service mesh.</p>
    ///
    /// <pre><code>     &lt;p&gt;Any inbound traffic that your virtual router expects should be specified as a
    /// &lt;code&gt;listener&lt;/code&gt;. &lt;/p&gt;
    /// &lt;p&gt;Virtual routers handle traffic for one or more virtual services within your mesh. After
    /// you create your virtual router, create and associate routes for your virtual router that
    /// direct incoming requests to different virtual nodes.&lt;/p&gt;
    /// </code></pre>
    async fn create_virtual_router(
        &self,
        input: CreateVirtualRouterInput,
    ) -> Result<CreateVirtualRouterOutput, RusotoError<CreateVirtualRouterError>>;

    /// <p>Creates a virtual service within a service mesh.</p>
    ///
    /// <pre><code>     &lt;p&gt;A virtual service is an abstraction of a real service that is provided by a virtual node
    /// directly or indirectly by means of a virtual router. Dependent services call your virtual
    /// service by its &lt;code&gt;virtualServiceName&lt;/code&gt;, and those requests are routed to the
    /// virtual node or virtual router that is specified as the provider for the virtual
    /// service.&lt;/p&gt;
    /// </code></pre>
    async fn create_virtual_service(
        &self,
        input: CreateVirtualServiceInput,
    ) -> Result<CreateVirtualServiceOutput, RusotoError<CreateVirtualServiceError>>;

    /// <p>Deletes an existing service mesh.</p>
    ///
    /// <pre><code>     &lt;p&gt;You must delete all resources (virtual services, routes, virtual routers, and virtual
    /// nodes) in the service mesh before you can delete the mesh itself.&lt;/p&gt;
    /// </code></pre>
    async fn delete_mesh(
        &self,
        input: DeleteMeshInput,
    ) -> Result<DeleteMeshOutput, RusotoError<DeleteMeshError>>;

    /// <p>Deletes an existing route.</p>
    async fn delete_route(
        &self,
        input: DeleteRouteInput,
    ) -> Result<DeleteRouteOutput, RusotoError<DeleteRouteError>>;

    /// <p>Deletes an existing virtual node.</p>
    ///
    /// <pre><code>     &lt;p&gt;You must delete any virtual services that list a virtual node as a service provider
    /// before you can delete the virtual node itself.&lt;/p&gt;
    /// </code></pre>
    async fn delete_virtual_node(
        &self,
        input: DeleteVirtualNodeInput,
    ) -> Result<DeleteVirtualNodeOutput, RusotoError<DeleteVirtualNodeError>>;

    /// <p>Deletes an existing virtual router.</p>
    ///
    /// <pre><code>     &lt;p&gt;You must delete any routes associated with the virtual router before you can delete the
    /// router itself.&lt;/p&gt;
    /// </code></pre>
    async fn delete_virtual_router(
        &self,
        input: DeleteVirtualRouterInput,
    ) -> Result<DeleteVirtualRouterOutput, RusotoError<DeleteVirtualRouterError>>;

    /// <p>Deletes an existing virtual service.</p>
    async fn delete_virtual_service(
        &self,
        input: DeleteVirtualServiceInput,
    ) -> Result<DeleteVirtualServiceOutput, RusotoError<DeleteVirtualServiceError>>;

    /// <p>Describes an existing service mesh.</p>
    async fn describe_mesh(
        &self,
        input: DescribeMeshInput,
    ) -> Result<DescribeMeshOutput, RusotoError<DescribeMeshError>>;

    /// <p>Describes an existing route.</p>
    async fn describe_route(
        &self,
        input: DescribeRouteInput,
    ) -> Result<DescribeRouteOutput, RusotoError<DescribeRouteError>>;

    /// <p>Describes an existing virtual node.</p>
    async fn describe_virtual_node(
        &self,
        input: DescribeVirtualNodeInput,
    ) -> Result<DescribeVirtualNodeOutput, RusotoError<DescribeVirtualNodeError>>;

    /// <p>Describes an existing virtual router.</p>
    async fn describe_virtual_router(
        &self,
        input: DescribeVirtualRouterInput,
    ) -> Result<DescribeVirtualRouterOutput, RusotoError<DescribeVirtualRouterError>>;

    /// <p>Describes an existing virtual service.</p>
    async fn describe_virtual_service(
        &self,
        input: DescribeVirtualServiceInput,
    ) -> Result<DescribeVirtualServiceOutput, RusotoError<DescribeVirtualServiceError>>;

    /// <p>Returns a list of existing service meshes.</p>
    async fn list_meshes(
        &self,
        input: ListMeshesInput,
    ) -> Result<ListMeshesOutput, RusotoError<ListMeshesError>>;

    /// <p>Returns a list of existing routes in a service mesh.</p>
    async fn list_routes(
        &self,
        input: ListRoutesInput,
    ) -> Result<ListRoutesOutput, RusotoError<ListRoutesError>>;

    /// <p>List the tags for an App Mesh resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceInput,
    ) -> Result<ListTagsForResourceOutput, RusotoError<ListTagsForResourceError>>;

    /// <p>Returns a list of existing virtual nodes.</p>
    async fn list_virtual_nodes(
        &self,
        input: ListVirtualNodesInput,
    ) -> Result<ListVirtualNodesOutput, RusotoError<ListVirtualNodesError>>;

    /// <p>Returns a list of existing virtual routers in a service mesh.</p>
    async fn list_virtual_routers(
        &self,
        input: ListVirtualRoutersInput,
    ) -> Result<ListVirtualRoutersOutput, RusotoError<ListVirtualRoutersError>>;

    /// <p>Returns a list of existing virtual services in a service mesh.</p>
    async fn list_virtual_services(
        &self,
        input: ListVirtualServicesInput,
    ) -> Result<ListVirtualServicesOutput, RusotoError<ListVirtualServicesError>>;

    /// <p>Associates the specified tags to a resource with the specified <code>resourceArn</code>.
    /// If existing tags on a resource aren't specified in the request parameters, they aren't
    /// changed. When a resource is deleted, the tags associated with that resource are also
    /// deleted.</p>
    async fn tag_resource(
        &self,
        input: TagResourceInput,
    ) -> Result<TagResourceOutput, RusotoError<TagResourceError>>;

    /// <p>Deletes specified tags from a resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceInput,
    ) -> Result<UntagResourceOutput, RusotoError<UntagResourceError>>;

    /// <p>Updates an existing service mesh.</p>
    async fn update_mesh(
        &self,
        input: UpdateMeshInput,
    ) -> Result<UpdateMeshOutput, RusotoError<UpdateMeshError>>;

    /// <p>Updates an existing route for a specified service mesh and virtual router.</p>
    async fn update_route(
        &self,
        input: UpdateRouteInput,
    ) -> Result<UpdateRouteOutput, RusotoError<UpdateRouteError>>;

    /// <p>Updates an existing virtual node in a specified service mesh.</p>
    async fn update_virtual_node(
        &self,
        input: UpdateVirtualNodeInput,
    ) -> Result<UpdateVirtualNodeOutput, RusotoError<UpdateVirtualNodeError>>;

    /// <p>Updates an existing virtual router in a specified service mesh.</p>
    async fn update_virtual_router(
        &self,
        input: UpdateVirtualRouterInput,
    ) -> Result<UpdateVirtualRouterOutput, RusotoError<UpdateVirtualRouterError>>;

    /// <p>Updates an existing virtual service in a specified service mesh.</p>
    async fn update_virtual_service(
        &self,
        input: UpdateVirtualServiceInput,
    ) -> Result<UpdateVirtualServiceOutput, RusotoError<UpdateVirtualServiceError>>;
}
/// A client for the AWS App Mesh API.
#[derive(Clone)]
pub struct AppMeshClient {
    client: Client,
    region: region::Region,
}

impl AppMeshClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> AppMeshClient {
        AppMeshClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> AppMeshClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        AppMeshClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> AppMeshClient {
        AppMeshClient { client, region }
    }
}

#[async_trait]
impl AppMesh for AppMeshClient {
    /// <p>Creates a service mesh. A service mesh is a logical boundary for network traffic between
    /// the services that reside within it.</p>
    ///
    /// <pre><code>     &lt;p&gt;After you create your service mesh, you can create virtual services, virtual nodes,
    /// virtual routers, and routes to distribute traffic between the applications in your
    /// mesh.&lt;/p&gt;
    /// </code></pre>
    async fn create_mesh(
        &self,
        input: CreateMeshInput,
    ) -> Result<CreateMeshOutput, RusotoError<CreateMeshError>> {
        let request_uri = "/v20190125/meshes";

        let mut request = SignedRequest::new("PUT", "appmesh", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateMeshOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateMeshError::from_response(response))
        }
    }

    /// <p>Creates a route that is associated with a virtual router.</p>
    ///
    /// <pre><code>     &lt;p&gt;You can use the &lt;code&gt;prefix&lt;/code&gt; parameter in your route specification for path-based
    /// routing of requests. For example, if your virtual service name is
    /// &lt;code&gt;my-service.local&lt;/code&gt; and you want the route to match requests to
    /// &lt;code&gt;my-service.local/metrics&lt;/code&gt;, your prefix should be
    /// &lt;code&gt;/metrics&lt;/code&gt;.&lt;/p&gt;
    /// &lt;p&gt;If your route matches a request, you can distribute traffic to one or more target
    /// virtual nodes with relative weighting.&lt;/p&gt;
    /// </code></pre>
    async fn create_route(
        &self,
        input: CreateRouteInput,
    ) -> Result<CreateRouteOutput, RusotoError<CreateRouteError>> {
        let request_uri = format!(
            "/v20190125/meshes/{mesh_name}/virtualRouter/{virtual_router_name}/routes",
            mesh_name = input.mesh_name,
            virtual_router_name = input.virtual_router_name
        );

        let mut request = SignedRequest::new("PUT", "appmesh", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateRouteOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateRouteError::from_response(response))
        }
    }

    /// <p>Creates a virtual node within a service mesh.</p>
    ///
    /// <pre><code>     &lt;p&gt;A virtual node acts as a logical pointer to a particular task group, such as an Amazon ECS
    /// service or a Kubernetes deployment. When you create a virtual node, you can specify the
    /// service discovery information for your task group.&lt;/p&gt;
    /// &lt;p&gt;Any inbound traffic that your virtual node expects should be specified as a
    /// &lt;code&gt;listener&lt;/code&gt;. Any outbound traffic that your virtual node expects to reach
    /// should be specified as a &lt;code&gt;backend&lt;/code&gt;.&lt;/p&gt;
    /// &lt;p&gt;The response metadata for your new virtual node contains the &lt;code&gt;arn&lt;/code&gt; that is
    /// associated with the virtual node. Set this value (either the full ARN or the truncated
    /// resource name: for example, &lt;code&gt;mesh/default/virtualNode/simpleapp&lt;/code&gt;) as the
    /// &lt;code&gt;APPMESH_VIRTUAL_NODE_NAME&lt;/code&gt; environment variable for your task group&#39;s Envoy
    /// proxy container in your task definition or pod spec. This is then mapped to the
    /// &lt;code&gt;node.id&lt;/code&gt; and &lt;code&gt;node.cluster&lt;/code&gt; Envoy parameters.&lt;/p&gt;
    /// &lt;note&gt;
    /// &lt;p&gt;If you require your Envoy stats or tracing to use a different name, you can override
    /// the &lt;code&gt;node.cluster&lt;/code&gt; value that is set by
    /// &lt;code&gt;APPMESH_VIRTUAL_NODE_NAME&lt;/code&gt; with the
    /// &lt;code&gt;APPMESH_VIRTUAL_NODE_CLUSTER&lt;/code&gt; environment variable.&lt;/p&gt;
    /// &lt;/note&gt;
    /// </code></pre>
    async fn create_virtual_node(
        &self,
        input: CreateVirtualNodeInput,
    ) -> Result<CreateVirtualNodeOutput, RusotoError<CreateVirtualNodeError>> {
        let request_uri = format!(
            "/v20190125/meshes/{mesh_name}/virtualNodes",
            mesh_name = input.mesh_name
        );

        let mut request = SignedRequest::new("PUT", "appmesh", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateVirtualNodeOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateVirtualNodeError::from_response(response))
        }
    }

    /// <p>Creates a virtual router within a service mesh.</p>
    ///
    /// <pre><code>     &lt;p&gt;Any inbound traffic that your virtual router expects should be specified as a
    /// &lt;code&gt;listener&lt;/code&gt;. &lt;/p&gt;
    /// &lt;p&gt;Virtual routers handle traffic for one or more virtual services within your mesh. After
    /// you create your virtual router, create and associate routes for your virtual router that
    /// direct incoming requests to different virtual nodes.&lt;/p&gt;
    /// </code></pre>
    async fn create_virtual_router(
        &self,
        input: CreateVirtualRouterInput,
    ) -> Result<CreateVirtualRouterOutput, RusotoError<CreateVirtualRouterError>> {
        let request_uri = format!(
            "/v20190125/meshes/{mesh_name}/virtualRouters",
            mesh_name = input.mesh_name
        );

        let mut request = SignedRequest::new("PUT", "appmesh", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateVirtualRouterOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateVirtualRouterError::from_response(response))
        }
    }

    /// <p>Creates a virtual service within a service mesh.</p>
    ///
    /// <pre><code>     &lt;p&gt;A virtual service is an abstraction of a real service that is provided by a virtual node
    /// directly or indirectly by means of a virtual router. Dependent services call your virtual
    /// service by its &lt;code&gt;virtualServiceName&lt;/code&gt;, and those requests are routed to the
    /// virtual node or virtual router that is specified as the provider for the virtual
    /// service.&lt;/p&gt;
    /// </code></pre>
    async fn create_virtual_service(
        &self,
        input: CreateVirtualServiceInput,
    ) -> Result<CreateVirtualServiceOutput, RusotoError<CreateVirtualServiceError>> {
        let request_uri = format!(
            "/v20190125/meshes/{mesh_name}/virtualServices",
            mesh_name = input.mesh_name
        );

        let mut request = SignedRequest::new("PUT", "appmesh", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateVirtualServiceOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateVirtualServiceError::from_response(response))
        }
    }

    /// <p>Deletes an existing service mesh.</p>
    ///
    /// <pre><code>     &lt;p&gt;You must delete all resources (virtual services, routes, virtual routers, and virtual
    /// nodes) in the service mesh before you can delete the mesh itself.&lt;/p&gt;
    /// </code></pre>
    async fn delete_mesh(
        &self,
        input: DeleteMeshInput,
    ) -> Result<DeleteMeshOutput, RusotoError<DeleteMeshError>> {
        let request_uri = format!("/v20190125/meshes/{mesh_name}", mesh_name = input.mesh_name);

        let mut request = SignedRequest::new("DELETE", "appmesh", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteMeshOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteMeshError::from_response(response))
        }
    }

    /// <p>Deletes an existing route.</p>
    async fn delete_route(
        &self,
        input: DeleteRouteInput,
    ) -> Result<DeleteRouteOutput, RusotoError<DeleteRouteError>> {
        let request_uri = format!(
            "/v20190125/meshes/{mesh_name}/virtualRouter/{virtual_router_name}/routes/{route_name}",
            mesh_name = input.mesh_name,
            route_name = input.route_name,
            virtual_router_name = input.virtual_router_name
        );

        let mut request = SignedRequest::new("DELETE", "appmesh", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteRouteOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteRouteError::from_response(response))
        }
    }

    /// <p>Deletes an existing virtual node.</p>
    ///
    /// <pre><code>     &lt;p&gt;You must delete any virtual services that list a virtual node as a service provider
    /// before you can delete the virtual node itself.&lt;/p&gt;
    /// </code></pre>
    async fn delete_virtual_node(
        &self,
        input: DeleteVirtualNodeInput,
    ) -> Result<DeleteVirtualNodeOutput, RusotoError<DeleteVirtualNodeError>> {
        let request_uri = format!(
            "/v20190125/meshes/{mesh_name}/virtualNodes/{virtual_node_name}",
            mesh_name = input.mesh_name,
            virtual_node_name = input.virtual_node_name
        );

        let mut request = SignedRequest::new("DELETE", "appmesh", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteVirtualNodeOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteVirtualNodeError::from_response(response))
        }
    }

    /// <p>Deletes an existing virtual router.</p>
    ///
    /// <pre><code>     &lt;p&gt;You must delete any routes associated with the virtual router before you can delete the
    /// router itself.&lt;/p&gt;
    /// </code></pre>
    async fn delete_virtual_router(
        &self,
        input: DeleteVirtualRouterInput,
    ) -> Result<DeleteVirtualRouterOutput, RusotoError<DeleteVirtualRouterError>> {
        let request_uri = format!(
            "/v20190125/meshes/{mesh_name}/virtualRouters/{virtual_router_name}",
            mesh_name = input.mesh_name,
            virtual_router_name = input.virtual_router_name
        );

        let mut request = SignedRequest::new("DELETE", "appmesh", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteVirtualRouterOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteVirtualRouterError::from_response(response))
        }
    }

    /// <p>Deletes an existing virtual service.</p>
    async fn delete_virtual_service(
        &self,
        input: DeleteVirtualServiceInput,
    ) -> Result<DeleteVirtualServiceOutput, RusotoError<DeleteVirtualServiceError>> {
        let request_uri = format!(
            "/v20190125/meshes/{mesh_name}/virtualServices/{virtual_service_name}",
            mesh_name = input.mesh_name,
            virtual_service_name = input.virtual_service_name
        );

        let mut request = SignedRequest::new("DELETE", "appmesh", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteVirtualServiceOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteVirtualServiceError::from_response(response))
        }
    }

    /// <p>Describes an existing service mesh.</p>
    async fn describe_mesh(
        &self,
        input: DescribeMeshInput,
    ) -> Result<DescribeMeshOutput, RusotoError<DescribeMeshError>> {
        let request_uri = format!("/v20190125/meshes/{mesh_name}", mesh_name = input.mesh_name);

        let mut request = SignedRequest::new("GET", "appmesh", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeMeshOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeMeshError::from_response(response))
        }
    }

    /// <p>Describes an existing route.</p>
    async fn describe_route(
        &self,
        input: DescribeRouteInput,
    ) -> Result<DescribeRouteOutput, RusotoError<DescribeRouteError>> {
        let request_uri = format!(
            "/v20190125/meshes/{mesh_name}/virtualRouter/{virtual_router_name}/routes/{route_name}",
            mesh_name = input.mesh_name,
            route_name = input.route_name,
            virtual_router_name = input.virtual_router_name
        );

        let mut request = SignedRequest::new("GET", "appmesh", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeRouteOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeRouteError::from_response(response))
        }
    }

    /// <p>Describes an existing virtual node.</p>
    async fn describe_virtual_node(
        &self,
        input: DescribeVirtualNodeInput,
    ) -> Result<DescribeVirtualNodeOutput, RusotoError<DescribeVirtualNodeError>> {
        let request_uri = format!(
            "/v20190125/meshes/{mesh_name}/virtualNodes/{virtual_node_name}",
            mesh_name = input.mesh_name,
            virtual_node_name = input.virtual_node_name
        );

        let mut request = SignedRequest::new("GET", "appmesh", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeVirtualNodeOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeVirtualNodeError::from_response(response))
        }
    }

    /// <p>Describes an existing virtual router.</p>
    async fn describe_virtual_router(
        &self,
        input: DescribeVirtualRouterInput,
    ) -> Result<DescribeVirtualRouterOutput, RusotoError<DescribeVirtualRouterError>> {
        let request_uri = format!(
            "/v20190125/meshes/{mesh_name}/virtualRouters/{virtual_router_name}",
            mesh_name = input.mesh_name,
            virtual_router_name = input.virtual_router_name
        );

        let mut request = SignedRequest::new("GET", "appmesh", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeVirtualRouterOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeVirtualRouterError::from_response(response))
        }
    }

    /// <p>Describes an existing virtual service.</p>
    async fn describe_virtual_service(
        &self,
        input: DescribeVirtualServiceInput,
    ) -> Result<DescribeVirtualServiceOutput, RusotoError<DescribeVirtualServiceError>> {
        let request_uri = format!(
            "/v20190125/meshes/{mesh_name}/virtualServices/{virtual_service_name}",
            mesh_name = input.mesh_name,
            virtual_service_name = input.virtual_service_name
        );

        let mut request = SignedRequest::new("GET", "appmesh", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeVirtualServiceOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeVirtualServiceError::from_response(response))
        }
    }

    /// <p>Returns a list of existing service meshes.</p>
    async fn list_meshes(
        &self,
        input: ListMeshesInput,
    ) -> Result<ListMeshesOutput, RusotoError<ListMeshesError>> {
        let request_uri = "/v20190125/meshes";

        let mut request = SignedRequest::new("GET", "appmesh", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.limit {
            params.put("limit", x);
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
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListMeshesOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListMeshesError::from_response(response))
        }
    }

    /// <p>Returns a list of existing routes in a service mesh.</p>
    async fn list_routes(
        &self,
        input: ListRoutesInput,
    ) -> Result<ListRoutesOutput, RusotoError<ListRoutesError>> {
        let request_uri = format!(
            "/v20190125/meshes/{mesh_name}/virtualRouter/{virtual_router_name}/routes",
            mesh_name = input.mesh_name,
            virtual_router_name = input.virtual_router_name
        );

        let mut request = SignedRequest::new("GET", "appmesh", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.limit {
            params.put("limit", x);
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
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListRoutesOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListRoutesError::from_response(response))
        }
    }

    /// <p>List the tags for an App Mesh resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceInput,
    ) -> Result<ListTagsForResourceOutput, RusotoError<ListTagsForResourceError>> {
        let request_uri = "/v20190125/tags";

        let mut request = SignedRequest::new("GET", "appmesh", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.limit {
            params.put("limit", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        params.put("resourceArn", &input.resource_arn);
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListTagsForResourceOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListTagsForResourceError::from_response(response))
        }
    }

    /// <p>Returns a list of existing virtual nodes.</p>
    async fn list_virtual_nodes(
        &self,
        input: ListVirtualNodesInput,
    ) -> Result<ListVirtualNodesOutput, RusotoError<ListVirtualNodesError>> {
        let request_uri = format!(
            "/v20190125/meshes/{mesh_name}/virtualNodes",
            mesh_name = input.mesh_name
        );

        let mut request = SignedRequest::new("GET", "appmesh", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.limit {
            params.put("limit", x);
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
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListVirtualNodesOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListVirtualNodesError::from_response(response))
        }
    }

    /// <p>Returns a list of existing virtual routers in a service mesh.</p>
    async fn list_virtual_routers(
        &self,
        input: ListVirtualRoutersInput,
    ) -> Result<ListVirtualRoutersOutput, RusotoError<ListVirtualRoutersError>> {
        let request_uri = format!(
            "/v20190125/meshes/{mesh_name}/virtualRouters",
            mesh_name = input.mesh_name
        );

        let mut request = SignedRequest::new("GET", "appmesh", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.limit {
            params.put("limit", x);
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
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListVirtualRoutersOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListVirtualRoutersError::from_response(response))
        }
    }

    /// <p>Returns a list of existing virtual services in a service mesh.</p>
    async fn list_virtual_services(
        &self,
        input: ListVirtualServicesInput,
    ) -> Result<ListVirtualServicesOutput, RusotoError<ListVirtualServicesError>> {
        let request_uri = format!(
            "/v20190125/meshes/{mesh_name}/virtualServices",
            mesh_name = input.mesh_name
        );

        let mut request = SignedRequest::new("GET", "appmesh", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.limit {
            params.put("limit", x);
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
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListVirtualServicesOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListVirtualServicesError::from_response(response))
        }
    }

    /// <p>Associates the specified tags to a resource with the specified <code>resourceArn</code>.
    /// If existing tags on a resource aren't specified in the request parameters, they aren't
    /// changed. When a resource is deleted, the tags associated with that resource are also
    /// deleted.</p>
    async fn tag_resource(
        &self,
        input: TagResourceInput,
    ) -> Result<TagResourceOutput, RusotoError<TagResourceError>> {
        let request_uri = "/v20190125/tag";

        let mut request = SignedRequest::new("PUT", "appmesh", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        params.put("resourceArn", &input.resource_arn);
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<TagResourceOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(TagResourceError::from_response(response))
        }
    }

    /// <p>Deletes specified tags from a resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceInput,
    ) -> Result<UntagResourceOutput, RusotoError<UntagResourceError>> {
        let request_uri = "/v20190125/untag";

        let mut request = SignedRequest::new("PUT", "appmesh", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        params.put("resourceArn", &input.resource_arn);
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UntagResourceOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UntagResourceError::from_response(response))
        }
    }

    /// <p>Updates an existing service mesh.</p>
    async fn update_mesh(
        &self,
        input: UpdateMeshInput,
    ) -> Result<UpdateMeshOutput, RusotoError<UpdateMeshError>> {
        let request_uri = format!("/v20190125/meshes/{mesh_name}", mesh_name = input.mesh_name);

        let mut request = SignedRequest::new("PUT", "appmesh", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateMeshOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateMeshError::from_response(response))
        }
    }

    /// <p>Updates an existing route for a specified service mesh and virtual router.</p>
    async fn update_route(
        &self,
        input: UpdateRouteInput,
    ) -> Result<UpdateRouteOutput, RusotoError<UpdateRouteError>> {
        let request_uri = format!(
            "/v20190125/meshes/{mesh_name}/virtualRouter/{virtual_router_name}/routes/{route_name}",
            mesh_name = input.mesh_name,
            route_name = input.route_name,
            virtual_router_name = input.virtual_router_name
        );

        let mut request = SignedRequest::new("PUT", "appmesh", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateRouteOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateRouteError::from_response(response))
        }
    }

    /// <p>Updates an existing virtual node in a specified service mesh.</p>
    async fn update_virtual_node(
        &self,
        input: UpdateVirtualNodeInput,
    ) -> Result<UpdateVirtualNodeOutput, RusotoError<UpdateVirtualNodeError>> {
        let request_uri = format!(
            "/v20190125/meshes/{mesh_name}/virtualNodes/{virtual_node_name}",
            mesh_name = input.mesh_name,
            virtual_node_name = input.virtual_node_name
        );

        let mut request = SignedRequest::new("PUT", "appmesh", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateVirtualNodeOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateVirtualNodeError::from_response(response))
        }
    }

    /// <p>Updates an existing virtual router in a specified service mesh.</p>
    async fn update_virtual_router(
        &self,
        input: UpdateVirtualRouterInput,
    ) -> Result<UpdateVirtualRouterOutput, RusotoError<UpdateVirtualRouterError>> {
        let request_uri = format!(
            "/v20190125/meshes/{mesh_name}/virtualRouters/{virtual_router_name}",
            mesh_name = input.mesh_name,
            virtual_router_name = input.virtual_router_name
        );

        let mut request = SignedRequest::new("PUT", "appmesh", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateVirtualRouterOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateVirtualRouterError::from_response(response))
        }
    }

    /// <p>Updates an existing virtual service in a specified service mesh.</p>
    async fn update_virtual_service(
        &self,
        input: UpdateVirtualServiceInput,
    ) -> Result<UpdateVirtualServiceOutput, RusotoError<UpdateVirtualServiceError>> {
        let request_uri = format!(
            "/v20190125/meshes/{mesh_name}/virtualServices/{virtual_service_name}",
            mesh_name = input.mesh_name,
            virtual_service_name = input.virtual_service_name
        );

        let mut request = SignedRequest::new("PUT", "appmesh", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateVirtualServiceOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateVirtualServiceError::from_response(response))
        }
    }
}
