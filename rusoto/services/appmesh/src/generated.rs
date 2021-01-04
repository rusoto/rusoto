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
/// <p>An object that represents the access logging information for a virtual node.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AccessLog {
    /// <p>The file object to send virtual node access logs to.</p>
    #[serde(rename = "file")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<FileAccessLog>,
}

/// <p><p>An object that represents the AWS Cloud Map attribute information for your virtual node.</p> <note> <p>AWS Cloud Map is not available in the eu-south-1 Region.</p> </note></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsCloudMapInstanceAttribute {
    /// <p>The name of an AWS Cloud Map service instance attribute key. Any AWS Cloud Map service instance that contains the specified key and value is returned.</p>
    #[serde(rename = "key")]
    pub key: String,
    /// <p>The value of an AWS Cloud Map service instance attribute key. Any AWS Cloud Map service instance that contains the specified key and value is returned.</p>
    #[serde(rename = "value")]
    pub value: String,
}

/// <p><p>An object that represents the AWS Cloud Map service discovery information for your virtual node.</p> <note> <p>AWS Cloud Map is not available in the eu-south-1 Region.</p> </note></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsCloudMapServiceDiscovery {
    /// <p>A string map that contains attributes with values that you can use to filter instances by any custom attribute that you specified when you registered the instance. Only instances that match all of the specified key/value pairs will be returned.</p>
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

/// <p>An object that represents the backends that a virtual node is expected to send outbound traffic to.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Backend {
    /// <p>Specifies a virtual service to use as a backend. </p>
    #[serde(rename = "virtualService")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_service: Option<VirtualServiceBackend>,
}

/// <p>An object that represents the default properties for a backend.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct BackendDefaults {
    /// <p>A reference to an object that represents a client policy.</p>
    #[serde(rename = "clientPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_policy: Option<ClientPolicy>,
}

/// <p>An object that represents a client policy.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ClientPolicy {
    /// <p>A reference to an object that represents a Transport Layer Security (TLS) client policy.</p>
    #[serde(rename = "tls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<ClientPolicyTls>,
}

/// <p>A reference to an object that represents a Transport Layer Security (TLS) client policy.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ClientPolicyTls {
    /// <p>Whether the policy is enforced. The default is <code>True</code>, if a value isn't specified.</p>
    #[serde(rename = "enforce")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforce: Option<bool>,
    /// <p>One or more ports that the policy is enforced for.</p>
    #[serde(rename = "ports")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<i64>>,
    /// <p>A reference to an object that represents a TLS validation context.</p>
    #[serde(rename = "validation")]
    pub validation: TlsValidationContext,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateGatewayRouteInput {
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. Up to 36 letters, numbers, hyphens, and underscores are allowed.</p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The name to use for the gateway route.</p>
    #[serde(rename = "gatewayRouteName")]
    pub gateway_route_name: String,
    /// <p>The name of the service mesh to create the gateway route in.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The AWS IAM account ID of the service mesh owner. If the account ID is not your own, then the account that you specify must share the mesh with your account before you can create the resource in the service mesh. For more information about mesh sharing, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/sharing.html">Working with shared meshes</a>.</p>
    #[serde(rename = "meshOwner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    /// <p>The gateway route specification to apply.</p>
    #[serde(rename = "spec")]
    pub spec: GatewayRouteSpec,
    /// <p>Optional metadata that you can apply to the gateway route to assist with categorization and organization. Each tag consists of a key and an optional value, both of which you define. Tag keys can have a maximum character length of 128 characters, and tag values can have a maximum length of 256 characters.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagRef>>,
    /// <p>The name of the virtual gateway to associate the gateway route with. If the virtual gateway is in a shared mesh, then you must be the owner of the virtual gateway resource.</p>
    #[serde(rename = "virtualGatewayName")]
    pub virtual_gateway_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateGatewayRouteOutput {
    /// <p>The full description of your gateway route following the create call.</p>
    #[serde(rename = "gatewayRoute")]
    pub gateway_route: GatewayRouteData,
}

/// <p><zonbook></zonbook><xhtml></xhtml></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateMeshInput {
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. Up to 36 letters, numbers, hyphens, and underscores are allowed.</p>
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
    /// <p>Optional metadata that you can apply to the service mesh to assist with categorization and organization. Each tag consists of a key and an optional value, both of which you define. Tag keys can have a maximum character length of 128 characters, and tag values can have a maximum length of 256 characters.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagRef>>,
}

/// <p><zonbook></zonbook><xhtml></xhtml></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateMeshOutput {
    /// <p>The full description of your service mesh following the create call.</p>
    #[serde(rename = "mesh")]
    pub mesh: MeshData,
}

/// <p><zonbook></zonbook><xhtml></xhtml></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateRouteInput {
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. Up to 36 letters, numbers, hyphens, and underscores are allowed.</p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The name of the service mesh to create the route in.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The AWS IAM account ID of the service mesh owner. If the account ID is not your own, then the account that you specify must share the mesh with your account before you can create the resource in the service mesh. For more information about mesh sharing, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/sharing.html">Working with shared meshes</a>.</p>
    #[serde(rename = "meshOwner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    /// <p>The name to use for the route.</p>
    #[serde(rename = "routeName")]
    pub route_name: String,
    /// <p>The route specification to apply.</p>
    #[serde(rename = "spec")]
    pub spec: RouteSpec,
    /// <p>Optional metadata that you can apply to the route to assist with categorization and organization. Each tag consists of a key and an optional value, both of which you define. Tag keys can have a maximum character length of 128 characters, and tag values can have a maximum length of 256 characters.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagRef>>,
    /// <p>The name of the virtual router in which to create the route. If the virtual router is in a shared mesh, then you must be the owner of the virtual router resource.</p>
    #[serde(rename = "virtualRouterName")]
    pub virtual_router_name: String,
}

/// <p><zonbook></zonbook><xhtml></xhtml></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateRouteOutput {
    /// <p>The full description of your mesh following the create call.</p>
    #[serde(rename = "route")]
    pub route: RouteData,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateVirtualGatewayInput {
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. Up to 36 letters, numbers, hyphens, and underscores are allowed.</p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The name of the service mesh to create the virtual gateway in.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The AWS IAM account ID of the service mesh owner. If the account ID is not your own, then the account that you specify must share the mesh with your account before you can create the resource in the service mesh. For more information about mesh sharing, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/sharing.html">Working with shared meshes</a>.</p>
    #[serde(rename = "meshOwner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    /// <p>The virtual gateway specification to apply.</p>
    #[serde(rename = "spec")]
    pub spec: VirtualGatewaySpec,
    /// <p>Optional metadata that you can apply to the virtual gateway to assist with categorization and organization. Each tag consists of a key and an optional value, both of which you define. Tag keys can have a maximum character length of 128 characters, and tag values can have a maximum length of 256 characters.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagRef>>,
    /// <p>The name to use for the virtual gateway.</p>
    #[serde(rename = "virtualGatewayName")]
    pub virtual_gateway_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateVirtualGatewayOutput {
    /// <p>The full description of your virtual gateway following the create call.</p>
    #[serde(rename = "virtualGateway")]
    pub virtual_gateway: VirtualGatewayData,
}

/// <p><zonbook></zonbook><xhtml></xhtml></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateVirtualNodeInput {
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. Up to 36 letters, numbers, hyphens, and underscores are allowed.</p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The name of the service mesh to create the virtual node in.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The AWS IAM account ID of the service mesh owner. If the account ID is not your own, then the account that you specify must share the mesh with your account before you can create the resource in the service mesh. For more information about mesh sharing, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/sharing.html">Working with shared meshes</a>.</p>
    #[serde(rename = "meshOwner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    /// <p>The virtual node specification to apply.</p>
    #[serde(rename = "spec")]
    pub spec: VirtualNodeSpec,
    /// <p>Optional metadata that you can apply to the virtual node to assist with categorization and organization. Each tag consists of a key and an optional value, both of which you define. Tag keys can have a maximum character length of 128 characters, and tag values can have a maximum length of 256 characters.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagRef>>,
    /// <p>The name to use for the virtual node.</p>
    #[serde(rename = "virtualNodeName")]
    pub virtual_node_name: String,
}

/// <p><zonbook></zonbook><xhtml></xhtml></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateVirtualNodeOutput {
    /// <p>The full description of your virtual node following the create call.</p>
    #[serde(rename = "virtualNode")]
    pub virtual_node: VirtualNodeData,
}

/// <p><zonbook></zonbook><xhtml></xhtml></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateVirtualRouterInput {
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. Up to 36 letters, numbers, hyphens, and underscores are allowed.</p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The name of the service mesh to create the virtual router in.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The AWS IAM account ID of the service mesh owner. If the account ID is not your own, then the account that you specify must share the mesh with your account before you can create the resource in the service mesh. For more information about mesh sharing, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/sharing.html">Working with shared meshes</a>.</p>
    #[serde(rename = "meshOwner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    /// <p>The virtual router specification to apply.</p>
    #[serde(rename = "spec")]
    pub spec: VirtualRouterSpec,
    /// <p>Optional metadata that you can apply to the virtual router to assist with categorization and organization. Each tag consists of a key and an optional value, both of which you define. Tag keys can have a maximum character length of 128 characters, and tag values can have a maximum length of 256 characters.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagRef>>,
    /// <p>The name to use for the virtual router.</p>
    #[serde(rename = "virtualRouterName")]
    pub virtual_router_name: String,
}

/// <p><zonbook></zonbook><xhtml></xhtml></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateVirtualRouterOutput {
    /// <p>The full description of your virtual router following the create call.</p>
    #[serde(rename = "virtualRouter")]
    pub virtual_router: VirtualRouterData,
}

/// <p><zonbook></zonbook><xhtml></xhtml></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateVirtualServiceInput {
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. Up to 36 letters, numbers, hyphens, and underscores are allowed.</p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The name of the service mesh to create the virtual service in.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The AWS IAM account ID of the service mesh owner. If the account ID is not your own, then the account that you specify must share the mesh with your account before you can create the resource in the service mesh. For more information about mesh sharing, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/sharing.html">Working with shared meshes</a>.</p>
    #[serde(rename = "meshOwner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    /// <p>The virtual service specification to apply.</p>
    #[serde(rename = "spec")]
    pub spec: VirtualServiceSpec,
    /// <p>Optional metadata that you can apply to the virtual service to assist with categorization and organization. Each tag consists of a key and an optional value, both of which you define. Tag keys can have a maximum character length of 128 characters, and tag values can have a maximum length of 256 characters.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagRef>>,
    /// <p>The name to use for the virtual service.</p>
    #[serde(rename = "virtualServiceName")]
    pub virtual_service_name: String,
}

/// <p><zonbook></zonbook><xhtml></xhtml></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateVirtualServiceOutput {
    /// <p>The full description of your virtual service following the create call.</p>
    #[serde(rename = "virtualService")]
    pub virtual_service: VirtualServiceData,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteGatewayRouteInput {
    /// <p>The name of the gateway route to delete.</p>
    #[serde(rename = "gatewayRouteName")]
    pub gateway_route_name: String,
    /// <p>The name of the service mesh to delete the gateway route from.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The AWS IAM account ID of the service mesh owner. If the account ID is not your own, then it's the ID of the account that shared the mesh with your account. For more information about mesh sharing, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/sharing.html">Working with shared meshes</a>.</p>
    #[serde(rename = "meshOwner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    /// <p>The name of the virtual gateway to delete the route from.</p>
    #[serde(rename = "virtualGatewayName")]
    pub virtual_gateway_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteGatewayRouteOutput {
    /// <p>The gateway route that was deleted.</p>
    #[serde(rename = "gatewayRoute")]
    pub gateway_route: GatewayRouteData,
}

/// <p><zonbook></zonbook><xhtml></xhtml></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteMeshInput {
    /// <p>The name of the service mesh to delete.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
}

/// <p><zonbook></zonbook><xhtml></xhtml></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteMeshOutput {
    /// <p>The service mesh that was deleted.</p>
    #[serde(rename = "mesh")]
    pub mesh: MeshData,
}

/// <p><zonbook></zonbook><xhtml></xhtml></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteRouteInput {
    /// <p>The name of the service mesh to delete the route in.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The AWS IAM account ID of the service mesh owner. If the account ID is not your own, then it's the ID of the account that shared the mesh with your account. For more information about mesh sharing, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/sharing.html">Working with shared meshes</a>.</p>
    #[serde(rename = "meshOwner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    /// <p>The name of the route to delete.</p>
    #[serde(rename = "routeName")]
    pub route_name: String,
    /// <p>The name of the virtual router to delete the route in.</p>
    #[serde(rename = "virtualRouterName")]
    pub virtual_router_name: String,
}

/// <p><zonbook></zonbook><xhtml></xhtml></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteRouteOutput {
    /// <p>The route that was deleted.</p>
    #[serde(rename = "route")]
    pub route: RouteData,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteVirtualGatewayInput {
    /// <p>The name of the service mesh to delete the virtual gateway from.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The AWS IAM account ID of the service mesh owner. If the account ID is not your own, then it's the ID of the account that shared the mesh with your account. For more information about mesh sharing, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/sharing.html">Working with shared meshes</a>.</p>
    #[serde(rename = "meshOwner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    /// <p>The name of the virtual gateway to delete.</p>
    #[serde(rename = "virtualGatewayName")]
    pub virtual_gateway_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteVirtualGatewayOutput {
    /// <p>The virtual gateway that was deleted.</p>
    #[serde(rename = "virtualGateway")]
    pub virtual_gateway: VirtualGatewayData,
}

/// <p><zonbook></zonbook><xhtml></xhtml></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteVirtualNodeInput {
    /// <p>The name of the service mesh to delete the virtual node in.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The AWS IAM account ID of the service mesh owner. If the account ID is not your own, then it's the ID of the account that shared the mesh with your account. For more information about mesh sharing, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/sharing.html">Working with shared meshes</a>.</p>
    #[serde(rename = "meshOwner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    /// <p>The name of the virtual node to delete.</p>
    #[serde(rename = "virtualNodeName")]
    pub virtual_node_name: String,
}

/// <p><zonbook></zonbook><xhtml></xhtml></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteVirtualNodeOutput {
    /// <p>The virtual node that was deleted.</p>
    #[serde(rename = "virtualNode")]
    pub virtual_node: VirtualNodeData,
}

/// <p><zonbook></zonbook><xhtml></xhtml></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteVirtualRouterInput {
    /// <p>The name of the service mesh to delete the virtual router in.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The AWS IAM account ID of the service mesh owner. If the account ID is not your own, then it's the ID of the account that shared the mesh with your account. For more information about mesh sharing, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/sharing.html">Working with shared meshes</a>.</p>
    #[serde(rename = "meshOwner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    /// <p>The name of the virtual router to delete.</p>
    #[serde(rename = "virtualRouterName")]
    pub virtual_router_name: String,
}

/// <p><zonbook></zonbook><xhtml></xhtml></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteVirtualRouterOutput {
    /// <p>The virtual router that was deleted.</p>
    #[serde(rename = "virtualRouter")]
    pub virtual_router: VirtualRouterData,
}

/// <p><zonbook></zonbook><xhtml></xhtml></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteVirtualServiceInput {
    /// <p>The name of the service mesh to delete the virtual service in.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The AWS IAM account ID of the service mesh owner. If the account ID is not your own, then it's the ID of the account that shared the mesh with your account. For more information about mesh sharing, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/sharing.html">Working with shared meshes</a>.</p>
    #[serde(rename = "meshOwner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    /// <p>The name of the virtual service to delete.</p>
    #[serde(rename = "virtualServiceName")]
    pub virtual_service_name: String,
}

/// <p><zonbook></zonbook><xhtml></xhtml></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteVirtualServiceOutput {
    /// <p>The virtual service that was deleted.</p>
    #[serde(rename = "virtualService")]
    pub virtual_service: VirtualServiceData,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeGatewayRouteInput {
    /// <p>The name of the gateway route to describe.</p>
    #[serde(rename = "gatewayRouteName")]
    pub gateway_route_name: String,
    /// <p>The name of the service mesh that the gateway route resides in.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The AWS IAM account ID of the service mesh owner. If the account ID is not your own, then it's the ID of the account that shared the mesh with your account. For more information about mesh sharing, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/sharing.html">Working with shared meshes</a>.</p>
    #[serde(rename = "meshOwner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    /// <p>The name of the virtual gateway that the gateway route is associated with.</p>
    #[serde(rename = "virtualGatewayName")]
    pub virtual_gateway_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeGatewayRouteOutput {
    /// <p>The full description of your gateway route.</p>
    #[serde(rename = "gatewayRoute")]
    pub gateway_route: GatewayRouteData,
}

/// <p><zonbook></zonbook><xhtml></xhtml></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeMeshInput {
    /// <p>The name of the service mesh to describe.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The AWS IAM account ID of the service mesh owner. If the account ID is not your own, then it's the ID of the account that shared the mesh with your account. For more information about mesh sharing, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/sharing.html">Working with shared meshes</a>.</p>
    #[serde(rename = "meshOwner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
}

/// <p><zonbook></zonbook><xhtml></xhtml></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeMeshOutput {
    /// <p>The full description of your service mesh.</p>
    #[serde(rename = "mesh")]
    pub mesh: MeshData,
}

/// <p><zonbook></zonbook><xhtml></xhtml></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeRouteInput {
    /// <p>The name of the service mesh that the route resides in.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The AWS IAM account ID of the service mesh owner. If the account ID is not your own, then it's the ID of the account that shared the mesh with your account. For more information about mesh sharing, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/sharing.html">Working with shared meshes</a>.</p>
    #[serde(rename = "meshOwner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    /// <p>The name of the route to describe.</p>
    #[serde(rename = "routeName")]
    pub route_name: String,
    /// <p>The name of the virtual router that the route is associated with.</p>
    #[serde(rename = "virtualRouterName")]
    pub virtual_router_name: String,
}

/// <p><zonbook></zonbook><xhtml></xhtml></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeRouteOutput {
    /// <p>The full description of your route.</p>
    #[serde(rename = "route")]
    pub route: RouteData,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeVirtualGatewayInput {
    /// <p>The name of the service mesh that the gateway route resides in.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The AWS IAM account ID of the service mesh owner. If the account ID is not your own, then it's the ID of the account that shared the mesh with your account. For more information about mesh sharing, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/sharing.html">Working with shared meshes</a>.</p>
    #[serde(rename = "meshOwner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    /// <p>The name of the virtual gateway to describe.</p>
    #[serde(rename = "virtualGatewayName")]
    pub virtual_gateway_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeVirtualGatewayOutput {
    /// <p>The full description of your virtual gateway.</p>
    #[serde(rename = "virtualGateway")]
    pub virtual_gateway: VirtualGatewayData,
}

/// <p><zonbook></zonbook><xhtml></xhtml></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeVirtualNodeInput {
    /// <p>The name of the service mesh that the virtual node resides in.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The AWS IAM account ID of the service mesh owner. If the account ID is not your own, then it's the ID of the account that shared the mesh with your account. For more information about mesh sharing, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/sharing.html">Working with shared meshes</a>.</p>
    #[serde(rename = "meshOwner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    /// <p>The name of the virtual node to describe.</p>
    #[serde(rename = "virtualNodeName")]
    pub virtual_node_name: String,
}

