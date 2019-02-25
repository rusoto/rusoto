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

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_slice;
use serde_json::Value as SerdeJsonValue;
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
pub struct CreateResourceShareRequest {
    /// <p>Indicates whether principals outside your organization can be associated with a resource share.</p>
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
pub struct DisassociateResourceShareRequest {
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
#[cfg_attr(test, derive(Serialize))]
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
pub struct EnableSharingWithAwsOrganizationRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct EnableSharingWithAwsOrganizationResponse {
    /// <p>Indicates whether the request succeeded.</p>
    #[serde(rename = "returnValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_value: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
#[cfg_attr(test, derive(Serialize))]
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
pub struct GetResourceShareAssociationsRequest {
    /// <p>The status of the association.</p>
    #[serde(rename = "associationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_status: Option<String>,
    /// <p>The association type.</p>
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
    /// <p>The principal.</p>
    #[serde(rename = "principal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "resourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    /// <p>The Amazon Resource Names (ARN) of the resource shares.</p>
    #[serde(rename = "resourceShareArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share_arns: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetResourceShareAssociationsResponse {
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the association.</p>
    #[serde(rename = "resourceShareAssociations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share_associations: Option<Vec<ResourceShareAssociation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
    /// <p>The resource type.</p>
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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
    /// <p>The resource type.</p>
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
pub struct Principal {
    /// <p>The time when the principal was associated with the resource share.</p>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>Indicates whether the principal belongs to the same organization as the AWS account that owns the resource share.</p>
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
pub struct ResourceShare {
    /// <p>Indicates whether principals outside your organization can be associated with a resource share.</p>
    #[serde(rename = "allowExternalPrincipals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_external_principals: Option<bool>,
    /// <p>The time when the resource share was created.</p>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
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
#[cfg_attr(test, derive(Serialize))]
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
    /// <p>Indicates whether the principal belongs to the same organization as the AWS account that owns the resource share.</p>
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
#[cfg_attr(test, derive(Serialize))]
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
    /// <p>The resources associated with the resource share.</p>
    #[serde(rename = "resourceShareAssociations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share_associations: Option<Vec<ResourceShareAssociation>>,
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
pub struct TagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource share.</p>
    #[serde(rename = "resourceShareArn")]
    pub resource_share_arn: String,
    /// <p>One or more tags.</p>
    #[serde(rename = "tags")]
    pub tags: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct TagResourceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UntagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource share.</p>
    #[serde(rename = "resourceShareArn")]
    pub resource_share_arn: String,
    /// <p>The tag keys of the tags to remove.</p>
    #[serde(rename = "tagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UntagResourceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateResourceShareRequest {
    /// <p>Indicates whether principals outside your organization can be associated with a resource share.</p>
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
#[cfg_attr(test, derive(Serialize))]
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

impl AcceptResourceShareInvitationError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> AcceptResourceShareInvitationError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                                "MalformedArnException" => return AcceptResourceShareInvitationError::MalformedArn(String::from(error_message)),
"OperationNotPermittedException" => return AcceptResourceShareInvitationError::OperationNotPermitted(String::from(error_message)),
"ResourceShareInvitationAlreadyAcceptedException" => return AcceptResourceShareInvitationError::ResourceShareInvitationAlreadyAccepted(String::from(error_message)),
"ResourceShareInvitationAlreadyRejectedException" => return AcceptResourceShareInvitationError::ResourceShareInvitationAlreadyRejected(String::from(error_message)),
"ResourceShareInvitationArnNotFoundException" => return AcceptResourceShareInvitationError::ResourceShareInvitationArnNotFound(String::from(error_message)),
"ResourceShareInvitationExpiredException" => return AcceptResourceShareInvitationError::ResourceShareInvitationExpired(String::from(error_message)),
"ServerInternalException" => return AcceptResourceShareInvitationError::ServerInternal(String::from(error_message)),
"ServiceUnavailableException" => return AcceptResourceShareInvitationError::ServiceUnavailable(String::from(error_message)),
"ValidationException" => return AcceptResourceShareInvitationError::Validation(error_message.to_string()),
_ => {}
                            }
        }
        return AcceptResourceShareInvitationError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for AcceptResourceShareInvitationError {
    fn from(err: serde_json::error::Error) -> AcceptResourceShareInvitationError {
        AcceptResourceShareInvitationError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for AcceptResourceShareInvitationError {
    fn from(err: CredentialsError) -> AcceptResourceShareInvitationError {
        AcceptResourceShareInvitationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AcceptResourceShareInvitationError {
    fn from(err: HttpDispatchError) -> AcceptResourceShareInvitationError {
        AcceptResourceShareInvitationError::HttpDispatch(err)
    }
}
impl From<io::Error> for AcceptResourceShareInvitationError {
    fn from(err: io::Error) -> AcceptResourceShareInvitationError {
        AcceptResourceShareInvitationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AcceptResourceShareInvitationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AcceptResourceShareInvitationError {
    fn description(&self) -> &str {
        match *self {
            AcceptResourceShareInvitationError::MalformedArn(ref cause) => cause,
            AcceptResourceShareInvitationError::OperationNotPermitted(ref cause) => cause,
            AcceptResourceShareInvitationError::ResourceShareInvitationAlreadyAccepted(
                ref cause,
            ) => cause,
            AcceptResourceShareInvitationError::ResourceShareInvitationAlreadyRejected(
                ref cause,
            ) => cause,
            AcceptResourceShareInvitationError::ResourceShareInvitationArnNotFound(ref cause) => {
                cause
            }
            AcceptResourceShareInvitationError::ResourceShareInvitationExpired(ref cause) => cause,
            AcceptResourceShareInvitationError::ServerInternal(ref cause) => cause,
            AcceptResourceShareInvitationError::ServiceUnavailable(ref cause) => cause,
            AcceptResourceShareInvitationError::Validation(ref cause) => cause,
            AcceptResourceShareInvitationError::Credentials(ref err) => err.description(),
            AcceptResourceShareInvitationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AcceptResourceShareInvitationError::ParseError(ref cause) => cause,
            AcceptResourceShareInvitationError::Unknown(_) => "unknown error",
        }
    }
}
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

impl AssociateResourceShareError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> AssociateResourceShareError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "IdempotentParameterMismatchException" => {
                    return AssociateResourceShareError::IdempotentParameterMismatch(String::from(
                        error_message,
                    ));
                }
                "InvalidClientTokenException" => {
                    return AssociateResourceShareError::InvalidClientToken(String::from(
                        error_message,
                    ));
                }
                "InvalidParameterException" => {
                    return AssociateResourceShareError::InvalidParameter(String::from(
                        error_message,
                    ));
                }
                "InvalidStateTransitionException" => {
                    return AssociateResourceShareError::InvalidStateTransition(String::from(
                        error_message,
                    ));
                }
                "MalformedArnException" => {
                    return AssociateResourceShareError::MalformedArn(String::from(error_message));
                }
                "OperationNotPermittedException" => {
                    return AssociateResourceShareError::OperationNotPermitted(String::from(
                        error_message,
                    ));
                }
                "ResourceShareLimitExceededException" => {
                    return AssociateResourceShareError::ResourceShareLimitExceeded(String::from(
                        error_message,
                    ));
                }
                "ServerInternalException" => {
                    return AssociateResourceShareError::ServerInternal(String::from(error_message));
                }
                "ServiceUnavailableException" => {
                    return AssociateResourceShareError::ServiceUnavailable(String::from(
                        error_message,
                    ));
                }
                "UnknownResourceException" => {
                    return AssociateResourceShareError::UnknownResource(String::from(error_message));
                }
                "ValidationException" => {
                    return AssociateResourceShareError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return AssociateResourceShareError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for AssociateResourceShareError {
    fn from(err: serde_json::error::Error) -> AssociateResourceShareError {
        AssociateResourceShareError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for AssociateResourceShareError {
    fn from(err: CredentialsError) -> AssociateResourceShareError {
        AssociateResourceShareError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AssociateResourceShareError {
    fn from(err: HttpDispatchError) -> AssociateResourceShareError {
        AssociateResourceShareError::HttpDispatch(err)
    }
}
impl From<io::Error> for AssociateResourceShareError {
    fn from(err: io::Error) -> AssociateResourceShareError {
        AssociateResourceShareError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AssociateResourceShareError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AssociateResourceShareError {
    fn description(&self) -> &str {
        match *self {
            AssociateResourceShareError::IdempotentParameterMismatch(ref cause) => cause,
            AssociateResourceShareError::InvalidClientToken(ref cause) => cause,
            AssociateResourceShareError::InvalidParameter(ref cause) => cause,
            AssociateResourceShareError::InvalidStateTransition(ref cause) => cause,
            AssociateResourceShareError::MalformedArn(ref cause) => cause,
            AssociateResourceShareError::OperationNotPermitted(ref cause) => cause,
            AssociateResourceShareError::ResourceShareLimitExceeded(ref cause) => cause,
            AssociateResourceShareError::ServerInternal(ref cause) => cause,
            AssociateResourceShareError::ServiceUnavailable(ref cause) => cause,
            AssociateResourceShareError::UnknownResource(ref cause) => cause,
            AssociateResourceShareError::Validation(ref cause) => cause,
            AssociateResourceShareError::Credentials(ref err) => err.description(),
            AssociateResourceShareError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AssociateResourceShareError::ParseError(ref cause) => cause,
            AssociateResourceShareError::Unknown(_) => "unknown error",
        }
    }
}
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
    /// <p>A specified resource was not found.</p>
    UnknownResource(String),
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

impl CreateResourceShareError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> CreateResourceShareError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "IdempotentParameterMismatchException" => {
                    return CreateResourceShareError::IdempotentParameterMismatch(String::from(
                        error_message,
                    ));
                }
                "InvalidClientTokenException" => {
                    return CreateResourceShareError::InvalidClientToken(String::from(error_message));
                }
                "InvalidParameterException" => {
                    return CreateResourceShareError::InvalidParameter(String::from(error_message));
                }
                "InvalidStateTransitionException" => {
                    return CreateResourceShareError::InvalidStateTransition(String::from(
                        error_message,
                    ));
                }
                "MalformedArnException" => {
                    return CreateResourceShareError::MalformedArn(String::from(error_message));
                }
                "OperationNotPermittedException" => {
                    return CreateResourceShareError::OperationNotPermitted(String::from(
                        error_message,
                    ));
                }
                "ResourceShareLimitExceededException" => {
                    return CreateResourceShareError::ResourceShareLimitExceeded(String::from(
                        error_message,
                    ));
                }
                "ServerInternalException" => {
                    return CreateResourceShareError::ServerInternal(String::from(error_message));
                }
                "ServiceUnavailableException" => {
                    return CreateResourceShareError::ServiceUnavailable(String::from(error_message));
                }
                "UnknownResourceException" => {
                    return CreateResourceShareError::UnknownResource(String::from(error_message));
                }
                "ValidationException" => {
                    return CreateResourceShareError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return CreateResourceShareError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateResourceShareError {
    fn from(err: serde_json::error::Error) -> CreateResourceShareError {
        CreateResourceShareError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateResourceShareError {
    fn from(err: CredentialsError) -> CreateResourceShareError {
        CreateResourceShareError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateResourceShareError {
    fn from(err: HttpDispatchError) -> CreateResourceShareError {
        CreateResourceShareError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateResourceShareError {
    fn from(err: io::Error) -> CreateResourceShareError {
        CreateResourceShareError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateResourceShareError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateResourceShareError {
    fn description(&self) -> &str {
        match *self {
            CreateResourceShareError::IdempotentParameterMismatch(ref cause) => cause,
            CreateResourceShareError::InvalidClientToken(ref cause) => cause,
            CreateResourceShareError::InvalidParameter(ref cause) => cause,
            CreateResourceShareError::InvalidStateTransition(ref cause) => cause,
            CreateResourceShareError::MalformedArn(ref cause) => cause,
            CreateResourceShareError::OperationNotPermitted(ref cause) => cause,
            CreateResourceShareError::ResourceShareLimitExceeded(ref cause) => cause,
            CreateResourceShareError::ServerInternal(ref cause) => cause,
            CreateResourceShareError::ServiceUnavailable(ref cause) => cause,
            CreateResourceShareError::UnknownResource(ref cause) => cause,
            CreateResourceShareError::Validation(ref cause) => cause,
            CreateResourceShareError::Credentials(ref err) => err.description(),
            CreateResourceShareError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateResourceShareError::ParseError(ref cause) => cause,
            CreateResourceShareError::Unknown(_) => "unknown error",
        }
    }
}
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

impl DeleteResourceShareError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DeleteResourceShareError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "IdempotentParameterMismatchException" => {
                    return DeleteResourceShareError::IdempotentParameterMismatch(String::from(
                        error_message,
                    ));
                }
                "InvalidClientTokenException" => {
                    return DeleteResourceShareError::InvalidClientToken(String::from(error_message));
                }
                "InvalidParameterException" => {
                    return DeleteResourceShareError::InvalidParameter(String::from(error_message));
                }
                "InvalidStateTransitionException" => {
                    return DeleteResourceShareError::InvalidStateTransition(String::from(
                        error_message,
                    ));
                }
                "MalformedArnException" => {
                    return DeleteResourceShareError::MalformedArn(String::from(error_message));
                }
                "OperationNotPermittedException" => {
                    return DeleteResourceShareError::OperationNotPermitted(String::from(
                        error_message,
                    ));
                }
                "ServerInternalException" => {
                    return DeleteResourceShareError::ServerInternal(String::from(error_message));
                }
                "ServiceUnavailableException" => {
                    return DeleteResourceShareError::ServiceUnavailable(String::from(error_message));
                }
                "UnknownResourceException" => {
                    return DeleteResourceShareError::UnknownResource(String::from(error_message));
                }
                "ValidationException" => {
                    return DeleteResourceShareError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DeleteResourceShareError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteResourceShareError {
    fn from(err: serde_json::error::Error) -> DeleteResourceShareError {
        DeleteResourceShareError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteResourceShareError {
    fn from(err: CredentialsError) -> DeleteResourceShareError {
        DeleteResourceShareError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteResourceShareError {
    fn from(err: HttpDispatchError) -> DeleteResourceShareError {
        DeleteResourceShareError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteResourceShareError {
    fn from(err: io::Error) -> DeleteResourceShareError {
        DeleteResourceShareError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteResourceShareError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteResourceShareError {
    fn description(&self) -> &str {
        match *self {
            DeleteResourceShareError::IdempotentParameterMismatch(ref cause) => cause,
            DeleteResourceShareError::InvalidClientToken(ref cause) => cause,
            DeleteResourceShareError::InvalidParameter(ref cause) => cause,
            DeleteResourceShareError::InvalidStateTransition(ref cause) => cause,
            DeleteResourceShareError::MalformedArn(ref cause) => cause,
            DeleteResourceShareError::OperationNotPermitted(ref cause) => cause,
            DeleteResourceShareError::ServerInternal(ref cause) => cause,
            DeleteResourceShareError::ServiceUnavailable(ref cause) => cause,
            DeleteResourceShareError::UnknownResource(ref cause) => cause,
            DeleteResourceShareError::Validation(ref cause) => cause,
            DeleteResourceShareError::Credentials(ref err) => err.description(),
            DeleteResourceShareError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteResourceShareError::ParseError(ref cause) => cause,
            DeleteResourceShareError::Unknown(_) => "unknown error",
        }
    }
}
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

impl DisassociateResourceShareError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DisassociateResourceShareError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "IdempotentParameterMismatchException" => {
                    return DisassociateResourceShareError::IdempotentParameterMismatch(
                        String::from(error_message),
                    );
                }
                "InvalidClientTokenException" => {
                    return DisassociateResourceShareError::InvalidClientToken(String::from(
                        error_message,
                    ));
                }
                "InvalidParameterException" => {
                    return DisassociateResourceShareError::InvalidParameter(String::from(
                        error_message,
                    ));
                }
                "InvalidStateTransitionException" => {
                    return DisassociateResourceShareError::InvalidStateTransition(String::from(
                        error_message,
                    ));
                }
                "MalformedArnException" => {
                    return DisassociateResourceShareError::MalformedArn(String::from(error_message));
                }
                "OperationNotPermittedException" => {
                    return DisassociateResourceShareError::OperationNotPermitted(String::from(
                        error_message,
                    ));
                }
                "ResourceShareLimitExceededException" => {
                    return DisassociateResourceShareError::ResourceShareLimitExceeded(String::from(
                        error_message,
                    ));
                }
                "ServerInternalException" => {
                    return DisassociateResourceShareError::ServerInternal(String::from(
                        error_message,
                    ));
                }
                "ServiceUnavailableException" => {
                    return DisassociateResourceShareError::ServiceUnavailable(String::from(
                        error_message,
                    ));
                }
                "UnknownResourceException" => {
                    return DisassociateResourceShareError::UnknownResource(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return DisassociateResourceShareError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DisassociateResourceShareError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DisassociateResourceShareError {
    fn from(err: serde_json::error::Error) -> DisassociateResourceShareError {
        DisassociateResourceShareError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DisassociateResourceShareError {
    fn from(err: CredentialsError) -> DisassociateResourceShareError {
        DisassociateResourceShareError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DisassociateResourceShareError {
    fn from(err: HttpDispatchError) -> DisassociateResourceShareError {
        DisassociateResourceShareError::HttpDispatch(err)
    }
}
impl From<io::Error> for DisassociateResourceShareError {
    fn from(err: io::Error) -> DisassociateResourceShareError {
        DisassociateResourceShareError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DisassociateResourceShareError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisassociateResourceShareError {
    fn description(&self) -> &str {
        match *self {
            DisassociateResourceShareError::IdempotentParameterMismatch(ref cause) => cause,
            DisassociateResourceShareError::InvalidClientToken(ref cause) => cause,
            DisassociateResourceShareError::InvalidParameter(ref cause) => cause,
            DisassociateResourceShareError::InvalidStateTransition(ref cause) => cause,
            DisassociateResourceShareError::MalformedArn(ref cause) => cause,
            DisassociateResourceShareError::OperationNotPermitted(ref cause) => cause,
            DisassociateResourceShareError::ResourceShareLimitExceeded(ref cause) => cause,
            DisassociateResourceShareError::ServerInternal(ref cause) => cause,
            DisassociateResourceShareError::ServiceUnavailable(ref cause) => cause,
            DisassociateResourceShareError::UnknownResource(ref cause) => cause,
            DisassociateResourceShareError::Validation(ref cause) => cause,
            DisassociateResourceShareError::Credentials(ref err) => err.description(),
            DisassociateResourceShareError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DisassociateResourceShareError::ParseError(ref cause) => cause,
            DisassociateResourceShareError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by EnableSharingWithAwsOrganization
#[derive(Debug, PartialEq)]
pub enum EnableSharingWithAwsOrganizationError {
    /// <p>The requested operation is not permitted.</p>
    OperationNotPermitted(String),
    /// <p>The service could not respond to the request due to an internal problem.</p>
    ServerInternal(String),
    /// <p>The service is not available.</p>
    ServiceUnavailable(String),
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

impl EnableSharingWithAwsOrganizationError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> EnableSharingWithAwsOrganizationError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "OperationNotPermittedException" => {
                    return EnableSharingWithAwsOrganizationError::OperationNotPermitted(
                        String::from(error_message),
                    );
                }
                "ServerInternalException" => {
                    return EnableSharingWithAwsOrganizationError::ServerInternal(String::from(
                        error_message,
                    ));
                }
                "ServiceUnavailableException" => {
                    return EnableSharingWithAwsOrganizationError::ServiceUnavailable(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return EnableSharingWithAwsOrganizationError::Validation(
                        error_message.to_string(),
                    );
                }
                _ => {}
            }
        }
        return EnableSharingWithAwsOrganizationError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for EnableSharingWithAwsOrganizationError {
    fn from(err: serde_json::error::Error) -> EnableSharingWithAwsOrganizationError {
        EnableSharingWithAwsOrganizationError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for EnableSharingWithAwsOrganizationError {
    fn from(err: CredentialsError) -> EnableSharingWithAwsOrganizationError {
        EnableSharingWithAwsOrganizationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for EnableSharingWithAwsOrganizationError {
    fn from(err: HttpDispatchError) -> EnableSharingWithAwsOrganizationError {
        EnableSharingWithAwsOrganizationError::HttpDispatch(err)
    }
}
impl From<io::Error> for EnableSharingWithAwsOrganizationError {
    fn from(err: io::Error) -> EnableSharingWithAwsOrganizationError {
        EnableSharingWithAwsOrganizationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for EnableSharingWithAwsOrganizationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for EnableSharingWithAwsOrganizationError {
    fn description(&self) -> &str {
        match *self {
            EnableSharingWithAwsOrganizationError::OperationNotPermitted(ref cause) => cause,
            EnableSharingWithAwsOrganizationError::ServerInternal(ref cause) => cause,
            EnableSharingWithAwsOrganizationError::ServiceUnavailable(ref cause) => cause,
            EnableSharingWithAwsOrganizationError::Validation(ref cause) => cause,
            EnableSharingWithAwsOrganizationError::Credentials(ref err) => err.description(),
            EnableSharingWithAwsOrganizationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            EnableSharingWithAwsOrganizationError::ParseError(ref cause) => cause,
            EnableSharingWithAwsOrganizationError::Unknown(_) => "unknown error",
        }
    }
}
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

impl GetResourcePoliciesError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetResourcePoliciesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InvalidNextTokenException" => {
                    return GetResourcePoliciesError::InvalidNextToken(String::from(error_message));
                }
                "InvalidParameterException" => {
                    return GetResourcePoliciesError::InvalidParameter(String::from(error_message));
                }
                "MalformedArnException" => {
                    return GetResourcePoliciesError::MalformedArn(String::from(error_message));
                }
                "ServerInternalException" => {
                    return GetResourcePoliciesError::ServerInternal(String::from(error_message));
                }
                "ServiceUnavailableException" => {
                    return GetResourcePoliciesError::ServiceUnavailable(String::from(error_message));
                }
                "ValidationException" => {
                    return GetResourcePoliciesError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return GetResourcePoliciesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetResourcePoliciesError {
    fn from(err: serde_json::error::Error) -> GetResourcePoliciesError {
        GetResourcePoliciesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetResourcePoliciesError {
    fn from(err: CredentialsError) -> GetResourcePoliciesError {
        GetResourcePoliciesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetResourcePoliciesError {
    fn from(err: HttpDispatchError) -> GetResourcePoliciesError {
        GetResourcePoliciesError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetResourcePoliciesError {
    fn from(err: io::Error) -> GetResourcePoliciesError {
        GetResourcePoliciesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetResourcePoliciesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetResourcePoliciesError {
    fn description(&self) -> &str {
        match *self {
            GetResourcePoliciesError::InvalidNextToken(ref cause) => cause,
            GetResourcePoliciesError::InvalidParameter(ref cause) => cause,
            GetResourcePoliciesError::MalformedArn(ref cause) => cause,
            GetResourcePoliciesError::ServerInternal(ref cause) => cause,
            GetResourcePoliciesError::ServiceUnavailable(ref cause) => cause,
            GetResourcePoliciesError::Validation(ref cause) => cause,
            GetResourcePoliciesError::Credentials(ref err) => err.description(),
            GetResourcePoliciesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetResourcePoliciesError::ParseError(ref cause) => cause,
            GetResourcePoliciesError::Unknown(_) => "unknown error",
        }
    }
}
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

impl GetResourceShareAssociationsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetResourceShareAssociationsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InvalidNextTokenException" => {
                    return GetResourceShareAssociationsError::InvalidNextToken(String::from(
                        error_message,
                    ));
                }
                "InvalidParameterException" => {
                    return GetResourceShareAssociationsError::InvalidParameter(String::from(
                        error_message,
                    ));
                }
                "MalformedArnException" => {
                    return GetResourceShareAssociationsError::MalformedArn(String::from(
                        error_message,
                    ));
                }
                "OperationNotPermittedException" => {
                    return GetResourceShareAssociationsError::OperationNotPermitted(String::from(
                        error_message,
                    ));
                }
                "ServerInternalException" => {
                    return GetResourceShareAssociationsError::ServerInternal(String::from(
                        error_message,
                    ));
                }
                "ServiceUnavailableException" => {
                    return GetResourceShareAssociationsError::ServiceUnavailable(String::from(
                        error_message,
                    ));
                }
                "UnknownResourceException" => {
                    return GetResourceShareAssociationsError::UnknownResource(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return GetResourceShareAssociationsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return GetResourceShareAssociationsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetResourceShareAssociationsError {
    fn from(err: serde_json::error::Error) -> GetResourceShareAssociationsError {
        GetResourceShareAssociationsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetResourceShareAssociationsError {
    fn from(err: CredentialsError) -> GetResourceShareAssociationsError {
        GetResourceShareAssociationsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetResourceShareAssociationsError {
    fn from(err: HttpDispatchError) -> GetResourceShareAssociationsError {
        GetResourceShareAssociationsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetResourceShareAssociationsError {
    fn from(err: io::Error) -> GetResourceShareAssociationsError {
        GetResourceShareAssociationsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetResourceShareAssociationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetResourceShareAssociationsError {
    fn description(&self) -> &str {
        match *self {
            GetResourceShareAssociationsError::InvalidNextToken(ref cause) => cause,
            GetResourceShareAssociationsError::InvalidParameter(ref cause) => cause,
            GetResourceShareAssociationsError::MalformedArn(ref cause) => cause,
            GetResourceShareAssociationsError::OperationNotPermitted(ref cause) => cause,
            GetResourceShareAssociationsError::ServerInternal(ref cause) => cause,
            GetResourceShareAssociationsError::ServiceUnavailable(ref cause) => cause,
            GetResourceShareAssociationsError::UnknownResource(ref cause) => cause,
            GetResourceShareAssociationsError::Validation(ref cause) => cause,
            GetResourceShareAssociationsError::Credentials(ref err) => err.description(),
            GetResourceShareAssociationsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetResourceShareAssociationsError::ParseError(ref cause) => cause,
            GetResourceShareAssociationsError::Unknown(_) => "unknown error",
        }
    }
}
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

impl GetResourceShareInvitationsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetResourceShareInvitationsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InvalidMaxResultsException" => {
                    return GetResourceShareInvitationsError::InvalidMaxResults(String::from(
                        error_message,
                    ));
                }
                "InvalidNextTokenException" => {
                    return GetResourceShareInvitationsError::InvalidNextToken(String::from(
                        error_message,
                    ));
                }
                "InvalidParameterException" => {
                    return GetResourceShareInvitationsError::InvalidParameter(String::from(
                        error_message,
                    ));
                }
                "MalformedArnException" => {
                    return GetResourceShareInvitationsError::MalformedArn(String::from(
                        error_message,
                    ));
                }
                "ResourceShareInvitationArnNotFoundException" => {
                    return GetResourceShareInvitationsError::ResourceShareInvitationArnNotFound(
                        String::from(error_message),
                    );
                }
                "ServerInternalException" => {
                    return GetResourceShareInvitationsError::ServerInternal(String::from(
                        error_message,
                    ));
                }
                "ServiceUnavailableException" => {
                    return GetResourceShareInvitationsError::ServiceUnavailable(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return GetResourceShareInvitationsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return GetResourceShareInvitationsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetResourceShareInvitationsError {
    fn from(err: serde_json::error::Error) -> GetResourceShareInvitationsError {
        GetResourceShareInvitationsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetResourceShareInvitationsError {
    fn from(err: CredentialsError) -> GetResourceShareInvitationsError {
        GetResourceShareInvitationsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetResourceShareInvitationsError {
    fn from(err: HttpDispatchError) -> GetResourceShareInvitationsError {
        GetResourceShareInvitationsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetResourceShareInvitationsError {
    fn from(err: io::Error) -> GetResourceShareInvitationsError {
        GetResourceShareInvitationsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetResourceShareInvitationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetResourceShareInvitationsError {
    fn description(&self) -> &str {
        match *self {
            GetResourceShareInvitationsError::InvalidMaxResults(ref cause) => cause,
            GetResourceShareInvitationsError::InvalidNextToken(ref cause) => cause,
            GetResourceShareInvitationsError::InvalidParameter(ref cause) => cause,
            GetResourceShareInvitationsError::MalformedArn(ref cause) => cause,
            GetResourceShareInvitationsError::ResourceShareInvitationArnNotFound(ref cause) => {
                cause
            }
            GetResourceShareInvitationsError::ServerInternal(ref cause) => cause,
            GetResourceShareInvitationsError::ServiceUnavailable(ref cause) => cause,
            GetResourceShareInvitationsError::Validation(ref cause) => cause,
            GetResourceShareInvitationsError::Credentials(ref err) => err.description(),
            GetResourceShareInvitationsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetResourceShareInvitationsError::ParseError(ref cause) => cause,
            GetResourceShareInvitationsError::Unknown(_) => "unknown error",
        }
    }
}
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

impl GetResourceSharesError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetResourceSharesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InvalidNextTokenException" => {
                    return GetResourceSharesError::InvalidNextToken(String::from(error_message));
                }
                "InvalidParameterException" => {
                    return GetResourceSharesError::InvalidParameter(String::from(error_message));
                }
                "MalformedArnException" => {
                    return GetResourceSharesError::MalformedArn(String::from(error_message));
                }
                "ServerInternalException" => {
                    return GetResourceSharesError::ServerInternal(String::from(error_message));
                }
                "ServiceUnavailableException" => {
                    return GetResourceSharesError::ServiceUnavailable(String::from(error_message));
                }
                "UnknownResourceException" => {
                    return GetResourceSharesError::UnknownResource(String::from(error_message));
                }
                "ValidationException" => {
                    return GetResourceSharesError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return GetResourceSharesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetResourceSharesError {
    fn from(err: serde_json::error::Error) -> GetResourceSharesError {
        GetResourceSharesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetResourceSharesError {
    fn from(err: CredentialsError) -> GetResourceSharesError {
        GetResourceSharesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetResourceSharesError {
    fn from(err: HttpDispatchError) -> GetResourceSharesError {
        GetResourceSharesError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetResourceSharesError {
    fn from(err: io::Error) -> GetResourceSharesError {
        GetResourceSharesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetResourceSharesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetResourceSharesError {
    fn description(&self) -> &str {
        match *self {
            GetResourceSharesError::InvalidNextToken(ref cause) => cause,
            GetResourceSharesError::InvalidParameter(ref cause) => cause,
            GetResourceSharesError::MalformedArn(ref cause) => cause,
            GetResourceSharesError::ServerInternal(ref cause) => cause,
            GetResourceSharesError::ServiceUnavailable(ref cause) => cause,
            GetResourceSharesError::UnknownResource(ref cause) => cause,
            GetResourceSharesError::Validation(ref cause) => cause,
            GetResourceSharesError::Credentials(ref err) => err.description(),
            GetResourceSharesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetResourceSharesError::ParseError(ref cause) => cause,
            GetResourceSharesError::Unknown(_) => "unknown error",
        }
    }
}
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

impl ListPrincipalsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> ListPrincipalsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InvalidNextTokenException" => {
                    return ListPrincipalsError::InvalidNextToken(String::from(error_message));
                }
                "InvalidParameterException" => {
                    return ListPrincipalsError::InvalidParameter(String::from(error_message));
                }
                "MalformedArnException" => {
                    return ListPrincipalsError::MalformedArn(String::from(error_message));
                }
                "ServerInternalException" => {
                    return ListPrincipalsError::ServerInternal(String::from(error_message));
                }
                "ServiceUnavailableException" => {
                    return ListPrincipalsError::ServiceUnavailable(String::from(error_message));
                }
                "UnknownResourceException" => {
                    return ListPrincipalsError::UnknownResource(String::from(error_message));
                }
                "ValidationException" => {
                    return ListPrincipalsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ListPrincipalsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListPrincipalsError {
    fn from(err: serde_json::error::Error) -> ListPrincipalsError {
        ListPrincipalsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListPrincipalsError {
    fn from(err: CredentialsError) -> ListPrincipalsError {
        ListPrincipalsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListPrincipalsError {
    fn from(err: HttpDispatchError) -> ListPrincipalsError {
        ListPrincipalsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListPrincipalsError {
    fn from(err: io::Error) -> ListPrincipalsError {
        ListPrincipalsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListPrincipalsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListPrincipalsError {
    fn description(&self) -> &str {
        match *self {
            ListPrincipalsError::InvalidNextToken(ref cause) => cause,
            ListPrincipalsError::InvalidParameter(ref cause) => cause,
            ListPrincipalsError::MalformedArn(ref cause) => cause,
            ListPrincipalsError::ServerInternal(ref cause) => cause,
            ListPrincipalsError::ServiceUnavailable(ref cause) => cause,
            ListPrincipalsError::UnknownResource(ref cause) => cause,
            ListPrincipalsError::Validation(ref cause) => cause,
            ListPrincipalsError::Credentials(ref err) => err.description(),
            ListPrincipalsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListPrincipalsError::ParseError(ref cause) => cause,
            ListPrincipalsError::Unknown(_) => "unknown error",
        }
    }
}
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

impl ListResourcesError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> ListResourcesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InvalidNextTokenException" => {
                    return ListResourcesError::InvalidNextToken(String::from(error_message));
                }
                "InvalidParameterException" => {
                    return ListResourcesError::InvalidParameter(String::from(error_message));
                }
                "InvalidResourceTypeException" => {
                    return ListResourcesError::InvalidResourceType(String::from(error_message));
                }
                "MalformedArnException" => {
                    return ListResourcesError::MalformedArn(String::from(error_message));
                }
                "ServerInternalException" => {
                    return ListResourcesError::ServerInternal(String::from(error_message));
                }
                "ServiceUnavailableException" => {
                    return ListResourcesError::ServiceUnavailable(String::from(error_message));
                }
                "UnknownResourceException" => {
                    return ListResourcesError::UnknownResource(String::from(error_message));
                }
                "ValidationException" => {
                    return ListResourcesError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ListResourcesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListResourcesError {
    fn from(err: serde_json::error::Error) -> ListResourcesError {
        ListResourcesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListResourcesError {
    fn from(err: CredentialsError) -> ListResourcesError {
        ListResourcesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListResourcesError {
    fn from(err: HttpDispatchError) -> ListResourcesError {
        ListResourcesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListResourcesError {
    fn from(err: io::Error) -> ListResourcesError {
        ListResourcesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListResourcesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListResourcesError {
    fn description(&self) -> &str {
        match *self {
            ListResourcesError::InvalidNextToken(ref cause) => cause,
            ListResourcesError::InvalidParameter(ref cause) => cause,
            ListResourcesError::InvalidResourceType(ref cause) => cause,
            ListResourcesError::MalformedArn(ref cause) => cause,
            ListResourcesError::ServerInternal(ref cause) => cause,
            ListResourcesError::ServiceUnavailable(ref cause) => cause,
            ListResourcesError::UnknownResource(ref cause) => cause,
            ListResourcesError::Validation(ref cause) => cause,
            ListResourcesError::Credentials(ref err) => err.description(),
            ListResourcesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListResourcesError::ParseError(ref cause) => cause,
            ListResourcesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by RejectResourceShareInvitation
#[derive(Debug, PartialEq)]
pub enum RejectResourceShareInvitationError {
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

impl RejectResourceShareInvitationError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RejectResourceShareInvitationError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                                "MalformedArnException" => return RejectResourceShareInvitationError::MalformedArn(String::from(error_message)),
"OperationNotPermittedException" => return RejectResourceShareInvitationError::OperationNotPermitted(String::from(error_message)),
"ResourceShareInvitationAlreadyAcceptedException" => return RejectResourceShareInvitationError::ResourceShareInvitationAlreadyAccepted(String::from(error_message)),
"ResourceShareInvitationAlreadyRejectedException" => return RejectResourceShareInvitationError::ResourceShareInvitationAlreadyRejected(String::from(error_message)),
"ResourceShareInvitationArnNotFoundException" => return RejectResourceShareInvitationError::ResourceShareInvitationArnNotFound(String::from(error_message)),
"ResourceShareInvitationExpiredException" => return RejectResourceShareInvitationError::ResourceShareInvitationExpired(String::from(error_message)),
"ServerInternalException" => return RejectResourceShareInvitationError::ServerInternal(String::from(error_message)),
"ServiceUnavailableException" => return RejectResourceShareInvitationError::ServiceUnavailable(String::from(error_message)),
"ValidationException" => return RejectResourceShareInvitationError::Validation(error_message.to_string()),
_ => {}
                            }
        }
        return RejectResourceShareInvitationError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for RejectResourceShareInvitationError {
    fn from(err: serde_json::error::Error) -> RejectResourceShareInvitationError {
        RejectResourceShareInvitationError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for RejectResourceShareInvitationError {
    fn from(err: CredentialsError) -> RejectResourceShareInvitationError {
        RejectResourceShareInvitationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RejectResourceShareInvitationError {
    fn from(err: HttpDispatchError) -> RejectResourceShareInvitationError {
        RejectResourceShareInvitationError::HttpDispatch(err)
    }
}
impl From<io::Error> for RejectResourceShareInvitationError {
    fn from(err: io::Error) -> RejectResourceShareInvitationError {
        RejectResourceShareInvitationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RejectResourceShareInvitationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RejectResourceShareInvitationError {
    fn description(&self) -> &str {
        match *self {
            RejectResourceShareInvitationError::MalformedArn(ref cause) => cause,
            RejectResourceShareInvitationError::OperationNotPermitted(ref cause) => cause,
            RejectResourceShareInvitationError::ResourceShareInvitationAlreadyAccepted(
                ref cause,
            ) => cause,
            RejectResourceShareInvitationError::ResourceShareInvitationAlreadyRejected(
                ref cause,
            ) => cause,
            RejectResourceShareInvitationError::ResourceShareInvitationArnNotFound(ref cause) => {
                cause
            }
            RejectResourceShareInvitationError::ResourceShareInvitationExpired(ref cause) => cause,
            RejectResourceShareInvitationError::ServerInternal(ref cause) => cause,
            RejectResourceShareInvitationError::ServiceUnavailable(ref cause) => cause,
            RejectResourceShareInvitationError::Validation(ref cause) => cause,
            RejectResourceShareInvitationError::Credentials(ref err) => err.description(),
            RejectResourceShareInvitationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RejectResourceShareInvitationError::ParseError(ref cause) => cause,
            RejectResourceShareInvitationError::Unknown(_) => "unknown error",
        }
    }
}
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

impl TagResourceError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> TagResourceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InvalidParameterException" => {
                    return TagResourceError::InvalidParameter(String::from(error_message));
                }
                "MalformedArnException" => {
                    return TagResourceError::MalformedArn(String::from(error_message));
                }
                "ResourceArnNotFoundException" => {
                    return TagResourceError::ResourceArnNotFound(String::from(error_message));
                }
                "ServerInternalException" => {
                    return TagResourceError::ServerInternal(String::from(error_message));
                }
                "ServiceUnavailableException" => {
                    return TagResourceError::ServiceUnavailable(String::from(error_message));
                }
                "TagLimitExceededException" => {
                    return TagResourceError::TagLimitExceeded(String::from(error_message));
                }
                "ValidationException" => {
                    return TagResourceError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return TagResourceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for TagResourceError {
    fn from(err: serde_json::error::Error) -> TagResourceError {
        TagResourceError::ParseError(err.description().to_string())
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
impl From<io::Error> for TagResourceError {
    fn from(err: io::Error) -> TagResourceError {
        TagResourceError::HttpDispatch(HttpDispatchError::from(err))
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
            TagResourceError::InvalidParameter(ref cause) => cause,
            TagResourceError::MalformedArn(ref cause) => cause,
            TagResourceError::ResourceArnNotFound(ref cause) => cause,
            TagResourceError::ServerInternal(ref cause) => cause,
            TagResourceError::ServiceUnavailable(ref cause) => cause,
            TagResourceError::TagLimitExceeded(ref cause) => cause,
            TagResourceError::Validation(ref cause) => cause,
            TagResourceError::Credentials(ref err) => err.description(),
            TagResourceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            TagResourceError::ParseError(ref cause) => cause,
            TagResourceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>A parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>The service could not respond to the request due to an internal problem.</p>
    ServerInternal(String),
    /// <p>The service is not available.</p>
    ServiceUnavailable(String),
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

impl UntagResourceError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> UntagResourceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InvalidParameterException" => {
                    return UntagResourceError::InvalidParameter(String::from(error_message));
                }
                "ServerInternalException" => {
                    return UntagResourceError::ServerInternal(String::from(error_message));
                }
                "ServiceUnavailableException" => {
                    return UntagResourceError::ServiceUnavailable(String::from(error_message));
                }
                "ValidationException" => {
                    return UntagResourceError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return UntagResourceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UntagResourceError {
    fn from(err: serde_json::error::Error) -> UntagResourceError {
        UntagResourceError::ParseError(err.description().to_string())
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
impl From<io::Error> for UntagResourceError {
    fn from(err: io::Error) -> UntagResourceError {
        UntagResourceError::HttpDispatch(HttpDispatchError::from(err))
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
            UntagResourceError::InvalidParameter(ref cause) => cause,
            UntagResourceError::ServerInternal(ref cause) => cause,
            UntagResourceError::ServiceUnavailable(ref cause) => cause,
            UntagResourceError::Validation(ref cause) => cause,
            UntagResourceError::Credentials(ref err) => err.description(),
            UntagResourceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UntagResourceError::ParseError(ref cause) => cause,
            UntagResourceError::Unknown(_) => "unknown error",
        }
    }
}
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

impl UpdateResourceShareError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> UpdateResourceShareError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "IdempotentParameterMismatchException" => {
                    return UpdateResourceShareError::IdempotentParameterMismatch(String::from(
                        error_message,
                    ));
                }
                "InvalidClientTokenException" => {
                    return UpdateResourceShareError::InvalidClientToken(String::from(error_message));
                }
                "InvalidParameterException" => {
                    return UpdateResourceShareError::InvalidParameter(String::from(error_message));
                }
                "MalformedArnException" => {
                    return UpdateResourceShareError::MalformedArn(String::from(error_message));
                }
                "MissingRequiredParameterException" => {
                    return UpdateResourceShareError::MissingRequiredParameter(String::from(
                        error_message,
                    ));
                }
                "OperationNotPermittedException" => {
                    return UpdateResourceShareError::OperationNotPermitted(String::from(
                        error_message,
                    ));
                }
                "ServerInternalException" => {
                    return UpdateResourceShareError::ServerInternal(String::from(error_message));
                }
                "ServiceUnavailableException" => {
                    return UpdateResourceShareError::ServiceUnavailable(String::from(error_message));
                }
                "UnknownResourceException" => {
                    return UpdateResourceShareError::UnknownResource(String::from(error_message));
                }
                "ValidationException" => {
                    return UpdateResourceShareError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return UpdateResourceShareError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateResourceShareError {
    fn from(err: serde_json::error::Error) -> UpdateResourceShareError {
        UpdateResourceShareError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateResourceShareError {
    fn from(err: CredentialsError) -> UpdateResourceShareError {
        UpdateResourceShareError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateResourceShareError {
    fn from(err: HttpDispatchError) -> UpdateResourceShareError {
        UpdateResourceShareError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateResourceShareError {
    fn from(err: io::Error) -> UpdateResourceShareError {
        UpdateResourceShareError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateResourceShareError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateResourceShareError {
    fn description(&self) -> &str {
        match *self {
            UpdateResourceShareError::IdempotentParameterMismatch(ref cause) => cause,
            UpdateResourceShareError::InvalidClientToken(ref cause) => cause,
            UpdateResourceShareError::InvalidParameter(ref cause) => cause,
            UpdateResourceShareError::MalformedArn(ref cause) => cause,
            UpdateResourceShareError::MissingRequiredParameter(ref cause) => cause,
            UpdateResourceShareError::OperationNotPermitted(ref cause) => cause,
            UpdateResourceShareError::ServerInternal(ref cause) => cause,
            UpdateResourceShareError::ServiceUnavailable(ref cause) => cause,
            UpdateResourceShareError::UnknownResource(ref cause) => cause,
            UpdateResourceShareError::Validation(ref cause) => cause,
            UpdateResourceShareError::Credentials(ref err) => err.description(),
            UpdateResourceShareError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateResourceShareError::ParseError(ref cause) => cause,
            UpdateResourceShareError::Unknown(_) => "unknown error",
        }
    }
}
/// Trait representing the capabilities of the RAM API. RAM clients implement this trait.
pub trait Ram {
    /// <p>Accepts an invitation to a resource share from another AWS account.</p>
    fn accept_resource_share_invitation(
        &self,
        input: AcceptResourceShareInvitationRequest,
    ) -> RusotoFuture<AcceptResourceShareInvitationResponse, AcceptResourceShareInvitationError>;

    /// <p>Associates the specified resource share with the specified principals and resources.</p>
    fn associate_resource_share(
        &self,
        input: AssociateResourceShareRequest,
    ) -> RusotoFuture<AssociateResourceShareResponse, AssociateResourceShareError>;

    /// <p>Creates a resource share.</p>
    fn create_resource_share(
        &self,
        input: CreateResourceShareRequest,
    ) -> RusotoFuture<CreateResourceShareResponse, CreateResourceShareError>;

    /// <p>Deletes the specified resource share.</p>
    fn delete_resource_share(
        &self,
        input: DeleteResourceShareRequest,
    ) -> RusotoFuture<DeleteResourceShareResponse, DeleteResourceShareError>;

    /// <p>Disassociates the specified principals or resources from the specified resource share.</p>
    fn disassociate_resource_share(
        &self,
        input: DisassociateResourceShareRequest,
    ) -> RusotoFuture<DisassociateResourceShareResponse, DisassociateResourceShareError>;

    /// <p>Enables resource sharing within your organization.</p>
    fn enable_sharing_with_aws_organization(
        &self,
    ) -> RusotoFuture<EnableSharingWithAwsOrganizationResponse, EnableSharingWithAwsOrganizationError>;

    /// <p>Gets the policies for the specifies resources.</p>
    fn get_resource_policies(
        &self,
        input: GetResourcePoliciesRequest,
    ) -> RusotoFuture<GetResourcePoliciesResponse, GetResourcePoliciesError>;

    /// <p>Gets the associations for the specified resource share.</p>
    fn get_resource_share_associations(
        &self,
        input: GetResourceShareAssociationsRequest,
    ) -> RusotoFuture<GetResourceShareAssociationsResponse, GetResourceShareAssociationsError>;

    /// <p>Gets the specified invitations for resource sharing.</p>
    fn get_resource_share_invitations(
        &self,
        input: GetResourceShareInvitationsRequest,
    ) -> RusotoFuture<GetResourceShareInvitationsResponse, GetResourceShareInvitationsError>;

    /// <p>Gets the specified resource shares or all of your resource shares.</p>
    fn get_resource_shares(
        &self,
        input: GetResourceSharesRequest,
    ) -> RusotoFuture<GetResourceSharesResponse, GetResourceSharesError>;

    /// <p>Lists the principals with access to the specified resource.</p>
    fn list_principals(
        &self,
        input: ListPrincipalsRequest,
    ) -> RusotoFuture<ListPrincipalsResponse, ListPrincipalsError>;

    /// <p>Lists the resources that the specified principal can access.</p>
    fn list_resources(
        &self,
        input: ListResourcesRequest,
    ) -> RusotoFuture<ListResourcesResponse, ListResourcesError>;

    /// <p>Rejects an invitation to a resource share from another AWS account.</p>
    fn reject_resource_share_invitation(
        &self,
        input: RejectResourceShareInvitationRequest,
    ) -> RusotoFuture<RejectResourceShareInvitationResponse, RejectResourceShareInvitationError>;

    /// <p>Adds the specified tags to the specified resource share.</p>
    fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> RusotoFuture<TagResourceResponse, TagResourceError>;

    /// <p>Removes the specified tags from the specified resource share.</p>
    fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> RusotoFuture<UntagResourceResponse, UntagResourceError>;

    /// <p>Updates the specified resource share.</p>
    fn update_resource_share(
        &self,
        input: UpdateResourceShareRequest,
    ) -> RusotoFuture<UpdateResourceShareResponse, UpdateResourceShareError>;
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
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> RamClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        RamClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl Ram for RamClient {
    /// <p>Accepts an invitation to a resource share from another AWS account.</p>
    fn accept_resource_share_invitation(
        &self,
        input: AcceptResourceShareInvitationRequest,
    ) -> RusotoFuture<AcceptResourceShareInvitationResponse, AcceptResourceShareInvitationError>
    {
        let request_uri = "/acceptresourceshareinvitation";

        let mut request = SignedRequest::new("POST", "ram", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<AcceptResourceShareInvitationResponse>(&body)
                            .unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AcceptResourceShareInvitationError::from_response(response))
                }))
            }
        })
    }

    /// <p>Associates the specified resource share with the specified principals and resources.</p>
    fn associate_resource_share(
        &self,
        input: AssociateResourceShareRequest,
    ) -> RusotoFuture<AssociateResourceShareResponse, AssociateResourceShareError> {
        let request_uri = "/associateresourceshare";

        let mut request = SignedRequest::new("POST", "ram", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<AssociateResourceShareResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(AssociateResourceShareError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Creates a resource share.</p>
    fn create_resource_share(
        &self,
        input: CreateResourceShareRequest,
    ) -> RusotoFuture<CreateResourceShareResponse, CreateResourceShareError> {
        let request_uri = "/createresourceshare";

        let mut request = SignedRequest::new("POST", "ram", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<CreateResourceShareResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(CreateResourceShareError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Deletes the specified resource share.</p>
    fn delete_resource_share(
        &self,
        input: DeleteResourceShareRequest,
    ) -> RusotoFuture<DeleteResourceShareResponse, DeleteResourceShareError> {
        let request_uri = "/deleteresourceshare";

        let mut request = SignedRequest::new("DELETE", "ram", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.client_token {
            params.put("clientToken", x);
        }
        params.put("resourceShareArn", &input.resource_share_arn);
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<DeleteResourceShareResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteResourceShareError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Disassociates the specified principals or resources from the specified resource share.</p>
    fn disassociate_resource_share(
        &self,
        input: DisassociateResourceShareRequest,
    ) -> RusotoFuture<DisassociateResourceShareResponse, DisassociateResourceShareError> {
        let request_uri = "/disassociateresourceshare";

        let mut request = SignedRequest::new("POST", "ram", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<DisassociateResourceShareResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DisassociateResourceShareError::from_response(response))
                }))
            }
        })
    }

    /// <p>Enables resource sharing within your organization.</p>
    fn enable_sharing_with_aws_organization(
        &self,
    ) -> RusotoFuture<EnableSharingWithAwsOrganizationResponse, EnableSharingWithAwsOrganizationError>
    {
        let request_uri = "/enablesharingwithawsorganization";

        let mut request = SignedRequest::new("POST", "ram", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<EnableSharingWithAwsOrganizationResponse>(&body)
                            .unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(EnableSharingWithAwsOrganizationError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Gets the policies for the specifies resources.</p>
    fn get_resource_policies(
        &self,
        input: GetResourcePoliciesRequest,
    ) -> RusotoFuture<GetResourcePoliciesResponse, GetResourcePoliciesError> {
        let request_uri = "/getresourcepolicies";

        let mut request = SignedRequest::new("POST", "ram", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<GetResourcePoliciesResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetResourcePoliciesError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Gets the associations for the specified resource share.</p>
    fn get_resource_share_associations(
        &self,
        input: GetResourceShareAssociationsRequest,
    ) -> RusotoFuture<GetResourceShareAssociationsResponse, GetResourceShareAssociationsError> {
        let request_uri = "/getresourceshareassociations";

        let mut request = SignedRequest::new("POST", "ram", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<GetResourceShareAssociationsResponse>(&body)
                            .unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetResourceShareAssociationsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Gets the specified invitations for resource sharing.</p>
    fn get_resource_share_invitations(
        &self,
        input: GetResourceShareInvitationsRequest,
    ) -> RusotoFuture<GetResourceShareInvitationsResponse, GetResourceShareInvitationsError> {
        let request_uri = "/getresourceshareinvitations";

        let mut request = SignedRequest::new("POST", "ram", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<GetResourceShareInvitationsResponse>(&body)
                            .unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetResourceShareInvitationsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Gets the specified resource shares or all of your resource shares.</p>
    fn get_resource_shares(
        &self,
        input: GetResourceSharesRequest,
    ) -> RusotoFuture<GetResourceSharesResponse, GetResourceSharesError> {
        let request_uri = "/getresourceshares";

        let mut request = SignedRequest::new("POST", "ram", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<GetResourceSharesResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetResourceSharesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists the principals with access to the specified resource.</p>
    fn list_principals(
        &self,
        input: ListPrincipalsRequest,
    ) -> RusotoFuture<ListPrincipalsResponse, ListPrincipalsError> {
        let request_uri = "/listprincipals";

        let mut request = SignedRequest::new("POST", "ram", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ListPrincipalsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListPrincipalsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists the resources that the specified principal can access.</p>
    fn list_resources(
        &self,
        input: ListResourcesRequest,
    ) -> RusotoFuture<ListResourcesResponse, ListResourcesError> {
        let request_uri = "/listresources";

        let mut request = SignedRequest::new("POST", "ram", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ListResourcesResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListResourcesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Rejects an invitation to a resource share from another AWS account.</p>
    fn reject_resource_share_invitation(
        &self,
        input: RejectResourceShareInvitationRequest,
    ) -> RusotoFuture<RejectResourceShareInvitationResponse, RejectResourceShareInvitationError>
    {
        let request_uri = "/rejectresourceshareinvitation";

        let mut request = SignedRequest::new("POST", "ram", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<RejectResourceShareInvitationResponse>(&body)
                            .unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(RejectResourceShareInvitationError::from_response(response))
                }))
            }
        })
    }

    /// <p>Adds the specified tags to the specified resource share.</p>
    fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> RusotoFuture<TagResourceResponse, TagResourceError> {
        let request_uri = "/tagresource";

        let mut request = SignedRequest::new("POST", "ram", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<TagResourceResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(TagResourceError::from_response(response))),
                )
            }
        })
    }

    /// <p>Removes the specified tags from the specified resource share.</p>
    fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> RusotoFuture<UntagResourceResponse, UntagResourceError> {
        let request_uri = "/untagresource";

        let mut request = SignedRequest::new("POST", "ram", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<UntagResourceResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UntagResourceError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates the specified resource share.</p>
    fn update_resource_share(
        &self,
        input: UpdateResourceShareRequest,
    ) -> RusotoFuture<UpdateResourceShareResponse, UpdateResourceShareError> {
        let request_uri = "/updateresourceshare";

        let mut request = SignedRequest::new("POST", "ram", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<UpdateResourceShareResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateResourceShareError::from_response(response))
                    }),
                )
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
