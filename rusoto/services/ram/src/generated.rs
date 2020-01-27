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
#[allow(warnings)]
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError};

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AcceptResourceShareInvitationRequest {
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the invitation.</p>
    #[serde(rename = "resourceShareInvitationArn")]
    pub resource_share_invitation_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AcceptResourceShareInvitationResponse {
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>Information about the invitation.</p>
    #[serde(rename = "resourceShareInvitation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share_invitation: Option<ResourceShareInvitation>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssociateResourceSharePermissionRequest {
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The ARN of the AWS RAM permission to associate with the resource share.</p>
    #[serde(rename = "permissionArn")]
    pub permission_arn: String,
    /// <p>Indicates whether the permission should replace the permissions that are currently associated with the resource share. Use <code>true</code> to replace the current permissions. Use <code>false</code> to add the permission to the current permission.</p>
    #[serde(rename = "replace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace: Option<bool>,
    /// <p>The Amazon Resource Name (ARN) of the resource share.</p>
    #[serde(rename = "resourceShareArn")]
    pub resource_share_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociateResourceSharePermissionResponse {
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>Indicates whether the request succeeded.</p>
    #[serde(rename = "returnValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_value: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssociateResourceShareRequest {
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The principals.</p>
    #[serde(rename = "principals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principals: Option<Vec<String>>,
    /// <p>The Amazon Resource Names (ARN) of the resources.</p>
    #[serde(rename = "resourceArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arns: Option<Vec<String>>,
    /// <p>The Amazon Resource Name (ARN) of the resource share.</p>
    #[serde(rename = "resourceShareArn")]
    pub resource_share_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociateResourceShareResponse {
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>Information about the associations.</p>
    #[serde(rename = "resourceShareAssociations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share_associations: Option<Vec<ResourceShareAssociation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateResourceShareRequest {
    /// <p>Indicates whether principals outside your AWS organization can be associated with a resource share.</p>
    #[serde(rename = "allowExternalPrincipals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_external_principals: Option<bool>,
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The name of the resource share.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The ARNs of the permissions to associate with the resource share. If you do not specify an ARN for the permission, AWS RAM automatically attaches the default version of the permission for each resource type.</p>
    #[serde(rename = "permissionArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_arns: Option<Vec<String>>,
    /// <p>The principals to associate with the resource share. The possible values are IDs of AWS accounts, the ARN of an OU or organization from AWS Organizations.</p>
    #[serde(rename = "principals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principals: Option<Vec<String>>,
    /// <p>The Amazon Resource Names (ARN) of the resources to associate with the resource share.</p>
    #[serde(rename = "resourceArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arns: Option<Vec<String>>,
    /// <p>One or more tags.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateResourceShareResponse {
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>Information about the resource share.</p>
    #[serde(rename = "resourceShare")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share: Option<ResourceShare>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteResourceShareRequest {
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the resource share.</p>
    #[serde(rename = "resourceShareArn")]
    pub resource_share_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteResourceShareResponse {
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>Indicates whether the request succeeded.</p>
    #[serde(rename = "returnValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_value: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateResourceSharePermissionRequest {
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The ARN of the permission to disassociate from the resource share.</p>
    #[serde(rename = "permissionArn")]
    pub permission_arn: String,
    /// <p>The Amazon Resource Name (ARN) of the resource share.</p>
    #[serde(rename = "resourceShareArn")]
    pub resource_share_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisassociateResourceSharePermissionResponse {
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>Indicates whether the request succeeded.</p>
    #[serde(rename = "returnValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_value: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateResourceShareRequest {
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The principals.</p>
    #[serde(rename = "principals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principals: Option<Vec<String>>,
    /// <p>The Amazon Resource Names (ARNs) of the resources.</p>
    #[serde(rename = "resourceArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arns: Option<Vec<String>>,
    /// <p>The Amazon Resource Name (ARN) of the resource share.</p>
    #[serde(rename = "resourceShareArn")]
    pub resource_share_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisassociateResourceShareResponse {
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>Information about the associations.</p>
    #[serde(rename = "resourceShareAssociations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share_associations: Option<Vec<ResourceShareAssociation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EnableSharingWithAwsOrganizationRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EnableSharingWithAwsOrganizationResponse {
    /// <p>Indicates whether the request succeeded.</p>
    #[serde(rename = "returnValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_value: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetPermissionRequest {
    /// <p>The ARN of the permission.</p>
    #[serde(rename = "permissionArn")]
    pub permission_arn: String,
    /// <p>The identifier for the version of the permission.</p>
    #[serde(rename = "permissionVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_version: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetPermissionResponse {
    /// <p>Information about the permission.</p>
    #[serde(rename = "permission")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission: Option<ResourceSharePermissionDetail>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetResourcePoliciesRequest {
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next page of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The principal.</p>
    #[serde(rename = "principal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal: Option<String>,
    /// <p>The Amazon Resource Names (ARN) of the resources.</p>
    #[serde(rename = "resourceArns")]
    pub resource_arns: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetResourcePoliciesResponse {
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A key policy document, in JSON format.</p>
    #[serde(rename = "policies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetResourceShareAssociationsRequest {
    /// <p>The association status.</p>
    #[serde(rename = "associationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_status: Option<String>,
    /// <p>The association type. Specify <code>PRINCIPAL</code> to list the principals that are associated with the specified resource share. Specify <code>RESOURCE</code> to list the resources that are associated with the specified resource share.</p>
    #[serde(rename = "associationType")]
    pub association_type: String,
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next page of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The principal. You cannot specify this parameter if the association type is <code>RESOURCE</code>.</p>
    #[serde(rename = "principal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the resource. You cannot specify this parameter if the association type is <code>PRINCIPAL</code>.</p>
    #[serde(rename = "resourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    /// <p>The Amazon Resource Names (ARN) of the resource shares.</p>
    #[serde(rename = "resourceShareArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share_arns: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetResourceShareAssociationsResponse {
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the associations.</p>
    #[serde(rename = "resourceShareAssociations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share_associations: Option<Vec<ResourceShareAssociation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetResourceShareInvitationsRequest {
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next page of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The Amazon Resource Names (ARN) of the resource shares.</p>
    #[serde(rename = "resourceShareArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share_arns: Option<Vec<String>>,
    /// <p>The Amazon Resource Names (ARN) of the invitations.</p>
    #[serde(rename = "resourceShareInvitationArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share_invitation_arns: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetResourceShareInvitationsResponse {
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the invitations.</p>
    #[serde(rename = "resourceShareInvitations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share_invitations: Option<Vec<ResourceShareInvitation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetResourceSharesRequest {
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The name of the resource share.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The token for the next page of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The type of owner.</p>
    #[serde(rename = "resourceOwner")]
    pub resource_owner: String,
    /// <p>The Amazon Resource Names (ARN) of the resource shares.</p>
    #[serde(rename = "resourceShareArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share_arns: Option<Vec<String>>,
    /// <p>The status of the resource share.</p>
    #[serde(rename = "resourceShareStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share_status: Option<String>,
    /// <p>One or more tag filters.</p>
    #[serde(rename = "tagFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_filters: Option<Vec<TagFilter>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetResourceSharesResponse {
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the resource shares.</p>
    #[serde(rename = "resourceShares")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_shares: Option<Vec<ResourceShare>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListPendingInvitationResourcesRequest {
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next page of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the invitation.</p>
    #[serde(rename = "resourceShareInvitationArn")]
    pub resource_share_invitation_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListPendingInvitationResourcesResponse {
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the resources included the resource share.</p>
    #[serde(rename = "resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<Resource>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListPermissionsRequest {
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next page of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Specifies the resource type for which to list permissions. For example, to list only permissions that apply to EC2 subnets, specify <code>ec2:Subnet</code>.</p>
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListPermissionsResponse {
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the permissions.</p>
    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<ResourceSharePermissionSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListPrincipalsRequest {
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next page of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The principals.</p>
    #[serde(rename = "principals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principals: Option<Vec<String>>,
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "resourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    /// <p>The type of owner.</p>
    #[serde(rename = "resourceOwner")]
    pub resource_owner: String,
    /// <p>The Amazon Resource Names (ARN) of the resource shares.</p>
    #[serde(rename = "resourceShareArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share_arns: Option<Vec<String>>,
    /// <p>The resource type.</p> <p>Valid values: <code>ec2:CapacityReservation</code> | <code>ec2:Subnet</code> | <code>ec2:TrafficMirrorTarget</code> | <code>ec2:TransitGateway</code> | <code>license-manager:LicenseConfiguration</code> | <code>rds:Cluster</code> | <code>route53resolver:ResolverRule</code> I <code>resource-groups:Group</code> </p>
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListPrincipalsResponse {
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The principals.</p>
    #[serde(rename = "principals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principals: Option<Vec<Principal>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListResourceSharePermissionsRequest {
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next page of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the resource share.</p>
    #[serde(rename = "resourceShareArn")]
    pub resource_share_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListResourceSharePermissionsResponse {
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The permissions associated with the resource share.</p>
    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<ResourceSharePermissionSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListResourcesRequest {
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next page of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The principal.</p>
    #[serde(rename = "principal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal: Option<String>,
    /// <p>The Amazon Resource Names (ARN) of the resources.</p>
    #[serde(rename = "resourceArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arns: Option<Vec<String>>,
    /// <p>The type of owner.</p>
    #[serde(rename = "resourceOwner")]
    pub resource_owner: String,
    /// <p>The Amazon Resource Names (ARN) of the resource shares.</p>
    #[serde(rename = "resourceShareArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share_arns: Option<Vec<String>>,
    /// <p>The resource type.</p> <p>Valid values: <code>ec2:CapacityReservation</code> | <code>ec2:Subnet</code> | <code>ec2:TrafficMirrorTarget</code> | <code>ec2:TransitGateway</code> | <code>license-manager:LicenseConfiguration</code> | <code>rds:Cluster</code> | <code>route53resolver:ResolverRule</code> | <code>resource-groups:Group</code> </p>
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListResourcesResponse {
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the resources.</p>
    #[serde(rename = "resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<Resource>>,
}

/// <p>Describes a principal for use with AWS Resource Access Manager.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Principal {
    /// <p>The time when the principal was associated with the resource share.</p>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>Indicates whether the principal belongs to the same AWS organization as the AWS account that owns the resource share.</p>
    #[serde(rename = "external")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external: Option<bool>,
    /// <p>The ID of the principal.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The time when the association was last updated.</p>
    #[serde(rename = "lastUpdatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the resource share.</p>
    #[serde(rename = "resourceShareArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PromoteResourceShareCreatedFromPolicyRequest {
    /// <p>The ARN of the resource share to promote.</p>
    #[serde(rename = "resourceShareArn")]
    pub resource_share_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PromoteResourceShareCreatedFromPolicyResponse {
    /// <p>Indicates whether the request succeeded.</p>
    #[serde(rename = "returnValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_value: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RejectResourceShareInvitationRequest {
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the invitation.</p>
    #[serde(rename = "resourceShareInvitationArn")]
    pub resource_share_invitation_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RejectResourceShareInvitationResponse {
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>Information about the invitation.</p>
    #[serde(rename = "resourceShareInvitation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share_invitation: Option<ResourceShareInvitation>,
}

/// <p>Describes a resource associated with a resource share.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Resource {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time when the resource was associated with the resource share.</p>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The time when the association was last updated.</p>
    #[serde(rename = "lastUpdatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    /// <p>The ARN of the resource group. This value is returned only if the resource is a resource group.</p>
    #[serde(rename = "resourceGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_arn: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the resource share.</p>
    #[serde(rename = "resourceShareArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share_arn: Option<String>,
    /// <p>The status of the resource.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>A message about the status of the resource.</p>
    #[serde(rename = "statusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// <p>The resource type.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Describes a resource share.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResourceShare {
    /// <p>Indicates whether principals outside your AWS organization can be associated with a resource share.</p>
    #[serde(rename = "allowExternalPrincipals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_external_principals: Option<bool>,
    /// <p>The time when the resource share was created.</p>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p><p>Indicates how the resource share was created. Possible values include:</p> <ul> <li> <p> <code>CREATED<em>FROM</em>POLICY</code> - Indicates that the resource share was created from an AWS Identity and Access Management (AWS IAM) policy attached to a resource. These resource shares are visible only to the AWS account that created it. They cannot be modified in AWS RAM.</p> </li> <li> <p> <code>PROMOTING<em>TO</em>STANDARD</code> - The resource share is in the process of being promoted. For more information, see <a>PromoteResourceShareCreatedFromPolicy</a>.</p> </li> <li> <p> <code>STANDARD</code> - Indicates that the resource share was created in AWS RAM using the console or APIs. These resource shares are visible to all principals. They can be modified in AWS RAM.</p> </li> </ul></p>
    #[serde(rename = "featureSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_set: Option<String>,
    /// <p>The time when the resource share was last updated.</p>
    #[serde(rename = "lastUpdatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    /// <p>The name of the resource share.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The ID of the AWS account that owns the resource share.</p>
    #[serde(rename = "owningAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owning_account_id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the resource share.</p>
    #[serde(rename = "resourceShareArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share_arn: Option<String>,
    /// <p>The status of the resource share.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>A message about the status of the resource share.</p>
    #[serde(rename = "statusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// <p>The tags for the resource share.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>Describes an association with a resource share.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResourceShareAssociation {
    /// <p>The associated entity. For resource associations, this is the ARN of the resource. For principal associations, this is the ID of an AWS account or the ARN of an OU or organization from AWS Organizations.</p>
    #[serde(rename = "associatedEntity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_entity: Option<String>,
    /// <p>The association type.</p>
    #[serde(rename = "associationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_type: Option<String>,
    /// <p>The time when the association was created.</p>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>Indicates whether the principal belongs to the same AWS organization as the AWS account that owns the resource share.</p>
    #[serde(rename = "external")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external: Option<bool>,
    /// <p>The time when the association was last updated.</p>
    #[serde(rename = "lastUpdatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the resource share.</p>
    #[serde(rename = "resourceShareArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share_arn: Option<String>,
    /// <p>The name of the resource share.</p>
    #[serde(rename = "resourceShareName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share_name: Option<String>,
    /// <p>The status of the association.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>A message about the status of the association.</p>
    #[serde(rename = "statusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

/// <p>Describes an invitation to join a resource share.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResourceShareInvitation {
    /// <p>The date and time when the invitation was sent.</p>
    #[serde(rename = "invitationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitation_timestamp: Option<f64>,
    /// <p>The ID of the AWS account that received the invitation.</p>
    #[serde(rename = "receiverAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receiver_account_id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the resource share.</p>
    #[serde(rename = "resourceShareArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share_arn: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the invitation.</p>
    #[serde(rename = "resourceShareInvitationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share_invitation_arn: Option<String>,
    /// <p>The name of the resource share.</p>
    #[serde(rename = "resourceShareName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share_name: Option<String>,
    /// <p>The ID of the AWS account that sent the invitation.</p>
    #[serde(rename = "senderAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_account_id: Option<String>,
    /// <p>The status of the invitation.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Information about an AWS RAM permission.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResourceSharePermissionDetail {
    /// <p>The ARN of the permission.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The date and time when the permission was created.</p>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The identifier for the version of the permission that is set as the default version.</p>
    #[serde(rename = "defaultVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_version: Option<bool>,
    /// <p>The date and time when the permission was last updated.</p>
    #[serde(rename = "lastUpdatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    /// <p>The name of the permission.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The permission's effect and actions in JSON format. The <code>effect</code> indicates whether the actions are allowed or denied. The <code>actions</code> list the API actions to which the principal is granted or denied access.</p>
    #[serde(rename = "permission")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission: Option<String>,
    /// <p>The resource type to which the permission applies.</p>
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>The identifier for the version of the permission.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>Information about a permission that is associated with a resource share.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResourceSharePermissionSummary {
    /// <p>The ARN of the permission.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The date and time when the permission was created.</p>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The identifier for the version of the permission that is set as the default version.</p>
    #[serde(rename = "defaultVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_version: Option<bool>,
    /// <p>The date and time when the permission was last updated.</p>
    #[serde(rename = "lastUpdatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    /// <p>The name of the permission.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The type of resource to which the permission applies.</p>
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>The current status of the permission.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The identifier for the version of the permission.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>Information about a tag.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>The key of the tag.</p>
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The value of the tag.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>Used to filter information based on tags.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagFilter {
    /// <p>The tag key.</p>
    #[serde(rename = "tagKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
    /// <p>The tag values.</p>
    #[serde(rename = "tagValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_values: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource share.</p>
    #[serde(rename = "resourceShareArn")]
    pub resource_share_arn: String,
    /// <p>One or more tags.</p>
    #[serde(rename = "tags")]
    pub tags: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource share.</p>
    #[serde(rename = "resourceShareArn")]
    pub resource_share_arn: String,
    /// <p>The tag keys of the tags to remove.</p>
    #[serde(rename = "tagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateResourceShareRequest {
    /// <p>Indicates whether principals outside your AWS organization can be associated with a resource share.</p>
    #[serde(rename = "allowExternalPrincipals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_external_principals: Option<bool>,
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The name of the resource share.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the resource share.</p>
    #[serde(rename = "resourceShareArn")]
    pub resource_share_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateResourceShareResponse {
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>Information about the resource share.</p>
    #[serde(rename = "resourceShare")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share: Option<ResourceShare>,
}

/// Errors returned by AcceptResourceShareInvitation
#[derive(Debug, PartialEq)]
pub enum AcceptResourceShareInvitationError {
    /// <p>A client token input parameter was reused with an operation, but at least one of the other input parameters is different from the previous call to the operation.</p>
    IdempotentParameterMismatch(String),
    /// <p>A client token is not valid.</p>
    InvalidClientToken(String),
    /// <p>The format of an Amazon Resource Name (ARN) is not valid.</p>
    MalformedArn(String),
    /// <p>The requested operation is not permitted.</p>
    OperationNotPermitted(String),
    /// <p>The invitation was already accepted.</p>
    ResourceShareInvitationAlreadyAccepted(String),
    /// <p>The invitation was already rejected.</p>
    ResourceShareInvitationAlreadyRejected(String),
    /// <p>The Amazon Resource Name (ARN) for an invitation was not found.</p>
    ResourceShareInvitationArnNotFound(String),
    /// <p>The invitation is expired.</p>
    ResourceShareInvitationExpired(String),
    /// <p>The service could not respond to the request due to an internal problem.</p>
    ServerInternal(String),
    /// <p>The service is not available.</p>
    ServiceUnavailable(String),
}

impl AcceptResourceShareInvitationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AcceptResourceShareInvitationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "IdempotentParameterMismatchException" => {
                    return RusotoError::Service(
                        AcceptResourceShareInvitationError::IdempotentParameterMismatch(err.msg),
                    )
                }
                "InvalidClientTokenException" => {
                    return RusotoError::Service(
                        AcceptResourceShareInvitationError::InvalidClientToken(err.msg),
                    )
                }
                "MalformedArnException" => {
                    return RusotoError::Service(AcceptResourceShareInvitationError::MalformedArn(
                        err.msg,
                    ))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(
                        AcceptResourceShareInvitationError::OperationNotPermitted(err.msg),
                    )
                }
                "ResourceShareInvitationAlreadyAcceptedException" => {
                    return RusotoError::Service(
                        AcceptResourceShareInvitationError::ResourceShareInvitationAlreadyAccepted(
                            err.msg,
                        ),
                    )
                }
                "ResourceShareInvitationAlreadyRejectedException" => {
                    return RusotoError::Service(
                        AcceptResourceShareInvitationError::ResourceShareInvitationAlreadyRejected(
                            err.msg,
                        ),
                    )
                }
                "ResourceShareInvitationArnNotFoundException" => {
                    return RusotoError::Service(
                        AcceptResourceShareInvitationError::ResourceShareInvitationArnNotFound(
                            err.msg,
                        ),
                    )
                }
                "ResourceShareInvitationExpiredException" => {
                    return RusotoError::Service(
                        AcceptResourceShareInvitationError::ResourceShareInvitationExpired(err.msg),
                    )
                }
                "ServerInternalException" => {
                    return RusotoError::Service(
                        AcceptResourceShareInvitationError::ServerInternal(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        AcceptResourceShareInvitationError::ServiceUnavailable(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AcceptResourceShareInvitationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AcceptResourceShareInvitationError::IdempotentParameterMismatch(ref cause) => {
                write!(f, "{}", cause)
            }
            AcceptResourceShareInvitationError::InvalidClientToken(ref cause) => {
                write!(f, "{}", cause)
            }
            AcceptResourceShareInvitationError::MalformedArn(ref cause) => write!(f, "{}", cause),
            AcceptResourceShareInvitationError::OperationNotPermitted(ref cause) => {
                write!(f, "{}", cause)
            }
            AcceptResourceShareInvitationError::ResourceShareInvitationAlreadyAccepted(
                ref cause,
            ) => write!(f, "{}", cause),
            AcceptResourceShareInvitationError::ResourceShareInvitationAlreadyRejected(
                ref cause,
            ) => write!(f, "{}", cause),
            AcceptResourceShareInvitationError::ResourceShareInvitationArnNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            AcceptResourceShareInvitationError::ResourceShareInvitationExpired(ref cause) => {
                write!(f, "{}", cause)
            }
            AcceptResourceShareInvitationError::ServerInternal(ref cause) => write!(f, "{}", cause),
            AcceptResourceShareInvitationError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for AcceptResourceShareInvitationError {}
/// Errors returned by AssociateResourceShare
#[derive(Debug, PartialEq)]
pub enum AssociateResourceShareError {
    /// <p>A client token input parameter was reused with an operation, but at least one of the other input parameters is different from the previous call to the operation.</p>
    IdempotentParameterMismatch(String),
    /// <p>A client token is not valid.</p>
    InvalidClientToken(String),
    /// <p>A parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>The requested state transition is not valid.</p>
    InvalidStateTransition(String),
    /// <p>The format of an Amazon Resource Name (ARN) is not valid.</p>
    MalformedArn(String),
    /// <p>The requested operation is not permitted.</p>
    OperationNotPermitted(String),
    /// <p>The requested resource share exceeds the limit for your account.</p>
    ResourceShareLimitExceeded(String),
    /// <p>The service could not respond to the request due to an internal problem.</p>
    ServerInternal(String),
    /// <p>The service is not available.</p>
    ServiceUnavailable(String),
    /// <p>A specified resource was not found.</p>
    UnknownResource(String),
}

impl AssociateResourceShareError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AssociateResourceShareError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "IdempotentParameterMismatchException" => {
                    return RusotoError::Service(
                        AssociateResourceShareError::IdempotentParameterMismatch(err.msg),
                    )
                }
                "InvalidClientTokenException" => {
                    return RusotoError::Service(AssociateResourceShareError::InvalidClientToken(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AssociateResourceShareError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidStateTransitionException" => {
                    return RusotoError::Service(
                        AssociateResourceShareError::InvalidStateTransition(err.msg),
                    )
                }
                "MalformedArnException" => {
                    return RusotoError::Service(AssociateResourceShareError::MalformedArn(err.msg))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(
                        AssociateResourceShareError::OperationNotPermitted(err.msg),
                    )
                }
                "ResourceShareLimitExceededException" => {
                    return RusotoError::Service(
                        AssociateResourceShareError::ResourceShareLimitExceeded(err.msg),
                    )
                }
                "ServerInternalException" => {
                    return RusotoError::Service(AssociateResourceShareError::ServerInternal(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(AssociateResourceShareError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "UnknownResourceException" => {
                    return RusotoError::Service(AssociateResourceShareError::UnknownResource(
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
impl fmt::Display for AssociateResourceShareError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociateResourceShareError::IdempotentParameterMismatch(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateResourceShareError::InvalidClientToken(ref cause) => write!(f, "{}", cause),
            AssociateResourceShareError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            AssociateResourceShareError::InvalidStateTransition(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateResourceShareError::MalformedArn(ref cause) => write!(f, "{}", cause),
            AssociateResourceShareError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            AssociateResourceShareError::ResourceShareLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateResourceShareError::ServerInternal(ref cause) => write!(f, "{}", cause),
            AssociateResourceShareError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            AssociateResourceShareError::UnknownResource(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AssociateResourceShareError {}
/// Errors returned by AssociateResourceSharePermission
#[derive(Debug, PartialEq)]
pub enum AssociateResourceSharePermissionError {
    /// <p>A client token is not valid.</p>
    InvalidClientToken(String),
    /// <p>A parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>The format of an Amazon Resource Name (ARN) is not valid.</p>
    MalformedArn(String),
    /// <p>The requested operation is not permitted.</p>
    OperationNotPermitted(String),
    /// <p>The service could not respond to the request due to an internal problem.</p>
    ServerInternal(String),
    /// <p>The service is not available.</p>
    ServiceUnavailable(String),
    /// <p>A specified resource was not found.</p>
    UnknownResource(String),
}

impl AssociateResourceSharePermissionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AssociateResourceSharePermissionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidClientTokenException" => {
                    return RusotoError::Service(
                        AssociateResourceSharePermissionError::InvalidClientToken(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        AssociateResourceSharePermissionError::InvalidParameter(err.msg),
                    )
                }
                "MalformedArnException" => {
                    return RusotoError::Service(
                        AssociateResourceSharePermissionError::MalformedArn(err.msg),
                    )
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(
                        AssociateResourceSharePermissionError::OperationNotPermitted(err.msg),
                    )
                }
                "ServerInternalException" => {
                    return RusotoError::Service(
                        AssociateResourceSharePermissionError::ServerInternal(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        AssociateResourceSharePermissionError::ServiceUnavailable(err.msg),
                    )
                }
                "UnknownResourceException" => {
                    return RusotoError::Service(
                        AssociateResourceSharePermissionError::UnknownResource(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AssociateResourceSharePermissionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociateResourceSharePermissionError::InvalidClientToken(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateResourceSharePermissionError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateResourceSharePermissionError::MalformedArn(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateResourceSharePermissionError::OperationNotPermitted(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateResourceSharePermissionError::ServerInternal(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateResourceSharePermissionError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateResourceSharePermissionError::UnknownResource(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for AssociateResourceSharePermissionError {}
/// Errors returned by CreateResourceShare
#[derive(Debug, PartialEq)]
pub enum CreateResourceShareError {
    /// <p>A client token input parameter was reused with an operation, but at least one of the other input parameters is different from the previous call to the operation.</p>
    IdempotentParameterMismatch(String),
    /// <p>A client token is not valid.</p>
    InvalidClientToken(String),
    /// <p>A parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>The requested state transition is not valid.</p>
    InvalidStateTransition(String),
    /// <p>The format of an Amazon Resource Name (ARN) is not valid.</p>
    MalformedArn(String),
    /// <p>The requested operation is not permitted.</p>
    OperationNotPermitted(String),
    /// <p>The requested resource share exceeds the limit for your account.</p>
    ResourceShareLimitExceeded(String),
    /// <p>The service could not respond to the request due to an internal problem.</p>
    ServerInternal(String),
    /// <p>The service is not available.</p>
    ServiceUnavailable(String),
    /// <p>The specified tag is a reserved word and cannot be used.</p>
    TagPolicyViolation(String),
    /// <p>A specified resource was not found.</p>
    UnknownResource(String),
}

impl CreateResourceShareError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateResourceShareError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "IdempotentParameterMismatchException" => {
                    return RusotoError::Service(
                        CreateResourceShareError::IdempotentParameterMismatch(err.msg),
                    )
                }
                "InvalidClientTokenException" => {
                    return RusotoError::Service(CreateResourceShareError::InvalidClientToken(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateResourceShareError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidStateTransitionException" => {
                    return RusotoError::Service(CreateResourceShareError::InvalidStateTransition(
                        err.msg,
                    ))
                }
                "MalformedArnException" => {
                    return RusotoError::Service(CreateResourceShareError::MalformedArn(err.msg))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(CreateResourceShareError::OperationNotPermitted(
                        err.msg,
                    ))
                }
                "ResourceShareLimitExceededException" => {
                    return RusotoError::Service(
                        CreateResourceShareError::ResourceShareLimitExceeded(err.msg),
                    )
                }
                "ServerInternalException" => {
                    return RusotoError::Service(CreateResourceShareError::ServerInternal(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateResourceShareError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "TagPolicyViolationException" => {
                    return RusotoError::Service(CreateResourceShareError::TagPolicyViolation(
                        err.msg,
                    ))
                }
                "UnknownResourceException" => {
                    return RusotoError::Service(CreateResourceShareError::UnknownResource(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateResourceShareError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateResourceShareError::IdempotentParameterMismatch(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateResourceShareError::InvalidClientToken(ref cause) => write!(f, "{}", cause),
            CreateResourceShareError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateResourceShareError::InvalidStateTransition(ref cause) => write!(f, "{}", cause),
            CreateResourceShareError::MalformedArn(ref cause) => write!(f, "{}", cause),
            CreateResourceShareError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            CreateResourceShareError::ResourceShareLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateResourceShareError::ServerInternal(ref cause) => write!(f, "{}", cause),
            CreateResourceShareError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            CreateResourceShareError::TagPolicyViolation(ref cause) => write!(f, "{}", cause),
            CreateResourceShareError::UnknownResource(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateResourceShareError {}
/// Errors returned by DeleteResourceShare
#[derive(Debug, PartialEq)]
pub enum DeleteResourceShareError {
    /// <p>A client token input parameter was reused with an operation, but at least one of the other input parameters is different from the previous call to the operation.</p>
    IdempotentParameterMismatch(String),
    /// <p>A client token is not valid.</p>
    InvalidClientToken(String),
    /// <p>A parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>The requested state transition is not valid.</p>
    InvalidStateTransition(String),
    /// <p>The format of an Amazon Resource Name (ARN) is not valid.</p>
    MalformedArn(String),
    /// <p>The requested operation is not permitted.</p>
    OperationNotPermitted(String),
    /// <p>The service could not respond to the request due to an internal problem.</p>
    ServerInternal(String),
    /// <p>The service is not available.</p>
    ServiceUnavailable(String),
    /// <p>A specified resource was not found.</p>
    UnknownResource(String),
}

impl DeleteResourceShareError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteResourceShareError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "IdempotentParameterMismatchException" => {
                    return RusotoError::Service(
                        DeleteResourceShareError::IdempotentParameterMismatch(err.msg),
                    )
                }
                "InvalidClientTokenException" => {
                    return RusotoError::Service(DeleteResourceShareError::InvalidClientToken(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteResourceShareError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidStateTransitionException" => {
                    return RusotoError::Service(DeleteResourceShareError::InvalidStateTransition(
                        err.msg,
                    ))
                }
                "MalformedArnException" => {
                    return RusotoError::Service(DeleteResourceShareError::MalformedArn(err.msg))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(DeleteResourceShareError::OperationNotPermitted(
                        err.msg,
                    ))
                }
                "ServerInternalException" => {
                    return RusotoError::Service(DeleteResourceShareError::ServerInternal(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteResourceShareError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "UnknownResourceException" => {
                    return RusotoError::Service(DeleteResourceShareError::UnknownResource(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteResourceShareError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteResourceShareError::IdempotentParameterMismatch(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteResourceShareError::InvalidClientToken(ref cause) => write!(f, "{}", cause),
            DeleteResourceShareError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteResourceShareError::InvalidStateTransition(ref cause) => write!(f, "{}", cause),
            DeleteResourceShareError::MalformedArn(ref cause) => write!(f, "{}", cause),
            DeleteResourceShareError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            DeleteResourceShareError::ServerInternal(ref cause) => write!(f, "{}", cause),
            DeleteResourceShareError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DeleteResourceShareError::UnknownResource(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteResourceShareError {}
/// Errors returned by DisassociateResourceShare
#[derive(Debug, PartialEq)]
pub enum DisassociateResourceShareError {
    /// <p>A client token input parameter was reused with an operation, but at least one of the other input parameters is different from the previous call to the operation.</p>
    IdempotentParameterMismatch(String),
    /// <p>A client token is not valid.</p>
    InvalidClientToken(String),
    /// <p>A parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>The requested state transition is not valid.</p>
    InvalidStateTransition(String),
    /// <p>The format of an Amazon Resource Name (ARN) is not valid.</p>
    MalformedArn(String),
    /// <p>The requested operation is not permitted.</p>
    OperationNotPermitted(String),
    /// <p>The requested resource share exceeds the limit for your account.</p>
    ResourceShareLimitExceeded(String),
    /// <p>The service could not respond to the request due to an internal problem.</p>
    ServerInternal(String),
    /// <p>The service is not available.</p>
    ServiceUnavailable(String),
    /// <p>A specified resource was not found.</p>
    UnknownResource(String),
}

impl DisassociateResourceShareError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisassociateResourceShareError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "IdempotentParameterMismatchException" => {
                    return RusotoError::Service(
                        DisassociateResourceShareError::IdempotentParameterMismatch(err.msg),
                    )
                }
                "InvalidClientTokenException" => {
                    return RusotoError::Service(
                        DisassociateResourceShareError::InvalidClientToken(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DisassociateResourceShareError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidStateTransitionException" => {
                    return RusotoError::Service(
                        DisassociateResourceShareError::InvalidStateTransition(err.msg),
                    )
                }
                "MalformedArnException" => {
                    return RusotoError::Service(DisassociateResourceShareError::MalformedArn(
                        err.msg,
                    ))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(
                        DisassociateResourceShareError::OperationNotPermitted(err.msg),
                    )
                }
                "ResourceShareLimitExceededException" => {
                    return RusotoError::Service(
                        DisassociateResourceShareError::ResourceShareLimitExceeded(err.msg),
                    )
                }
                "ServerInternalException" => {
                    return RusotoError::Service(DisassociateResourceShareError::ServerInternal(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        DisassociateResourceShareError::ServiceUnavailable(err.msg),
                    )
                }
                "UnknownResourceException" => {
                    return RusotoError::Service(DisassociateResourceShareError::UnknownResource(
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
impl fmt::Display for DisassociateResourceShareError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateResourceShareError::IdempotentParameterMismatch(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateResourceShareError::InvalidClientToken(ref cause) => write!(f, "{}", cause),
            DisassociateResourceShareError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DisassociateResourceShareError::InvalidStateTransition(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateResourceShareError::MalformedArn(ref cause) => write!(f, "{}", cause),
            DisassociateResourceShareError::OperationNotPermitted(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateResourceShareError::ResourceShareLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateResourceShareError::ServerInternal(ref cause) => write!(f, "{}", cause),
            DisassociateResourceShareError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DisassociateResourceShareError::UnknownResource(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DisassociateResourceShareError {}
/// Errors returned by DisassociateResourceSharePermission
#[derive(Debug, PartialEq)]
pub enum DisassociateResourceSharePermissionError {
    /// <p>A client token is not valid.</p>
    InvalidClientToken(String),
    /// <p>A parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>The format of an Amazon Resource Name (ARN) is not valid.</p>
    MalformedArn(String),
    /// <p>The requested operation is not permitted.</p>
    OperationNotPermitted(String),
    /// <p>The service could not respond to the request due to an internal problem.</p>
    ServerInternal(String),
    /// <p>The service is not available.</p>
    ServiceUnavailable(String),
    /// <p>A specified resource was not found.</p>
    UnknownResource(String),
}

impl DisassociateResourceSharePermissionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisassociateResourceSharePermissionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidClientTokenException" => {
                    return RusotoError::Service(
                        DisassociateResourceSharePermissionError::InvalidClientToken(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        DisassociateResourceSharePermissionError::InvalidParameter(err.msg),
                    )
                }
                "MalformedArnException" => {
                    return RusotoError::Service(
                        DisassociateResourceSharePermissionError::MalformedArn(err.msg),
                    )
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(
                        DisassociateResourceSharePermissionError::OperationNotPermitted(err.msg),
                    )
                }
                "ServerInternalException" => {
                    return RusotoError::Service(
                        DisassociateResourceSharePermissionError::ServerInternal(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        DisassociateResourceSharePermissionError::ServiceUnavailable(err.msg),
                    )
                }
                "UnknownResourceException" => {
                    return RusotoError::Service(
                        DisassociateResourceSharePermissionError::UnknownResource(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DisassociateResourceSharePermissionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateResourceSharePermissionError::InvalidClientToken(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateResourceSharePermissionError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateResourceSharePermissionError::MalformedArn(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateResourceSharePermissionError::OperationNotPermitted(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateResourceSharePermissionError::ServerInternal(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateResourceSharePermissionError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateResourceSharePermissionError::UnknownResource(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DisassociateResourceSharePermissionError {}
/// Errors returned by EnableSharingWithAwsOrganization
#[derive(Debug, PartialEq)]
pub enum EnableSharingWithAwsOrganizationError {
    /// <p>The requested operation is not permitted.</p>
    OperationNotPermitted(String),
    /// <p>The service could not respond to the request due to an internal problem.</p>
    ServerInternal(String),
    /// <p>The service is not available.</p>
    ServiceUnavailable(String),
}

impl EnableSharingWithAwsOrganizationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<EnableSharingWithAwsOrganizationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "OperationNotPermittedException" => {
                    return RusotoError::Service(
                        EnableSharingWithAwsOrganizationError::OperationNotPermitted(err.msg),
                    )
                }
                "ServerInternalException" => {
                    return RusotoError::Service(
                        EnableSharingWithAwsOrganizationError::ServerInternal(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        EnableSharingWithAwsOrganizationError::ServiceUnavailable(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for EnableSharingWithAwsOrganizationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EnableSharingWithAwsOrganizationError::OperationNotPermitted(ref cause) => {
                write!(f, "{}", cause)
            }
            EnableSharingWithAwsOrganizationError::ServerInternal(ref cause) => {
                write!(f, "{}", cause)
            }
            EnableSharingWithAwsOrganizationError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for EnableSharingWithAwsOrganizationError {}
/// Errors returned by GetPermission
#[derive(Debug, PartialEq)]
pub enum GetPermissionError {
    /// <p>A parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>The format of an Amazon Resource Name (ARN) is not valid.</p>
    MalformedArn(String),
    /// <p>The requested operation is not permitted.</p>
    OperationNotPermitted(String),
    /// <p>The service could not respond to the request due to an internal problem.</p>
    ServerInternal(String),
    /// <p>The service is not available.</p>
    ServiceUnavailable(String),
    /// <p>A specified resource was not found.</p>
    UnknownResource(String),
}

impl GetPermissionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetPermissionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(GetPermissionError::InvalidParameter(err.msg))
                }
                "MalformedArnException" => {
                    return RusotoError::Service(GetPermissionError::MalformedArn(err.msg))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(GetPermissionError::OperationNotPermitted(err.msg))
                }
                "ServerInternalException" => {
                    return RusotoError::Service(GetPermissionError::ServerInternal(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(GetPermissionError::ServiceUnavailable(err.msg))
                }
                "UnknownResourceException" => {
                    return RusotoError::Service(GetPermissionError::UnknownResource(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetPermissionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetPermissionError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetPermissionError::MalformedArn(ref cause) => write!(f, "{}", cause),
            GetPermissionError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            GetPermissionError::ServerInternal(ref cause) => write!(f, "{}", cause),
            GetPermissionError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            GetPermissionError::UnknownResource(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetPermissionError {}
/// Errors returned by GetResourcePolicies
#[derive(Debug, PartialEq)]
pub enum GetResourcePoliciesError {
    /// <p>The specified value for NextToken is not valid.</p>
    InvalidNextToken(String),
    /// <p>A parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>The format of an Amazon Resource Name (ARN) is not valid.</p>
    MalformedArn(String),
    /// <p>The service could not respond to the request due to an internal problem.</p>
    ServerInternal(String),
    /// <p>The service is not available.</p>
    ServiceUnavailable(String),
}

impl GetResourcePoliciesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetResourcePoliciesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidNextTokenException" => {
                    return RusotoError::Service(GetResourcePoliciesError::InvalidNextToken(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetResourcePoliciesError::InvalidParameter(
                        err.msg,
                    ))
                }
                "MalformedArnException" => {
                    return RusotoError::Service(GetResourcePoliciesError::MalformedArn(err.msg))
                }
                "ServerInternalException" => {
                    return RusotoError::Service(GetResourcePoliciesError::ServerInternal(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(GetResourcePoliciesError::ServiceUnavailable(
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
impl fmt::Display for GetResourcePoliciesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetResourcePoliciesError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            GetResourcePoliciesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetResourcePoliciesError::MalformedArn(ref cause) => write!(f, "{}", cause),
            GetResourcePoliciesError::ServerInternal(ref cause) => write!(f, "{}", cause),
            GetResourcePoliciesError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetResourcePoliciesError {}
/// Errors returned by GetResourceShareAssociations
#[derive(Debug, PartialEq)]
pub enum GetResourceShareAssociationsError {
    /// <p>The specified value for NextToken is not valid.</p>
    InvalidNextToken(String),
    /// <p>A parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>The format of an Amazon Resource Name (ARN) is not valid.</p>
    MalformedArn(String),
    /// <p>The requested operation is not permitted.</p>
    OperationNotPermitted(String),
    /// <p>The service could not respond to the request due to an internal problem.</p>
    ServerInternal(String),
    /// <p>The service is not available.</p>
    ServiceUnavailable(String),
    /// <p>A specified resource was not found.</p>
    UnknownResource(String),
}

impl GetResourceShareAssociationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetResourceShareAssociationsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidNextTokenException" => {
                    return RusotoError::Service(
                        GetResourceShareAssociationsError::InvalidNextToken(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        GetResourceShareAssociationsError::InvalidParameter(err.msg),
                    )
                }
                "MalformedArnException" => {
                    return RusotoError::Service(GetResourceShareAssociationsError::MalformedArn(
                        err.msg,
                    ))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(
                        GetResourceShareAssociationsError::OperationNotPermitted(err.msg),
                    )
                }
                "ServerInternalException" => {
                    return RusotoError::Service(GetResourceShareAssociationsError::ServerInternal(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        GetResourceShareAssociationsError::ServiceUnavailable(err.msg),
                    )
                }
                "UnknownResourceException" => {
                    return RusotoError::Service(
                        GetResourceShareAssociationsError::UnknownResource(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetResourceShareAssociationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetResourceShareAssociationsError::InvalidNextToken(ref cause) => {
                write!(f, "{}", cause)
            }
            GetResourceShareAssociationsError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            GetResourceShareAssociationsError::MalformedArn(ref cause) => write!(f, "{}", cause),
            GetResourceShareAssociationsError::OperationNotPermitted(ref cause) => {
                write!(f, "{}", cause)
            }
            GetResourceShareAssociationsError::ServerInternal(ref cause) => write!(f, "{}", cause),
            GetResourceShareAssociationsError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            GetResourceShareAssociationsError::UnknownResource(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetResourceShareAssociationsError {}
/// Errors returned by GetResourceShareInvitations
#[derive(Debug, PartialEq)]
pub enum GetResourceShareInvitationsError {
    /// <p>The specified value for MaxResults is not valid.</p>
    InvalidMaxResults(String),
    /// <p>The specified value for NextToken is not valid.</p>
    InvalidNextToken(String),
    /// <p>A parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>The format of an Amazon Resource Name (ARN) is not valid.</p>
    MalformedArn(String),
    /// <p>The Amazon Resource Name (ARN) for an invitation was not found.</p>
    ResourceShareInvitationArnNotFound(String),
    /// <p>The service could not respond to the request due to an internal problem.</p>
    ServerInternal(String),
    /// <p>The service is not available.</p>
    ServiceUnavailable(String),
}

impl GetResourceShareInvitationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetResourceShareInvitationsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidMaxResultsException" => {
                    return RusotoError::Service(
                        GetResourceShareInvitationsError::InvalidMaxResults(err.msg),
                    )
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(
                        GetResourceShareInvitationsError::InvalidNextToken(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        GetResourceShareInvitationsError::InvalidParameter(err.msg),
                    )
                }
                "MalformedArnException" => {
                    return RusotoError::Service(GetResourceShareInvitationsError::MalformedArn(
                        err.msg,
                    ))
                }
                "ResourceShareInvitationArnNotFoundException" => {
                    return RusotoError::Service(
                        GetResourceShareInvitationsError::ResourceShareInvitationArnNotFound(
                            err.msg,
                        ),
                    )
                }
                "ServerInternalException" => {
                    return RusotoError::Service(GetResourceShareInvitationsError::ServerInternal(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        GetResourceShareInvitationsError::ServiceUnavailable(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetResourceShareInvitationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetResourceShareInvitationsError::InvalidMaxResults(ref cause) => {
                write!(f, "{}", cause)
            }
            GetResourceShareInvitationsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            GetResourceShareInvitationsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetResourceShareInvitationsError::MalformedArn(ref cause) => write!(f, "{}", cause),
            GetResourceShareInvitationsError::ResourceShareInvitationArnNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            GetResourceShareInvitationsError::ServerInternal(ref cause) => write!(f, "{}", cause),
            GetResourceShareInvitationsError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetResourceShareInvitationsError {}
/// Errors returned by GetResourceShares
#[derive(Debug, PartialEq)]
pub enum GetResourceSharesError {
    /// <p>The specified value for NextToken is not valid.</p>
    InvalidNextToken(String),
    /// <p>A parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>The format of an Amazon Resource Name (ARN) is not valid.</p>
    MalformedArn(String),
    /// <p>The service could not respond to the request due to an internal problem.</p>
    ServerInternal(String),
    /// <p>The service is not available.</p>
    ServiceUnavailable(String),
    /// <p>A specified resource was not found.</p>
    UnknownResource(String),
}

impl GetResourceSharesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetResourceSharesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidNextTokenException" => {
                    return RusotoError::Service(GetResourceSharesError::InvalidNextToken(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetResourceSharesError::InvalidParameter(err.msg))
                }
                "MalformedArnException" => {
                    return RusotoError::Service(GetResourceSharesError::MalformedArn(err.msg))
                }
                "ServerInternalException" => {
                    return RusotoError::Service(GetResourceSharesError::ServerInternal(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(GetResourceSharesError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "UnknownResourceException" => {
                    return RusotoError::Service(GetResourceSharesError::UnknownResource(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetResourceSharesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetResourceSharesError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            GetResourceSharesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetResourceSharesError::MalformedArn(ref cause) => write!(f, "{}", cause),
            GetResourceSharesError::ServerInternal(ref cause) => write!(f, "{}", cause),
            GetResourceSharesError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            GetResourceSharesError::UnknownResource(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetResourceSharesError {}
/// Errors returned by ListPendingInvitationResources
#[derive(Debug, PartialEq)]
pub enum ListPendingInvitationResourcesError {
    /// <p>The specified value for NextToken is not valid.</p>
    InvalidNextToken(String),
    /// <p>A parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>The format of an Amazon Resource Name (ARN) is not valid.</p>
    MalformedArn(String),
    /// <p>A required input parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>The invitation was already rejected.</p>
    ResourceShareInvitationAlreadyRejected(String),
    /// <p>The Amazon Resource Name (ARN) for an invitation was not found.</p>
    ResourceShareInvitationArnNotFound(String),
    /// <p>The invitation is expired.</p>
    ResourceShareInvitationExpired(String),
    /// <p>The service could not respond to the request due to an internal problem.</p>
    ServerInternal(String),
    /// <p>The service is not available.</p>
    ServiceUnavailable(String),
}

impl ListPendingInvitationResourcesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListPendingInvitationResourcesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidNextTokenException" => {
                    return RusotoError::Service(
                        ListPendingInvitationResourcesError::InvalidNextToken(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        ListPendingInvitationResourcesError::InvalidParameter(err.msg),
                    )
                }
                "MalformedArnException" => {
                    return RusotoError::Service(ListPendingInvitationResourcesError::MalformedArn(
                        err.msg,
                    ))
                }
                "MissingRequiredParameterException" => {
                    return RusotoError::Service(
                        ListPendingInvitationResourcesError::MissingRequiredParameter(err.msg),
                    )
                }
                "ResourceShareInvitationAlreadyRejectedException" => {
                    return RusotoError::Service(
                        ListPendingInvitationResourcesError::ResourceShareInvitationAlreadyRejected(
                            err.msg,
                        ),
                    )
                }
                "ResourceShareInvitationArnNotFoundException" => {
                    return RusotoError::Service(
                        ListPendingInvitationResourcesError::ResourceShareInvitationArnNotFound(
                            err.msg,
                        ),
                    )
                }
                "ResourceShareInvitationExpiredException" => {
                    return RusotoError::Service(
                        ListPendingInvitationResourcesError::ResourceShareInvitationExpired(
                            err.msg,
                        ),
                    )
                }
                "ServerInternalException" => {
                    return RusotoError::Service(
                        ListPendingInvitationResourcesError::ServerInternal(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        ListPendingInvitationResourcesError::ServiceUnavailable(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListPendingInvitationResourcesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListPendingInvitationResourcesError::InvalidNextToken(ref cause) => {
                write!(f, "{}", cause)
            }
            ListPendingInvitationResourcesError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            ListPendingInvitationResourcesError::MalformedArn(ref cause) => write!(f, "{}", cause),
            ListPendingInvitationResourcesError::MissingRequiredParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            ListPendingInvitationResourcesError::ResourceShareInvitationAlreadyRejected(
                ref cause,
            ) => write!(f, "{}", cause),
            ListPendingInvitationResourcesError::ResourceShareInvitationArnNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            ListPendingInvitationResourcesError::ResourceShareInvitationExpired(ref cause) => {
                write!(f, "{}", cause)
            }
            ListPendingInvitationResourcesError::ServerInternal(ref cause) => {
                write!(f, "{}", cause)
            }
            ListPendingInvitationResourcesError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListPendingInvitationResourcesError {}
/// Errors returned by ListPermissions
#[derive(Debug, PartialEq)]
pub enum ListPermissionsError {
    /// <p>The specified value for NextToken is not valid.</p>
    InvalidNextToken(String),
    /// <p>A parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>The requested operation is not permitted.</p>
    OperationNotPermitted(String),
    /// <p>The service could not respond to the request due to an internal problem.</p>
    ServerInternal(String),
    /// <p>The service is not available.</p>
    ServiceUnavailable(String),
}

impl ListPermissionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListPermissionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListPermissionsError::InvalidNextToken(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListPermissionsError::InvalidParameter(err.msg))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(ListPermissionsError::OperationNotPermitted(
                        err.msg,
                    ))
                }
                "ServerInternalException" => {
                    return RusotoError::Service(ListPermissionsError::ServerInternal(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListPermissionsError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListPermissionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListPermissionsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListPermissionsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListPermissionsError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            ListPermissionsError::ServerInternal(ref cause) => write!(f, "{}", cause),
            ListPermissionsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListPermissionsError {}
/// Errors returned by ListPrincipals
#[derive(Debug, PartialEq)]
pub enum ListPrincipalsError {
    /// <p>The specified value for NextToken is not valid.</p>
    InvalidNextToken(String),
    /// <p>A parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>The format of an Amazon Resource Name (ARN) is not valid.</p>
    MalformedArn(String),
    /// <p>The service could not respond to the request due to an internal problem.</p>
    ServerInternal(String),
    /// <p>The service is not available.</p>
    ServiceUnavailable(String),
    /// <p>A specified resource was not found.</p>
    UnknownResource(String),
}

impl ListPrincipalsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListPrincipalsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListPrincipalsError::InvalidNextToken(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListPrincipalsError::InvalidParameter(err.msg))
                }
                "MalformedArnException" => {
                    return RusotoError::Service(ListPrincipalsError::MalformedArn(err.msg))
                }
                "ServerInternalException" => {
                    return RusotoError::Service(ListPrincipalsError::ServerInternal(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListPrincipalsError::ServiceUnavailable(err.msg))
                }
                "UnknownResourceException" => {
                    return RusotoError::Service(ListPrincipalsError::UnknownResource(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListPrincipalsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListPrincipalsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListPrincipalsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListPrincipalsError::MalformedArn(ref cause) => write!(f, "{}", cause),
            ListPrincipalsError::ServerInternal(ref cause) => write!(f, "{}", cause),
            ListPrincipalsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            ListPrincipalsError::UnknownResource(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListPrincipalsError {}
/// Errors returned by ListResourceSharePermissions
#[derive(Debug, PartialEq)]
pub enum ListResourceSharePermissionsError {
    /// <p>The specified value for NextToken is not valid.</p>
    InvalidNextToken(String),
    /// <p>A parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>The format of an Amazon Resource Name (ARN) is not valid.</p>
    MalformedArn(String),
    /// <p>The requested operation is not permitted.</p>
    OperationNotPermitted(String),
    /// <p>The service could not respond to the request due to an internal problem.</p>
    ServerInternal(String),
    /// <p>The service is not available.</p>
    ServiceUnavailable(String),
    /// <p>A specified resource was not found.</p>
    UnknownResource(String),
}

impl ListResourceSharePermissionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListResourceSharePermissionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidNextTokenException" => {
                    return RusotoError::Service(
                        ListResourceSharePermissionsError::InvalidNextToken(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        ListResourceSharePermissionsError::InvalidParameter(err.msg),
                    )
                }
                "MalformedArnException" => {
                    return RusotoError::Service(ListResourceSharePermissionsError::MalformedArn(
                        err.msg,
                    ))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(
                        ListResourceSharePermissionsError::OperationNotPermitted(err.msg),
                    )
                }
                "ServerInternalException" => {
                    return RusotoError::Service(ListResourceSharePermissionsError::ServerInternal(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        ListResourceSharePermissionsError::ServiceUnavailable(err.msg),
                    )
                }
                "UnknownResourceException" => {
                    return RusotoError::Service(
                        ListResourceSharePermissionsError::UnknownResource(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListResourceSharePermissionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListResourceSharePermissionsError::InvalidNextToken(ref cause) => {
                write!(f, "{}", cause)
            }
            ListResourceSharePermissionsError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            ListResourceSharePermissionsError::MalformedArn(ref cause) => write!(f, "{}", cause),
            ListResourceSharePermissionsError::OperationNotPermitted(ref cause) => {
                write!(f, "{}", cause)
            }
            ListResourceSharePermissionsError::ServerInternal(ref cause) => write!(f, "{}", cause),
            ListResourceSharePermissionsError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            ListResourceSharePermissionsError::UnknownResource(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListResourceSharePermissionsError {}
/// Errors returned by ListResources
#[derive(Debug, PartialEq)]
pub enum ListResourcesError {
    /// <p>The specified value for NextToken is not valid.</p>
    InvalidNextToken(String),
    /// <p>A parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>The specified resource type is not valid.</p>
    InvalidResourceType(String),
    /// <p>The format of an Amazon Resource Name (ARN) is not valid.</p>
    MalformedArn(String),
    /// <p>The service could not respond to the request due to an internal problem.</p>
    ServerInternal(String),
    /// <p>The service is not available.</p>
    ServiceUnavailable(String),
    /// <p>A specified resource was not found.</p>
    UnknownResource(String),
}

impl ListResourcesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListResourcesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListResourcesError::InvalidNextToken(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListResourcesError::InvalidParameter(err.msg))
                }
                "InvalidResourceTypeException" => {
                    return RusotoError::Service(ListResourcesError::InvalidResourceType(err.msg))
                }
                "MalformedArnException" => {
                    return RusotoError::Service(ListResourcesError::MalformedArn(err.msg))
                }
                "ServerInternalException" => {
                    return RusotoError::Service(ListResourcesError::ServerInternal(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListResourcesError::ServiceUnavailable(err.msg))
                }
                "UnknownResourceException" => {
                    return RusotoError::Service(ListResourcesError::UnknownResource(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListResourcesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListResourcesError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListResourcesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListResourcesError::InvalidResourceType(ref cause) => write!(f, "{}", cause),
            ListResourcesError::MalformedArn(ref cause) => write!(f, "{}", cause),
            ListResourcesError::ServerInternal(ref cause) => write!(f, "{}", cause),
            ListResourcesError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            ListResourcesError::UnknownResource(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListResourcesError {}
/// Errors returned by PromoteResourceShareCreatedFromPolicy
#[derive(Debug, PartialEq)]
pub enum PromoteResourceShareCreatedFromPolicyError {
    /// <p>A parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>The format of an Amazon Resource Name (ARN) is not valid.</p>
    MalformedArn(String),
    /// <p>A required input parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>The requested operation is not permitted.</p>
    OperationNotPermitted(String),
    /// <p>The service could not respond to the request due to an internal problem.</p>
    ServerInternal(String),
    /// <p>The service is not available.</p>
    ServiceUnavailable(String),
}

impl PromoteResourceShareCreatedFromPolicyError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PromoteResourceShareCreatedFromPolicyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        PromoteResourceShareCreatedFromPolicyError::InvalidParameter(err.msg),
                    )
                }
                "MalformedArnException" => {
                    return RusotoError::Service(
                        PromoteResourceShareCreatedFromPolicyError::MalformedArn(err.msg),
                    )
                }
                "MissingRequiredParameterException" => {
                    return RusotoError::Service(
                        PromoteResourceShareCreatedFromPolicyError::MissingRequiredParameter(
                            err.msg,
                        ),
                    )
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(
                        PromoteResourceShareCreatedFromPolicyError::OperationNotPermitted(err.msg),
                    )
                }
                "ServerInternalException" => {
                    return RusotoError::Service(
                        PromoteResourceShareCreatedFromPolicyError::ServerInternal(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        PromoteResourceShareCreatedFromPolicyError::ServiceUnavailable(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PromoteResourceShareCreatedFromPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PromoteResourceShareCreatedFromPolicyError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            PromoteResourceShareCreatedFromPolicyError::MalformedArn(ref cause) => {
                write!(f, "{}", cause)
            }
            PromoteResourceShareCreatedFromPolicyError::MissingRequiredParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            PromoteResourceShareCreatedFromPolicyError::OperationNotPermitted(ref cause) => {
                write!(f, "{}", cause)
            }
            PromoteResourceShareCreatedFromPolicyError::ServerInternal(ref cause) => {
                write!(f, "{}", cause)
            }
            PromoteResourceShareCreatedFromPolicyError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for PromoteResourceShareCreatedFromPolicyError {}
/// Errors returned by RejectResourceShareInvitation
#[derive(Debug, PartialEq)]
pub enum RejectResourceShareInvitationError {
    /// <p>A client token input parameter was reused with an operation, but at least one of the other input parameters is different from the previous call to the operation.</p>
    IdempotentParameterMismatch(String),
    /// <p>A client token is not valid.</p>
    InvalidClientToken(String),
    /// <p>The format of an Amazon Resource Name (ARN) is not valid.</p>
    MalformedArn(String),
    /// <p>The requested operation is not permitted.</p>
    OperationNotPermitted(String),
    /// <p>The invitation was already accepted.</p>
    ResourceShareInvitationAlreadyAccepted(String),
    /// <p>The invitation was already rejected.</p>
    ResourceShareInvitationAlreadyRejected(String),
    /// <p>The Amazon Resource Name (ARN) for an invitation was not found.</p>
    ResourceShareInvitationArnNotFound(String),
    /// <p>The invitation is expired.</p>
    ResourceShareInvitationExpired(String),
    /// <p>The service could not respond to the request due to an internal problem.</p>
    ServerInternal(String),
    /// <p>The service is not available.</p>
    ServiceUnavailable(String),
}

impl RejectResourceShareInvitationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<RejectResourceShareInvitationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "IdempotentParameterMismatchException" => {
                    return RusotoError::Service(
                        RejectResourceShareInvitationError::IdempotentParameterMismatch(err.msg),
                    )
                }
                "InvalidClientTokenException" => {
                    return RusotoError::Service(
                        RejectResourceShareInvitationError::InvalidClientToken(err.msg),
                    )
                }
                "MalformedArnException" => {
                    return RusotoError::Service(RejectResourceShareInvitationError::MalformedArn(
                        err.msg,
                    ))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(
                        RejectResourceShareInvitationError::OperationNotPermitted(err.msg),
                    )
                }
                "ResourceShareInvitationAlreadyAcceptedException" => {
                    return RusotoError::Service(
                        RejectResourceShareInvitationError::ResourceShareInvitationAlreadyAccepted(
                            err.msg,
                        ),
                    )
                }
                "ResourceShareInvitationAlreadyRejectedException" => {
                    return RusotoError::Service(
                        RejectResourceShareInvitationError::ResourceShareInvitationAlreadyRejected(
                            err.msg,
                        ),
                    )
                }
                "ResourceShareInvitationArnNotFoundException" => {
                    return RusotoError::Service(
                        RejectResourceShareInvitationError::ResourceShareInvitationArnNotFound(
                            err.msg,
                        ),
                    )
                }
                "ResourceShareInvitationExpiredException" => {
                    return RusotoError::Service(
                        RejectResourceShareInvitationError::ResourceShareInvitationExpired(err.msg),
                    )
                }
                "ServerInternalException" => {
                    return RusotoError::Service(
                        RejectResourceShareInvitationError::ServerInternal(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        RejectResourceShareInvitationError::ServiceUnavailable(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RejectResourceShareInvitationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RejectResourceShareInvitationError::IdempotentParameterMismatch(ref cause) => {
                write!(f, "{}", cause)
            }
            RejectResourceShareInvitationError::InvalidClientToken(ref cause) => {
                write!(f, "{}", cause)
            }
            RejectResourceShareInvitationError::MalformedArn(ref cause) => write!(f, "{}", cause),
            RejectResourceShareInvitationError::OperationNotPermitted(ref cause) => {
                write!(f, "{}", cause)
            }
            RejectResourceShareInvitationError::ResourceShareInvitationAlreadyAccepted(
                ref cause,
            ) => write!(f, "{}", cause),
            RejectResourceShareInvitationError::ResourceShareInvitationAlreadyRejected(
                ref cause,
            ) => write!(f, "{}", cause),
            RejectResourceShareInvitationError::ResourceShareInvitationArnNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            RejectResourceShareInvitationError::ResourceShareInvitationExpired(ref cause) => {
                write!(f, "{}", cause)
            }
            RejectResourceShareInvitationError::ServerInternal(ref cause) => write!(f, "{}", cause),
            RejectResourceShareInvitationError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for RejectResourceShareInvitationError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>A parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>The format of an Amazon Resource Name (ARN) is not valid.</p>
    MalformedArn(String),
    /// <p>An Amazon Resource Name (ARN) was not found.</p>
    ResourceArnNotFound(String),
    /// <p>The service could not respond to the request due to an internal problem.</p>
    ServerInternal(String),
    /// <p>The service is not available.</p>
    ServiceUnavailable(String),
    /// <p>The requested tags exceed the limit for your account.</p>
    TagLimitExceeded(String),
    /// <p>The specified tag is a reserved word and cannot be used.</p>
    TagPolicyViolation(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(TagResourceError::InvalidParameter(err.msg))
                }
                "MalformedArnException" => {
                    return RusotoError::Service(TagResourceError::MalformedArn(err.msg))
                }
                "ResourceArnNotFoundException" => {
                    return RusotoError::Service(TagResourceError::ResourceArnNotFound(err.msg))
                }
                "ServerInternalException" => {
                    return RusotoError::Service(TagResourceError::ServerInternal(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(TagResourceError::ServiceUnavailable(err.msg))
                }
                "TagLimitExceededException" => {
                    return RusotoError::Service(TagResourceError::TagLimitExceeded(err.msg))
                }
                "TagPolicyViolationException" => {
                    return RusotoError::Service(TagResourceError::TagPolicyViolation(err.msg))
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
            TagResourceError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            TagResourceError::MalformedArn(ref cause) => write!(f, "{}", cause),
            TagResourceError::ResourceArnNotFound(ref cause) => write!(f, "{}", cause),
            TagResourceError::ServerInternal(ref cause) => write!(f, "{}", cause),
            TagResourceError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            TagResourceError::TagLimitExceeded(ref cause) => write!(f, "{}", cause),
            TagResourceError::TagPolicyViolation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>A parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>The service could not respond to the request due to an internal problem.</p>
    ServerInternal(String),
    /// <p>The service is not available.</p>
    ServiceUnavailable(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(UntagResourceError::InvalidParameter(err.msg))
                }
                "ServerInternalException" => {
                    return RusotoError::Service(UntagResourceError::ServerInternal(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UntagResourceError::ServiceUnavailable(err.msg))
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
            UntagResourceError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ServerInternal(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateResourceShare
#[derive(Debug, PartialEq)]
pub enum UpdateResourceShareError {
    /// <p>A client token input parameter was reused with an operation, but at least one of the other input parameters is different from the previous call to the operation.</p>
    IdempotentParameterMismatch(String),
    /// <p>A client token is not valid.</p>
    InvalidClientToken(String),
    /// <p>A parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>The format of an Amazon Resource Name (ARN) is not valid.</p>
    MalformedArn(String),
    /// <p>A required input parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>The requested operation is not permitted.</p>
    OperationNotPermitted(String),
    /// <p>The service could not respond to the request due to an internal problem.</p>
    ServerInternal(String),
    /// <p>The service is not available.</p>
    ServiceUnavailable(String),
    /// <p>A specified resource was not found.</p>
    UnknownResource(String),
}

impl UpdateResourceShareError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateResourceShareError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "IdempotentParameterMismatchException" => {
                    return RusotoError::Service(
                        UpdateResourceShareError::IdempotentParameterMismatch(err.msg),
                    )
                }
                "InvalidClientTokenException" => {
                    return RusotoError::Service(UpdateResourceShareError::InvalidClientToken(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateResourceShareError::InvalidParameter(
                        err.msg,
                    ))
                }
                "MalformedArnException" => {
                    return RusotoError::Service(UpdateResourceShareError::MalformedArn(err.msg))
                }
                "MissingRequiredParameterException" => {
                    return RusotoError::Service(
                        UpdateResourceShareError::MissingRequiredParameter(err.msg),
                    )
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(UpdateResourceShareError::OperationNotPermitted(
                        err.msg,
                    ))
                }
                "ServerInternalException" => {
                    return RusotoError::Service(UpdateResourceShareError::ServerInternal(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdateResourceShareError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "UnknownResourceException" => {
                    return RusotoError::Service(UpdateResourceShareError::UnknownResource(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateResourceShareError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateResourceShareError::IdempotentParameterMismatch(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateResourceShareError::InvalidClientToken(ref cause) => write!(f, "{}", cause),
            UpdateResourceShareError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateResourceShareError::MalformedArn(ref cause) => write!(f, "{}", cause),
            UpdateResourceShareError::MissingRequiredParameter(ref cause) => write!(f, "{}", cause),
            UpdateResourceShareError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            UpdateResourceShareError::ServerInternal(ref cause) => write!(f, "{}", cause),
            UpdateResourceShareError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            UpdateResourceShareError::UnknownResource(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateResourceShareError {}
/// Trait representing the capabilities of the RAM API. RAM clients implement this trait.
#[async_trait]
pub trait Ram {
    /// <p>Accepts an invitation to a resource share from another AWS account.</p>
    async fn accept_resource_share_invitation(
        &self,
        input: AcceptResourceShareInvitationRequest,
    ) -> Result<
        AcceptResourceShareInvitationResponse,
        RusotoError<AcceptResourceShareInvitationError>,
    >;

    /// <p>Associates the specified resource share with the specified principals and resources.</p>
    async fn associate_resource_share(
        &self,
        input: AssociateResourceShareRequest,
    ) -> Result<AssociateResourceShareResponse, RusotoError<AssociateResourceShareError>>;

    /// <p>Associates a permission with a resource share.</p>
    async fn associate_resource_share_permission(
        &self,
        input: AssociateResourceSharePermissionRequest,
    ) -> Result<
        AssociateResourceSharePermissionResponse,
        RusotoError<AssociateResourceSharePermissionError>,
    >;

    /// <p>Creates a resource share.</p>
    async fn create_resource_share(
        &self,
        input: CreateResourceShareRequest,
    ) -> Result<CreateResourceShareResponse, RusotoError<CreateResourceShareError>>;

    /// <p>Deletes the specified resource share.</p>
    async fn delete_resource_share(
        &self,
        input: DeleteResourceShareRequest,
    ) -> Result<DeleteResourceShareResponse, RusotoError<DeleteResourceShareError>>;

    /// <p>Disassociates the specified principals or resources from the specified resource share.</p>
    async fn disassociate_resource_share(
        &self,
        input: DisassociateResourceShareRequest,
    ) -> Result<DisassociateResourceShareResponse, RusotoError<DisassociateResourceShareError>>;

    /// <p>Disassociates an AWS RAM permission from a resource share.</p>
    async fn disassociate_resource_share_permission(
        &self,
        input: DisassociateResourceSharePermissionRequest,
    ) -> Result<
        DisassociateResourceSharePermissionResponse,
        RusotoError<DisassociateResourceSharePermissionError>,
    >;

    /// <p>Enables resource sharing within your AWS Organization.</p> <p>The caller must be the master account for the AWS Organization.</p>
    async fn enable_sharing_with_aws_organization(
        &self,
    ) -> Result<
        EnableSharingWithAwsOrganizationResponse,
        RusotoError<EnableSharingWithAwsOrganizationError>,
    >;

    /// <p>Gets the contents of an AWS RAM permission in JSON format.</p>
    async fn get_permission(
        &self,
        input: GetPermissionRequest,
    ) -> Result<GetPermissionResponse, RusotoError<GetPermissionError>>;

    /// <p>Gets the policies for the specified resources that you own and have shared.</p>
    async fn get_resource_policies(
        &self,
        input: GetResourcePoliciesRequest,
    ) -> Result<GetResourcePoliciesResponse, RusotoError<GetResourcePoliciesError>>;

    /// <p>Gets the resources or principals for the resource shares that you own.</p>
    async fn get_resource_share_associations(
        &self,
        input: GetResourceShareAssociationsRequest,
    ) -> Result<GetResourceShareAssociationsResponse, RusotoError<GetResourceShareAssociationsError>>;

    /// <p>Gets the invitations for resource sharing that you've received.</p>
    async fn get_resource_share_invitations(
        &self,
        input: GetResourceShareInvitationsRequest,
    ) -> Result<GetResourceShareInvitationsResponse, RusotoError<GetResourceShareInvitationsError>>;

    /// <p>Gets the resource shares that you own or the resource shares that are shared with you.</p>
    async fn get_resource_shares(
        &self,
        input: GetResourceSharesRequest,
    ) -> Result<GetResourceSharesResponse, RusotoError<GetResourceSharesError>>;

    /// <p>Lists the resources in a resource share that is shared with you but that the invitation is still pending for.</p>
    async fn list_pending_invitation_resources(
        &self,
        input: ListPendingInvitationResourcesRequest,
    ) -> Result<
        ListPendingInvitationResourcesResponse,
        RusotoError<ListPendingInvitationResourcesError>,
    >;

    /// <p>Lists the AWS RAM permissions.</p>
    async fn list_permissions(
        &self,
        input: ListPermissionsRequest,
    ) -> Result<ListPermissionsResponse, RusotoError<ListPermissionsError>>;

    /// <p>Lists the principals that you have shared resources with or that have shared resources with you.</p>
    async fn list_principals(
        &self,
        input: ListPrincipalsRequest,
    ) -> Result<ListPrincipalsResponse, RusotoError<ListPrincipalsError>>;

    /// <p>Lists the AWS RAM permissions that are associated with a resource share.</p>
    async fn list_resource_share_permissions(
        &self,
        input: ListResourceSharePermissionsRequest,
    ) -> Result<ListResourceSharePermissionsResponse, RusotoError<ListResourceSharePermissionsError>>;

    /// <p>Lists the resources that you added to a resource shares or the resources that are shared with you.</p>
    async fn list_resources(
        &self,
        input: ListResourcesRequest,
    ) -> Result<ListResourcesResponse, RusotoError<ListResourcesError>>;

    /// <p><p>Resource shares that were created by attaching a policy to a resource are visible only to the resource share owner, and the resource share cannot be modified in AWS RAM.</p> <p>Use this API action to promote the resource share. When you promote the resource share, it becomes:</p> <ul> <li> <p>Visible to all principals that it is shared with.</p> </li> <li> <p>Modifiable in AWS RAM.</p> </li> </ul></p>
    async fn promote_resource_share_created_from_policy(
        &self,
        input: PromoteResourceShareCreatedFromPolicyRequest,
    ) -> Result<
        PromoteResourceShareCreatedFromPolicyResponse,
        RusotoError<PromoteResourceShareCreatedFromPolicyError>,
    >;

    /// <p>Rejects an invitation to a resource share from another AWS account.</p>
    async fn reject_resource_share_invitation(
        &self,
        input: RejectResourceShareInvitationRequest,
    ) -> Result<
        RejectResourceShareInvitationResponse,
        RusotoError<RejectResourceShareInvitationError>,
    >;

    /// <p>Adds the specified tags to the specified resource share that you own.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>>;

    /// <p>Removes the specified tags from the specified resource share that you own.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>>;

    /// <p>Updates the specified resource share that you own.</p>
    async fn update_resource_share(
        &self,
        input: UpdateResourceShareRequest,
    ) -> Result<UpdateResourceShareResponse, RusotoError<UpdateResourceShareError>>;
}
/// A client for the RAM API.
#[derive(Clone)]
pub struct RamClient {
    client: Client,
    region: region::Region,
}

impl RamClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> RamClient {
        RamClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> RamClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        RamClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> RamClient {
        RamClient { client, region }
    }
}

#[async_trait]
impl Ram for RamClient {
    /// <p>Accepts an invitation to a resource share from another AWS account.</p>
    async fn accept_resource_share_invitation(
        &self,
        input: AcceptResourceShareInvitationRequest,
    ) -> Result<
        AcceptResourceShareInvitationResponse,
        RusotoError<AcceptResourceShareInvitationError>,
    > {
        let request_uri = "/acceptresourceshareinvitation";

        let mut request = SignedRequest::new("POST", "ram", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<AcceptResourceShareInvitationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(AcceptResourceShareInvitationError::from_response(response))
        }
    }

    /// <p>Associates the specified resource share with the specified principals and resources.</p>
    async fn associate_resource_share(
        &self,
        input: AssociateResourceShareRequest,
    ) -> Result<AssociateResourceShareResponse, RusotoError<AssociateResourceShareError>> {
        let request_uri = "/associateresourceshare";

        let mut request = SignedRequest::new("POST", "ram", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<AssociateResourceShareResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(AssociateResourceShareError::from_response(response))
        }
    }

    /// <p>Associates a permission with a resource share.</p>
    async fn associate_resource_share_permission(
        &self,
        input: AssociateResourceSharePermissionRequest,
    ) -> Result<
        AssociateResourceSharePermissionResponse,
        RusotoError<AssociateResourceSharePermissionError>,
    > {
        let request_uri = "/associateresourcesharepermission";

        let mut request = SignedRequest::new("POST", "ram", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<AssociateResourceSharePermissionResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(AssociateResourceSharePermissionError::from_response(
                response,
            ))
        }
    }

    /// <p>Creates a resource share.</p>
    async fn create_resource_share(
        &self,
        input: CreateResourceShareRequest,
    ) -> Result<CreateResourceShareResponse, RusotoError<CreateResourceShareError>> {
        let request_uri = "/createresourceshare";

        let mut request = SignedRequest::new("POST", "ram", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateResourceShareResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateResourceShareError::from_response(response))
        }
    }

    /// <p>Deletes the specified resource share.</p>
    async fn delete_resource_share(
        &self,
        input: DeleteResourceShareRequest,
    ) -> Result<DeleteResourceShareResponse, RusotoError<DeleteResourceShareError>> {
        let request_uri = "/deleteresourceshare";

        let mut request = SignedRequest::new("DELETE", "ram", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.client_token {
            params.put("clientToken", x);
        }
        params.put("resourceShareArn", &input.resource_share_arn);
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteResourceShareResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteResourceShareError::from_response(response))
        }
    }

    /// <p>Disassociates the specified principals or resources from the specified resource share.</p>
    async fn disassociate_resource_share(
        &self,
        input: DisassociateResourceShareRequest,
    ) -> Result<DisassociateResourceShareResponse, RusotoError<DisassociateResourceShareError>>
    {
        let request_uri = "/disassociateresourceshare";

        let mut request = SignedRequest::new("POST", "ram", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DisassociateResourceShareResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DisassociateResourceShareError::from_response(response))
        }
    }

    /// <p>Disassociates an AWS RAM permission from a resource share.</p>
    async fn disassociate_resource_share_permission(
        &self,
        input: DisassociateResourceSharePermissionRequest,
    ) -> Result<
        DisassociateResourceSharePermissionResponse,
        RusotoError<DisassociateResourceSharePermissionError>,
    > {
        let request_uri = "/disassociateresourcesharepermission";

        let mut request = SignedRequest::new("POST", "ram", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DisassociateResourceSharePermissionResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DisassociateResourceSharePermissionError::from_response(
                response,
            ))
        }
    }

    /// <p>Enables resource sharing within your AWS Organization.</p> <p>The caller must be the master account for the AWS Organization.</p>
    async fn enable_sharing_with_aws_organization(
        &self,
    ) -> Result<
        EnableSharingWithAwsOrganizationResponse,
        RusotoError<EnableSharingWithAwsOrganizationError>,
    > {
        let request_uri = "/enablesharingwithawsorganization";

        let mut request = SignedRequest::new("POST", "ram", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<EnableSharingWithAwsOrganizationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(EnableSharingWithAwsOrganizationError::from_response(
                response,
            ))
        }
    }

    /// <p>Gets the contents of an AWS RAM permission in JSON format.</p>
    async fn get_permission(
        &self,
        input: GetPermissionRequest,
    ) -> Result<GetPermissionResponse, RusotoError<GetPermissionError>> {
        let request_uri = "/getpermission";

        let mut request = SignedRequest::new("POST", "ram", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetPermissionResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetPermissionError::from_response(response))
        }
    }

    /// <p>Gets the policies for the specified resources that you own and have shared.</p>
    async fn get_resource_policies(
        &self,
        input: GetResourcePoliciesRequest,
    ) -> Result<GetResourcePoliciesResponse, RusotoError<GetResourcePoliciesError>> {
        let request_uri = "/getresourcepolicies";

        let mut request = SignedRequest::new("POST", "ram", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetResourcePoliciesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetResourcePoliciesError::from_response(response))
        }
    }

    /// <p>Gets the resources or principals for the resource shares that you own.</p>
    async fn get_resource_share_associations(
        &self,
        input: GetResourceShareAssociationsRequest,
    ) -> Result<GetResourceShareAssociationsResponse, RusotoError<GetResourceShareAssociationsError>>
    {
        let request_uri = "/getresourceshareassociations";

        let mut request = SignedRequest::new("POST", "ram", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetResourceShareAssociationsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetResourceShareAssociationsError::from_response(response))
        }
    }

    /// <p>Gets the invitations for resource sharing that you've received.</p>
    async fn get_resource_share_invitations(
        &self,
        input: GetResourceShareInvitationsRequest,
    ) -> Result<GetResourceShareInvitationsResponse, RusotoError<GetResourceShareInvitationsError>>
    {
        let request_uri = "/getresourceshareinvitations";

        let mut request = SignedRequest::new("POST", "ram", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetResourceShareInvitationsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetResourceShareInvitationsError::from_response(response))
        }
    }

    /// <p>Gets the resource shares that you own or the resource shares that are shared with you.</p>
    async fn get_resource_shares(
        &self,
        input: GetResourceSharesRequest,
    ) -> Result<GetResourceSharesResponse, RusotoError<GetResourceSharesError>> {
        let request_uri = "/getresourceshares";

        let mut request = SignedRequest::new("POST", "ram", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetResourceSharesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetResourceSharesError::from_response(response))
        }
    }

    /// <p>Lists the resources in a resource share that is shared with you but that the invitation is still pending for.</p>
    async fn list_pending_invitation_resources(
        &self,
        input: ListPendingInvitationResourcesRequest,
    ) -> Result<
        ListPendingInvitationResourcesResponse,
        RusotoError<ListPendingInvitationResourcesError>,
    > {
        let request_uri = "/listpendinginvitationresources";

        let mut request = SignedRequest::new("POST", "ram", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListPendingInvitationResourcesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListPendingInvitationResourcesError::from_response(response))
        }
    }

    /// <p>Lists the AWS RAM permissions.</p>
    async fn list_permissions(
        &self,
        input: ListPermissionsRequest,
    ) -> Result<ListPermissionsResponse, RusotoError<ListPermissionsError>> {
        let request_uri = "/listpermissions";

        let mut request = SignedRequest::new("POST", "ram", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListPermissionsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListPermissionsError::from_response(response))
        }
    }

    /// <p>Lists the principals that you have shared resources with or that have shared resources with you.</p>
    async fn list_principals(
        &self,
        input: ListPrincipalsRequest,
    ) -> Result<ListPrincipalsResponse, RusotoError<ListPrincipalsError>> {
        let request_uri = "/listprincipals";

        let mut request = SignedRequest::new("POST", "ram", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListPrincipalsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListPrincipalsError::from_response(response))
        }
    }

    /// <p>Lists the AWS RAM permissions that are associated with a resource share.</p>
    async fn list_resource_share_permissions(
        &self,
        input: ListResourceSharePermissionsRequest,
    ) -> Result<ListResourceSharePermissionsResponse, RusotoError<ListResourceSharePermissionsError>>
    {
        let request_uri = "/listresourcesharepermissions";

        let mut request = SignedRequest::new("POST", "ram", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListResourceSharePermissionsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListResourceSharePermissionsError::from_response(response))
        }
    }

    /// <p>Lists the resources that you added to a resource shares or the resources that are shared with you.</p>
    async fn list_resources(
        &self,
        input: ListResourcesRequest,
    ) -> Result<ListResourcesResponse, RusotoError<ListResourcesError>> {
        let request_uri = "/listresources";

        let mut request = SignedRequest::new("POST", "ram", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListResourcesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListResourcesError::from_response(response))
        }
    }

    /// <p><p>Resource shares that were created by attaching a policy to a resource are visible only to the resource share owner, and the resource share cannot be modified in AWS RAM.</p> <p>Use this API action to promote the resource share. When you promote the resource share, it becomes:</p> <ul> <li> <p>Visible to all principals that it is shared with.</p> </li> <li> <p>Modifiable in AWS RAM.</p> </li> </ul></p>
    async fn promote_resource_share_created_from_policy(
        &self,
        input: PromoteResourceShareCreatedFromPolicyRequest,
    ) -> Result<
        PromoteResourceShareCreatedFromPolicyResponse,
        RusotoError<PromoteResourceShareCreatedFromPolicyError>,
    > {
        let request_uri = "/promoteresourcesharecreatedfrompolicy";

        let mut request = SignedRequest::new("POST", "ram", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        params.put("resourceShareArn", &input.resource_share_arn);
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PromoteResourceShareCreatedFromPolicyResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PromoteResourceShareCreatedFromPolicyError::from_response(
                response,
            ))
        }
    }

    /// <p>Rejects an invitation to a resource share from another AWS account.</p>
    async fn reject_resource_share_invitation(
        &self,
        input: RejectResourceShareInvitationRequest,
    ) -> Result<
        RejectResourceShareInvitationResponse,
        RusotoError<RejectResourceShareInvitationError>,
    > {
        let request_uri = "/rejectresourceshareinvitation";

        let mut request = SignedRequest::new("POST", "ram", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<RejectResourceShareInvitationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(RejectResourceShareInvitationError::from_response(response))
        }
    }

    /// <p>Adds the specified tags to the specified resource share that you own.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>> {
        let request_uri = "/tagresource";

        let mut request = SignedRequest::new("POST", "ram", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<TagResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(TagResourceError::from_response(response))
        }
    }

    /// <p>Removes the specified tags from the specified resource share that you own.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>> {
        let request_uri = "/untagresource";

        let mut request = SignedRequest::new("POST", "ram", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UntagResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UntagResourceError::from_response(response))
        }
    }

    /// <p>Updates the specified resource share that you own.</p>
    async fn update_resource_share(
        &self,
        input: UpdateResourceShareRequest,
    ) -> Result<UpdateResourceShareResponse, RusotoError<UpdateResourceShareError>> {
        let request_uri = "/updateresourceshare";

        let mut request = SignedRequest::new("POST", "ram", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateResourceShareResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateResourceShareError::from_response(response))
        }
    }
}