/// <p><zonbook></zonbook><xhtml></xhtml></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeVirtualNodeOutput {
    /// <p>The full description of your virtual node.</p>
    #[serde(rename = "virtualNode")]
    pub virtual_node: VirtualNodeData,
}

/// <p><zonbook></zonbook><xhtml></xhtml></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeVirtualRouterInput {
    /// <p>The name of the service mesh that the virtual router resides in.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The AWS IAM account ID of the service mesh owner. If the account ID is not your own, then it's the ID of the account that shared the mesh with your account. For more information about mesh sharing, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/sharing.html">Working with shared meshes</a>.</p>
    #[serde(rename = "meshOwner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    /// <p>The name of the virtual router to describe.</p>
    #[serde(rename = "virtualRouterName")]
    pub virtual_router_name: String,
}

/// <p><zonbook></zonbook><xhtml></xhtml></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeVirtualRouterOutput {
    /// <p>The full description of your virtual router.</p>
    #[serde(rename = "virtualRouter")]
    pub virtual_router: VirtualRouterData,
}

/// <p><zonbook></zonbook><xhtml></xhtml></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeVirtualServiceInput {
    /// <p>The name of the service mesh that the virtual service resides in.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The AWS IAM account ID of the service mesh owner. If the account ID is not your own, then it's the ID of the account that shared the mesh with your account. For more information about mesh sharing, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/sharing.html">Working with shared meshes</a>.</p>
    #[serde(rename = "meshOwner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    /// <p>The name of the virtual service to describe.</p>
    #[serde(rename = "virtualServiceName")]
    pub virtual_service_name: String,
}

/// <p><zonbook></zonbook><xhtml></xhtml></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeVirtualServiceOutput {
    /// <p>The full description of your virtual service.</p>
    #[serde(rename = "virtualService")]
    pub virtual_service: VirtualServiceData,
}

/// <p>An object that represents the DNS service discovery information for your virtual node.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DnsServiceDiscovery {
    /// <p>Specifies the DNS service discovery hostname for the virtual node. </p>
    #[serde(rename = "hostname")]
    pub hostname: String,
}

/// <p>An object that represents a duration of time.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct EgressFilter {
    /// <p>The egress filter type. By default, the type is <code>DROP_ALL</code>, which allows egress only from virtual nodes to other defined resources in the service mesh (and any traffic to <code>*.amazonaws.com</code> for AWS API calls). You can set the egress filter type to <code>ALLOW_ALL</code> to allow egress to any endpoint inside or outside of the service mesh.</p>
    #[serde(rename = "type")]
    pub type_: String,
}

/// <p>An object that represents an access log file.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct FileAccessLog {
    /// <p><p>The file path to write access logs to. You can use <code>/dev/stdout</code> to send access logs to standard out and configure your Envoy container to use a log driver, such as <code>awslogs</code>, to export the access logs to a log storage service such as Amazon CloudWatch Logs. You can also specify a path in the Envoy container&#39;s file system to write the files to disk.</p> <note> <p>The Envoy process must have write permissions to the path that you specify here. Otherwise, Envoy fails to bootstrap properly.</p> </note></p>
    #[serde(rename = "path")]
    pub path: String,
}

/// <p>An object that represents a gateway route returned by a describe operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GatewayRouteData {
    /// <p>The name of the gateway route.</p>
    #[serde(rename = "gatewayRouteName")]
    pub gateway_route_name: String,
    /// <p>The name of the service mesh that the resource resides in. </p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    #[serde(rename = "metadata")]
    pub metadata: ResourceMetadata,
    /// <p>The specifications of the gateway route.</p>
    #[serde(rename = "spec")]
    pub spec: GatewayRouteSpec,
    /// <p>The status of the gateway route.</p>
    #[serde(rename = "status")]
    pub status: GatewayRouteStatus,
    /// <p>The virtual gateway that the gateway route is associated with.</p>
    #[serde(rename = "virtualGatewayName")]
    pub virtual_gateway_name: String,
}

/// <p>An object that represents a gateway route returned by a list operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GatewayRouteRef {
    /// <p>The full Amazon Resource Name (ARN) for the gateway route.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>The Unix epoch timestamp in seconds for when the resource was created.</p>
    #[serde(rename = "createdAt")]
    pub created_at: f64,
    /// <p>The name of the gateway route.</p>
    #[serde(rename = "gatewayRouteName")]
    pub gateway_route_name: String,
    /// <p>The Unix epoch timestamp in seconds for when the resource was last updated.</p>
    #[serde(rename = "lastUpdatedAt")]
    pub last_updated_at: f64,
    /// <p>The name of the service mesh that the resource resides in. </p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The AWS IAM account ID of the service mesh owner. If the account ID is not your own, then it's the ID of the account that shared the mesh with your account. For more information about mesh sharing, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/sharing.html">Working with shared meshes</a>.</p>
    #[serde(rename = "meshOwner")]
    pub mesh_owner: String,
    /// <p>The AWS IAM account ID of the resource owner. If the account ID is not your own, then it's the ID of the mesh owner or of another account that the mesh is shared with. For more information about mesh sharing, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/sharing.html">Working with shared meshes</a>.</p>
    #[serde(rename = "resourceOwner")]
    pub resource_owner: String,
    /// <p>The version of the resource. Resources are created at version 1, and this version is incremented each time that they're updated.</p>
    #[serde(rename = "version")]
    pub version: i64,
    /// <p>The virtual gateway that the gateway route is associated with.</p>
    #[serde(rename = "virtualGatewayName")]
    pub virtual_gateway_name: String,
}

/// <p>An object that represents a gateway route specification. Specify one gateway route type.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct GatewayRouteSpec {
    /// <p>An object that represents the specification of a gRPC gateway route.</p>
    #[serde(rename = "grpcRoute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grpc_route: Option<GrpcGatewayRoute>,
    /// <p>An object that represents the specification of an HTTP/2 gateway route.</p>
    #[serde(rename = "http2Route")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_2_route: Option<HttpGatewayRoute>,
    /// <p>An object that represents the specification of an HTTP gateway route.</p>
    #[serde(rename = "httpRoute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_route: Option<HttpGatewayRoute>,
}

/// <p>An object that represents the current status of a gateway route.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GatewayRouteStatus {
    /// <p>The current status for the gateway route.</p>
    #[serde(rename = "status")]
    pub status: String,
}

/// <p>An object that represents a gateway route target.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct GatewayRouteTarget {
    /// <p>An object that represents a virtual service gateway route target.</p>
    #[serde(rename = "virtualService")]
    pub virtual_service: GatewayRouteVirtualService,
}

/// <p>An object that represents the virtual service that traffic is routed to.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct GatewayRouteVirtualService {
    /// <p>The name of the virtual service that traffic is routed to.</p>
    #[serde(rename = "virtualServiceName")]
    pub virtual_service_name: String,
}

/// <p>An object that represents a gRPC gateway route.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct GrpcGatewayRoute {
    /// <p>An object that represents the action to take if a match is determined.</p>
    #[serde(rename = "action")]
    pub action: GrpcGatewayRouteAction,
    /// <p>An object that represents the criteria for determining a request match.</p>
    #[serde(rename = "match")]
    pub route_match: Option<GrpcGatewayRouteMatch>,
}

/// <p>An object that represents the action to take if a match is determined.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct GrpcGatewayRouteAction {
    /// <p>An object that represents the target that traffic is routed to when a request matches the gateway route.</p>
    #[serde(rename = "target")]
    pub target: GatewayRouteTarget,
}

/// <p>An object that represents the criteria for determining a request match.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct GrpcGatewayRouteMatch {
    /// <p>The fully qualified domain name for the service to match from the request.</p>
    #[serde(rename = "serviceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
}

/// <p>An object that represents a retry policy. Specify at least one value for at least one of the types of <code>RetryEvents</code>, a value for <code>maxRetries</code>, and a value for <code>perRetryTimeout</code>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct GrpcRetryPolicy {
    /// <p>Specify at least one of the valid values.</p>
    #[serde(rename = "grpcRetryEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grpc_retry_events: Option<Vec<String>>,
    /// <p><p>Specify at least one of the following values.</p> <ul> <li> <p> <b>server-error</b> – HTTP status codes 500, 501, 502, 503, 504, 505, 506, 507, 508, 510, and 511</p> </li> <li> <p> <b>gateway-error</b> – HTTP status codes 502, 503, and 504</p> </li> <li> <p> <b>client-error</b> – HTTP status code 409</p> </li> <li> <p> <b>stream-error</b> – Retry on refused stream</p> </li> </ul></p>
    #[serde(rename = "httpRetryEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_retry_events: Option<Vec<String>>,
    /// <p>The maximum number of retry attempts.</p>
    #[serde(rename = "maxRetries")]
    pub max_retries: i64,
    /// <p>The timeout for each retry attempt.</p>
    #[serde(rename = "perRetryTimeout")]
    pub per_retry_timeout: Duration,
    /// <p>Specify a valid value. The event occurs before any processing of a request has started and is encountered when the upstream is temporarily or permanently unavailable.</p>
    #[serde(rename = "tcpRetryEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_retry_events: Option<Vec<String>>,
}

/// <p>An object that represents a gRPC route type.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
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
    /// <p>An object that represents types of timeouts. </p>
    #[serde(rename = "timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<GrpcTimeout>,
}

/// <p>An object that represents the action to take if a match is determined.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct GrpcRouteAction {
    /// <p>An object that represents the targets that traffic is routed to when a request matches the route.</p>
    #[serde(rename = "weightedTargets")]
    pub weighted_targets: Vec<WeightedTarget>,
}

/// <p>An object that represents the criteria for determining a request match.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
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

/// <p>An object that represents types of timeouts. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct GrpcTimeout {
    /// <p>An object that represents an idle timeout. An idle timeout bounds the amount of time that a connection may be idle. The default value is none.</p>
    #[serde(rename = "idle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle: Option<Duration>,
    /// <p>An object that represents a per request timeout. The default value is 15 seconds. If you set a higher timeout, then make sure that the higher value is set for each App Mesh resource in a conversation. For example, if a virtual node backend uses a virtual router provider to route to another virtual node, then the timeout should be greater than 15 seconds for the source and destination virtual node and the route.</p>
    #[serde(rename = "perRequest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_request: Option<Duration>,
}

/// <p>An object that represents the method and value to match with the header value sent in a request. Specify one match method.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct HealthCheckPolicy {
    /// <p>The number of consecutive successful health checks that must occur before declaring listener healthy.</p>
    #[serde(rename = "healthyThreshold")]
    pub healthy_threshold: i64,
    /// <p>The time period in milliseconds between each health check execution.</p>
    #[serde(rename = "intervalMillis")]
    pub interval_millis: i64,
    /// <p>The destination path for the health check request. This value is only used if the specified protocol is HTTP or HTTP/2. For any other protocol, this value is ignored.</p>
    #[serde(rename = "path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// <p>The destination port for the health check request. This port must match the port defined in the <a>PortMapping</a> for the listener.</p>
    #[serde(rename = "port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    /// <p>The protocol for the health check request. If you specify <code>grpc</code>, then your service must conform to the <a href="https://github.com/grpc/grpc/blob/master/doc/health-checking.md">GRPC Health Checking Protocol</a>.</p>
    #[serde(rename = "protocol")]
    pub protocol: String,
    /// <p>The amount of time to wait when receiving a response from the health check, in milliseconds.</p>
    #[serde(rename = "timeoutMillis")]
    pub timeout_millis: i64,
    /// <p>The number of consecutive failed health checks that must occur before declaring a virtual node unhealthy. </p>
    #[serde(rename = "unhealthyThreshold")]
    pub unhealthy_threshold: i64,
}

/// <p>An object that represents an HTTP gateway route.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct HttpGatewayRoute {
    /// <p>An object that represents the action to take if a match is determined.</p>
    #[serde(rename = "action")]
    pub action: HttpGatewayRouteAction,
    /// <p>An object that represents the criteria for determining a request match.</p>
    #[serde(rename = "match")]
    pub route_match: Option<HttpGatewayRouteMatch>,
}

/// <p>An object that represents the action to take if a match is determined.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct HttpGatewayRouteAction {
    /// <p>An object that represents the target that traffic is routed to when a request matches the gateway route.</p>
    #[serde(rename = "target")]
    pub target: GatewayRouteTarget,
}

/// <p>An object that represents the criteria for determining a request match.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct HttpGatewayRouteMatch {
    /// <p>Specifies the path to match requests with. This parameter must always start with <code>/</code>, which by itself matches all requests to the virtual service name. You can also match for path-based routing of requests. For example, if your virtual service name is <code>my-service.local</code> and you want the route to match requests to <code>my-service.local/metrics</code>, your prefix should be <code>/metrics</code>.</p>
    #[serde(rename = "prefix")]
    pub prefix: String,
}

/// <p>An object that represents a retry policy. Specify at least one value for at least one of the types of <code>RetryEvents</code>, a value for <code>maxRetries</code>, and a value for <code>perRetryTimeout</code>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct HttpRetryPolicy {
    /// <p><p>Specify at least one of the following values.</p> <ul> <li> <p> <b>server-error</b> – HTTP status codes 500, 501, 502, 503, 504, 505, 506, 507, 508, 510, and 511</p> </li> <li> <p> <b>gateway-error</b> – HTTP status codes 502, 503, and 504</p> </li> <li> <p> <b>client-error</b> – HTTP status code 409</p> </li> <li> <p> <b>stream-error</b> – Retry on refused stream</p> </li> </ul></p>
    #[serde(rename = "httpRetryEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_retry_events: Option<Vec<String>>,
    /// <p>The maximum number of retry attempts.</p>
    #[serde(rename = "maxRetries")]
    pub max_retries: i64,
    /// <p>The timeout for each retry attempt.</p>
    #[serde(rename = "perRetryTimeout")]
    pub per_retry_timeout: Duration,
    /// <p>Specify a valid value. The event occurs before any processing of a request has started and is encountered when the upstream is temporarily or permanently unavailable.</p>
    #[serde(rename = "tcpRetryEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_retry_events: Option<Vec<String>>,
}

/// <p>An object that represents an HTTP or HTTP/2 route type.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
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
    /// <p>An object that represents types of timeouts. </p>
    #[serde(rename = "timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<HttpTimeout>,
}

/// <p>An object that represents the action to take if a match is determined.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct HttpRouteAction {
    /// <p>An object that represents the targets that traffic is routed to when a request matches the route.</p>
    #[serde(rename = "weightedTargets")]
    pub weighted_targets: Vec<WeightedTarget>,
}

/// <p>An object that represents the HTTP header in the request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
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

/// <p>An object that represents the requirements for a route to match HTTP requests for a virtual router.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct HttpRouteMatch {
    /// <p>An object that represents the client request headers to match on.</p>
    #[serde(rename = "headers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<HttpRouteHeader>>,
    /// <p>The client request method to match on. Specify only one.</p>
    #[serde(rename = "method")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    /// <p>Specifies the path to match requests with. This parameter must always start with <code>/</code>, which by itself matches all requests to the virtual service name. You can also match for path-based routing of requests. For example, if your virtual service name is <code>my-service.local</code> and you want the route to match requests to <code>my-service.local/metrics</code>, your prefix should be <code>/metrics</code>.</p>
    #[serde(rename = "prefix")]
    pub prefix: String,
    /// <p>The client request scheme to match on. Specify only one.</p>
    #[serde(rename = "scheme")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
}

/// <p>An object that represents types of timeouts. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct HttpTimeout {
    /// <p>An object that represents an idle timeout. An idle timeout bounds the amount of time that a connection may be idle. The default value is none.</p>
    #[serde(rename = "idle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle: Option<Duration>,
    /// <p>An object that represents a per request timeout. The default value is 15 seconds. If you set a higher timeout, then make sure that the higher value is set for each App Mesh resource in a conversation. For example, if a virtual node backend uses a virtual router provider to route to another virtual node, then the timeout should be greater than 15 seconds for the source and destination virtual node and the route.</p>
    #[serde(rename = "perRequest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_request: Option<Duration>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListGatewayRoutesInput {
    /// <p>The maximum number of results returned by <code>ListGatewayRoutes</code> in paginated output. When you use this parameter, <code>ListGatewayRoutes</code> returns only <code>limit</code> results in a single page along with a <code>nextToken</code> response element. You can see the remaining results of the initial request by sending another <code>ListGatewayRoutes</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If you don't use this parameter, <code>ListGatewayRoutes</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The name of the service mesh to list gateway routes in.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The AWS IAM account ID of the service mesh owner. If the account ID is not your own, then it's the ID of the account that shared the mesh with your account. For more information about mesh sharing, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/sharing.html">Working with shared meshes</a>.</p>
    #[serde(rename = "meshOwner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    /// <p>The <code>nextToken</code> value returned from a previous paginated <code>ListGatewayRoutes</code> request where <code>limit</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the virtual gateway to list gateway routes in.</p>
    #[serde(rename = "virtualGatewayName")]
    pub virtual_gateway_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListGatewayRoutesOutput {
    /// <p>The list of existing gateway routes for the specified service mesh and virtual gateway.</p>
    #[serde(rename = "gatewayRoutes")]
    pub gateway_routes: Vec<GatewayRouteRef>,
    /// <p>The <code>nextToken</code> value to include in a future <code>ListGatewayRoutes</code> request. When the results of a <code>ListGatewayRoutes</code> request exceed <code>limit</code>, you can use this value to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p><zonbook></zonbook><xhtml></xhtml></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListMeshesInput {
    /// <p>The maximum number of results returned by <code>ListMeshes</code> in paginated output. When you use this parameter, <code>ListMeshes</code> returns only <code>limit</code> results in a single page along with a <code>nextToken</code> response element. You can see the remaining results of the initial request by sending another <code>ListMeshes</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If you don't use this parameter, <code>ListMeshes</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p><p>The <code>nextToken</code> value returned from a previous paginated <code>ListMeshes</code> request where <code>limit</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value.</p> <note> <p>This token should be treated as an opaque identifier that is used only to retrieve the next items in a list and not for other programmatic purposes.</p> </note></p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p><zonbook></zonbook><xhtml></xhtml></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListMeshesOutput {
    /// <p>The list of existing service meshes.</p>
    #[serde(rename = "meshes")]
    pub meshes: Vec<MeshRef>,
    /// <p>The <code>nextToken</code> value to include in a future <code>ListMeshes</code> request. When the results of a <code>ListMeshes</code> request exceed <code>limit</code>, you can use this value to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p><zonbook></zonbook><xhtml></xhtml></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListRoutesInput {
    /// <p>The maximum number of results returned by <code>ListRoutes</code> in paginated output. When you use this parameter, <code>ListRoutes</code> returns only <code>limit</code> results in a single page along with a <code>nextToken</code> response element. You can see the remaining results of the initial request by sending another <code>ListRoutes</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If you don't use this parameter, <code>ListRoutes</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The name of the service mesh to list routes in.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The AWS IAM account ID of the service mesh owner. If the account ID is not your own, then it's the ID of the account that shared the mesh with your account. For more information about mesh sharing, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/sharing.html">Working with shared meshes</a>.</p>
    #[serde(rename = "meshOwner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    /// <p>The <code>nextToken</code> value returned from a previous paginated <code>ListRoutes</code> request where <code>limit</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the virtual router to list routes in.</p>
    #[serde(rename = "virtualRouterName")]
    pub virtual_router_name: String,
}

