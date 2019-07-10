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

#[allow(warnings)]
use futures::future;
use futures::Future;
use rusoto_core::compression::CompressRequestPayload;
use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError, RusotoFuture};

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
use serde_json;
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
}

impl AcceptResourceShareInvitationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AcceptResourceShareInvitationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
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
        return RusotoError::Unknown(res);
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
        return RusotoError::Unknown(res);
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
                "UnknownResourceException" => {
                    return RusotoError::Service(CreateResourceShareError::UnknownResource(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        return RusotoError::Unknown(res);
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
        return RusotoError::Unknown(res);
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
        return RusotoError::Unknown(res);
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
        return RusotoError::Unknown(res);
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
        return RusotoError::Unknown(res);
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
        return RusotoError::Unknown(res);
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
        return RusotoError::Unknown(res);
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
        return RusotoError::Unknown(res);
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
        return RusotoError::Unknown(res);
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
}

impl RejectResourceShareInvitationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<RejectResourceShareInvitationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
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
        return RusotoError::Unknown(res);
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
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        return RusotoError::Unknown(res);
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
        return RusotoError::Unknown(res);
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
            region,
        }
    }

    pub fn new_with<P, D, C>(
        request_dispatcher: D,
        credentials_provider: P,
        payload_compressor: Option<C>,
        region: region::Region,
    ) -> RamClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
        C: CompressRequestPayload + Send + Sync + 'static,
    {
        RamClient {
            client: Client::new_with(credentials_provider, request_dispatcher, payload_compressor),
            region,
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
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<AcceptResourceShareInvitationResponse, _>()?;

                    Ok(result)
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
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<AssociateResourceShareResponse, _>()?;

                    Ok(result)
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
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateResourceShareResponse, _>()?;

                    Ok(result)
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
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteResourceShareResponse, _>()?;

                    Ok(result)
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
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DisassociateResourceShareResponse, _>()?;

                    Ok(result)
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
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<EnableSharingWithAwsOrganizationResponse, _>()?;

                    Ok(result)
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
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetResourcePoliciesResponse, _>()?;

                    Ok(result)
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
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetResourceShareAssociationsResponse, _>()?;

                    Ok(result)
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
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetResourceShareInvitationsResponse, _>()?;

                    Ok(result)
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
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetResourceSharesResponse, _>()?;

                    Ok(result)
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
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListPrincipalsResponse, _>()?;

                    Ok(result)
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
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListResourcesResponse, _>()?;

                    Ok(result)
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
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<RejectResourceShareInvitationResponse, _>()?;

                    Ok(result)
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
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<TagResourceResponse, _>()?;

                    Ok(result)
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
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UntagResourceResponse, _>()?;

                    Ok(result)
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
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateResourceShareResponse, _>()?;

                    Ok(result)
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