/// <p><zonbook></zonbook><xhtml></xhtml></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListRoutesOutput {
    /// <p>The <code>nextToken</code> value to include in a future <code>ListRoutes</code> request. When the results of a <code>ListRoutes</code> request exceed <code>limit</code>, you can use this value to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of existing routes for the specified service mesh and virtual router.</p>
    #[serde(rename = "routes")]
    pub routes: Vec<RouteRef>,
}

/// <p><zonbook></zonbook><xhtml></xhtml></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceInput {
    /// <p>The maximum number of tag results returned by <code>ListTagsForResource</code> in paginated output. When this parameter is used, <code>ListTagsForResource</code> returns only <code>limit</code> results in a single page along with a <code>nextToken</code> response element. You can see the remaining results of the initial request by sending another <code>ListTagsForResource</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If you don't use this parameter, <code>ListTagsForResource</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The <code>nextToken</code> value returned from a previous paginated <code>ListTagsForResource</code> request where <code>limit</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The Amazon Resource Name (ARN) that identifies the resource to list the tags for.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
}

/// <p><zonbook></zonbook><xhtml></xhtml></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceOutput {
    /// <p>The <code>nextToken</code> value to include in a future <code>ListTagsForResource</code> request. When the results of a <code>ListTagsForResource</code> request exceed <code>limit</code>, you can use this value to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The tags for the resource.</p>
    #[serde(rename = "tags")]
    pub tags: Vec<TagRef>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListVirtualGatewaysInput {
    /// <p>The maximum number of results returned by <code>ListVirtualGateways</code> in paginated output. When you use this parameter, <code>ListVirtualGateways</code> returns only <code>limit</code> results in a single page along with a <code>nextToken</code> response element. You can see the remaining results of the initial request by sending another <code>ListVirtualGateways</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If you don't use this parameter, <code>ListVirtualGateways</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The name of the service mesh to list virtual gateways in.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The AWS IAM account ID of the service mesh owner. If the account ID is not your own, then it's the ID of the account that shared the mesh with your account. For more information about mesh sharing, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/sharing.html">Working with shared meshes</a>.</p>
    #[serde(rename = "meshOwner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    /// <p>The <code>nextToken</code> value returned from a previous paginated <code>ListVirtualGateways</code> request where <code>limit</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListVirtualGatewaysOutput {
    /// <p>The <code>nextToken</code> value to include in a future <code>ListVirtualGateways</code> request. When the results of a <code>ListVirtualGateways</code> request exceed <code>limit</code>, you can use this value to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of existing virtual gateways for the specified service mesh.</p>
    #[serde(rename = "virtualGateways")]
    pub virtual_gateways: Vec<VirtualGatewayRef>,
}

/// <p><zonbook></zonbook><xhtml></xhtml></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListVirtualNodesInput {
    /// <p>The maximum number of results returned by <code>ListVirtualNodes</code> in paginated output. When you use this parameter, <code>ListVirtualNodes</code> returns only <code>limit</code> results in a single page along with a <code>nextToken</code> response element. You can see the remaining results of the initial request by sending another <code>ListVirtualNodes</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If you don't use this parameter, <code>ListVirtualNodes</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The name of the service mesh to list virtual nodes in.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The AWS IAM account ID of the service mesh owner. If the account ID is not your own, then it's the ID of the account that shared the mesh with your account. For more information about mesh sharing, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/sharing.html">Working with shared meshes</a>.</p>
    #[serde(rename = "meshOwner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    /// <p>The <code>nextToken</code> value returned from a previous paginated <code>ListVirtualNodes</code> request where <code>limit</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p><zonbook></zonbook><xhtml></xhtml></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListVirtualNodesOutput {
    /// <p>The <code>nextToken</code> value to include in a future <code>ListVirtualNodes</code> request. When the results of a <code>ListVirtualNodes</code> request exceed <code>limit</code>, you can use this value to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of existing virtual nodes for the specified service mesh.</p>
    #[serde(rename = "virtualNodes")]
    pub virtual_nodes: Vec<VirtualNodeRef>,
}

/// <p><zonbook></zonbook><xhtml></xhtml></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListVirtualRoutersInput {
    /// <p>The maximum number of results returned by <code>ListVirtualRouters</code> in paginated output. When you use this parameter, <code>ListVirtualRouters</code> returns only <code>limit</code> results in a single page along with a <code>nextToken</code> response element. You can see the remaining results of the initial request by sending another <code>ListVirtualRouters</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If you don't use this parameter, <code>ListVirtualRouters</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The name of the service mesh to list virtual routers in.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The AWS IAM account ID of the service mesh owner. If the account ID is not your own, then it's the ID of the account that shared the mesh with your account. For more information about mesh sharing, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/sharing.html">Working with shared meshes</a>.</p>
    #[serde(rename = "meshOwner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    /// <p>The <code>nextToken</code> value returned from a previous paginated <code>ListVirtualRouters</code> request where <code>limit</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p><zonbook></zonbook><xhtml></xhtml></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListVirtualRoutersOutput {
    /// <p>The <code>nextToken</code> value to include in a future <code>ListVirtualRouters</code> request. When the results of a <code>ListVirtualRouters</code> request exceed <code>limit</code>, you can use this value to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of existing virtual routers for the specified service mesh.</p>
    #[serde(rename = "virtualRouters")]
    pub virtual_routers: Vec<VirtualRouterRef>,
}

/// <p><zonbook></zonbook><xhtml></xhtml></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListVirtualServicesInput {
    /// <p>The maximum number of results returned by <code>ListVirtualServices</code> in paginated output. When you use this parameter, <code>ListVirtualServices</code> returns only <code>limit</code> results in a single page along with a <code>nextToken</code> response element. You can see the remaining results of the initial request by sending another <code>ListVirtualServices</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If you don't use this parameter, <code>ListVirtualServices</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The name of the service mesh to list virtual services in.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The AWS IAM account ID of the service mesh owner. If the account ID is not your own, then it's the ID of the account that shared the mesh with your account. For more information about mesh sharing, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/sharing.html">Working with shared meshes</a>.</p>
    #[serde(rename = "meshOwner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    /// <p>The <code>nextToken</code> value returned from a previous paginated <code>ListVirtualServices</code> request where <code>limit</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p><zonbook></zonbook><xhtml></xhtml></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListVirtualServicesOutput {
    /// <p>The <code>nextToken</code> value to include in a future <code>ListVirtualServices</code> request. When the results of a <code>ListVirtualServices</code> request exceed <code>limit</code>, you can use this value to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of existing virtual services for the specified service mesh.</p>
    #[serde(rename = "virtualServices")]
    pub virtual_services: Vec<VirtualServiceRef>,
}

/// <p>An object that represents a listener for a virtual node.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Listener {
    /// <p>The connection pool information for the listener.</p>
    #[serde(rename = "connectionPool")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_pool: Option<VirtualNodeConnectionPool>,
    /// <p>The health check information for the listener.</p>
    #[serde(rename = "healthCheck")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check: Option<HealthCheckPolicy>,
    /// <p>The outlier detection information for the listener.</p>
    #[serde(rename = "outlierDetection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outlier_detection: Option<OutlierDetection>,
    /// <p>The port mapping information for the listener.</p>
    #[serde(rename = "portMapping")]
    pub port_mapping: PortMapping,
    /// <p>An object that represents timeouts for different protocols.</p>
    #[serde(rename = "timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<ListenerTimeout>,
    /// <p>A reference to an object that represents the Transport Layer Security (TLS) properties for a listener.</p>
    #[serde(rename = "tls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<ListenerTls>,
}

/// <p>An object that represents timeouts for different protocols.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ListenerTimeout {
    #[serde(rename = "grpc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grpc: Option<GrpcTimeout>,
    /// <p>An object that represents types of timeouts. </p>
    #[serde(rename = "http")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http: Option<HttpTimeout>,
    /// <p>An object that represents types of timeouts. </p>
    #[serde(rename = "http2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_2: Option<HttpTimeout>,
    /// <p>An object that represents types of timeouts. </p>
    #[serde(rename = "tcp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp: Option<TcpTimeout>,
}

/// <p>An object that represents the Transport Layer Security (TLS) properties for a listener.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ListenerTls {
    /// <p>A reference to an object that represents a listener's TLS certificate.</p>
    #[serde(rename = "certificate")]
    pub certificate: ListenerTlsCertificate,
    /// <p><p>Specify one of the following modes.</p> <ul> <li> <p> <b/>STRICT – Listener only accepts connections with TLS enabled. </p> </li> <li> <p> <b/>PERMISSIVE – Listener accepts connections with or without TLS enabled.</p> </li> <li> <p> <b/>DISABLED – Listener only accepts connections without TLS. </p> </li> </ul></p>
    #[serde(rename = "mode")]
    pub mode: String,
}

/// <p>An object that represents an AWS Certicate Manager (ACM) certificate.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ListenerTlsAcmCertificate {
    /// <p>The Amazon Resource Name (ARN) for the certificate. The certificate must meet specific requirements and you must have proxy authorization enabled. For more information, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/tls.html#virtual-node-tls-prerequisites">Transport Layer Security (TLS)</a>.</p>
    #[serde(rename = "certificateArn")]
    pub certificate_arn: String,
}

/// <p>An object that represents a listener's Transport Layer Security (TLS) certificate.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ListenerTlsCertificate {
    /// <p>A reference to an object that represents an AWS Certicate Manager (ACM) certificate.</p>
    #[serde(rename = "acm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acm: Option<ListenerTlsAcmCertificate>,
    /// <p>A reference to an object that represents a local file certificate.</p>
    #[serde(rename = "file")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<ListenerTlsFileCertificate>,
}

/// <p>An object that represents a local file certificate. The certificate must meet specific requirements and you must have proxy authorization enabled. For more information, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/tls.html#virtual-node-tls-prerequisites">Transport Layer Security (TLS)</a>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ListenerTlsFileCertificate {
    /// <p>The certificate chain for the certificate.</p>
    #[serde(rename = "certificateChain")]
    pub certificate_chain: String,
    /// <p>The private key for a certificate stored on the file system of the virtual node that the proxy is running on.</p>
    #[serde(rename = "privateKey")]
    pub private_key: String,
}

/// <p>An object that represents the logging information for a virtual node.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Logging {
    /// <p>The access log configuration for a virtual node.</p>
    #[serde(rename = "accessLog")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_log: Option<AccessLog>,
}

/// <p>An object that represents the range of values to match on. The first character of the range is included in the range, though the last character is not. For example, if the range specified were 1-100, only values 1-99 would be matched.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MatchRange {
    /// <p>The end of the range.</p>
    #[serde(rename = "end")]
    pub end: i64,
    /// <p>The start of the range.</p>
    #[serde(rename = "start")]
    pub start: i64,
}

/// <p>An object that represents a service mesh returned by a describe operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MeshRef {
    /// <p>The full Amazon Resource Name (ARN) of the service mesh.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>The Unix epoch timestamp in seconds for when the resource was created.</p>
    #[serde(rename = "createdAt")]
    pub created_at: f64,
    /// <p>The Unix epoch timestamp in seconds for when the resource was last updated.</p>
    #[serde(rename = "lastUpdatedAt")]
    pub last_updated_at: f64,
    /// <p>The name of the service mesh.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The AWS IAM account ID of the service mesh owner. If the account ID is not your own, then it's the ID of the account that shared the mesh with your account. For more information about mesh sharing, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/sharing.html">Working with shared meshes</a>.</p>
    #[serde(rename = "meshOwner")]
    pub mesh_owner: String,
    /// <p>The AWS IAM account ID of the resource owner. If the account ID is not your own, then it's the ID of the mesh owner or of another account that the mesh is shared with. For more information about mesh sharing, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/sharing.html">Working with shared meshes</a>.</p>
    #[serde(rename = "resourceOwner")]
    pub resource_owner: String,
    /// <p>The version of the resource. Resources are created at version 1, and this version is incremented each time that they're updated.</p>
    #[serde(rename = "version")]
    pub version: i64,
}

/// <p>An object that represents the specification of a service mesh.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MeshSpec {
    /// <p>The egress filter rules for the service mesh.</p>
    #[serde(rename = "egressFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_filter: Option<EgressFilter>,
}

/// <p>An object that represents the status of a service mesh.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MeshStatus {
    /// <p>The current mesh status.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>An object that represents the outlier detection for a virtual node's listener.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct OutlierDetection {
    /// <p>The base amount of time for which a host is ejected.</p>
    #[serde(rename = "baseEjectionDuration")]
    pub base_ejection_duration: Duration,
    /// <p>The time interval between ejection sweep analysis.</p>
    #[serde(rename = "interval")]
    pub interval: Duration,
    /// <p>Maximum percentage of hosts in load balancing pool for upstream service that can be ejected. Will eject at least one host regardless of the value.</p>
    #[serde(rename = "maxEjectionPercent")]
    pub max_ejection_percent: i64,
    /// <p>Number of consecutive <code>5xx</code> errors required for ejection. </p>
    #[serde(rename = "maxServerErrors")]
    pub max_server_errors: i64,
}

/// <p>An object that represents a port mapping.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PortMapping {
    /// <p>The port used for the port mapping.</p>
    #[serde(rename = "port")]
    pub port: i64,
    /// <p>The protocol used for the port mapping. Specify one protocol.</p>
    #[serde(rename = "protocol")]
    pub protocol: String,
}

/// <p>An object that represents metadata for a resource.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
    /// <p>The AWS IAM account ID of the service mesh owner. If the account ID is not your own, then it's the ID of the account that shared the mesh with your account. For more information about mesh sharing, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/sharing.html">Working with shared meshes</a>.</p>
    #[serde(rename = "meshOwner")]
    pub mesh_owner: String,
    /// <p>The AWS IAM account ID of the resource owner. If the account ID is not your own, then it's the ID of the mesh owner or of another account that the mesh is shared with. For more information about mesh sharing, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/sharing.html">Working with shared meshes</a>.</p>
    #[serde(rename = "resourceOwner")]
    pub resource_owner: String,
    /// <p>The unique identifier for the resource.</p>
    #[serde(rename = "uid")]
    pub uid: String,
    /// <p>The version of the resource. Resources are created at version 1, and this version is incremented each time that they're updated.</p>
    #[serde(rename = "version")]
    pub version: i64,
}

/// <p>An object that represents a route returned by a describe operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RouteRef {
    /// <p>The full Amazon Resource Name (ARN) for the route.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>The Unix epoch timestamp in seconds for when the resource was created.</p>
    #[serde(rename = "createdAt")]
    pub created_at: f64,
    /// <p>The Unix epoch timestamp in seconds for when the resource was last updated.</p>
    #[serde(rename = "lastUpdatedAt")]
    pub last_updated_at: f64,
    /// <p>The name of the service mesh that the route resides in.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The AWS IAM account ID of the service mesh owner. If the account ID is not your own, then it's the ID of the account that shared the mesh with your account. For more information about mesh sharing, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/sharing.html">Working with shared meshes</a>.</p>
    #[serde(rename = "meshOwner")]
    pub mesh_owner: String,
    /// <p>The AWS IAM account ID of the resource owner. If the account ID is not your own, then it's the ID of the mesh owner or of another account that the mesh is shared with. For more information about mesh sharing, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/sharing.html">Working with shared meshes</a>.</p>
    #[serde(rename = "resourceOwner")]
    pub resource_owner: String,
    /// <p>The name of the route.</p>
    #[serde(rename = "routeName")]
    pub route_name: String,
    /// <p>The version of the resource. Resources are created at version 1, and this version is incremented each time that they're updated.</p>
    #[serde(rename = "version")]
    pub version: i64,
    /// <p>The virtual router that the route is associated with.</p>
    #[serde(rename = "virtualRouterName")]
    pub virtual_router_name: String,
}

/// <p>An object that represents a route specification. Specify one route type.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct RouteSpec {
    /// <p>An object that represents the specification of a gRPC route.</p>
    #[serde(rename = "grpcRoute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grpc_route: Option<GrpcRoute>,
    /// <p>An object that represents the specification of an HTTP/2 route.</p>
    #[serde(rename = "http2Route")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_2_route: Option<HttpRoute>,
    /// <p>An object that represents the specification of an HTTP route.</p>
    #[serde(rename = "httpRoute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_route: Option<HttpRoute>,
    /// <p>The priority for the route. Routes are matched based on the specified value, where 0 is the highest priority.</p>
    #[serde(rename = "priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    /// <p>An object that represents the specification of a TCP route.</p>
    #[serde(rename = "tcpRoute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_route: Option<TcpRoute>,
}

/// <p>An object that represents the current status of a route.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RouteStatus {
    /// <p>The current status for the route.</p>
    #[serde(rename = "status")]
    pub status: String,
}

/// <p>An object that represents the service discovery information for a virtual node.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
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

/// <p>Optional metadata that you apply to a resource to assist with categorization and organization. Each tag consists of a key and an optional value, both of which you define. Tag keys can have a maximum character length of 128 characters, and tag values can have a maximum length of 256 characters.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TagRef {
    /// <p>One part of a key-value pair that make up a tag. A <code>key</code> is a general label that acts like a category for more specific tag values.</p>
    #[serde(rename = "key")]
    pub key: String,
    /// <p>The optional part of a key-value pair that make up a tag. A <code>value</code> acts as a descriptor within a tag category (key).</p>
    #[serde(rename = "value")]
    pub value: String,
}

/// <p><zonbook></zonbook><xhtml></xhtml></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceInput {
    /// <p>The Amazon Resource Name (ARN) of the resource to add tags to.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The tags to add to the resource. A tag is an array of key-value pairs. Tag keys can have a maximum character length of 128 characters, and tag values can have a maximum length of 256 characters.</p>
    #[serde(rename = "tags")]
    pub tags: Vec<TagRef>,
}

/// <p><zonbook></zonbook><xhtml></xhtml></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceOutput {}

/// <p>An object that represents a TCP route type.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TcpRoute {
    /// <p>The action to take if a match is determined.</p>
    #[serde(rename = "action")]
    pub action: TcpRouteAction,
    /// <p>An object that represents types of timeouts. </p>
    #[serde(rename = "timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<TcpTimeout>,
}

/// <p>An object that represents the action to take if a match is determined.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TcpRouteAction {
    /// <p>An object that represents the targets that traffic is routed to when a request matches the route.</p>
    #[serde(rename = "weightedTargets")]
    pub weighted_targets: Vec<WeightedTarget>,
}

/// <p>An object that represents types of timeouts. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TcpTimeout {
    /// <p>An object that represents an idle timeout. An idle timeout bounds the amount of time that a connection may be idle. The default value is none.</p>
    #[serde(rename = "idle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle: Option<Duration>,
}

/// <p>An object that represents a Transport Layer Security (TLS) validation context.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TlsValidationContext {
    /// <p>A reference to an object that represents a TLS validation context trust.</p>
    #[serde(rename = "trust")]
    pub trust: TlsValidationContextTrust,
}

/// <p>An object that represents a TLS validation context trust for an AWS Certicate Manager (ACM) certificate.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TlsValidationContextAcmTrust {
    /// <p>One or more ACM Amazon Resource Name (ARN)s.</p>
    #[serde(rename = "certificateAuthorityArns")]
    pub certificate_authority_arns: Vec<String>,
}

/// <p>An object that represents a Transport Layer Security (TLS) validation context trust for a local file.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TlsValidationContextFileTrust {
    /// <p>The certificate trust chain for a certificate stored on the file system of the virtual node that the proxy is running on.</p>
    #[serde(rename = "certificateChain")]
    pub certificate_chain: String,
}

/// <p>An object that represents a Transport Layer Security (TLS) validation context trust.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TlsValidationContextTrust {
    /// <p>A reference to an object that represents a TLS validation context trust for an AWS Certicate Manager (ACM) certificate.</p>
    #[serde(rename = "acm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acm: Option<TlsValidationContextAcmTrust>,
    /// <p>An object that represents a TLS validation context trust for a local file.</p>
    #[serde(rename = "file")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<TlsValidationContextFileTrust>,
}

/// <p><zonbook></zonbook><xhtml></xhtml></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceInput {
    /// <p>The Amazon Resource Name (ARN) of the resource to delete tags from.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The keys of the tags to be removed.</p>
    #[serde(rename = "tagKeys")]
    pub tag_keys: Vec<String>,
}

/// <p><zonbook></zonbook><xhtml></xhtml></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceOutput {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateGatewayRouteInput {
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. Up to 36 letters, numbers, hyphens, and underscores are allowed.</p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The name of the gateway route to update.</p>
    #[serde(rename = "gatewayRouteName")]
    pub gateway_route_name: String,
    /// <p>The name of the service mesh that the gateway route resides in.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The AWS IAM account ID of the service mesh owner. If the account ID is not your own, then it's the ID of the account that shared the mesh with your account. For more information about mesh sharing, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/sharing.html">Working with shared meshes</a>.</p>
    #[serde(rename = "meshOwner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    /// <p>The new gateway route specification to apply. This overwrites the existing data.</p>
    #[serde(rename = "spec")]
    pub spec: GatewayRouteSpec,
    /// <p>The name of the virtual gateway that the gateway route is associated with.</p>
    #[serde(rename = "virtualGatewayName")]
    pub virtual_gateway_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateGatewayRouteOutput {
    /// <p>A full description of the gateway route that was updated.</p>
    #[serde(rename = "gatewayRoute")]
    pub gateway_route: GatewayRouteData,
}

/// <p><zonbook></zonbook><xhtml></xhtml></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateMeshInput {
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. Up to 36 letters, numbers, hyphens, and underscores are allowed.</p>
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

/// <p><zonbook></zonbook><xhtml></xhtml></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateMeshOutput {
    #[serde(rename = "mesh")]
    pub mesh: MeshData,
}

/// <p><zonbook></zonbook><xhtml></xhtml></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateRouteInput {
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. Up to 36 letters, numbers, hyphens, and underscores are allowed.</p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The name of the service mesh that the route resides in.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The AWS IAM account ID of the service mesh owner. If the account ID is not your own, then it's the ID of the account that shared the mesh with your account. For more information about mesh sharing, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/sharing.html">Working with shared meshes</a>.</p>
    #[serde(rename = "meshOwner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
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

/// <p><zonbook></zonbook><xhtml></xhtml></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateRouteOutput {
    /// <p>A full description of the route that was updated.</p>
    #[serde(rename = "route")]
    pub route: RouteData,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateVirtualGatewayInput {
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. Up to 36 letters, numbers, hyphens, and underscores are allowed.</p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The name of the service mesh that the virtual gateway resides in.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The AWS IAM account ID of the service mesh owner. If the account ID is not your own, then it's the ID of the account that shared the mesh with your account. For more information about mesh sharing, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/sharing.html">Working with shared meshes</a>.</p>
    #[serde(rename = "meshOwner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    /// <p>The new virtual gateway specification to apply. This overwrites the existing data.</p>
    #[serde(rename = "spec")]
    pub spec: VirtualGatewaySpec,
    /// <p>The name of the virtual gateway to update.</p>
    #[serde(rename = "virtualGatewayName")]
    pub virtual_gateway_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateVirtualGatewayOutput {
    /// <p>A full description of the virtual gateway that was updated.</p>
    #[serde(rename = "virtualGateway")]
    pub virtual_gateway: VirtualGatewayData,
}

/// <p><zonbook></zonbook><xhtml></xhtml></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateVirtualNodeInput {
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. Up to 36 letters, numbers, hyphens, and underscores are allowed.</p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The name of the service mesh that the virtual node resides in.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The AWS IAM account ID of the service mesh owner. If the account ID is not your own, then it's the ID of the account that shared the mesh with your account. For more information about mesh sharing, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/sharing.html">Working with shared meshes</a>.</p>
    #[serde(rename = "meshOwner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    /// <p>The new virtual node specification to apply. This overwrites the existing data.</p>
    #[serde(rename = "spec")]
    pub spec: VirtualNodeSpec,
    /// <p>The name of the virtual node to update.</p>
    #[serde(rename = "virtualNodeName")]
    pub virtual_node_name: String,
}

/// <p><zonbook></zonbook><xhtml></xhtml></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateVirtualNodeOutput {
    /// <p>A full description of the virtual node that was updated.</p>
    #[serde(rename = "virtualNode")]
    pub virtual_node: VirtualNodeData,
}

/// <p><zonbook></zonbook><xhtml></xhtml></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateVirtualRouterInput {
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. Up to 36 letters, numbers, hyphens, and underscores are allowed.</p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The name of the service mesh that the virtual router resides in.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The AWS IAM account ID of the service mesh owner. If the account ID is not your own, then it's the ID of the account that shared the mesh with your account. For more information about mesh sharing, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/sharing.html">Working with shared meshes</a>.</p>
    #[serde(rename = "meshOwner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    /// <p>The new virtual router specification to apply. This overwrites the existing data.</p>
    #[serde(rename = "spec")]
    pub spec: VirtualRouterSpec,
    /// <p>The name of the virtual router to update.</p>
    #[serde(rename = "virtualRouterName")]
    pub virtual_router_name: String,
}

/// <p><zonbook></zonbook><xhtml></xhtml></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateVirtualRouterOutput {
    /// <p>A full description of the virtual router that was updated.</p>
    #[serde(rename = "virtualRouter")]
    pub virtual_router: VirtualRouterData,
}

/// <p><zonbook></zonbook><xhtml></xhtml></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateVirtualServiceInput {
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. Up to 36 letters, numbers, hyphens, and underscores are allowed.</p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The name of the service mesh that the virtual service resides in.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The AWS IAM account ID of the service mesh owner. If the account ID is not your own, then it's the ID of the account that shared the mesh with your account. For more information about mesh sharing, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/sharing.html">Working with shared meshes</a>.</p>
    #[serde(rename = "meshOwner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,
    /// <p>The new virtual service specification to apply. This overwrites the existing data.</p>
    #[serde(rename = "spec")]
    pub spec: VirtualServiceSpec,
    /// <p>The name of the virtual service to update.</p>
    #[serde(rename = "virtualServiceName")]
    pub virtual_service_name: String,
}

/// <p><zonbook></zonbook><xhtml></xhtml></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateVirtualServiceOutput {
    /// <p>A full description of the virtual service that was updated.</p>
    #[serde(rename = "virtualService")]
    pub virtual_service: VirtualServiceData,
}

/// <p>The access log configuration for a virtual gateway.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VirtualGatewayAccessLog {
    /// <p>The file object to send virtual gateway access logs to.</p>
    #[serde(rename = "file")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<VirtualGatewayFileAccessLog>,
}

/// <p>An object that represents the default properties for a backend.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VirtualGatewayBackendDefaults {
    /// <p>A reference to an object that represents a client policy.</p>
    #[serde(rename = "clientPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_policy: Option<VirtualGatewayClientPolicy>,
}

/// <p>An object that represents a client policy.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VirtualGatewayClientPolicy {
    /// <p>A reference to an object that represents a Transport Layer Security (TLS) client policy.</p>
    #[serde(rename = "tls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<VirtualGatewayClientPolicyTls>,
}

/// <p>An object that represents a Transport Layer Security (TLS) client policy.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VirtualGatewayClientPolicyTls {
    /// <p>Whether the policy is enforced. The default is <code>True</code>, if a value isn't specified.</p>
    #[serde(rename = "enforce")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforce: Option<bool>,
    /// <p>One or more ports that the policy is enforced for.</p>
    #[serde(rename = "ports")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<i64>>,
    /// <p>A reference to an object that represents a TLS validation context.</p>
    #[serde(rename = "validation")]
    pub validation: VirtualGatewayTlsValidationContext,
}

/// <p>An object that represents the type of virtual gateway connection pool.</p> <p>Only one protocol is used at a time and should be the same protocol as the one chosen under port mapping.</p> <p>If not present the default value for <code>maxPendingRequests</code> is <code>2147483647</code>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VirtualGatewayConnectionPool {
    /// <p>An object that represents a type of connection pool. </p>
    #[serde(rename = "grpc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grpc: Option<VirtualGatewayGrpcConnectionPool>,
    /// <p>An object that represents a type of connection pool.</p>
    #[serde(rename = "http")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http: Option<VirtualGatewayHttpConnectionPool>,
    /// <p>An object that represents a type of connection pool.</p>
    #[serde(rename = "http2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_2: Option<VirtualGatewayHttp2ConnectionPool>,
}

/// <p>An object that represents a virtual gateway returned by a describe operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct VirtualGatewayData {
    /// <p>The name of the service mesh that the virtual gateway resides in.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    #[serde(rename = "metadata")]
    pub metadata: ResourceMetadata,
    /// <p>The specifications of the virtual gateway.</p>
    #[serde(rename = "spec")]
    pub spec: VirtualGatewaySpec,
    /// <p>The current status of the virtual gateway.</p>
    #[serde(rename = "status")]
    pub status: VirtualGatewayStatus,
    /// <p>The name of the virtual gateway.</p>
    #[serde(rename = "virtualGatewayName")]
    pub virtual_gateway_name: String,
}

/// <p>An object that represents an access log file.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VirtualGatewayFileAccessLog {
    /// <p>The file path to write access logs to. You can use <code>/dev/stdout</code> to send access logs to standard out and configure your Envoy container to use a log driver, such as <code>awslogs</code>, to export the access logs to a log storage service such as Amazon CloudWatch Logs. You can also specify a path in the Envoy container's file system to write the files to disk.</p>
    #[serde(rename = "path")]
    pub path: String,
}

/// <p>An object that represents a type of connection pool.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VirtualGatewayGrpcConnectionPool {
    /// <p>Maximum number of inflight requests Envoy can concurrently support across hosts in upstream cluster.</p>
    #[serde(rename = "maxRequests")]
    pub max_requests: i64,
}

/// <p>An object that represents the health check policy for a virtual gateway's listener.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VirtualGatewayHealthCheckPolicy {
    /// <p>The number of consecutive successful health checks that must occur before declaring the listener healthy.</p>
    #[serde(rename = "healthyThreshold")]
    pub healthy_threshold: i64,
    /// <p>The time period in milliseconds between each health check execution.</p>
    #[serde(rename = "intervalMillis")]
    pub interval_millis: i64,
    /// <p>The destination path for the health check request. This value is only used if the specified protocol is HTTP or HTTP/2. For any other protocol, this value is ignored.</p>
    #[serde(rename = "path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// <p>The destination port for the health check request. This port must match the port defined in the <a>PortMapping</a> for the listener.</p>
    #[serde(rename = "port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    /// <p>The protocol for the health check request. If you specify <code>grpc</code>, then your service must conform to the <a href="https://github.com/grpc/grpc/blob/master/doc/health-checking.md">GRPC Health Checking Protocol</a>.</p>
    #[serde(rename = "protocol")]
    pub protocol: String,
    /// <p>The amount of time to wait when receiving a response from the health check, in milliseconds.</p>
    #[serde(rename = "timeoutMillis")]
    pub timeout_millis: i64,
    /// <p>The number of consecutive failed health checks that must occur before declaring a virtual gateway unhealthy.</p>
    #[serde(rename = "unhealthyThreshold")]
    pub unhealthy_threshold: i64,
}

/// <p>An object that represents a type of connection pool.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VirtualGatewayHttp2ConnectionPool {
    /// <p>Maximum number of inflight requests Envoy can concurrently support across hosts in upstream cluster.</p>
    #[serde(rename = "maxRequests")]
    pub max_requests: i64,
}

/// <p>An object that represents a type of connection pool.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VirtualGatewayHttpConnectionPool {
    /// <p>Maximum number of outbound TCP connections Envoy can establish concurrently with all hosts in upstream cluster.</p>
    #[serde(rename = "maxConnections")]
    pub max_connections: i64,
    /// <p>Number of overflowing requests after <code>max_connections</code> Envoy will queue to upstream cluster.</p>
    #[serde(rename = "maxPendingRequests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_pending_requests: Option<i64>,
}

/// <p>An object that represents a listener for a virtual gateway.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VirtualGatewayListener {
    /// <p>The connection pool information for the virtual gateway listener.</p>
    #[serde(rename = "connectionPool")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_pool: Option<VirtualGatewayConnectionPool>,
    /// <p>The health check information for the listener.</p>
    #[serde(rename = "healthCheck")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check: Option<VirtualGatewayHealthCheckPolicy>,
    /// <p>The port mapping information for the listener.</p>
    #[serde(rename = "portMapping")]
    pub port_mapping: VirtualGatewayPortMapping,
    /// <p>A reference to an object that represents the Transport Layer Security (TLS) properties for the listener.</p>
    #[serde(rename = "tls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<VirtualGatewayListenerTls>,
}

/// <p>An object that represents the Transport Layer Security (TLS) properties for a listener.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VirtualGatewayListenerTls {
    /// <p>An object that represents a Transport Layer Security (TLS) certificate.</p>
    #[serde(rename = "certificate")]
    pub certificate: VirtualGatewayListenerTlsCertificate,
    /// <p><p>Specify one of the following modes.</p> <ul> <li> <p> <b/>STRICT – Listener only accepts connections with TLS enabled. </p> </li> <li> <p> <b/>PERMISSIVE – Listener accepts connections with or without TLS enabled.</p> </li> <li> <p> <b/>DISABLED – Listener only accepts connections without TLS. </p> </li> </ul></p>
    #[serde(rename = "mode")]
    pub mode: String,
}

/// <p>An object that represents an AWS Certicate Manager (ACM) certificate.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VirtualGatewayListenerTlsAcmCertificate {
    /// <p>The Amazon Resource Name (ARN) for the certificate. The certificate must meet specific requirements and you must have proxy authorization enabled. For more information, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/tls.html#virtual-node-tls-prerequisites">Transport Layer Security (TLS)</a>.</p>
    #[serde(rename = "certificateArn")]
    pub certificate_arn: String,
}

/// <p>An object that represents a listener's Transport Layer Security (TLS) certificate.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VirtualGatewayListenerTlsCertificate {
    /// <p>A reference to an object that represents an AWS Certicate Manager (ACM) certificate.</p>
    #[serde(rename = "acm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acm: Option<VirtualGatewayListenerTlsAcmCertificate>,
    /// <p>A reference to an object that represents a local file certificate.</p>
    #[serde(rename = "file")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<VirtualGatewayListenerTlsFileCertificate>,
}

/// <p>An object that represents a local file certificate. The certificate must meet specific requirements and you must have proxy authorization enabled. For more information, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/tls.html#virtual-node-tls-prerequisites">Transport Layer Security (TLS)</a>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VirtualGatewayListenerTlsFileCertificate {
    /// <p>The certificate chain for the certificate.</p>
    #[serde(rename = "certificateChain")]
    pub certificate_chain: String,
    /// <p>The private key for a certificate stored on the file system of the mesh endpoint that the proxy is running on.</p>
    #[serde(rename = "privateKey")]
    pub private_key: String,
}

/// <p>An object that represents logging information.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VirtualGatewayLogging {
    /// <p>The access log configuration.</p>
    #[serde(rename = "accessLog")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_log: Option<VirtualGatewayAccessLog>,
}

/// <p>An object that represents a port mapping.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VirtualGatewayPortMapping {
    /// <p>The port used for the port mapping. Specify one protocol.</p>
    #[serde(rename = "port")]
    pub port: i64,
    /// <p>The protocol used for the port mapping.</p>
    #[serde(rename = "protocol")]
    pub protocol: String,
}

/// <p>An object that represents a virtual gateway returned by a list operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct VirtualGatewayRef {
    /// <p>The full Amazon Resource Name (ARN) for the resource.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>The Unix epoch timestamp in seconds for when the resource was created.</p>
    #[serde(rename = "createdAt")]
    pub created_at: f64,
    /// <p>The Unix epoch timestamp in seconds for when the resource was last updated.</p>
    #[serde(rename = "lastUpdatedAt")]
    pub last_updated_at: f64,
    /// <p>The name of the service mesh that the resource resides in.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The AWS IAM account ID of the service mesh owner. If the account ID is not your own, then it's the ID of the account that shared the mesh with your account. For more information about mesh sharing, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/sharing.html">Working with shared meshes</a>.</p>
    #[serde(rename = "meshOwner")]
    pub mesh_owner: String,
    /// <p>The AWS IAM account ID of the resource owner. If the account ID is not your own, then it's the ID of the mesh owner or of another account that the mesh is shared with. For more information about mesh sharing, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/sharing.html">Working with shared meshes</a>.</p>
    #[serde(rename = "resourceOwner")]
    pub resource_owner: String,
    /// <p>The version of the resource. Resources are created at version 1, and this version is incremented each time that they're updated.</p>
    #[serde(rename = "version")]
    pub version: i64,
    /// <p>The name of the resource.</p>
    #[serde(rename = "virtualGatewayName")]
    pub virtual_gateway_name: String,
}

/// <p>An object that represents the specification of a service mesh resource.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VirtualGatewaySpec {
    /// <p>A reference to an object that represents the defaults for backends.</p>
    #[serde(rename = "backendDefaults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_defaults: Option<VirtualGatewayBackendDefaults>,
    /// <p>The listeners that the mesh endpoint is expected to receive inbound traffic from. You can specify one listener.</p>
    #[serde(rename = "listeners")]
    pub listeners: Vec<VirtualGatewayListener>,
    #[serde(rename = "logging")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<VirtualGatewayLogging>,
}

/// <p>An object that represents the status of the mesh resource.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct VirtualGatewayStatus {
    /// <p>The current status.</p>
    #[serde(rename = "status")]
    pub status: String,
}

/// <p>An object that represents a Transport Layer Security (TLS) validation context.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VirtualGatewayTlsValidationContext {
    /// <p>A reference to an object that represents a TLS validation context trust.</p>
    #[serde(rename = "trust")]
    pub trust: VirtualGatewayTlsValidationContextTrust,
}

/// <p>An object that represents a TLS validation context trust for an AWS Certicate Manager (ACM) certificate.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VirtualGatewayTlsValidationContextAcmTrust {
    /// <p>One or more ACM Amazon Resource Name (ARN)s.</p>
    #[serde(rename = "certificateAuthorityArns")]
    pub certificate_authority_arns: Vec<String>,
}

/// <p>An object that represents a Transport Layer Security (TLS) validation context trust for a local file.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VirtualGatewayTlsValidationContextFileTrust {
    /// <p>The certificate trust chain for a certificate stored on the file system of the virtual node that the proxy is running on.</p>
    #[serde(rename = "certificateChain")]
    pub certificate_chain: String,
}

/// <p>An object that represents a Transport Layer Security (TLS) validation context trust.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VirtualGatewayTlsValidationContextTrust {
    /// <p>A reference to an object that represents a TLS validation context trust for an AWS Certicate Manager (ACM) certificate.</p>
    #[serde(rename = "acm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acm: Option<VirtualGatewayTlsValidationContextAcmTrust>,
    /// <p>An object that represents a TLS validation context trust for a local file.</p>
    #[serde(rename = "file")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<VirtualGatewayTlsValidationContextFileTrust>,
}

/// <p><p>An object that represents the type of virtual node connection pool.</p> <p>Only one protocol is used at a time and should be the same protocol as the one chosen under port mapping.</p> <p>If not present the default value for <code>maxPendingRequests</code> is <code>2147483647</code>.</p> <p/></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VirtualNodeConnectionPool {
    /// <p>An object that represents a type of connection pool.</p>
    #[serde(rename = "grpc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grpc: Option<VirtualNodeGrpcConnectionPool>,
    /// <p>An object that represents a type of connection pool.</p>
    #[serde(rename = "http")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http: Option<VirtualNodeHttpConnectionPool>,
    /// <p>An object that represents a type of connection pool.</p>
    #[serde(rename = "http2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_2: Option<VirtualNodeHttp2ConnectionPool>,
    /// <p>An object that represents a type of connection pool.</p>
    #[serde(rename = "tcp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp: Option<VirtualNodeTcpConnectionPool>,
}

/// <p>An object that represents a virtual node returned by a describe operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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

/// <p>An object that represents a type of connection pool.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VirtualNodeGrpcConnectionPool {
    /// <p>Maximum number of inflight requests Envoy can concurrently support across hosts in upstream cluster.</p>
    #[serde(rename = "maxRequests")]
    pub max_requests: i64,
}

/// <p>An object that represents a type of connection pool.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VirtualNodeHttp2ConnectionPool {
    /// <p>Maximum number of inflight requests Envoy can concurrently support across hosts in upstream cluster.</p>
    #[serde(rename = "maxRequests")]
    pub max_requests: i64,
}

/// <p>An object that represents a type of connection pool.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VirtualNodeHttpConnectionPool {
    /// <p>Maximum number of outbound TCP connections Envoy can establish concurrently with all hosts in upstream cluster.</p>
    #[serde(rename = "maxConnections")]
    pub max_connections: i64,
    /// <p>Number of overflowing requests after <code>max_connections</code> Envoy will queue to upstream cluster.</p>
    #[serde(rename = "maxPendingRequests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_pending_requests: Option<i64>,
}

/// <p>An object that represents a virtual node returned by a list operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct VirtualNodeRef {
    /// <p>The full Amazon Resource Name (ARN) for the virtual node.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>The Unix epoch timestamp in seconds for when the resource was created.</p>
    #[serde(rename = "createdAt")]
    pub created_at: f64,
    /// <p>The Unix epoch timestamp in seconds for when the resource was last updated.</p>
    #[serde(rename = "lastUpdatedAt")]
    pub last_updated_at: f64,
    /// <p>The name of the service mesh that the virtual node resides in.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The AWS IAM account ID of the service mesh owner. If the account ID is not your own, then it's the ID of the account that shared the mesh with your account. For more information about mesh sharing, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/sharing.html">Working with shared meshes</a>.</p>
    #[serde(rename = "meshOwner")]
    pub mesh_owner: String,
    /// <p>The AWS IAM account ID of the resource owner. If the account ID is not your own, then it's the ID of the mesh owner or of another account that the mesh is shared with. For more information about mesh sharing, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/sharing.html">Working with shared meshes</a>.</p>
    #[serde(rename = "resourceOwner")]
    pub resource_owner: String,
    /// <p>The version of the resource. Resources are created at version 1, and this version is incremented each time that they're updated.</p>
    #[serde(rename = "version")]
    pub version: i64,
    /// <p>The name of the virtual node.</p>
    #[serde(rename = "virtualNodeName")]
    pub virtual_node_name: String,
}

/// <p>An object that represents a virtual node service provider.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VirtualNodeServiceProvider {
    /// <p>The name of the virtual node that is acting as a service provider.</p>
    #[serde(rename = "virtualNodeName")]
    pub virtual_node_name: String,
}

/// <p>An object that represents the specification of a virtual node.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VirtualNodeSpec {
    /// <p>A reference to an object that represents the defaults for backends.</p>
    #[serde(rename = "backendDefaults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_defaults: Option<BackendDefaults>,
    /// <p>The backends that the virtual node is expected to send outbound traffic to.</p>
    #[serde(rename = "backends")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backends: Option<Vec<Backend>>,
    /// <p>The listener that the virtual node is expected to receive inbound traffic from. You can specify one listener.</p>
    #[serde(rename = "listeners")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listeners: Option<Vec<Listener>>,
    /// <p>The inbound and outbound access logging information for the virtual node.</p>
    #[serde(rename = "logging")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<Logging>,
    /// <p>The service discovery information for the virtual node. If your virtual node does not expect ingress traffic, you can omit this parameter. If you specify a <code>listener</code>, then you must specify service discovery information.</p>
    #[serde(rename = "serviceDiscovery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_discovery: Option<ServiceDiscovery>,
}

/// <p>An object that represents the current status of the virtual node.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct VirtualNodeStatus {
    /// <p>The current status of the virtual node.</p>
    #[serde(rename = "status")]
    pub status: String,
}

/// <p>An object that represents a type of connection pool.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VirtualNodeTcpConnectionPool {
    /// <p>Maximum number of outbound TCP connections Envoy can establish concurrently with all hosts in upstream cluster.</p>
    #[serde(rename = "maxConnections")]
    pub max_connections: i64,
}

/// <p>An object that represents a virtual router returned by a describe operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VirtualRouterListener {
    #[serde(rename = "portMapping")]
    pub port_mapping: PortMapping,
}

/// <p>An object that represents a virtual router returned by a list operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct VirtualRouterRef {
    /// <p>The full Amazon Resource Name (ARN) for the virtual router.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>The Unix epoch timestamp in seconds for when the resource was created.</p>
    #[serde(rename = "createdAt")]
    pub created_at: f64,
    /// <p>The Unix epoch timestamp in seconds for when the resource was last updated.</p>
    #[serde(rename = "lastUpdatedAt")]
    pub last_updated_at: f64,
    /// <p>The name of the service mesh that the virtual router resides in.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The AWS IAM account ID of the service mesh owner. If the account ID is not your own, then it's the ID of the account that shared the mesh with your account. For more information about mesh sharing, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/sharing.html">Working with shared meshes</a>.</p>
    #[serde(rename = "meshOwner")]
    pub mesh_owner: String,
    /// <p>The AWS IAM account ID of the resource owner. If the account ID is not your own, then it's the ID of the mesh owner or of another account that the mesh is shared with. For more information about mesh sharing, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/sharing.html">Working with shared meshes</a>.</p>
    #[serde(rename = "resourceOwner")]
    pub resource_owner: String,
    /// <p>The version of the resource. Resources are created at version 1, and this version is incremented each time that they're updated.</p>
    #[serde(rename = "version")]
    pub version: i64,
    /// <p>The name of the virtual router.</p>
    #[serde(rename = "virtualRouterName")]
    pub virtual_router_name: String,
}

/// <p>An object that represents a virtual node service provider.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VirtualRouterServiceProvider {
    /// <p>The name of the virtual router that is acting as a service provider.</p>
    #[serde(rename = "virtualRouterName")]
    pub virtual_router_name: String,
}

/// <p>An object that represents the specification of a virtual router.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VirtualRouterSpec {
    /// <p>The listeners that the virtual router is expected to receive inbound traffic from. You can specify one listener.</p>
    #[serde(rename = "listeners")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listeners: Option<Vec<VirtualRouterListener>>,
}

/// <p>An object that represents the status of a virtual router. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct VirtualRouterStatus {
    /// <p>The current status of the virtual router.</p>
    #[serde(rename = "status")]
    pub status: String,
}

/// <p>An object that represents a virtual service backend for a virtual node.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VirtualServiceBackend {
    /// <p>A reference to an object that represents the client policy for a backend.</p>
    #[serde(rename = "clientPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_policy: Option<ClientPolicy>,
    /// <p>The name of the virtual service that is acting as a virtual node backend.</p>
    #[serde(rename = "virtualServiceName")]
    pub virtual_service_name: String,
}

/// <p>An object that represents a virtual service returned by a describe operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct VirtualServiceRef {
    /// <p>The full Amazon Resource Name (ARN) for the virtual service.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>The Unix epoch timestamp in seconds for when the resource was created.</p>
    #[serde(rename = "createdAt")]
    pub created_at: f64,
    /// <p>The Unix epoch timestamp in seconds for when the resource was last updated.</p>
    #[serde(rename = "lastUpdatedAt")]
    pub last_updated_at: f64,
    /// <p>The name of the service mesh that the virtual service resides in.</p>
    #[serde(rename = "meshName")]
    pub mesh_name: String,
    /// <p>The AWS IAM account ID of the service mesh owner. If the account ID is not your own, then it's the ID of the account that shared the mesh with your account. For more information about mesh sharing, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/sharing.html">Working with shared meshes</a>.</p>
    #[serde(rename = "meshOwner")]
    pub mesh_owner: String,
    /// <p>The AWS IAM account ID of the resource owner. If the account ID is not your own, then it's the ID of the mesh owner or of another account that the mesh is shared with. For more information about mesh sharing, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/sharing.html">Working with shared meshes</a>.</p>
    #[serde(rename = "resourceOwner")]
    pub resource_owner: String,
    /// <p>The version of the resource. Resources are created at version 1, and this version is incremented each time that they're updated.</p>
    #[serde(rename = "version")]
    pub version: i64,
    /// <p>The name of the virtual service.</p>
    #[serde(rename = "virtualServiceName")]
    pub virtual_service_name: String,
}

/// <p>An object that represents the specification of a virtual service.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VirtualServiceSpec {
    /// <p>The App Mesh object that is acting as the provider for a virtual service. You can specify a single virtual node or virtual router.</p>
    #[serde(rename = "provider")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<VirtualServiceProvider>,
}

/// <p>An object that represents the status of a virtual service.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct VirtualServiceStatus {
    /// <p>The current status of the virtual service.</p>
    #[serde(rename = "status")]
    pub status: String,
}

/// <p>An object that represents a target and its relative weight. Traffic is distributed across targets according to their relative weight. For example, a weighted target with a relative weight of 50 receives five times as much traffic as one with a relative weight of 10. The total weight for all targets combined must be less than or equal to 100.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct WeightedTarget {
    /// <p>The virtual node to associate with the weighted target.</p>
    #[serde(rename = "virtualNode")]
    pub virtual_node: String,
    /// <p>The relative weight of the weighted target.</p>
    #[serde(rename = "weight")]
    pub weight: i64,
}

/// Errors returned by CreateGatewayRoute
#[derive(Debug, PartialEq)]
pub enum CreateGatewayRouteError {
    /// <p>The request syntax was malformed. Check your request syntax and try again.</p>
    BadRequest(String),
    /// <p>The request contains a client token that was used for a previous update resource call with different specifications. Try the request again with a new client token.</p>
    Conflict(String),
    /// <p>You don't have permissions to perform this action.</p>
    Forbidden(String),
    /// <p>The request processing has failed because of an unknown error, exception, or failure.</p>
    InternalServerError(String),
    /// <p>You have exceeded a service limit for your account. For more information, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/service-quotas.html">Service Limits</a> in the <i>AWS App Mesh User Guide</i>.</p>
    LimitExceeded(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your account. For best results, use an increasing or variable sleep interval between requests.</p>
    TooManyRequests(String),
}

impl CreateGatewayRouteError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateGatewayRouteError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateGatewayRouteError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateGatewayRouteError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateGatewayRouteError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateGatewayRouteError::InternalServerError(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateGatewayRouteError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateGatewayRouteError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateGatewayRouteError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateGatewayRouteError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateGatewayRouteError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateGatewayRouteError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateGatewayRouteError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateGatewayRouteError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreateGatewayRouteError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreateGatewayRouteError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateGatewayRouteError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateGatewayRouteError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            CreateGatewayRouteError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateGatewayRouteError {}
/// Errors returned by CreateMesh
#[derive(Debug, PartialEq)]
pub enum CreateMeshError {
    /// <p>The request syntax was malformed. Check your request syntax and try again.</p>
    BadRequest(String),
    /// <p>The request contains a client token that was used for a previous update resource call with different specifications. Try the request again with a new client token.</p>
    Conflict(String),
    /// <p>You don't have permissions to perform this action.</p>
    Forbidden(String),
    /// <p>The request processing has failed because of an unknown error, exception, or failure.</p>
    InternalServerError(String),
    /// <p>You have exceeded a service limit for your account. For more information, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/service-quotas.html">Service Limits</a> in the <i>AWS App Mesh User Guide</i>.</p>
    LimitExceeded(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your account. For best results, use an increasing or variable sleep interval between requests.</p>
    TooManyRequests(String),
}

impl CreateMeshError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateMeshError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
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
    /// <p>The request contains a client token that was used for a previous update resource call with different specifications. Try the request again with a new client token.</p>
    Conflict(String),
    /// <p>You don't have permissions to perform this action.</p>
    Forbidden(String),
    /// <p>The request processing has failed because of an unknown error, exception, or failure.</p>
    InternalServerError(String),
    /// <p>You have exceeded a service limit for your account. For more information, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/service-quotas.html">Service Limits</a> in the <i>AWS App Mesh User Guide</i>.</p>
    LimitExceeded(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your account. For best results, use an increasing or variable sleep interval between requests.</p>
    TooManyRequests(String),
}

impl CreateRouteError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateRouteError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
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
/// Errors returned by CreateVirtualGateway
#[derive(Debug, PartialEq)]
pub enum CreateVirtualGatewayError {
    /// <p>The request syntax was malformed. Check your request syntax and try again.</p>
    BadRequest(String),
    /// <p>The request contains a client token that was used for a previous update resource call with different specifications. Try the request again with a new client token.</p>
    Conflict(String),
    /// <p>You don't have permissions to perform this action.</p>
    Forbidden(String),
    /// <p>The request processing has failed because of an unknown error, exception, or failure.</p>
    InternalServerError(String),
    /// <p>You have exceeded a service limit for your account. For more information, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/service-quotas.html">Service Limits</a> in the <i>AWS App Mesh User Guide</i>.</p>
    LimitExceeded(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your account. For best results, use an increasing or variable sleep interval between requests.</p>
    TooManyRequests(String),
}

impl CreateVirtualGatewayError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateVirtualGatewayError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateVirtualGatewayError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateVirtualGatewayError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateVirtualGatewayError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateVirtualGatewayError::InternalServerError(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateVirtualGatewayError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateVirtualGatewayError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateVirtualGatewayError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateVirtualGatewayError::TooManyRequests(
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
impl fmt::Display for CreateVirtualGatewayError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateVirtualGatewayError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateVirtualGatewayError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateVirtualGatewayError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreateVirtualGatewayError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreateVirtualGatewayError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateVirtualGatewayError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateVirtualGatewayError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            CreateVirtualGatewayError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateVirtualGatewayError {}
/// Errors returned by CreateVirtualNode
#[derive(Debug, PartialEq)]
pub enum CreateVirtualNodeError {
    /// <p>The request syntax was malformed. Check your request syntax and try again.</p>
    BadRequest(String),
    /// <p>The request contains a client token that was used for a previous update resource call with different specifications. Try the request again with a new client token.</p>
    Conflict(String),
    /// <p>You don't have permissions to perform this action.</p>
    Forbidden(String),
    /// <p>The request processing has failed because of an unknown error, exception, or failure.</p>
    InternalServerError(String),
    /// <p>You have exceeded a service limit for your account. For more information, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/service-quotas.html">Service Limits</a> in the <i>AWS App Mesh User Guide</i>.</p>
    LimitExceeded(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your account. For best results, use an increasing or variable sleep interval between requests.</p>
    TooManyRequests(String),
}

impl CreateVirtualNodeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateVirtualNodeError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
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
    /// <p>The request contains a client token that was used for a previous update resource call with different specifications. Try the request again with a new client token.</p>
    Conflict(String),
    /// <p>You don't have permissions to perform this action.</p>
    Forbidden(String),
    /// <p>The request processing has failed because of an unknown error, exception, or failure.</p>
    InternalServerError(String),
    /// <p>You have exceeded a service limit for your account. For more information, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/service-quotas.html">Service Limits</a> in the <i>AWS App Mesh User Guide</i>.</p>
    LimitExceeded(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your account. For best results, use an increasing or variable sleep interval between requests.</p>
    TooManyRequests(String),
}

impl CreateVirtualRouterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateVirtualRouterError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
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
    /// <p>The request contains a client token that was used for a previous update resource call with different specifications. Try the request again with a new client token.</p>
    Conflict(String),
    /// <p>You don't have permissions to perform this action.</p>
    Forbidden(String),
    /// <p>The request processing has failed because of an unknown error, exception, or failure.</p>
    InternalServerError(String),
    /// <p>You have exceeded a service limit for your account. For more information, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/service-quotas.html">Service Limits</a> in the <i>AWS App Mesh User Guide</i>.</p>
    LimitExceeded(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your account. For best results, use an increasing or variable sleep interval between requests.</p>
    TooManyRequests(String),
}

impl CreateVirtualServiceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateVirtualServiceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
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
/// Errors returned by DeleteGatewayRoute
#[derive(Debug, PartialEq)]
pub enum DeleteGatewayRouteError {
    /// <p>The request syntax was malformed. Check your request syntax and try again.</p>
    BadRequest(String),
    /// <p>You don't have permissions to perform this action.</p>
    Forbidden(String),
    /// <p>The request processing has failed because of an unknown error, exception, or failure.</p>
    InternalServerError(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>You can't delete the specified resource because it's in use or required by another resource.</p>
    ResourceInUse(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your account. For best results, use an increasing or variable sleep interval between requests.</p>
    TooManyRequests(String),
}

impl DeleteGatewayRouteError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteGatewayRouteError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteGatewayRouteError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteGatewayRouteError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteGatewayRouteError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteGatewayRouteError::NotFound(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteGatewayRouteError::ResourceInUse(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteGatewayRouteError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteGatewayRouteError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteGatewayRouteError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteGatewayRouteError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteGatewayRouteError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteGatewayRouteError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteGatewayRouteError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteGatewayRouteError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            DeleteGatewayRouteError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DeleteGatewayRouteError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteGatewayRouteError {}
/// Errors returned by DeleteMesh
#[derive(Debug, PartialEq)]
pub enum DeleteMeshError {
    /// <p>The request syntax was malformed. Check your request syntax and try again.</p>
    BadRequest(String),
    /// <p>You don't have permissions to perform this action.</p>
    Forbidden(String),
    /// <p>The request processing has failed because of an unknown error, exception, or failure.</p>
    InternalServerError(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>You can't delete the specified resource because it's in use or required by another resource.</p>
    ResourceInUse(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your account. For best results, use an increasing or variable sleep interval between requests.</p>
    TooManyRequests(String),
}

impl DeleteMeshError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteMeshError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
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
    /// <p>The request processing has failed because of an unknown error, exception, or failure.</p>
    InternalServerError(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>You can't delete the specified resource because it's in use or required by another resource.</p>
    ResourceInUse(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your account. For best results, use an increasing or variable sleep interval between requests.</p>
    TooManyRequests(String),
}

impl DeleteRouteError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteRouteError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
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
/// Errors returned by DeleteVirtualGateway
#[derive(Debug, PartialEq)]
pub enum DeleteVirtualGatewayError {
    /// <p>The request syntax was malformed. Check your request syntax and try again.</p>
    BadRequest(String),
    /// <p>You don't have permissions to perform this action.</p>
    Forbidden(String),
    /// <p>The request processing has failed because of an unknown error, exception, or failure.</p>
    InternalServerError(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>You can't delete the specified resource because it's in use or required by another resource.</p>
    ResourceInUse(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your account. For best results, use an increasing or variable sleep interval between requests.</p>
    TooManyRequests(String),
}

impl DeleteVirtualGatewayError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteVirtualGatewayError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteVirtualGatewayError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteVirtualGatewayError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteVirtualGatewayError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteVirtualGatewayError::NotFound(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteVirtualGatewayError::ResourceInUse(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteVirtualGatewayError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteVirtualGatewayError::TooManyRequests(
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
impl fmt::Display for DeleteVirtualGatewayError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteVirtualGatewayError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteVirtualGatewayError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteVirtualGatewayError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteVirtualGatewayError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteVirtualGatewayError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            DeleteVirtualGatewayError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DeleteVirtualGatewayError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteVirtualGatewayError {}
/// Errors returned by DeleteVirtualNode
#[derive(Debug, PartialEq)]
pub enum DeleteVirtualNodeError {
    /// <p>The request syntax was malformed. Check your request syntax and try again.</p>
    BadRequest(String),
    /// <p>You don't have permissions to perform this action.</p>
    Forbidden(String),
    /// <p>The request processing has failed because of an unknown error, exception, or failure.</p>
    InternalServerError(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>You can't delete the specified resource because it's in use or required by another resource.</p>
    ResourceInUse(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your account. For best results, use an increasing or variable sleep interval between requests.</p>
    TooManyRequests(String),
}

impl DeleteVirtualNodeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteVirtualNodeError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
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
    /// <p>The request processing has failed because of an unknown error, exception, or failure.</p>
    InternalServerError(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>You can't delete the specified resource because it's in use or required by another resource.</p>
    ResourceInUse(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your account. For best results, use an increasing or variable sleep interval between requests.</p>
    TooManyRequests(String),
}

impl DeleteVirtualRouterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteVirtualRouterError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
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
    /// <p>The request processing has failed because of an unknown error, exception, or failure.</p>
    InternalServerError(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>You can't delete the specified resource because it's in use or required by another resource.</p>
    ResourceInUse(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your account. For best results, use an increasing or variable sleep interval between requests.</p>
    TooManyRequests(String),
}

impl DeleteVirtualServiceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteVirtualServiceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
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
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteVirtualServiceError::ResourceInUse(err.msg))
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
            DeleteVirtualServiceError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            DeleteVirtualServiceError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DeleteVirtualServiceError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteVirtualServiceError {}
/// Errors returned by DescribeGatewayRoute
#[derive(Debug, PartialEq)]
pub enum DescribeGatewayRouteError {
    /// <p>The request syntax was malformed. Check your request syntax and try again.</p>
    BadRequest(String),
    /// <p>You don't have permissions to perform this action.</p>
    Forbidden(String),
    /// <p>The request processing has failed because of an unknown error, exception, or failure.</p>
    InternalServerError(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your account. For best results, use an increasing or variable sleep interval between requests.</p>
    TooManyRequests(String),
}

impl DescribeGatewayRouteError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeGatewayRouteError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DescribeGatewayRouteError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DescribeGatewayRouteError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DescribeGatewayRouteError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeGatewayRouteError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeGatewayRouteError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeGatewayRouteError::TooManyRequests(
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
impl fmt::Display for DescribeGatewayRouteError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeGatewayRouteError::BadRequest(ref cause) => write!(f, "{}", cause),
            DescribeGatewayRouteError::Forbidden(ref cause) => write!(f, "{}", cause),
            DescribeGatewayRouteError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DescribeGatewayRouteError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribeGatewayRouteError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DescribeGatewayRouteError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeGatewayRouteError {}
/// Errors returned by DescribeMesh
#[derive(Debug, PartialEq)]
pub enum DescribeMeshError {
    /// <p>The request syntax was malformed. Check your request syntax and try again.</p>
    BadRequest(String),
    /// <p>You don't have permissions to perform this action.</p>
    Forbidden(String),
    /// <p>The request processing has failed because of an unknown error, exception, or failure.</p>
    InternalServerError(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your account. For best results, use an increasing or variable sleep interval between requests.</p>
    TooManyRequests(String),
}

impl DescribeMeshError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeMeshError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
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
    /// <p>The request processing has failed because of an unknown error, exception, or failure.</p>
    InternalServerError(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your account. For best results, use an increasing or variable sleep interval between requests.</p>
    TooManyRequests(String),
}

impl DescribeRouteError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeRouteError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
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
/// Errors returned by DescribeVirtualGateway
#[derive(Debug, PartialEq)]
pub enum DescribeVirtualGatewayError {
    /// <p>The request syntax was malformed. Check your request syntax and try again.</p>
    BadRequest(String),
    /// <p>You don't have permissions to perform this action.</p>
    Forbidden(String),
    /// <p>The request processing has failed because of an unknown error, exception, or failure.</p>
    InternalServerError(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your account. For best results, use an increasing or variable sleep interval between requests.</p>
    TooManyRequests(String),
}

impl DescribeVirtualGatewayError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeVirtualGatewayError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DescribeVirtualGatewayError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DescribeVirtualGatewayError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DescribeVirtualGatewayError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeVirtualGatewayError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeVirtualGatewayError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeVirtualGatewayError::TooManyRequests(
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
impl fmt::Display for DescribeVirtualGatewayError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeVirtualGatewayError::BadRequest(ref cause) => write!(f, "{}", cause),
            DescribeVirtualGatewayError::Forbidden(ref cause) => write!(f, "{}", cause),
            DescribeVirtualGatewayError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DescribeVirtualGatewayError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribeVirtualGatewayError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DescribeVirtualGatewayError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeVirtualGatewayError {}
/// Errors returned by DescribeVirtualNode
#[derive(Debug, PartialEq)]
pub enum DescribeVirtualNodeError {
    /// <p>The request syntax was malformed. Check your request syntax and try again.</p>
    BadRequest(String),
    /// <p>You don't have permissions to perform this action.</p>
    Forbidden(String),
    /// <p>The request processing has failed because of an unknown error, exception, or failure.</p>
    InternalServerError(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your account. For best results, use an increasing or variable sleep interval between requests.</p>
    TooManyRequests(String),
}

impl DescribeVirtualNodeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeVirtualNodeError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
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
    /// <p>The request processing has failed because of an unknown error, exception, or failure.</p>
    InternalServerError(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your account. For best results, use an increasing or variable sleep interval between requests.</p>
    TooManyRequests(String),
}

impl DescribeVirtualRouterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeVirtualRouterError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
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
    /// <p>The request processing has failed because of an unknown error, exception, or failure.</p>
    InternalServerError(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your account. For best results, use an increasing or variable sleep interval between requests.</p>
    TooManyRequests(String),
}

impl DescribeVirtualServiceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeVirtualServiceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
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
/// Errors returned by ListGatewayRoutes
#[derive(Debug, PartialEq)]
pub enum ListGatewayRoutesError {
    /// <p>The request syntax was malformed. Check your request syntax and try again.</p>
    BadRequest(String),
    /// <p>You don't have permissions to perform this action.</p>
    Forbidden(String),
    /// <p>The request processing has failed because of an unknown error, exception, or failure.</p>
    InternalServerError(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your account. For best results, use an increasing or variable sleep interval between requests.</p>
    TooManyRequests(String),
}

impl ListGatewayRoutesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListGatewayRoutesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListGatewayRoutesError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListGatewayRoutesError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListGatewayRoutesError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListGatewayRoutesError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListGatewayRoutesError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListGatewayRoutesError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListGatewayRoutesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListGatewayRoutesError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListGatewayRoutesError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListGatewayRoutesError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListGatewayRoutesError::NotFound(ref cause) => write!(f, "{}", cause),
            ListGatewayRoutesError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            ListGatewayRoutesError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListGatewayRoutesError {}
/// Errors returned by ListMeshes
#[derive(Debug, PartialEq)]
pub enum ListMeshesError {
    /// <p>The request syntax was malformed. Check your request syntax and try again.</p>
    BadRequest(String),
    /// <p>You don't have permissions to perform this action.</p>
    Forbidden(String),
    /// <p>The request processing has failed because of an unknown error, exception, or failure.</p>
    InternalServerError(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your account. For best results, use an increasing or variable sleep interval between requests.</p>
    TooManyRequests(String),
}

impl ListMeshesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListMeshesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
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
    /// <p>The request processing has failed because of an unknown error, exception, or failure.</p>
    InternalServerError(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your account. For best results, use an increasing or variable sleep interval between requests.</p>
    TooManyRequests(String),
}

impl ListRoutesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListRoutesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
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
    /// <p>The request processing has failed because of an unknown error, exception, or failure.</p>
    InternalServerError(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your account. For best results, use an increasing or variable sleep interval between requests.</p>
    TooManyRequests(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
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
/// Errors returned by ListVirtualGateways
#[derive(Debug, PartialEq)]
pub enum ListVirtualGatewaysError {
    /// <p>The request syntax was malformed. Check your request syntax and try again.</p>
    BadRequest(String),
    /// <p>You don't have permissions to perform this action.</p>
    Forbidden(String),
    /// <p>The request processing has failed because of an unknown error, exception, or failure.</p>
    InternalServerError(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your account. For best results, use an increasing or variable sleep interval between requests.</p>
    TooManyRequests(String),
}

impl ListVirtualGatewaysError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListVirtualGatewaysError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListVirtualGatewaysError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListVirtualGatewaysError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListVirtualGatewaysError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListVirtualGatewaysError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListVirtualGatewaysError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListVirtualGatewaysError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListVirtualGatewaysError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListVirtualGatewaysError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListVirtualGatewaysError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListVirtualGatewaysError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListVirtualGatewaysError::NotFound(ref cause) => write!(f, "{}", cause),
            ListVirtualGatewaysError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            ListVirtualGatewaysError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListVirtualGatewaysError {}
/// Errors returned by ListVirtualNodes
#[derive(Debug, PartialEq)]
pub enum ListVirtualNodesError {
    /// <p>The request syntax was malformed. Check your request syntax and try again.</p>
    BadRequest(String),
    /// <p>You don't have permissions to perform this action.</p>
    Forbidden(String),
    /// <p>The request processing has failed because of an unknown error, exception, or failure.</p>
    InternalServerError(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your account. For best results, use an increasing or variable sleep interval between requests.</p>
    TooManyRequests(String),
}

impl ListVirtualNodesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListVirtualNodesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
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
    /// <p>The request processing has failed because of an unknown error, exception, or failure.</p>
    InternalServerError(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your account. For best results, use an increasing or variable sleep interval between requests.</p>
    TooManyRequests(String),
}

impl ListVirtualRoutersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListVirtualRoutersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
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
    /// <p>The request processing has failed because of an unknown error, exception, or failure.</p>
    InternalServerError(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your account. For best results, use an increasing or variable sleep interval between requests.</p>
    TooManyRequests(String),
}

impl ListVirtualServicesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListVirtualServicesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
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
    /// <p>The request processing has failed because of an unknown error, exception, or failure.</p>
    InternalServerError(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your account. For best results, use an increasing or variable sleep interval between requests.</p>
    TooManyRequests(String),
    /// <p>The request exceeds the maximum allowed number of tags allowed per resource. The current limit is 50 user tags per resource. You must reduce the number of tags in the request. None of the tags in this request were applied.</p>
    TooManyTags(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
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
    /// <p>The request processing has failed because of an unknown error, exception, or failure.</p>
    InternalServerError(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your account. For best results, use an increasing or variable sleep interval between requests.</p>
    TooManyRequests(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
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
/// Errors returned by UpdateGatewayRoute
#[derive(Debug, PartialEq)]
pub enum UpdateGatewayRouteError {
    /// <p>The request syntax was malformed. Check your request syntax and try again.</p>
    BadRequest(String),
    /// <p>The request contains a client token that was used for a previous update resource call with different specifications. Try the request again with a new client token.</p>
    Conflict(String),
    /// <p>You don't have permissions to perform this action.</p>
    Forbidden(String),
    /// <p>The request processing has failed because of an unknown error, exception, or failure.</p>
    InternalServerError(String),
    /// <p>You have exceeded a service limit for your account. For more information, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/service-quotas.html">Service Limits</a> in the <i>AWS App Mesh User Guide</i>.</p>
    LimitExceeded(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your account. For best results, use an increasing or variable sleep interval between requests.</p>
    TooManyRequests(String),
}

impl UpdateGatewayRouteError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateGatewayRouteError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateGatewayRouteError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateGatewayRouteError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateGatewayRouteError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateGatewayRouteError::InternalServerError(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateGatewayRouteError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateGatewayRouteError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdateGatewayRouteError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateGatewayRouteError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateGatewayRouteError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateGatewayRouteError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateGatewayRouteError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateGatewayRouteError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateGatewayRouteError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateGatewayRouteError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateGatewayRouteError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateGatewayRouteError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            UpdateGatewayRouteError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateGatewayRouteError {}
/// Errors returned by UpdateMesh
#[derive(Debug, PartialEq)]
pub enum UpdateMeshError {
    /// <p>The request syntax was malformed. Check your request syntax and try again.</p>
    BadRequest(String),
    /// <p>The request contains a client token that was used for a previous update resource call with different specifications. Try the request again with a new client token.</p>
    Conflict(String),
    /// <p>You don't have permissions to perform this action.</p>
    Forbidden(String),
    /// <p>The request processing has failed because of an unknown error, exception, or failure.</p>
    InternalServerError(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your account. For best results, use an increasing or variable sleep interval between requests.</p>
    TooManyRequests(String),
}

impl UpdateMeshError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateMeshError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
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
    /// <p>The request contains a client token that was used for a previous update resource call with different specifications. Try the request again with a new client token.</p>
    Conflict(String),
    /// <p>You don't have permissions to perform this action.</p>
    Forbidden(String),
    /// <p>The request processing has failed because of an unknown error, exception, or failure.</p>
    InternalServerError(String),
    /// <p>You have exceeded a service limit for your account. For more information, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/service-quotas.html">Service Limits</a> in the <i>AWS App Mesh User Guide</i>.</p>
    LimitExceeded(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your account. For best results, use an increasing or variable sleep interval between requests.</p>
    TooManyRequests(String),
}

impl UpdateRouteError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateRouteError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
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
/// Errors returned by UpdateVirtualGateway
#[derive(Debug, PartialEq)]
pub enum UpdateVirtualGatewayError {
    /// <p>The request syntax was malformed. Check your request syntax and try again.</p>
    BadRequest(String),
    /// <p>The request contains a client token that was used for a previous update resource call with different specifications. Try the request again with a new client token.</p>
    Conflict(String),
    /// <p>You don't have permissions to perform this action.</p>
    Forbidden(String),
    /// <p>The request processing has failed because of an unknown error, exception, or failure.</p>
    InternalServerError(String),
    /// <p>You have exceeded a service limit for your account. For more information, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/service-quotas.html">Service Limits</a> in the <i>AWS App Mesh User Guide</i>.</p>
    LimitExceeded(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your account. For best results, use an increasing or variable sleep interval between requests.</p>
    TooManyRequests(String),
}

impl UpdateVirtualGatewayError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateVirtualGatewayError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateVirtualGatewayError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateVirtualGatewayError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateVirtualGatewayError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateVirtualGatewayError::InternalServerError(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateVirtualGatewayError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateVirtualGatewayError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdateVirtualGatewayError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateVirtualGatewayError::TooManyRequests(
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
impl fmt::Display for UpdateVirtualGatewayError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateVirtualGatewayError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateVirtualGatewayError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateVirtualGatewayError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateVirtualGatewayError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateVirtualGatewayError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateVirtualGatewayError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateVirtualGatewayError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            UpdateVirtualGatewayError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateVirtualGatewayError {}
/// Errors returned by UpdateVirtualNode
#[derive(Debug, PartialEq)]
pub enum UpdateVirtualNodeError {
    /// <p>The request syntax was malformed. Check your request syntax and try again.</p>
    BadRequest(String),
    /// <p>The request contains a client token that was used for a previous update resource call with different specifications. Try the request again with a new client token.</p>
    Conflict(String),
    /// <p>You don't have permissions to perform this action.</p>
    Forbidden(String),
    /// <p>The request processing has failed because of an unknown error, exception, or failure.</p>
    InternalServerError(String),
    /// <p>You have exceeded a service limit for your account. For more information, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/service-quotas.html">Service Limits</a> in the <i>AWS App Mesh User Guide</i>.</p>
    LimitExceeded(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your account. For best results, use an increasing or variable sleep interval between requests.</p>
    TooManyRequests(String),
}

impl UpdateVirtualNodeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateVirtualNodeError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
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
    /// <p>The request contains a client token that was used for a previous update resource call with different specifications. Try the request again with a new client token.</p>
    Conflict(String),
    /// <p>You don't have permissions to perform this action.</p>
    Forbidden(String),
    /// <p>The request processing has failed because of an unknown error, exception, or failure.</p>
    InternalServerError(String),
    /// <p>You have exceeded a service limit for your account. For more information, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/service-quotas.html">Service Limits</a> in the <i>AWS App Mesh User Guide</i>.</p>
    LimitExceeded(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your account. For best results, use an increasing or variable sleep interval between requests.</p>
    TooManyRequests(String),
}

impl UpdateVirtualRouterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateVirtualRouterError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
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
    /// <p>The request contains a client token that was used for a previous update resource call with different specifications. Try the request again with a new client token.</p>
    Conflict(String),
    /// <p>You don't have permissions to perform this action.</p>
    Forbidden(String),
    /// <p>The request processing has failed because of an unknown error, exception, or failure.</p>
    InternalServerError(String),
    /// <p>You have exceeded a service limit for your account. For more information, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/service-quotas.html">Service Limits</a> in the <i>AWS App Mesh User Guide</i>.</p>
    LimitExceeded(String),
    /// <p>The specified resource doesn't exist. Check your request syntax and try again.</p>
    NotFound(String),
    /// <p>The request has failed due to a temporary failure of the service.</p>
    ServiceUnavailable(String),
    /// <p>The maximum request rate permitted by the App Mesh APIs has been exceeded for your account. For best results, use an increasing or variable sleep interval between requests.</p>
    TooManyRequests(String),
}

impl UpdateVirtualServiceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateVirtualServiceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
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
    /// <p>Creates a gateway route.</p> <p>A gateway route is attached to a virtual gateway and routes traffic to an existing virtual service. If a route matches a request, it can distribute traffic to a target virtual service.</p> <p>For more information about gateway routes, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/gateway-routes.html">Gateway routes</a>.</p>
    async fn create_gateway_route(
        &self,
        input: CreateGatewayRouteInput,
    ) -> Result<CreateGatewayRouteOutput, RusotoError<CreateGatewayRouteError>>;

    /// <p>Creates a service mesh.</p> <p> A service mesh is a logical boundary for network traffic between services that are represented by resources within the mesh. After you create your service mesh, you can create virtual services, virtual nodes, virtual routers, and routes to distribute traffic between the applications in your mesh.</p> <p>For more information about service meshes, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/meshes.html">Service meshes</a>.</p>
    async fn create_mesh(
        &self,
        input: CreateMeshInput,
    ) -> Result<CreateMeshOutput, RusotoError<CreateMeshError>>;

    /// <p>Creates a route that is associated with a virtual router.</p> <p> You can route several different protocols and define a retry policy for a route. Traffic can be routed to one or more virtual nodes.</p> <p>For more information about routes, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/routes.html">Routes</a>.</p>
    async fn create_route(
        &self,
        input: CreateRouteInput,
    ) -> Result<CreateRouteOutput, RusotoError<CreateRouteError>>;

    /// <p>Creates a virtual gateway.</p> <p>A virtual gateway allows resources outside your mesh to communicate to resources that are inside your mesh. The virtual gateway represents an Envoy proxy running in an Amazon ECS task, in a Kubernetes service, or on an Amazon EC2 instance. Unlike a virtual node, which represents an Envoy running with an application, a virtual gateway represents Envoy deployed by itself.</p> <p>For more information about virtual gateways, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/virtual_gateways.html">Virtual gateways</a>. </p>
    async fn create_virtual_gateway(
        &self,
        input: CreateVirtualGatewayInput,
    ) -> Result<CreateVirtualGatewayOutput, RusotoError<CreateVirtualGatewayError>>;

    /// <p>Creates a virtual node within a service mesh.</p> <p> A virtual node acts as a logical pointer to a particular task group, such as an Amazon ECS service or a Kubernetes deployment. When you create a virtual node, you can specify the service discovery information for your task group, and whether the proxy running in a task group will communicate with other proxies using Transport Layer Security (TLS).</p> <p>You define a <code>listener</code> for any inbound traffic that your virtual node expects. Any virtual service that your virtual node expects to communicate to is specified as a <code>backend</code>.</p> <p>The response metadata for your new virtual node contains the <code>arn</code> that is associated with the virtual node. Set this value to the full ARN; for example, <code>arn:aws:appmesh:us-west-2:123456789012:myMesh/default/virtualNode/myApp</code>) as the <code>APPMESH_RESOURCE_ARN</code> environment variable for your task group's Envoy proxy container in your task definition or pod spec. This is then mapped to the <code>node.id</code> and <code>node.cluster</code> Envoy parameters.</p> <note> <p>By default, App Mesh uses the name of the resource you specified in <code>APPMESH_RESOURCE_ARN</code> when Envoy is referring to itself in metrics and traces. You can override this behavior by setting the <code>APPMESH_RESOURCE_CLUSTER</code> environment variable with your own name.</p> <p>AWS Cloud Map is not available in the eu-south-1 Region.</p> </note> <p>For more information about virtual nodes, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/virtual_nodes.html">Virtual nodes</a>. You must be using <code>1.15.0</code> or later of the Envoy image when setting these variables. For more information about App Mesh Envoy variables, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/envoy.html">Envoy image</a> in the AWS App Mesh User Guide.</p>
    async fn create_virtual_node(
        &self,
        input: CreateVirtualNodeInput,
    ) -> Result<CreateVirtualNodeOutput, RusotoError<CreateVirtualNodeError>>;

    /// <p>Creates a virtual router within a service mesh.</p> <p>Specify a <code>listener</code> for any inbound traffic that your virtual router receives. Create a virtual router for each protocol and port that you need to route. Virtual routers handle traffic for one or more virtual services within your mesh. After you create your virtual router, create and associate routes for your virtual router that direct incoming requests to different virtual nodes.</p> <p>For more information about virtual routers, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/virtual_routers.html">Virtual routers</a>.</p>
    async fn create_virtual_router(
        &self,
        input: CreateVirtualRouterInput,
    ) -> Result<CreateVirtualRouterOutput, RusotoError<CreateVirtualRouterError>>;

    /// <p>Creates a virtual service within a service mesh.</p> <p>A virtual service is an abstraction of a real service that is provided by a virtual node directly or indirectly by means of a virtual router. Dependent services call your virtual service by its <code>virtualServiceName</code>, and those requests are routed to the virtual node or virtual router that is specified as the provider for the virtual service.</p> <p>For more information about virtual services, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/virtual_services.html">Virtual services</a>.</p>
    async fn create_virtual_service(
        &self,
        input: CreateVirtualServiceInput,
    ) -> Result<CreateVirtualServiceOutput, RusotoError<CreateVirtualServiceError>>;

    /// <p>Deletes an existing gateway route.</p>
    async fn delete_gateway_route(
        &self,
        input: DeleteGatewayRouteInput,
    ) -> Result<DeleteGatewayRouteOutput, RusotoError<DeleteGatewayRouteError>>;

    /// <p>Deletes an existing service mesh.</p> <p>You must delete all resources (virtual services, routes, virtual routers, and virtual nodes) in the service mesh before you can delete the mesh itself.</p>
    async fn delete_mesh(
        &self,
        input: DeleteMeshInput,
    ) -> Result<DeleteMeshOutput, RusotoError<DeleteMeshError>>;

    /// <p>Deletes an existing route.</p>
    async fn delete_route(
        &self,
        input: DeleteRouteInput,
    ) -> Result<DeleteRouteOutput, RusotoError<DeleteRouteError>>;

    /// <p>Deletes an existing virtual gateway. You cannot delete a virtual gateway if any gateway routes are associated to it.</p>
    async fn delete_virtual_gateway(
        &self,
        input: DeleteVirtualGatewayInput,
    ) -> Result<DeleteVirtualGatewayOutput, RusotoError<DeleteVirtualGatewayError>>;

    /// <p>Deletes an existing virtual node.</p> <p>You must delete any virtual services that list a virtual node as a service provider before you can delete the virtual node itself.</p>
    async fn delete_virtual_node(
        &self,
        input: DeleteVirtualNodeInput,
    ) -> Result<DeleteVirtualNodeOutput, RusotoError<DeleteVirtualNodeError>>;

    /// <p>Deletes an existing virtual router.</p> <p>You must delete any routes associated with the virtual router before you can delete the router itself.</p>
    async fn delete_virtual_router(
        &self,
        input: DeleteVirtualRouterInput,
    ) -> Result<DeleteVirtualRouterOutput, RusotoError<DeleteVirtualRouterError>>;

    /// <p>Deletes an existing virtual service.</p>
    async fn delete_virtual_service(
        &self,
        input: DeleteVirtualServiceInput,
    ) -> Result<DeleteVirtualServiceOutput, RusotoError<DeleteVirtualServiceError>>;

    /// <p>Describes an existing gateway route.</p>
    async fn describe_gateway_route(
        &self,
        input: DescribeGatewayRouteInput,
    ) -> Result<DescribeGatewayRouteOutput, RusotoError<DescribeGatewayRouteError>>;

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

    /// <p>Describes an existing virtual gateway.</p>
    async fn describe_virtual_gateway(
        &self,
        input: DescribeVirtualGatewayInput,
    ) -> Result<DescribeVirtualGatewayOutput, RusotoError<DescribeVirtualGatewayError>>;

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

    /// <p>Returns a list of existing gateway routes that are associated to a virtual gateway.</p>
    async fn list_gateway_routes(
        &self,
        input: ListGatewayRoutesInput,
    ) -> Result<ListGatewayRoutesOutput, RusotoError<ListGatewayRoutesError>>;

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

    /// <p>Returns a list of existing virtual gateways in a service mesh.</p>
    async fn list_virtual_gateways(
        &self,
        input: ListVirtualGatewaysInput,
    ) -> Result<ListVirtualGatewaysOutput, RusotoError<ListVirtualGatewaysError>>;

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

    /// <p>Associates the specified tags to a resource with the specified <code>resourceArn</code>. If existing tags on a resource aren't specified in the request parameters, they aren't changed. When a resource is deleted, the tags associated with that resource are also deleted.</p>
    async fn tag_resource(
        &self,
        input: TagResourceInput,
    ) -> Result<TagResourceOutput, RusotoError<TagResourceError>>;

    /// <p>Deletes specified tags from a resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceInput,
    ) -> Result<UntagResourceOutput, RusotoError<UntagResourceError>>;

    /// <p>Updates an existing gateway route that is associated to a specified virtual gateway in a service mesh.</p>
    async fn update_gateway_route(
        &self,
        input: UpdateGatewayRouteInput,
    ) -> Result<UpdateGatewayRouteOutput, RusotoError<UpdateGatewayRouteError>>;

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

    /// <p>Updates an existing virtual gateway in a specified service mesh.</p>
    async fn update_virtual_gateway(
        &self,
        input: UpdateVirtualGatewayInput,
    ) -> Result<UpdateVirtualGatewayOutput, RusotoError<UpdateVirtualGatewayError>>;

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
    /// <p>Creates a gateway route.</p> <p>A gateway route is attached to a virtual gateway and routes traffic to an existing virtual service. If a route matches a request, it can distribute traffic to a target virtual service.</p> <p>For more information about gateway routes, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/gateway-routes.html">Gateway routes</a>.</p>
    #[allow(unused_mut)]
    async fn create_gateway_route(
        &self,
        input: CreateGatewayRouteInput,
    ) -> Result<CreateGatewayRouteOutput, RusotoError<CreateGatewayRouteError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/v20190125/meshes/{mesh_name}/virtualGateway/{virtual_gateway_name}/gatewayRoutes",
            mesh_name = input.mesh_name,
            virtual_gateway_name = input.virtual_gateway_name
        );

        let mut request = SignedRequest::new("PUT", "appmesh", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        if let Some(ref x) = input.mesh_owner {
            params.put("meshOwner", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateGatewayRouteOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateGatewayRouteError::from_response(response))
        }
    }

    /// <p>Creates a service mesh.</p> <p> A service mesh is a logical boundary for network traffic between services that are represented by resources within the mesh. After you create your service mesh, you can create virtual services, virtual nodes, virtual routers, and routes to distribute traffic between the applications in your mesh.</p> <p>For more information about service meshes, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/meshes.html">Service meshes</a>.</p>
    #[allow(unused_mut)]
    async fn create_mesh(
        &self,
        input: CreateMeshInput,
    ) -> Result<CreateMeshOutput, RusotoError<CreateMeshError>> {
        #![allow(clippy::needless_update)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateMeshOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateMeshError::from_response(response))
        }
    }

    /// <p>Creates a route that is associated with a virtual router.</p> <p> You can route several different protocols and define a retry policy for a route. Traffic can be routed to one or more virtual nodes.</p> <p>For more information about routes, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/routes.html">Routes</a>.</p>
    #[allow(unused_mut)]
    async fn create_route(
        &self,
        input: CreateRouteInput,
    ) -> Result<CreateRouteOutput, RusotoError<CreateRouteError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/v20190125/meshes/{mesh_name}/virtualRouter/{virtual_router_name}/routes",
            mesh_name = input.mesh_name,
            virtual_router_name = input.virtual_router_name
        );

        let mut request = SignedRequest::new("PUT", "appmesh", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        if let Some(ref x) = input.mesh_owner {
            params.put("meshOwner", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateRouteOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateRouteError::from_response(response))
        }
    }

    /// <p>Creates a virtual gateway.</p> <p>A virtual gateway allows resources outside your mesh to communicate to resources that are inside your mesh. The virtual gateway represents an Envoy proxy running in an Amazon ECS task, in a Kubernetes service, or on an Amazon EC2 instance. Unlike a virtual node, which represents an Envoy running with an application, a virtual gateway represents Envoy deployed by itself.</p> <p>For more information about virtual gateways, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/virtual_gateways.html">Virtual gateways</a>. </p>
    #[allow(unused_mut)]
    async fn create_virtual_gateway(
        &self,
        input: CreateVirtualGatewayInput,
    ) -> Result<CreateVirtualGatewayOutput, RusotoError<CreateVirtualGatewayError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/v20190125/meshes/{mesh_name}/virtualGateways",
            mesh_name = input.mesh_name
        );

        let mut request = SignedRequest::new("PUT", "appmesh", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        if let Some(ref x) = input.mesh_owner {
            params.put("meshOwner", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateVirtualGatewayOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateVirtualGatewayError::from_response(response))
        }
    }

    /// <p>Creates a virtual node within a service mesh.</p> <p> A virtual node acts as a logical pointer to a particular task group, such as an Amazon ECS service or a Kubernetes deployment. When you create a virtual node, you can specify the service discovery information for your task group, and whether the proxy running in a task group will communicate with other proxies using Transport Layer Security (TLS).</p> <p>You define a <code>listener</code> for any inbound traffic that your virtual node expects. Any virtual service that your virtual node expects to communicate to is specified as a <code>backend</code>.</p> <p>The response metadata for your new virtual node contains the <code>arn</code> that is associated with the virtual node. Set this value to the full ARN; for example, <code>arn:aws:appmesh:us-west-2:123456789012:myMesh/default/virtualNode/myApp</code>) as the <code>APPMESH_RESOURCE_ARN</code> environment variable for your task group's Envoy proxy container in your task definition or pod spec. This is then mapped to the <code>node.id</code> and <code>node.cluster</code> Envoy parameters.</p> <note> <p>By default, App Mesh uses the name of the resource you specified in <code>APPMESH_RESOURCE_ARN</code> when Envoy is referring to itself in metrics and traces. You can override this behavior by setting the <code>APPMESH_RESOURCE_CLUSTER</code> environment variable with your own name.</p> <p>AWS Cloud Map is not available in the eu-south-1 Region.</p> </note> <p>For more information about virtual nodes, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/virtual_nodes.html">Virtual nodes</a>. You must be using <code>1.15.0</code> or later of the Envoy image when setting these variables. For more information about App Mesh Envoy variables, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/envoy.html">Envoy image</a> in the AWS App Mesh User Guide.</p>
    #[allow(unused_mut)]
    async fn create_virtual_node(
        &self,
        input: CreateVirtualNodeInput,
    ) -> Result<CreateVirtualNodeOutput, RusotoError<CreateVirtualNodeError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/v20190125/meshes/{mesh_name}/virtualNodes",
            mesh_name = input.mesh_name
        );

        let mut request = SignedRequest::new("PUT", "appmesh", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        if let Some(ref x) = input.mesh_owner {
            params.put("meshOwner", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateVirtualNodeOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateVirtualNodeError::from_response(response))
        }
    }

    /// <p>Creates a virtual router within a service mesh.</p> <p>Specify a <code>listener</code> for any inbound traffic that your virtual router receives. Create a virtual router for each protocol and port that you need to route. Virtual routers handle traffic for one or more virtual services within your mesh. After you create your virtual router, create and associate routes for your virtual router that direct incoming requests to different virtual nodes.</p> <p>For more information about virtual routers, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/virtual_routers.html">Virtual routers</a>.</p>
    #[allow(unused_mut)]
    async fn create_virtual_router(
        &self,
        input: CreateVirtualRouterInput,
    ) -> Result<CreateVirtualRouterOutput, RusotoError<CreateVirtualRouterError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/v20190125/meshes/{mesh_name}/virtualRouters",
            mesh_name = input.mesh_name
        );

        let mut request = SignedRequest::new("PUT", "appmesh", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        if let Some(ref x) = input.mesh_owner {
            params.put("meshOwner", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateVirtualRouterOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateVirtualRouterError::from_response(response))
        }
    }

    /// <p>Creates a virtual service within a service mesh.</p> <p>A virtual service is an abstraction of a real service that is provided by a virtual node directly or indirectly by means of a virtual router. Dependent services call your virtual service by its <code>virtualServiceName</code>, and those requests are routed to the virtual node or virtual router that is specified as the provider for the virtual service.</p> <p>For more information about virtual services, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/virtual_services.html">Virtual services</a>.</p>
    #[allow(unused_mut)]
    async fn create_virtual_service(
        &self,
        input: CreateVirtualServiceInput,
    ) -> Result<CreateVirtualServiceOutput, RusotoError<CreateVirtualServiceError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/v20190125/meshes/{mesh_name}/virtualServices",
            mesh_name = input.mesh_name
        );

        let mut request = SignedRequest::new("PUT", "appmesh", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        if let Some(ref x) = input.mesh_owner {
            params.put("meshOwner", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateVirtualServiceOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateVirtualServiceError::from_response(response))
        }
    }

    /// <p>Deletes an existing gateway route.</p>
    #[allow(unused_mut)]
    async fn delete_gateway_route(
        &self,
        input: DeleteGatewayRouteInput,
    ) -> Result<DeleteGatewayRouteOutput, RusotoError<DeleteGatewayRouteError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!("/v20190125/meshes/{mesh_name}/virtualGateway/{virtual_gateway_name}/gatewayRoutes/{gateway_route_name}", gateway_route_name = input.gateway_route_name, mesh_name = input.mesh_name, virtual_gateway_name = input.virtual_gateway_name);

        let mut request = SignedRequest::new("DELETE", "appmesh", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.mesh_owner {
            params.put("meshOwner", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteGatewayRouteOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteGatewayRouteError::from_response(response))
        }
    }

    /// <p>Deletes an existing service mesh.</p> <p>You must delete all resources (virtual services, routes, virtual routers, and virtual nodes) in the service mesh before you can delete the mesh itself.</p>
    #[allow(unused_mut)]
    async fn delete_mesh(
        &self,
        input: DeleteMeshInput,
    ) -> Result<DeleteMeshOutput, RusotoError<DeleteMeshError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!("/v20190125/meshes/{mesh_name}", mesh_name = input.mesh_name);

        let mut request = SignedRequest::new("DELETE", "appmesh", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteMeshOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteMeshError::from_response(response))
        }
    }

    /// <p>Deletes an existing route.</p>
    #[allow(unused_mut)]
    async fn delete_route(
        &self,
        input: DeleteRouteInput,
    ) -> Result<DeleteRouteOutput, RusotoError<DeleteRouteError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/v20190125/meshes/{mesh_name}/virtualRouter/{virtual_router_name}/routes/{route_name}",
            mesh_name = input.mesh_name,
            route_name = input.route_name,
            virtual_router_name = input.virtual_router_name
        );

        let mut request = SignedRequest::new("DELETE", "appmesh", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.mesh_owner {
            params.put("meshOwner", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteRouteOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteRouteError::from_response(response))
        }
    }

    /// <p>Deletes an existing virtual gateway. You cannot delete a virtual gateway if any gateway routes are associated to it.</p>
    #[allow(unused_mut)]
    async fn delete_virtual_gateway(
        &self,
        input: DeleteVirtualGatewayInput,
    ) -> Result<DeleteVirtualGatewayOutput, RusotoError<DeleteVirtualGatewayError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/v20190125/meshes/{mesh_name}/virtualGateways/{virtual_gateway_name}",
            mesh_name = input.mesh_name,
            virtual_gateway_name = input.virtual_gateway_name
        );

        let mut request = SignedRequest::new("DELETE", "appmesh", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.mesh_owner {
            params.put("meshOwner", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteVirtualGatewayOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteVirtualGatewayError::from_response(response))
        }
    }

    /// <p>Deletes an existing virtual node.</p> <p>You must delete any virtual services that list a virtual node as a service provider before you can delete the virtual node itself.</p>
    #[allow(unused_mut)]
    async fn delete_virtual_node(
        &self,
        input: DeleteVirtualNodeInput,
    ) -> Result<DeleteVirtualNodeOutput, RusotoError<DeleteVirtualNodeError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/v20190125/meshes/{mesh_name}/virtualNodes/{virtual_node_name}",
            mesh_name = input.mesh_name,
            virtual_node_name = input.virtual_node_name
        );

        let mut request = SignedRequest::new("DELETE", "appmesh", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.mesh_owner {
            params.put("meshOwner", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteVirtualNodeOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteVirtualNodeError::from_response(response))
        }
    }

    /// <p>Deletes an existing virtual router.</p> <p>You must delete any routes associated with the virtual router before you can delete the router itself.</p>
    #[allow(unused_mut)]
    async fn delete_virtual_router(
        &self,
        input: DeleteVirtualRouterInput,
    ) -> Result<DeleteVirtualRouterOutput, RusotoError<DeleteVirtualRouterError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/v20190125/meshes/{mesh_name}/virtualRouters/{virtual_router_name}",
            mesh_name = input.mesh_name,
            virtual_router_name = input.virtual_router_name
        );

        let mut request = SignedRequest::new("DELETE", "appmesh", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.mesh_owner {
            params.put("meshOwner", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteVirtualRouterOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteVirtualRouterError::from_response(response))
        }
    }

    /// <p>Deletes an existing virtual service.</p>
    #[allow(unused_mut)]
    async fn delete_virtual_service(
        &self,
        input: DeleteVirtualServiceInput,
    ) -> Result<DeleteVirtualServiceOutput, RusotoError<DeleteVirtualServiceError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/v20190125/meshes/{mesh_name}/virtualServices/{virtual_service_name}",
            mesh_name = input.mesh_name,
            virtual_service_name = input.virtual_service_name
        );

        let mut request = SignedRequest::new("DELETE", "appmesh", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.mesh_owner {
            params.put("meshOwner", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteVirtualServiceOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteVirtualServiceError::from_response(response))
        }
    }

    /// <p>Describes an existing gateway route.</p>
    #[allow(unused_mut)]
    async fn describe_gateway_route(
        &self,
        input: DescribeGatewayRouteInput,
    ) -> Result<DescribeGatewayRouteOutput, RusotoError<DescribeGatewayRouteError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!("/v20190125/meshes/{mesh_name}/virtualGateway/{virtual_gateway_name}/gatewayRoutes/{gateway_route_name}", gateway_route_name = input.gateway_route_name, mesh_name = input.mesh_name, virtual_gateway_name = input.virtual_gateway_name);

        let mut request = SignedRequest::new("GET", "appmesh", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.mesh_owner {
            params.put("meshOwner", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeGatewayRouteOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeGatewayRouteError::from_response(response))
        }
    }

    /// <p>Describes an existing service mesh.</p>
    #[allow(unused_mut)]
    async fn describe_mesh(
        &self,
        input: DescribeMeshInput,
    ) -> Result<DescribeMeshOutput, RusotoError<DescribeMeshError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!("/v20190125/meshes/{mesh_name}", mesh_name = input.mesh_name);

        let mut request = SignedRequest::new("GET", "appmesh", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.mesh_owner {
            params.put("meshOwner", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeMeshOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeMeshError::from_response(response))
        }
    }

    /// <p>Describes an existing route.</p>
    #[allow(unused_mut)]
    async fn describe_route(
        &self,
        input: DescribeRouteInput,
    ) -> Result<DescribeRouteOutput, RusotoError<DescribeRouteError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/v20190125/meshes/{mesh_name}/virtualRouter/{virtual_router_name}/routes/{route_name}",
            mesh_name = input.mesh_name,
            route_name = input.route_name,
            virtual_router_name = input.virtual_router_name
        );

        let mut request = SignedRequest::new("GET", "appmesh", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.mesh_owner {
            params.put("meshOwner", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeRouteOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeRouteError::from_response(response))
        }
    }

    /// <p>Describes an existing virtual gateway.</p>
    #[allow(unused_mut)]
    async fn describe_virtual_gateway(
        &self,
        input: DescribeVirtualGatewayInput,
    ) -> Result<DescribeVirtualGatewayOutput, RusotoError<DescribeVirtualGatewayError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/v20190125/meshes/{mesh_name}/virtualGateways/{virtual_gateway_name}",
            mesh_name = input.mesh_name,
            virtual_gateway_name = input.virtual_gateway_name
        );

        let mut request = SignedRequest::new("GET", "appmesh", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.mesh_owner {
            params.put("meshOwner", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeVirtualGatewayOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeVirtualGatewayError::from_response(response))
        }
    }

    /// <p>Describes an existing virtual node.</p>
    #[allow(unused_mut)]
    async fn describe_virtual_node(
        &self,
        input: DescribeVirtualNodeInput,
    ) -> Result<DescribeVirtualNodeOutput, RusotoError<DescribeVirtualNodeError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/v20190125/meshes/{mesh_name}/virtualNodes/{virtual_node_name}",
            mesh_name = input.mesh_name,
            virtual_node_name = input.virtual_node_name
        );

        let mut request = SignedRequest::new("GET", "appmesh", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.mesh_owner {
            params.put("meshOwner", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeVirtualNodeOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeVirtualNodeError::from_response(response))
        }
    }

    /// <p>Describes an existing virtual router.</p>
    #[allow(unused_mut)]
    async fn describe_virtual_router(
        &self,
        input: DescribeVirtualRouterInput,
    ) -> Result<DescribeVirtualRouterOutput, RusotoError<DescribeVirtualRouterError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/v20190125/meshes/{mesh_name}/virtualRouters/{virtual_router_name}",
            mesh_name = input.mesh_name,
            virtual_router_name = input.virtual_router_name
        );

        let mut request = SignedRequest::new("GET", "appmesh", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.mesh_owner {
            params.put("meshOwner", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeVirtualRouterOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeVirtualRouterError::from_response(response))
        }
    }

    /// <p>Describes an existing virtual service.</p>
    #[allow(unused_mut)]
    async fn describe_virtual_service(
        &self,
        input: DescribeVirtualServiceInput,
    ) -> Result<DescribeVirtualServiceOutput, RusotoError<DescribeVirtualServiceError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/v20190125/meshes/{mesh_name}/virtualServices/{virtual_service_name}",
            mesh_name = input.mesh_name,
            virtual_service_name = input.virtual_service_name
        );

        let mut request = SignedRequest::new("GET", "appmesh", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.mesh_owner {
            params.put("meshOwner", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeVirtualServiceOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeVirtualServiceError::from_response(response))
        }
    }

    /// <p>Returns a list of existing gateway routes that are associated to a virtual gateway.</p>
    #[allow(unused_mut)]
    async fn list_gateway_routes(
        &self,
        input: ListGatewayRoutesInput,
    ) -> Result<ListGatewayRoutesOutput, RusotoError<ListGatewayRoutesError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/v20190125/meshes/{mesh_name}/virtualGateway/{virtual_gateway_name}/gatewayRoutes",
            mesh_name = input.mesh_name,
            virtual_gateway_name = input.virtual_gateway_name
        );

        let mut request = SignedRequest::new("GET", "appmesh", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.limit {
            params.put("limit", x);
        }
        if let Some(ref x) = input.mesh_owner {
            params.put("meshOwner", x);
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListGatewayRoutesOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListGatewayRoutesError::from_response(response))
        }
    }

    /// <p>Returns a list of existing service meshes.</p>
    #[allow(unused_mut)]
    async fn list_meshes(
        &self,
        input: ListMeshesInput,
    ) -> Result<ListMeshesOutput, RusotoError<ListMeshesError>> {
        #![allow(clippy::needless_update)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListMeshesOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListMeshesError::from_response(response))
        }
    }

    /// <p>Returns a list of existing routes in a service mesh.</p>
    #[allow(unused_mut)]
    async fn list_routes(
        &self,
        input: ListRoutesInput,
    ) -> Result<ListRoutesOutput, RusotoError<ListRoutesError>> {
        #![allow(clippy::needless_update)]
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
        if let Some(ref x) = input.mesh_owner {
            params.put("meshOwner", x);
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListRoutesOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListRoutesError::from_response(response))
        }
    }

    /// <p>List the tags for an App Mesh resource.</p>
    #[allow(unused_mut)]
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceInput,
    ) -> Result<ListTagsForResourceOutput, RusotoError<ListTagsForResourceError>> {
        #![allow(clippy::needless_update)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListTagsForResourceOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListTagsForResourceError::from_response(response))
        }
    }

    /// <p>Returns a list of existing virtual gateways in a service mesh.</p>
    #[allow(unused_mut)]
    async fn list_virtual_gateways(
        &self,
        input: ListVirtualGatewaysInput,
    ) -> Result<ListVirtualGatewaysOutput, RusotoError<ListVirtualGatewaysError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/v20190125/meshes/{mesh_name}/virtualGateways",
            mesh_name = input.mesh_name
        );

        let mut request = SignedRequest::new("GET", "appmesh", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.limit {
            params.put("limit", x);
        }
        if let Some(ref x) = input.mesh_owner {
            params.put("meshOwner", x);
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListVirtualGatewaysOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListVirtualGatewaysError::from_response(response))
        }
    }

    /// <p>Returns a list of existing virtual nodes.</p>
    #[allow(unused_mut)]
    async fn list_virtual_nodes(
        &self,
        input: ListVirtualNodesInput,
    ) -> Result<ListVirtualNodesOutput, RusotoError<ListVirtualNodesError>> {
        #![allow(clippy::needless_update)]
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
        if let Some(ref x) = input.mesh_owner {
            params.put("meshOwner", x);
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListVirtualNodesOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListVirtualNodesError::from_response(response))
        }
    }

    /// <p>Returns a list of existing virtual routers in a service mesh.</p>
    #[allow(unused_mut)]
    async fn list_virtual_routers(
        &self,
        input: ListVirtualRoutersInput,
    ) -> Result<ListVirtualRoutersOutput, RusotoError<ListVirtualRoutersError>> {
        #![allow(clippy::needless_update)]
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
        if let Some(ref x) = input.mesh_owner {
            params.put("meshOwner", x);
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListVirtualRoutersOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListVirtualRoutersError::from_response(response))
        }
    }

    /// <p>Returns a list of existing virtual services in a service mesh.</p>
    #[allow(unused_mut)]
    async fn list_virtual_services(
        &self,
        input: ListVirtualServicesInput,
    ) -> Result<ListVirtualServicesOutput, RusotoError<ListVirtualServicesError>> {
        #![allow(clippy::needless_update)]
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
        if let Some(ref x) = input.mesh_owner {
            params.put("meshOwner", x);
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListVirtualServicesOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListVirtualServicesError::from_response(response))
        }
    }

    /// <p>Associates the specified tags to a resource with the specified <code>resourceArn</code>. If existing tags on a resource aren't specified in the request parameters, they aren't changed. When a resource is deleted, the tags associated with that resource are also deleted.</p>
    #[allow(unused_mut)]
    async fn tag_resource(
        &self,
        input: TagResourceInput,
    ) -> Result<TagResourceOutput, RusotoError<TagResourceError>> {
        #![allow(clippy::needless_update)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<TagResourceOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(TagResourceError::from_response(response))
        }
    }

    /// <p>Deletes specified tags from a resource.</p>
    #[allow(unused_mut)]
    async fn untag_resource(
        &self,
        input: UntagResourceInput,
    ) -> Result<UntagResourceOutput, RusotoError<UntagResourceError>> {
        #![allow(clippy::needless_update)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UntagResourceOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UntagResourceError::from_response(response))
        }
    }

    /// <p>Updates an existing gateway route that is associated to a specified virtual gateway in a service mesh.</p>
    #[allow(unused_mut)]
    async fn update_gateway_route(
        &self,
        input: UpdateGatewayRouteInput,
    ) -> Result<UpdateGatewayRouteOutput, RusotoError<UpdateGatewayRouteError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!("/v20190125/meshes/{mesh_name}/virtualGateway/{virtual_gateway_name}/gatewayRoutes/{gateway_route_name}", gateway_route_name = input.gateway_route_name, mesh_name = input.mesh_name, virtual_gateway_name = input.virtual_gateway_name);

        let mut request = SignedRequest::new("PUT", "appmesh", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        if let Some(ref x) = input.mesh_owner {
            params.put("meshOwner", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateGatewayRouteOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateGatewayRouteError::from_response(response))
        }
    }

    /// <p>Updates an existing service mesh.</p>
    #[allow(unused_mut)]
    async fn update_mesh(
        &self,
        input: UpdateMeshInput,
    ) -> Result<UpdateMeshOutput, RusotoError<UpdateMeshError>> {
        #![allow(clippy::needless_update)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateMeshOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateMeshError::from_response(response))
        }
    }

    /// <p>Updates an existing route for a specified service mesh and virtual router.</p>
    #[allow(unused_mut)]
    async fn update_route(
        &self,
        input: UpdateRouteInput,
    ) -> Result<UpdateRouteOutput, RusotoError<UpdateRouteError>> {
        #![allow(clippy::needless_update)]
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

        let mut params = Params::new();
        if let Some(ref x) = input.mesh_owner {
            params.put("meshOwner", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateRouteOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateRouteError::from_response(response))
        }
    }

    /// <p>Updates an existing virtual gateway in a specified service mesh.</p>
    #[allow(unused_mut)]
    async fn update_virtual_gateway(
        &self,
        input: UpdateVirtualGatewayInput,
    ) -> Result<UpdateVirtualGatewayOutput, RusotoError<UpdateVirtualGatewayError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/v20190125/meshes/{mesh_name}/virtualGateways/{virtual_gateway_name}",
            mesh_name = input.mesh_name,
            virtual_gateway_name = input.virtual_gateway_name
        );

        let mut request = SignedRequest::new("PUT", "appmesh", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        if let Some(ref x) = input.mesh_owner {
            params.put("meshOwner", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateVirtualGatewayOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateVirtualGatewayError::from_response(response))
        }
    }

    /// <p>Updates an existing virtual node in a specified service mesh.</p>
    #[allow(unused_mut)]
    async fn update_virtual_node(
        &self,
        input: UpdateVirtualNodeInput,
    ) -> Result<UpdateVirtualNodeOutput, RusotoError<UpdateVirtualNodeError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/v20190125/meshes/{mesh_name}/virtualNodes/{virtual_node_name}",
            mesh_name = input.mesh_name,
            virtual_node_name = input.virtual_node_name
        );

        let mut request = SignedRequest::new("PUT", "appmesh", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        if let Some(ref x) = input.mesh_owner {
            params.put("meshOwner", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateVirtualNodeOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateVirtualNodeError::from_response(response))
        }
    }

    /// <p>Updates an existing virtual router in a specified service mesh.</p>
    #[allow(unused_mut)]
    async fn update_virtual_router(
        &self,
        input: UpdateVirtualRouterInput,
    ) -> Result<UpdateVirtualRouterOutput, RusotoError<UpdateVirtualRouterError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/v20190125/meshes/{mesh_name}/virtualRouters/{virtual_router_name}",
            mesh_name = input.mesh_name,
            virtual_router_name = input.virtual_router_name
        );

        let mut request = SignedRequest::new("PUT", "appmesh", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        if let Some(ref x) = input.mesh_owner {
            params.put("meshOwner", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateVirtualRouterOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateVirtualRouterError::from_response(response))
        }
    }

    /// <p>Updates an existing virtual service in a specified service mesh.</p>
    #[allow(unused_mut)]
    async fn update_virtual_service(
        &self,
        input: UpdateVirtualServiceInput,
    ) -> Result<UpdateVirtualServiceOutput, RusotoError<UpdateVirtualServiceError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/v20190125/meshes/{mesh_name}/virtualServices/{virtual_service_name}",
            mesh_name = input.mesh_name,
            virtual_service_name = input.virtual_service_name
        );

        let mut request = SignedRequest::new("PUT", "appmesh", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        if let Some(ref x) = input.mesh_owner {
            params.put("meshOwner", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateVirtualServiceOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateVirtualServiceError::from_response(response))
        }
    }
}
