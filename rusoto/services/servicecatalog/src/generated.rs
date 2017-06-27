#[allow(warnings)]
use hyper::Client;
use hyper::status::StatusCode;
use rusoto_core::request::DispatchSignedRequest;
use rusoto_core::region;

use std::fmt;
use std::error::Error;
use rusoto_core::request::HttpDispatchError;
use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};

use serde_json;
use rusoto_core::signature::SignedRequest;
use serde_json::Value as SerdeJsonValue;
use serde_json::from_str;
pub type AcceptLanguage = String;
#[derive(Default,Debug,Clone,Serialize)]
pub struct AcceptPortfolioShareInput {
    #[doc="<p>The language code to use for this operation. Supported language codes are as follows:</p> <p>\"en\" (English)</p> <p>\"jp\" (Japanese)</p> <p>\"zh\" (Chinese)</p> <p>If no code is specified, \"en\" is used as the default.</p>"]
    #[serde(rename="AcceptLanguage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub accept_language: Option<AcceptLanguage>,
    #[doc="<p>The portfolio identifier.</p>"]
    #[serde(rename="PortfolioId")]
    pub portfolio_id: Id,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct AcceptPortfolioShareOutput;

#[doc="<p>The access level to limit results.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct AccessLevelFilter {
    #[doc="<p>Specifies the access level.</p> <p> <code>Account</code> allows results at the account level. </p> <p> <code>Role</code> allows results based on the federated role of the specified user.</p> <p> <code>User</code> allows results limited to the specified user. </p>"]
    #[serde(rename="Key")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub key: Option<AccessLevelFilterKey>,
    #[doc="<p>Specifies the user to which the access level applies. A value of <code>Self</code> is currently supported.</p>"]
    #[serde(rename="Value")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub value: Option<AccessLevelFilterValue>,
}

pub type AccessLevelFilterKey = String;
pub type AccessLevelFilterValue = String;
pub type AccountId = String;
pub type AccountIds = Vec<AccountId>;
pub type AddTags = Vec<Tag>;
pub type AllowedValue = String;
pub type AllowedValues = Vec<AllowedValue>;
pub type ApproximateCount = i64;
#[derive(Default,Debug,Clone,Serialize)]
pub struct AssociatePrincipalWithPortfolioInput {
    #[doc="<p>The language code to use for this operation. Supported language codes are as follows:</p> <p>\"en\" (English)</p> <p>\"jp\" (Japanese)</p> <p>\"zh\" (Chinese)</p> <p>If no code is specified, \"en\" is used as the default.</p>"]
    #[serde(rename="AcceptLanguage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub accept_language: Option<AcceptLanguage>,
    #[doc="<p>The portfolio identifier.</p>"]
    #[serde(rename="PortfolioId")]
    pub portfolio_id: Id,
    #[doc="<p>The ARN representing the principal (IAM user, role, or group).</p>"]
    #[serde(rename="PrincipalARN")]
    pub principal_arn: PrincipalARN,
    #[doc="<p>The principal type. Must be <code>IAM</code> </p>"]
    #[serde(rename="PrincipalType")]
    pub principal_type: PrincipalType,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct AssociatePrincipalWithPortfolioOutput;

#[derive(Default,Debug,Clone,Serialize)]
pub struct AssociateProductWithPortfolioInput {
    #[doc="<p>The language code to use for this operation. Supported language codes are as follows:</p> <p>\"en\" (English)</p> <p>\"jp\" (Japanese)</p> <p>\"zh\" (Chinese)</p> <p>If no code is specified, \"en\" is used as the default.</p>"]
    #[serde(rename="AcceptLanguage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub accept_language: Option<AcceptLanguage>,
    #[doc="<p>The portfolio identifier.</p>"]
    #[serde(rename="PortfolioId")]
    pub portfolio_id: Id,
    #[doc="<p>The product identifier.</p>"]
    #[serde(rename="ProductId")]
    pub product_id: Id,
    #[doc="<p>The identifier of the source portfolio to use with this association.</p>"]
    #[serde(rename="SourcePortfolioId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub source_portfolio_id: Option<Id>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct AssociateProductWithPortfolioOutput;

pub type AttributeValue = String;
pub type ConstraintDescription = String;
#[doc="<p>Detailed constraint information.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ConstraintDetail {
    #[doc="<p>The identifier of the constraint.</p>"]
    #[serde(rename="ConstraintId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub constraint_id: Option<Id>,
    #[doc="<p>The text description of the constraint.</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<ConstraintDescription>,
    #[doc="<p>The owner of the constraint.</p>"]
    #[serde(rename="Owner")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub owner: Option<AccountId>,
    #[doc="<p>The type of the constraint.</p>"]
    #[serde(rename="Type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub type_: Option<ConstraintType>,
}

pub type ConstraintDetails = Vec<ConstraintDetail>;
pub type ConstraintParameters = String;
pub type ConstraintSummaries = Vec<ConstraintSummary>;
#[doc="<p>An administrator-specified constraint to apply when provisioning a product.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ConstraintSummary {
    #[doc="<p>The text description of the constraint.</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<ConstraintDescription>,
    #[doc="<p>The type of the constraint. </p>"]
    #[serde(rename="Type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub type_: Option<ConstraintType>,
}

pub type ConstraintType = String;
#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateConstraintInput {
    #[doc="<p>The language code to use for this operation. Supported language codes are as follows:</p> <p>\"en\" (English)</p> <p>\"jp\" (Japanese)</p> <p>\"zh\" (Chinese)</p> <p>If no code is specified, \"en\" is used as the default.</p>"]
    #[serde(rename="AcceptLanguage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub accept_language: Option<AcceptLanguage>,
    #[doc="<p>The text description of the constraint.</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<ConstraintDescription>,
    #[doc="<p>A token to disambiguate duplicate requests. You can create multiple resources using the same input in multiple requests, provided that you also specify a different idempotency token for each request.</p>"]
    #[serde(rename="IdempotencyToken")]
    pub idempotency_token: IdempotencyToken,
    #[doc="<p>The constraint parameters. Expected values vary depending on which <b>Type</b> is specified. For examples, see the bottom of this topic.</p> <p>For Type <code>LAUNCH</code>, the <code>RoleArn</code> property is required. </p> <p>For Type <code>NOTIFICATION</code>, the <code>NotificationArns</code> property is required.</p> <p>For Type <code>TEMPLATE</code>, the <code>Rules</code> property is required.</p>"]
    #[serde(rename="Parameters")]
    pub parameters: ConstraintParameters,
    #[doc="<p>The portfolio identifier.</p>"]
    #[serde(rename="PortfolioId")]
    pub portfolio_id: Id,
    #[doc="<p>The product identifier.</p>"]
    #[serde(rename="ProductId")]
    pub product_id: Id,
    #[doc="<p>The type of the constraint. Case-sensitive valid values are: <code>LAUNCH</code>, <code>NOTIFICATION</code>, or <code>TEMPLATE</code>. </p>"]
    #[serde(rename="Type")]
    pub type_: ConstraintType,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateConstraintOutput {
    #[doc="<p>The resulting detailed constraint information.</p>"]
    #[serde(rename="ConstraintDetail")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub constraint_detail: Option<ConstraintDetail>,
    #[doc="<p>The resulting constraint parameters.</p>"]
    #[serde(rename="ConstraintParameters")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub constraint_parameters: Option<ConstraintParameters>,
    #[doc="<p>The status of the current request.</p>"]
    #[serde(rename="Status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<Status>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct CreatePortfolioInput {
    #[doc="<p>The language code to use for this operation. Supported language codes are as follows:</p> <p>\"en\" (English)</p> <p>\"jp\" (Japanese)</p> <p>\"zh\" (Chinese)</p> <p>If no code is specified, \"en\" is used as the default.</p>"]
    #[serde(rename="AcceptLanguage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub accept_language: Option<AcceptLanguage>,
    #[doc="<p>The text description of the portfolio.</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<PortfolioDescription>,
    #[doc="<p>The name to use for display purposes.</p>"]
    #[serde(rename="DisplayName")]
    pub display_name: PortfolioDisplayName,
    #[doc="<p>A token to disambiguate duplicate requests. You can create multiple resources using the same input in multiple requests, provided that you also specify a different idempotency token for each request.</p>"]
    #[serde(rename="IdempotencyToken")]
    pub idempotency_token: IdempotencyToken,
    #[doc="<p>The name of the portfolio provider.</p>"]
    #[serde(rename="ProviderName")]
    pub provider_name: ProviderName,
    #[doc="<p>Tags to associate with the new portfolio.</p>"]
    #[serde(rename="Tags")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tags: Option<AddTags>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreatePortfolioOutput {
    #[doc="<p>The resulting detailed portfolio information.</p>"]
    #[serde(rename="PortfolioDetail")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub portfolio_detail: Option<PortfolioDetail>,
    #[doc="<p>Tags successfully associated with the new portfolio.</p>"]
    #[serde(rename="Tags")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tags: Option<Tags>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct CreatePortfolioShareInput {
    #[doc="<p>The language code to use for this operation. Supported language codes are as follows:</p> <p>\"en\" (English)</p> <p>\"jp\" (Japanese)</p> <p>\"zh\" (Chinese)</p> <p>If no code is specified, \"en\" is used as the default.</p>"]
    #[serde(rename="AcceptLanguage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub accept_language: Option<AcceptLanguage>,
    #[doc="<p>The account ID with which to share the portfolio.</p>"]
    #[serde(rename="AccountId")]
    pub account_id: AccountId,
    #[doc="<p>The portfolio identifier.</p>"]
    #[serde(rename="PortfolioId")]
    pub portfolio_id: Id,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreatePortfolioShareOutput;

#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateProductInput {
    #[doc="<p>The language code to use for this operation. Supported language codes are as follows:</p> <p>\"en\" (English)</p> <p>\"jp\" (Japanese)</p> <p>\"zh\" (Chinese)</p> <p>If no code is specified, \"en\" is used as the default.</p>"]
    #[serde(rename="AcceptLanguage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub accept_language: Option<AcceptLanguage>,
    #[doc="<p>The text description of the product.</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<ProductViewShortDescription>,
    #[doc="<p>The distributor of the product.</p>"]
    #[serde(rename="Distributor")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub distributor: Option<ProductViewOwner>,
    #[doc="<p>A token to disambiguate duplicate requests. You can create multiple resources using the same input in multiple requests, provided that you also specify a different idempotency token for each request.</p>"]
    #[serde(rename="IdempotencyToken")]
    pub idempotency_token: IdempotencyToken,
    #[doc="<p>The name of the product.</p>"]
    #[serde(rename="Name")]
    pub name: ProductViewName,
    #[doc="<p>The owner of the product.</p>"]
    #[serde(rename="Owner")]
    pub owner: ProductViewOwner,
    #[doc="<p>The type of the product to create.</p>"]
    #[serde(rename="ProductType")]
    pub product_type: ProductType,
    #[doc="<p>Parameters for the provisioning artifact.</p>"]
    #[serde(rename="ProvisioningArtifactParameters")]
    pub provisioning_artifact_parameters: ProvisioningArtifactProperties,
    #[doc="<p>Support information about the product.</p>"]
    #[serde(rename="SupportDescription")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub support_description: Option<SupportDescription>,
    #[doc="<p>Contact email for product support.</p>"]
    #[serde(rename="SupportEmail")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub support_email: Option<SupportEmail>,
    #[doc="<p>Contact URL for product support.</p>"]
    #[serde(rename="SupportUrl")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub support_url: Option<SupportUrl>,
    #[doc="<p>Tags to associate with the new product.</p>"]
    #[serde(rename="Tags")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tags: Option<AddTags>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateProductOutput {
    #[doc="<p>The resulting detailed product view information.</p>"]
    #[serde(rename="ProductViewDetail")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub product_view_detail: Option<ProductViewDetail>,
    #[doc="<p>The resulting detailed provisioning artifact information.</p>"]
    #[serde(rename="ProvisioningArtifactDetail")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub provisioning_artifact_detail: Option<ProvisioningArtifactDetail>,
    #[doc="<p>Tags successfully associated with the new product.</p>"]
    #[serde(rename="Tags")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tags: Option<Tags>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateProvisioningArtifactInput {
    #[doc="<p>The language code to use for this operation. Supported language codes are as follows:</p> <p>\"en\" (English)</p> <p>\"jp\" (Japanese)</p> <p>\"zh\" (Chinese)</p> <p>If no code is specified, \"en\" is used as the default.</p>"]
    #[serde(rename="AcceptLanguage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub accept_language: Option<AcceptLanguage>,
    #[doc="<p>A token to disambiguate duplicate requests. You can create multiple resources using the same input in multiple requests, provided that you also specify a different idempotency token for each request.</p>"]
    #[serde(rename="IdempotencyToken")]
    pub idempotency_token: IdempotencyToken,
    #[doc="<p>The parameters to use when creating the new provisioning artifact.</p>"]
    #[serde(rename="Parameters")]
    pub parameters: ProvisioningArtifactProperties,
    #[doc="<p>The product identifier.</p>"]
    #[serde(rename="ProductId")]
    pub product_id: Id,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateProvisioningArtifactOutput {
    #[doc="<p>Additional information about the creation request for the provisioning artifact.</p>"]
    #[serde(rename="Info")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub info: Option<ProvisioningArtifactInfo>,
    #[doc="<p>The resulting detailed provisioning artifact information.</p>"]
    #[serde(rename="ProvisioningArtifactDetail")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub provisioning_artifact_detail: Option<ProvisioningArtifactDetail>,
    #[doc="<p>The status of the current request.</p>"]
    #[serde(rename="Status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<Status>,
}

pub type CreatedTime = f64;
pub type CreationTime = f64;
pub type DefaultValue = String;
#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteConstraintInput {
    #[doc="<p>The language code to use for this operation. Supported language codes are as follows:</p> <p>\"en\" (English)</p> <p>\"jp\" (Japanese)</p> <p>\"zh\" (Chinese)</p> <p>If no code is specified, \"en\" is used as the default.</p>"]
    #[serde(rename="AcceptLanguage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub accept_language: Option<AcceptLanguage>,
    #[doc="<p>The identifier of the constraint to delete.</p>"]
    #[serde(rename="Id")]
    pub id: Id,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteConstraintOutput;

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeletePortfolioInput {
    #[doc="<p>The language code to use for this operation. Supported language codes are as follows:</p> <p>\"en\" (English)</p> <p>\"jp\" (Japanese)</p> <p>\"zh\" (Chinese)</p> <p>If no code is specified, \"en\" is used as the default.</p>"]
    #[serde(rename="AcceptLanguage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub accept_language: Option<AcceptLanguage>,
    #[doc="<p>The identifier of the portfolio for the delete request.</p>"]
    #[serde(rename="Id")]
    pub id: Id,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeletePortfolioOutput;

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeletePortfolioShareInput {
    #[doc="<p>The language code to use for this operation. Supported language codes are as follows:</p> <p>\"en\" (English)</p> <p>\"jp\" (Japanese)</p> <p>\"zh\" (Chinese)</p> <p>If no code is specified, \"en\" is used as the default.</p>"]
    #[serde(rename="AcceptLanguage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub accept_language: Option<AcceptLanguage>,
    #[doc="<p>The account ID associated with the share to delete.</p>"]
    #[serde(rename="AccountId")]
    pub account_id: AccountId,
    #[doc="<p>The portfolio identifier.</p>"]
    #[serde(rename="PortfolioId")]
    pub portfolio_id: Id,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeletePortfolioShareOutput;

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteProductInput {
    #[doc="<p>The language code to use for this operation. Supported language codes are as follows:</p> <p>\"en\" (English)</p> <p>\"jp\" (Japanese)</p> <p>\"zh\" (Chinese)</p> <p>If no code is specified, \"en\" is used as the default.</p>"]
    #[serde(rename="AcceptLanguage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub accept_language: Option<AcceptLanguage>,
    #[doc="<p>The identifier of the product for the delete request.</p>"]
    #[serde(rename="Id")]
    pub id: Id,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteProductOutput;

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteProvisioningArtifactInput {
    #[doc="<p>The language code to use for this operation. Supported language codes are as follows:</p> <p>\"en\" (English)</p> <p>\"jp\" (Japanese)</p> <p>\"zh\" (Chinese)</p> <p>If no code is specified, \"en\" is used as the default.</p>"]
    #[serde(rename="AcceptLanguage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub accept_language: Option<AcceptLanguage>,
    #[doc="<p>The product identifier.</p>"]
    #[serde(rename="ProductId")]
    pub product_id: Id,
    #[doc="<p>The identifier of the provisioning artifact for the delete request. This is sometimes referred to as the product version.</p>"]
    #[serde(rename="ProvisioningArtifactId")]
    pub provisioning_artifact_id: Id,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteProvisioningArtifactOutput;

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeConstraintInput {
    #[doc="<p>The language code to use for this operation. Supported language codes are as follows:</p> <p>\"en\" (English)</p> <p>\"jp\" (Japanese)</p> <p>\"zh\" (Chinese)</p> <p>If no code is specified, \"en\" is used as the default.</p>"]
    #[serde(rename="AcceptLanguage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub accept_language: Option<AcceptLanguage>,
    #[doc="<p>The identifier of the constraint.</p>"]
    #[serde(rename="Id")]
    pub id: Id,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeConstraintOutput {
    #[doc="<p>Detailed constraint information.</p>"]
    #[serde(rename="ConstraintDetail")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub constraint_detail: Option<ConstraintDetail>,
    #[doc="<p>The current parameters associated with the specified constraint.</p>"]
    #[serde(rename="ConstraintParameters")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub constraint_parameters: Option<ConstraintParameters>,
    #[doc="<p>The status of the current request.</p>"]
    #[serde(rename="Status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<Status>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribePortfolioInput {
    #[doc="<p>The language code to use for this operation. Supported language codes are as follows:</p> <p>\"en\" (English)</p> <p>\"jp\" (Japanese)</p> <p>\"zh\" (Chinese)</p> <p>If no code is specified, \"en\" is used as the default.</p>"]
    #[serde(rename="AcceptLanguage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub accept_language: Option<AcceptLanguage>,
    #[doc="<p>The identifier of the portfolio for which to retrieve information.</p>"]
    #[serde(rename="Id")]
    pub id: Id,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribePortfolioOutput {
    #[doc="<p>Detailed portfolio information.</p>"]
    #[serde(rename="PortfolioDetail")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub portfolio_detail: Option<PortfolioDetail>,
    #[doc="<p>Tags associated with the portfolio.</p>"]
    #[serde(rename="Tags")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tags: Option<Tags>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeProductAsAdminInput {
    #[doc="<p>The language code to use for this operation. Supported language codes are as follows:</p> <p>\"en\" (English)</p> <p>\"jp\" (Japanese)</p> <p>\"zh\" (Chinese)</p> <p>If no code is specified, \"en\" is used as the default.</p>"]
    #[serde(rename="AcceptLanguage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub accept_language: Option<AcceptLanguage>,
    #[doc="<p>The identifier of the product for which to retrieve information.</p>"]
    #[serde(rename="Id")]
    pub id: Id,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeProductAsAdminOutput {
    #[doc="<p>Detailed product view information.</p>"]
    #[serde(rename="ProductViewDetail")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub product_view_detail: Option<ProductViewDetail>,
    #[doc="<p>A list of provisioning artifact summaries for the product.</p>"]
    #[serde(rename="ProvisioningArtifactSummaries")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub provisioning_artifact_summaries: Option<ProvisioningArtifactSummaries>,
    #[doc="<p>Tags associated with the product.</p>"]
    #[serde(rename="Tags")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tags: Option<Tags>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeProductInput {
    #[doc="<p>The language code to use for this operation. Supported language codes are as follows:</p> <p>\"en\" (English)</p> <p>\"jp\" (Japanese)</p> <p>\"zh\" (Chinese)</p> <p>If no code is specified, \"en\" is used as the default.</p>"]
    #[serde(rename="AcceptLanguage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub accept_language: Option<AcceptLanguage>,
    #[doc="<p>The <code>ProductId</code> of the product to describe.</p>"]
    #[serde(rename="Id")]
    pub id: Id,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeProductOutput {
    #[doc="<p>The summary metadata about the specified product.</p>"]
    #[serde(rename="ProductViewSummary")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub product_view_summary: Option<ProductViewSummary>,
    #[doc="<p>A list of provisioning artifact objects for the specified product. The <code>ProvisioningArtifacts</code> parameter represent the ways the specified product can be provisioned.</p>"]
    #[serde(rename="ProvisioningArtifacts")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub provisioning_artifacts: Option<ProvisioningArtifacts>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeProductViewInput {
    #[doc="<p>The language code to use for this operation. Supported language codes are as follows:</p> <p>\"en\" (English)</p> <p>\"jp\" (Japanese)</p> <p>\"zh\" (Chinese)</p> <p>If no code is specified, \"en\" is used as the default.</p>"]
    #[serde(rename="AcceptLanguage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub accept_language: Option<AcceptLanguage>,
    #[doc="<p>The <code>ProductViewId</code> of the product to describe.</p>"]
    #[serde(rename="Id")]
    pub id: Id,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeProductViewOutput {
    #[doc="<p>The summary metadata about the specified product.</p>"]
    #[serde(rename="ProductViewSummary")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub product_view_summary: Option<ProductViewSummary>,
    #[doc="<p>A list of provisioning artifact objects for the specified product. The <code>ProvisioningArtifacts</code> represent the ways in which the specified product can be provisioned.</p>"]
    #[serde(rename="ProvisioningArtifacts")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub provisioning_artifacts: Option<ProvisioningArtifacts>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeProvisionedProductInput {
    #[doc="<p>The language code to use for this operation. Supported language codes are as follows:</p> <p>\"en\" (English)</p> <p>\"jp\" (Japanese)</p> <p>\"zh\" (Chinese)</p> <p>If no code is specified, \"en\" is used as the default.</p>"]
    #[serde(rename="AcceptLanguage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub accept_language: Option<AcceptLanguage>,
    #[doc="<p>The provisioned product identifier.</p>"]
    #[serde(rename="Id")]
    pub id: Id,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeProvisionedProductOutput {
    #[doc="<p>Detailed provisioned product information.</p>"]
    #[serde(rename="ProvisionedProductDetail")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub provisioned_product_detail: Option<ProvisionedProductDetail>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeProvisioningArtifactInput {
    #[doc="<p>The language code to use for this operation. Supported language codes are as follows:</p> <p>\"en\" (English)</p> <p>\"jp\" (Japanese)</p> <p>\"zh\" (Chinese)</p> <p>If no code is specified, \"en\" is used as the default.</p>"]
    #[serde(rename="AcceptLanguage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub accept_language: Option<AcceptLanguage>,
    #[doc="<p>The product identifier.</p>"]
    #[serde(rename="ProductId")]
    pub product_id: Id,
    #[doc="<p>The identifier of the provisioning artifact. This is sometimes referred to as the product version.</p>"]
    #[serde(rename="ProvisioningArtifactId")]
    pub provisioning_artifact_id: Id,
    #[doc="<p>Selects verbose results. If set to true, the CloudFormation template is returned.</p>"]
    #[serde(rename="Verbose")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub verbose: Option<Verbose>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeProvisioningArtifactOutput {
    #[doc="<p>Additional information about the provisioning artifact.</p>"]
    #[serde(rename="Info")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub info: Option<ProvisioningArtifactInfo>,
    #[doc="<p>Detailed provisioning artifact information.</p>"]
    #[serde(rename="ProvisioningArtifactDetail")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub provisioning_artifact_detail: Option<ProvisioningArtifactDetail>,
    #[doc="<p>The status of the current request.</p>"]
    #[serde(rename="Status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<Status>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeProvisioningParametersInput {
    #[doc="<p>The language code to use for this operation. Supported language codes are as follows:</p> <p>\"en\" (English)</p> <p>\"jp\" (Japanese)</p> <p>\"zh\" (Chinese)</p> <p>If no code is specified, \"en\" is used as the default.</p>"]
    #[serde(rename="AcceptLanguage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub accept_language: Option<AcceptLanguage>,
    #[doc="<p>The identifier of the path for this product's provisioning. This value is optional if the product has a default path, and is required if there is more than one path for the specified product.</p>"]
    #[serde(rename="PathId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub path_id: Option<Id>,
    #[doc="<p>The product identifier.</p>"]
    #[serde(rename="ProductId")]
    pub product_id: Id,
    #[doc="<p>The provisioning artifact identifier for this product. This is sometimes referred to as the product version.</p>"]
    #[serde(rename="ProvisioningArtifactId")]
    pub provisioning_artifact_id: Id,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeProvisioningParametersOutput {
    #[doc="<p>The list of constraint summaries that apply to provisioning this product.</p>"]
    #[serde(rename="ConstraintSummaries")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub constraint_summaries: Option<ConstraintSummaries>,
    #[doc="<p>The list of parameters used to successfully provision the product. Each parameter includes a list of allowable values and additional metadata about each parameter.</p>"]
    #[serde(rename="ProvisioningArtifactParameters")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub provisioning_artifact_parameters: Option<ProvisioningArtifactParameters>,
    #[doc="<p>Any additional metadata specifically related to the provisioning of the product. For example, see the <code>Version</code> field of the CloudFormation template.</p>"]
    #[serde(rename="UsageInstructions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub usage_instructions: Option<UsageInstructions>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeRecordInput {
    #[doc="<p>The language code to use for this operation. Supported language codes are as follows:</p> <p>\"en\" (English)</p> <p>\"jp\" (Japanese)</p> <p>\"zh\" (Chinese)</p> <p>If no code is specified, \"en\" is used as the default.</p>"]
    #[serde(rename="AcceptLanguage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub accept_language: Option<AcceptLanguage>,
    #[doc="<p>The record identifier of the ProvisionedProduct object for which to retrieve output information. This is the <code>RecordDetail.RecordId</code> obtained from the request operation's response.</p>"]
    #[serde(rename="Id")]
    pub id: Id,
    #[doc="<p>The maximum number of items to return in the results. If more results exist than fit in the specified <code>PageSize</code>, the value of <code>NextPageToken</code> in the response is non-null.</p>"]
    #[serde(rename="PageSize")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub page_size: Option<PageSize>,
    #[doc="<p>The page token of the first page retrieved. If null, this retrieves the first page of size <code>PageSize</code>.</p>"]
    #[serde(rename="PageToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub page_token: Option<PageToken>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeRecordOutput {
    #[doc="<p>The page token to use to retrieve the next page of results for this operation. If there are no more pages, this value is null.</p>"]
    #[serde(rename="NextPageToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_page_token: Option<PageToken>,
    #[doc="<p>Detailed record information for the specified product. </p>"]
    #[serde(rename="RecordDetail")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub record_detail: Option<RecordDetail>,
    #[doc="<p>A list of outputs for the specified Product object created as the result of a request. For example, a CloudFormation-backed product that creates an S3 bucket would have an output for the S3 bucket URL.</p>"]
    #[serde(rename="RecordOutputs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub record_outputs: Option<RecordOutputs>,
}

pub type Description = String;
#[derive(Default,Debug,Clone,Serialize)]
pub struct DisassociatePrincipalFromPortfolioInput {
    #[doc="<p>The language code to use for this operation. Supported language codes are as follows:</p> <p>\"en\" (English)</p> <p>\"jp\" (Japanese)</p> <p>\"zh\" (Chinese)</p> <p>If no code is specified, \"en\" is used as the default.</p>"]
    #[serde(rename="AcceptLanguage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub accept_language: Option<AcceptLanguage>,
    #[doc="<p>The portfolio identifier.</p>"]
    #[serde(rename="PortfolioId")]
    pub portfolio_id: Id,
    #[doc="<p>The ARN representing the principal (IAM user, role, or group).</p>"]
    #[serde(rename="PrincipalARN")]
    pub principal_arn: PrincipalARN,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DisassociatePrincipalFromPortfolioOutput;

#[derive(Default,Debug,Clone,Serialize)]
pub struct DisassociateProductFromPortfolioInput {
    #[doc="<p>The language code to use for this operation. Supported language codes are as follows:</p> <p>\"en\" (English)</p> <p>\"jp\" (Japanese)</p> <p>\"zh\" (Chinese)</p> <p>If no code is specified, \"en\" is used as the default.</p>"]
    #[serde(rename="AcceptLanguage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub accept_language: Option<AcceptLanguage>,
    #[doc="<p>The portfolio identifier.</p>"]
    #[serde(rename="PortfolioId")]
    pub portfolio_id: Id,
    #[doc="<p>The product identifier.</p>"]
    #[serde(rename="ProductId")]
    pub product_id: Id,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DisassociateProductFromPortfolioOutput;

pub type ErrorCode = String;
pub type ErrorDescription = String;
pub type HasDefaultPath = bool;
pub type Id = String;
pub type IdempotencyToken = String;
pub type IgnoreErrors = bool;
pub type InstructionType = String;
pub type InstructionValue = String;
pub type LastRequestId = String;
pub type LaunchPathSummaries = Vec<LaunchPathSummary>;
#[doc="<p>Summary information about a path for a user to have access to a specified product.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct LaunchPathSummary {
    #[doc="<p>List of constraints on the portfolio-product relationship.</p>"]
    #[serde(rename="ConstraintSummaries")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub constraint_summaries: Option<ConstraintSummaries>,
    #[doc="<p>The unique identifier of the product path.</p>"]
    #[serde(rename="Id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<Id>,
    #[doc="<p>Corresponds to the name of the portfolio to which the user was assigned.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<PortfolioName>,
    #[doc="<p>List of tags used by this launch path.</p>"]
    #[serde(rename="Tags")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tags: Option<Tags>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListAcceptedPortfolioSharesInput {
    #[doc="<p>The language code to use for this operation. Supported language codes are as follows:</p> <p>\"en\" (English)</p> <p>\"jp\" (Japanese)</p> <p>\"zh\" (Chinese)</p> <p>If no code is specified, \"en\" is used as the default.</p>"]
    #[serde(rename="AcceptLanguage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub accept_language: Option<AcceptLanguage>,
    #[doc="<p>The maximum number of items to return in the results. If more results exist than fit in the specified <code>PageSize</code>, the value of <code>NextPageToken</code> in the response is non-null.</p>"]
    #[serde(rename="PageSize")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub page_size: Option<PageSize>,
    #[doc="<p>The page token of the first page retrieved. If null, this retrieves the first page of size <code>PageSize</code>.</p>"]
    #[serde(rename="PageToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub page_token: Option<PageToken>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListAcceptedPortfolioSharesOutput {
    #[doc="<p>The page token to use to retrieve the next page of results for this operation. If there are no more pages, this value is null.</p>"]
    #[serde(rename="NextPageToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_page_token: Option<PageToken>,
    #[doc="<p>List of detailed portfolio information objects.</p>"]
    #[serde(rename="PortfolioDetails")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub portfolio_details: Option<PortfolioDetails>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListConstraintsForPortfolioInput {
    #[doc="<p>The language code to use for this operation. Supported language codes are as follows:</p> <p>\"en\" (English)</p> <p>\"jp\" (Japanese)</p> <p>\"zh\" (Chinese)</p> <p>If no code is specified, \"en\" is used as the default.</p>"]
    #[serde(rename="AcceptLanguage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub accept_language: Option<AcceptLanguage>,
    #[doc="<p>The maximum number of items to return in the results. If more results exist than fit in the specified <code>PageSize</code>, the value of <code>NextPageToken</code> in the response is non-null.</p>"]
    #[serde(rename="PageSize")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub page_size: Option<PageSize>,
    #[doc="<p>The page token of the first page retrieved. If null, this retrieves the first page of size <code>PageSize</code>.</p>"]
    #[serde(rename="PageToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub page_token: Option<PageToken>,
    #[doc="<p>The portfolio identifier.</p>"]
    #[serde(rename="PortfolioId")]
    pub portfolio_id: Id,
    #[doc="<p>The product identifier.</p>"]
    #[serde(rename="ProductId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub product_id: Option<Id>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListConstraintsForPortfolioOutput {
    #[doc="<p>List of detailed constraint information objects.</p>"]
    #[serde(rename="ConstraintDetails")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub constraint_details: Option<ConstraintDetails>,
    #[doc="<p>The page token to use to retrieve the next page of results for this operation. If there are no more pages, this value is null.</p>"]
    #[serde(rename="NextPageToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_page_token: Option<PageToken>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListLaunchPathsInput {
    #[doc="<p>The language code to use for this operation. Supported language codes are as follows:</p> <p>\"en\" (English)</p> <p>\"jp\" (Japanese)</p> <p>\"zh\" (Chinese)</p> <p>If no code is specified, \"en\" is used as the default.</p>"]
    #[serde(rename="AcceptLanguage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub accept_language: Option<AcceptLanguage>,
    #[doc="<p>The maximum number of items to return in the results. If more results exist than fit in the specified <code>PageSize</code>, the value of <code>NextPageToken</code> in the response is non-null.</p>"]
    #[serde(rename="PageSize")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub page_size: Option<PageSize>,
    #[doc="<p>The page token of the first page retrieved. If null, this retrieves the first page of size <code>PageSize</code>.</p>"]
    #[serde(rename="PageToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub page_token: Option<PageToken>,
    #[doc="<p>The product identifier.. Identifies the product for which to retrieve <code>LaunchPathSummaries</code> information.</p>"]
    #[serde(rename="ProductId")]
    pub product_id: Id,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListLaunchPathsOutput {
    #[doc="<p>List of launch path information summaries for the specified <code>PageToken</code>.</p>"]
    #[serde(rename="LaunchPathSummaries")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub launch_path_summaries: Option<LaunchPathSummaries>,
    #[doc="<p>The page token to use to retrieve the next page of results for this operation. If there are no more pages, this value is null.</p>"]
    #[serde(rename="NextPageToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_page_token: Option<PageToken>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListPortfolioAccessInput {
    #[doc="<p>The language code to use for this operation. Supported language codes are as follows:</p> <p>\"en\" (English)</p> <p>\"jp\" (Japanese)</p> <p>\"zh\" (Chinese)</p> <p>If no code is specified, \"en\" is used as the default.</p>"]
    #[serde(rename="AcceptLanguage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub accept_language: Option<AcceptLanguage>,
    #[doc="<p>The portfolio identifier.</p>"]
    #[serde(rename="PortfolioId")]
    pub portfolio_id: Id,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListPortfolioAccessOutput {
    #[doc="<p>List of account IDs associated with access to the portfolio.</p>"]
    #[serde(rename="AccountIds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub account_ids: Option<AccountIds>,
    #[doc="<p>The page token to use to retrieve the next page of results for this operation. If there are no more pages, this value is null.</p>"]
    #[serde(rename="NextPageToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_page_token: Option<PageToken>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListPortfoliosForProductInput {
    #[doc="<p>The language code to use for this operation. Supported language codes are as follows:</p> <p>\"en\" (English)</p> <p>\"jp\" (Japanese)</p> <p>\"zh\" (Chinese)</p> <p>If no code is specified, \"en\" is used as the default.</p>"]
    #[serde(rename="AcceptLanguage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub accept_language: Option<AcceptLanguage>,
    #[doc="<p>The maximum number of items to return in the results. If more results exist than fit in the specified <code>PageSize</code>, the value of <code>NextPageToken</code> in the response is non-null.</p>"]
    #[serde(rename="PageSize")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub page_size: Option<PageSize>,
    #[doc="<p>The page token of the first page retrieved. If null, this retrieves the first page of size <code>PageSize</code>.</p>"]
    #[serde(rename="PageToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub page_token: Option<PageToken>,
    #[doc="<p>The product identifier.</p>"]
    #[serde(rename="ProductId")]
    pub product_id: Id,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListPortfoliosForProductOutput {
    #[doc="<p>The page token to use to retrieve the next page of results for this operation. If there are no more pages, this value is null.</p>"]
    #[serde(rename="NextPageToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_page_token: Option<PageToken>,
    #[doc="<p>List of detailed portfolio information objects.</p>"]
    #[serde(rename="PortfolioDetails")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub portfolio_details: Option<PortfolioDetails>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListPortfoliosInput {
    #[doc="<p>The language code to use for this operation. Supported language codes are as follows:</p> <p>\"en\" (English)</p> <p>\"jp\" (Japanese)</p> <p>\"zh\" (Chinese)</p> <p>If no code is specified, \"en\" is used as the default.</p>"]
    #[serde(rename="AcceptLanguage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub accept_language: Option<AcceptLanguage>,
    #[doc="<p>The maximum number of items to return in the results. If more results exist than fit in the specified <code>PageSize</code>, the value of <code>NextPageToken</code> in the response is non-null.</p>"]
    #[serde(rename="PageSize")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub page_size: Option<PageSize>,
    #[doc="<p>The page token of the first page retrieved. If null, this retrieves the first page of size <code>PageSize</code>.</p>"]
    #[serde(rename="PageToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub page_token: Option<PageToken>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListPortfoliosOutput {
    #[doc="<p>The page token to use to retrieve the next page of results for this operation. If there are no more pages, this value is null.</p>"]
    #[serde(rename="NextPageToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_page_token: Option<PageToken>,
    #[doc="<p>List of detailed portfolio information objects.</p>"]
    #[serde(rename="PortfolioDetails")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub portfolio_details: Option<PortfolioDetails>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListPrincipalsForPortfolioInput {
    #[doc="<p>The language code to use for this operation. Supported language codes are as follows:</p> <p>\"en\" (English)</p> <p>\"jp\" (Japanese)</p> <p>\"zh\" (Chinese)</p> <p>If no code is specified, \"en\" is used as the default.</p>"]
    #[serde(rename="AcceptLanguage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub accept_language: Option<AcceptLanguage>,
    #[doc="<p>The maximum number of items to return in the results. If more results exist than fit in the specified <code>PageSize</code>, the value of <code>NextPageToken</code> in the response is non-null.</p>"]
    #[serde(rename="PageSize")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub page_size: Option<PageSize>,
    #[doc="<p>The page token of the first page retrieved. If null, this retrieves the first page of size <code>PageSize</code>.</p>"]
    #[serde(rename="PageToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub page_token: Option<PageToken>,
    #[doc="<p>The portfolio identifier.</p>"]
    #[serde(rename="PortfolioId")]
    pub portfolio_id: Id,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListPrincipalsForPortfolioOutput {
    #[doc="<p>The page token to use to retrieve the next page of results for this operation. If there are no more pages, this value is null.</p>"]
    #[serde(rename="NextPageToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_page_token: Option<PageToken>,
    #[doc="<p>The IAM principals (users or roles) associated with the portfolio.</p>"]
    #[serde(rename="Principals")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub principals: Option<Principals>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListProvisioningArtifactsInput {
    #[doc="<p>The language code to use for this operation. Supported language codes are as follows:</p> <p>\"en\" (English)</p> <p>\"jp\" (Japanese)</p> <p>\"zh\" (Chinese)</p> <p>If no code is specified, \"en\" is used as the default.</p>"]
    #[serde(rename="AcceptLanguage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub accept_language: Option<AcceptLanguage>,
    #[doc="<p>The product identifier.</p>"]
    #[serde(rename="ProductId")]
    pub product_id: Id,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListProvisioningArtifactsOutput {
    #[doc="<p>The page token to use to retrieve the next page of results for this operation. If there are no more pages, this value is null.</p>"]
    #[serde(rename="NextPageToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_page_token: Option<PageToken>,
    #[doc="<p>List of detailed provisioning artifact information objects.</p>"]
    #[serde(rename="ProvisioningArtifactDetails")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub provisioning_artifact_details: Option<ProvisioningArtifactDetails>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListRecordHistoryInput {
    #[doc="<p>The language code to use for this operation. Supported language codes are as follows:</p> <p>\"en\" (English)</p> <p>\"jp\" (Japanese)</p> <p>\"zh\" (Chinese)</p> <p>If no code is specified, \"en\" is used as the default.</p>"]
    #[serde(rename="AcceptLanguage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub accept_language: Option<AcceptLanguage>,
    #[doc="<p>The access level for obtaining results. If left unspecified, <code>User</code> level access is used.</p>"]
    #[serde(rename="AccessLevelFilter")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub access_level_filter: Option<AccessLevelFilter>,
    #[doc="<p>The maximum number of items to return in the results. If more results exist than fit in the specified <code>PageSize</code>, the value of <code>NextPageToken</code> in the response is non-null.</p>"]
    #[serde(rename="PageSize")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub page_size: Option<PageSize>,
    #[doc="<p>The page token of the first page retrieved. If null, this retrieves the first page of size <code>PageSize</code>.</p>"]
    #[serde(rename="PageToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub page_token: Option<PageToken>,
    #[doc="<p>The filter to limit search results. </p>"]
    #[serde(rename="SearchFilter")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub search_filter: Option<ListRecordHistorySearchFilter>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListRecordHistoryOutput {
    #[doc="<p>The page token to use to retrieve the next page of results for this operation. If there are no more pages, this value is null.</p>"]
    #[serde(rename="NextPageToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_page_token: Option<PageToken>,
    #[doc="<p>A list of record detail objects, listed in reverse chronological order.</p>"]
    #[serde(rename="RecordDetails")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub record_details: Option<RecordDetails>,
}

#[doc="<p>The search filter to limit results when listing request history records.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ListRecordHistorySearchFilter {
    #[doc="<p>The filter key.</p>"]
    #[serde(rename="Key")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub key: Option<SearchFilterKey>,
    #[doc="<p>The filter value for <code>Key</code>.</p>"]
    #[serde(rename="Value")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub value: Option<SearchFilterValue>,
}

pub type NoEcho = bool;
pub type NotificationArn = String;
pub type NotificationArns = Vec<NotificationArn>;
pub type OutputKey = String;
pub type OutputValue = String;
pub type PageSize = i64;
pub type PageToken = String;
#[doc="<p>The constraints that the administrator has put on the parameter.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ParameterConstraints {
    #[doc="<p>The values that the administrator has allowed for the parameter.</p>"]
    #[serde(rename="AllowedValues")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allowed_values: Option<AllowedValues>,
}

pub type ParameterKey = String;
pub type ParameterType = String;
pub type ParameterValue = String;
pub type PortfolioDescription = String;
#[doc="<p>Detailed portfolio information.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct PortfolioDetail {
    #[doc="<p>The ARN assigned to the portfolio.</p>"]
    #[serde(rename="ARN")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub arn: Option<ResourceARN>,
    #[doc="<p>The UTC timestamp of the creation time.</p>"]
    #[serde(rename="CreatedTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_time: Option<CreationTime>,
    #[doc="<p>The text description of the portfolio.</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<PortfolioDescription>,
    #[doc="<p>The name to use for display purposes.</p>"]
    #[serde(rename="DisplayName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub display_name: Option<PortfolioDisplayName>,
    #[doc="<p>The identifier for the portfolio.</p>"]
    #[serde(rename="Id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<Id>,
    #[doc="<p>The name of the portfolio provider.</p>"]
    #[serde(rename="ProviderName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub provider_name: Option<ProviderName>,
}

pub type PortfolioDetails = Vec<PortfolioDetail>;
pub type PortfolioDisplayName = String;
pub type PortfolioName = String;
#[doc="<p>A principal's ARN and type.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Principal {
    #[doc="<p>The ARN representing the principal (IAM user, role, or group).</p>"]
    #[serde(rename="PrincipalARN")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub principal_arn: Option<PrincipalARN>,
    #[doc="<p>The principal type. Must be <code>IAM</code> </p>"]
    #[serde(rename="PrincipalType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub principal_type: Option<PrincipalType>,
}

pub type PrincipalARN = String;
pub type PrincipalType = String;
pub type Principals = Vec<Principal>;
pub type ProductSource = String;
pub type ProductType = String;
pub type ProductViewAggregationType = String;
#[doc="<p>A single product view aggregation value/count pair, containing metadata about each product to which the calling user has access.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ProductViewAggregationValue {
    #[doc="<p>An approximate count of the products that match the value.</p>"]
    #[serde(rename="ApproximateCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub approximate_count: Option<ApproximateCount>,
    #[doc="<p>The value of the product view aggregation.</p>"]
    #[serde(rename="Value")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub value: Option<AttributeValue>,
}

pub type ProductViewAggregationValues = Vec<ProductViewAggregationValue>;
pub type ProductViewAggregations = ::std::collections::HashMap<ProductViewAggregationType,
                                                               ProductViewAggregationValues>;
#[doc="<p>Detailed product view information.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ProductViewDetail {
    #[doc="<p>The UTC timestamp of the creation time.</p>"]
    #[serde(rename="CreatedTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_time: Option<CreatedTime>,
    #[doc="<p>The ARN associated with the product.</p>"]
    #[serde(rename="ProductARN")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub product_arn: Option<ResourceARN>,
    #[doc="<p>The summary metadata about the specified product view.</p>"]
    #[serde(rename="ProductViewSummary")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub product_view_summary: Option<ProductViewSummary>,
    #[doc="<p>Current status of the product.</p> <p> <code>AVAILABLE</code> - Product is available for use.</p> <p> <code>CREATING</code> - Creation of product started, not ready for use.</p> <p> <code>FAILED</code> - Action on product failed.</p>"]
    #[serde(rename="Status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<Status>,
}

pub type ProductViewDetails = Vec<ProductViewDetail>;
pub type ProductViewDistributor = String;
pub type ProductViewFilterBy = String;
pub type ProductViewFilterValue = String;
pub type ProductViewFilterValues = Vec<ProductViewFilterValue>;
pub type ProductViewFilters = ::std::collections::HashMap<ProductViewFilterBy,
                                                          ProductViewFilterValues>;
pub type ProductViewName = String;
pub type ProductViewOwner = String;
pub type ProductViewShortDescription = String;
pub type ProductViewSortBy = String;
pub type ProductViewSummaries = Vec<ProductViewSummary>;
#[doc="<p>The summary metadata about the specified product.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ProductViewSummary {
    #[doc="<p>The distributor of the product. Contact the product administrator for the significance of this value.</p>"]
    #[serde(rename="Distributor")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub distributor: Option<ProductViewDistributor>,
    #[doc="<p>A value of <code>false</code> indicates that the product does not have a default path, while a value of <code>true</code> indicates that it does. If it's false, call <a>ListLaunchPaths</a> to disambiguate between paths. If true, <a>ListLaunchPaths</a> is not required, and the output of the <a>ProductViewSummary</a> operation can be used directly with <a>DescribeProvisioningParameters</a>.</p>"]
    #[serde(rename="HasDefaultPath")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_default_path: Option<HasDefaultPath>,
    #[doc="<p>The product view identifier.</p>"]
    #[serde(rename="Id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<Id>,
    #[doc="<p>The name of the product.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<ProductViewName>,
    #[doc="<p>The owner of the product. Contact the product administrator for the significance of this value.</p>"]
    #[serde(rename="Owner")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub owner: Option<ProductViewOwner>,
    #[doc="<p>The product identifier.</p>"]
    #[serde(rename="ProductId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub product_id: Option<Id>,
    #[doc="<p>Short description of the product.</p>"]
    #[serde(rename="ShortDescription")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub short_description: Option<ProductViewShortDescription>,
    #[doc="<p>The description of the support for this Product.</p>"]
    #[serde(rename="SupportDescription")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub support_description: Option<SupportDescription>,
    #[doc="<p>The email contact information to obtain support for this Product.</p>"]
    #[serde(rename="SupportEmail")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub support_email: Option<SupportEmail>,
    #[doc="<p>The URL information to obtain support for this Product.</p>"]
    #[serde(rename="SupportUrl")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub support_url: Option<SupportUrl>,
    #[doc="<p>The product type. Contact the product administrator for the significance of this value. If this value is <code>MARKETPLACE</code>, the product was created by AWS Marketplace.</p>"]
    #[serde(rename="Type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub type_: Option<ProductType>,
}

pub type ProviderName = String;
#[derive(Default,Debug,Clone,Serialize)]
pub struct ProvisionProductInput {
    #[doc="<p>The language code to use for this operation. Supported language codes are as follows:</p> <p>\"en\" (English)</p> <p>\"jp\" (Japanese)</p> <p>\"zh\" (Chinese)</p> <p>If no code is specified, \"en\" is used as the default.</p>"]
    #[serde(rename="AcceptLanguage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub accept_language: Option<AcceptLanguage>,
    #[doc="<p>Passed to CloudFormation. The SNS topic ARNs to which to publish stack-related events.</p>"]
    #[serde(rename="NotificationArns")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub notification_arns: Option<NotificationArns>,
    #[doc="<p>The identifier of the path for this product's provisioning. This value is optional if the product has a default path, and is required if there is more than one path for the specified product.</p>"]
    #[serde(rename="PathId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub path_id: Option<Id>,
    #[doc="<p>The product identifier.</p>"]
    #[serde(rename="ProductId")]
    pub product_id: Id,
    #[doc="<p>An idempotency token that uniquely identifies the provisioning request. </p>"]
    #[serde(rename="ProvisionToken")]
    pub provision_token: IdempotencyToken,
    #[doc="<p>A user-friendly name to identify the ProvisionedProduct object. This value must be unique for the AWS account and cannot be updated after the product is provisioned.</p>"]
    #[serde(rename="ProvisionedProductName")]
    pub provisioned_product_name: ProvisionedProductName,
    #[doc="<p>The provisioning artifact identifier for this product. This is sometimes referred to as the product version.</p>"]
    #[serde(rename="ProvisioningArtifactId")]
    pub provisioning_artifact_id: Id,
    #[doc="<p>Parameters specified by the administrator that are required for provisioning the product.</p>"]
    #[serde(rename="ProvisioningParameters")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub provisioning_parameters: Option<ProvisioningParameters>,
    #[doc="<p>A list of tags to use as provisioning options.</p>"]
    #[serde(rename="Tags")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tags: Option<Tags>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ProvisionProductOutput {
    #[doc="<p>The detailed result of the <a>ProvisionProduct</a> request, containing the inputs made to that request, the current state of the request, a pointer to the ProvisionedProduct object of the request, and a list of any errors that the request encountered. </p>"]
    #[serde(rename="RecordDetail")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub record_detail: Option<RecordDetail>,
}

#[doc="<p>Detailed information about a ProvisionedProduct object.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ProvisionedProductDetail {
    #[doc="<p>The ARN associated with the ProvisionedProduct object.</p>"]
    #[serde(rename="Arn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub arn: Option<ProvisionedProductNameOrArn>,
    #[doc="<p>The UTC timestamp of the creation time.</p>"]
    #[serde(rename="CreatedTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_time: Option<CreatedTime>,
    #[doc="<p>The identifier of the ProvisionedProduct object.</p>"]
    #[serde(rename="Id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<ProvisionedProductId>,
    #[doc="<p>A token to disambiguate duplicate requests. You can create multiple resources using the same input in multiple requests, provided that you also specify a different idempotency token for each request.</p>"]
    #[serde(rename="IdempotencyToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub idempotency_token: Option<IdempotencyToken>,
    #[doc="<p>The record identifier of the last request performed on this ProvisionedProduct object.</p>"]
    #[serde(rename="LastRecordId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_record_id: Option<LastRequestId>,
    #[doc="<p>The user-friendly name of the ProvisionedProduct object.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<ProvisionedProductNameOrArn>,
    #[doc="<p>The current status of the ProvisionedProduct.</p> <p> <code>AVAILABLE</code> - Stable state, ready to perform any operation. The most recent action request succeeded and completed.</p> <p> <code>UNDER_CHANGE</code> - Transitive state, operations performed may or may not have valid results. Wait for an <code>AVAILABLE</code> status before performing operations.</p> <p> <code>TAINTED</code> - Stable state, ready to perform any operation. The stack has completed the requested operation but is not exactly what was requested. For example, a request to update to a new version failed and the stack rolled back to the current version. </p> <p> <code>ERROR</code> - Something unexpected happened such that the provisioned product exists but the stack is not running. For example, CloudFormation received an invalid parameter value and could not launch the stack.</p>"]
    #[serde(rename="Status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<ProvisionedProductStatus>,
    #[doc="<p>The current status message of the ProvisionedProduct.</p>"]
    #[serde(rename="StatusMessage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status_message: Option<ProvisionedProductStatusMessage>,
    #[doc="<p>The type of the ProvisionedProduct object.</p>"]
    #[serde(rename="Type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub type_: Option<ProvisionedProductType>,
}

pub type ProvisionedProductDetails = Vec<ProvisionedProductDetail>;
pub type ProvisionedProductId = String;
pub type ProvisionedProductName = String;
pub type ProvisionedProductNameOrArn = String;
pub type ProvisionedProductStatus = String;
pub type ProvisionedProductStatusMessage = String;
pub type ProvisionedProductType = String;
#[doc="<p>Contains information indicating the ways in which a product can be provisioned.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ProvisioningArtifact {
    #[doc="<p>The UTC timestamp of the creation time.</p>"]
    #[serde(rename="CreatedTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_time: Option<ProvisioningArtifactCreatedTime>,
    #[doc="<p>The text description of the artifact.</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<ProvisioningArtifactDescription>,
    #[doc="<p>The identifier for the artifact. This is sometimes referred to as the product version.</p>"]
    #[serde(rename="Id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<Id>,
    #[doc="<p>The name of the artifact.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<ProvisioningArtifactName>,
}

pub type ProvisioningArtifactCreatedTime = f64;
pub type ProvisioningArtifactDescription = String;
#[doc="<p>Detailed provisioning artifact information.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ProvisioningArtifactDetail {
    #[doc="<p>The UTC timestamp of the creation time.</p>"]
    #[serde(rename="CreatedTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_time: Option<CreationTime>,
    #[doc="<p>The text description of the provisioning artifact.</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<ProvisioningArtifactName>,
    #[doc="<p>The identifier of the provisioning artifact. This is sometimes referred to as the product version.</p>"]
    #[serde(rename="Id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<Id>,
    #[doc="<p>The name assigned to the provisioning artifact.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<ProvisioningArtifactName>,
    #[doc="<p>The type of the provisioning artifact. The following provisioning artifact types are used by AWS Marketplace products:</p> <p> <code>MARKETPLACE_AMI</code> - AMI products.</p> <p> <code>MARKETPLACE_CAR</code> - CAR (Cluster and AWS Resources) products.</p>"]
    #[serde(rename="Type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub type_: Option<ProvisioningArtifactType>,
}

pub type ProvisioningArtifactDetails = Vec<ProvisioningArtifactDetail>;
pub type ProvisioningArtifactInfo = ::std::collections::HashMap<ProvisioningArtifactInfoKey,
                                                                ProvisioningArtifactInfoValue>;
pub type ProvisioningArtifactInfoKey = String;
pub type ProvisioningArtifactInfoValue = String;
pub type ProvisioningArtifactName = String;
#[doc="<p>A parameter used to successfully provision the product. This value includes a list of allowable values and additional metadata. </p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ProvisioningArtifactParameter {
    #[doc="<p>The default value for this parameter.</p>"]
    #[serde(rename="DefaultValue")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_value: Option<DefaultValue>,
    #[doc="<p>The text description of the parameter.</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<Description>,
    #[doc="<p>If this value is true, the value for this parameter is obfuscated from view when the parameter is retrieved. This parameter is used to hide sensitive information.</p>"]
    #[serde(rename="IsNoEcho")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub is_no_echo: Option<NoEcho>,
    #[doc="<p>The list of constraints that the administrator has put on the parameter.</p>"]
    #[serde(rename="ParameterConstraints")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub parameter_constraints: Option<ParameterConstraints>,
    #[doc="<p>The parameter key. </p>"]
    #[serde(rename="ParameterKey")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub parameter_key: Option<ParameterKey>,
    #[doc="<p>The parameter type.</p>"]
    #[serde(rename="ParameterType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub parameter_type: Option<ParameterType>,
}

pub type ProvisioningArtifactParameters = Vec<ProvisioningArtifactParameter>;
#[doc="<p>Provisioning artifact properties. For example request JSON, see <a>CreateProvisioningArtifact</a>.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ProvisioningArtifactProperties {
    #[doc="<p>The text description of the provisioning artifact properties.</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<ProvisioningArtifactDescription>,
    #[doc="<p>Additional information about the provisioning artifact properties. When using this element in a request, you must specify <code>LoadTemplateFromURL</code>. For more information, see <a>CreateProvisioningArtifact</a>.</p>"]
    #[serde(rename="Info")]
    pub info: ProvisioningArtifactInfo,
    #[doc="<p>The name assigned to the provisioning artifact properties.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<ProvisioningArtifactName>,
    #[doc="<p>The type of the provisioning artifact properties. The following provisioning artifact property types are used by AWS Marketplace products:</p> <p> <code>MARKETPLACE_AMI</code> - AMI products.</p> <p> <code>MARKETPLACE_CAR</code> - CAR (Cluster and AWS Resources) products.</p>"]
    #[serde(rename="Type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub type_: Option<ProvisioningArtifactType>,
}

pub type ProvisioningArtifactSummaries = Vec<ProvisioningArtifactSummary>;
#[doc="<p>Summary information about a provisioning artifact.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ProvisioningArtifactSummary {
    #[doc="<p>The UTC timestamp of the creation time.</p>"]
    #[serde(rename="CreatedTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_time: Option<ProvisioningArtifactCreatedTime>,
    #[doc="<p>The provisioning artifact description.</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<ProvisioningArtifactDescription>,
    #[doc="<p>The provisioning artifact identifier.</p>"]
    #[serde(rename="Id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<Id>,
    #[doc="<p>The provisioning artifact name.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<ProvisioningArtifactName>,
    #[doc="<p>The provisioning artifact metadata. This data is used with products created by AWS Marketplace.</p>"]
    #[serde(rename="ProvisioningArtifactMetadata")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub provisioning_artifact_metadata: Option<ProvisioningArtifactInfo>,
}

pub type ProvisioningArtifactType = String;
pub type ProvisioningArtifacts = Vec<ProvisioningArtifact>;
#[doc="<p>The parameter key-value pairs used to provision a product.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ProvisioningParameter {
    #[doc="<p>The <code>ProvisioningArtifactParameter.ParameterKey</code> parameter from <a>DescribeProvisioningParameters</a>.</p>"]
    #[serde(rename="Key")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub key: Option<ParameterKey>,
    #[doc="<p>The value to use for provisioning. Any constraints on this value can be found in <code>ProvisioningArtifactParameter</code> for <code>Key</code>.</p>"]
    #[serde(rename="Value")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub value: Option<ParameterValue>,
}

pub type ProvisioningParameters = Vec<ProvisioningParameter>;
#[doc="<p>The full details of a specific ProvisionedProduct object.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct RecordDetail {
    #[doc="<p>The UTC timestamp of the creation time.</p>"]
    #[serde(rename="CreatedTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_time: Option<CreatedTime>,
    #[doc="<p>The identifier of the path for this product's provisioning.</p>"]
    #[serde(rename="PathId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub path_id: Option<Id>,
    #[doc="<p>The product identifier.</p>"]
    #[serde(rename="ProductId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub product_id: Option<Id>,
    #[doc="<p>The identifier of the ProvisionedProduct object.</p>"]
    #[serde(rename="ProvisionedProductId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub provisioned_product_id: Option<Id>,
    #[doc="<p>The user-friendly name of the ProvisionedProduct object.</p>"]
    #[serde(rename="ProvisionedProductName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub provisioned_product_name: Option<ProvisionedProductName>,
    #[doc="<p>The type of the ProvisionedProduct object.</p>"]
    #[serde(rename="ProvisionedProductType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub provisioned_product_type: Option<ProvisionedProductType>,
    #[doc="<p>The provisioning artifact identifier for this product. This is sometimes referred to as the product version.</p>"]
    #[serde(rename="ProvisioningArtifactId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub provisioning_artifact_id: Option<Id>,
    #[doc="<p>A list of errors that occurred while processing the request.</p>"]
    #[serde(rename="RecordErrors")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub record_errors: Option<RecordErrors>,
    #[doc="<p>The identifier of the ProvisionedProduct object record.</p>"]
    #[serde(rename="RecordId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub record_id: Option<Id>,
    #[doc="<p>List of tags associated with this record.</p>"]
    #[serde(rename="RecordTags")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub record_tags: Option<RecordTags>,
    #[doc="<p>The record type for this record.</p>"]
    #[serde(rename="RecordType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub record_type: Option<RecordType>,
    #[doc="<p>The status of the ProvisionedProduct object.</p> <p> <code>CREATED</code> - Request created but the operation has not yet started.</p> <p> <code>IN_PROGRESS</code> - The requested operation is in-progress.</p> <p> <code>IN_PROGRESS_IN_ERROR</code> - The provisioned product is under change but the requested operation failed and some remediation is occurring. For example, a roll-back.</p> <p> <code>SUCCEEDED</code> - The requested operation has successfully completed.</p> <p> <code>FAILED</code> - The requested operation has completed but has failed. Investigate using the error messages returned.</p>"]
    #[serde(rename="Status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<RecordStatus>,
    #[doc="<p>The time when the record for the ProvisionedProduct object was last updated.</p>"]
    #[serde(rename="UpdatedTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_time: Option<UpdatedTime>,
}

pub type RecordDetails = Vec<RecordDetail>;
#[doc="<p>The error code and description resulting from an operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct RecordError {
    #[doc="<p>The numeric value of the error.</p>"]
    #[serde(rename="Code")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub code: Option<ErrorCode>,
    #[doc="<p>The text description of the error.</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<ErrorDescription>,
}

pub type RecordErrors = Vec<RecordError>;
#[doc="<p>An output for the specified Product object created as the result of a request. For example, a CloudFormation-backed product that creates an S3 bucket would have an output for the S3 bucket URL.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct RecordOutput {
    #[doc="<p>The text description of the output.</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<Description>,
    #[doc="<p>The output key.</p>"]
    #[serde(rename="OutputKey")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub output_key: Option<OutputKey>,
    #[doc="<p>The output value.</p>"]
    #[serde(rename="OutputValue")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub output_value: Option<OutputValue>,
}

pub type RecordOutputs = Vec<RecordOutput>;
pub type RecordStatus = String;
#[doc="<p>A tag associated with the record, stored as a key-value pair.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct RecordTag {
    #[doc="<p>The key for this tag.</p>"]
    #[serde(rename="Key")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub key: Option<RecordTagKey>,
    #[doc="<p>The value for this tag.</p>"]
    #[serde(rename="Value")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub value: Option<RecordTagValue>,
}

pub type RecordTagKey = String;
pub type RecordTagValue = String;
pub type RecordTags = Vec<RecordTag>;
pub type RecordType = String;
#[derive(Default,Debug,Clone,Serialize)]
pub struct RejectPortfolioShareInput {
    #[doc="<p>The language code to use for this operation. Supported language codes are as follows:</p> <p>\"en\" (English)</p> <p>\"jp\" (Japanese)</p> <p>\"zh\" (Chinese)</p> <p>If no code is specified, \"en\" is used as the default.</p>"]
    #[serde(rename="AcceptLanguage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub accept_language: Option<AcceptLanguage>,
    #[doc="<p>The portfolio identifier.</p>"]
    #[serde(rename="PortfolioId")]
    pub portfolio_id: Id,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct RejectPortfolioShareOutput;

pub type ResourceARN = String;
#[derive(Default,Debug,Clone,Serialize)]
pub struct ScanProvisionedProductsInput {
    #[doc="<p>The language code to use for this operation. Supported language codes are as follows:</p> <p>\"en\" (English)</p> <p>\"jp\" (Japanese)</p> <p>\"zh\" (Chinese)</p> <p>If no code is specified, \"en\" is used as the default.</p>"]
    #[serde(rename="AcceptLanguage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub accept_language: Option<AcceptLanguage>,
    #[doc="<p>The access level for obtaining results. If left unspecified, <code>User</code> level access is used.</p>"]
    #[serde(rename="AccessLevelFilter")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub access_level_filter: Option<AccessLevelFilter>,
    #[doc="<p>The maximum number of items to return in the results. If more results exist than fit in the specified <code>PageSize</code>, the value of <code>NextPageToken</code> in the response is non-null.</p>"]
    #[serde(rename="PageSize")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub page_size: Option<PageSize>,
    #[doc="<p>The page token of the first page retrieved. If null, this retrieves the first page of size <code>PageSize</code>.</p>"]
    #[serde(rename="PageToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub page_token: Option<PageToken>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ScanProvisionedProductsOutput {
    #[doc="<p>The page token to use to retrieve the next page of results for this operation. If there are no more pages, this value is null.</p>"]
    #[serde(rename="NextPageToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_page_token: Option<PageToken>,
    #[doc="<p>A list of ProvisionedProduct detail objects.</p>"]
    #[serde(rename="ProvisionedProducts")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub provisioned_products: Option<ProvisionedProductDetails>,
}

pub type SearchFilterKey = String;
pub type SearchFilterValue = String;
#[derive(Default,Debug,Clone,Serialize)]
pub struct SearchProductsAsAdminInput {
    #[doc="<p>The language code to use for this operation. Supported language codes are as follows:</p> <p>\"en\" (English)</p> <p>\"jp\" (Japanese)</p> <p>\"zh\" (Chinese)</p> <p>If no code is specified, \"en\" is used as the default.</p>"]
    #[serde(rename="AcceptLanguage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub accept_language: Option<AcceptLanguage>,
    #[doc="<p>The list of filters with which to limit search results. If no search filters are specified, the output is all the products to which the administrator has access.</p>"]
    #[serde(rename="Filters")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub filters: Option<ProductViewFilters>,
    #[doc="<p>The maximum number of items to return in the results. If more results exist than fit in the specified <code>PageSize</code>, the value of <code>NextPageToken</code> in the response is non-null.</p>"]
    #[serde(rename="PageSize")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub page_size: Option<PageSize>,
    #[doc="<p>The page token of the first page retrieved. If null, this retrieves the first page of size <code>PageSize</code>.</p>"]
    #[serde(rename="PageToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub page_token: Option<PageToken>,
    #[doc="<p>The portfolio identifier.</p>"]
    #[serde(rename="PortfolioId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub portfolio_id: Option<Id>,
    #[doc="<p>Access level of the source of the product.</p>"]
    #[serde(rename="ProductSource")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub product_source: Option<ProductSource>,
    #[doc="<p>The sort field specifier. If no value is specified, results are not sorted.</p>"]
    #[serde(rename="SortBy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sort_by: Option<ProductViewSortBy>,
    #[doc="<p>The sort order specifier. If no value is specified, results are not sorted.</p>"]
    #[serde(rename="SortOrder")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sort_order: Option<SortOrder>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct SearchProductsAsAdminOutput {
    #[doc="<p>The page token to use to retrieve the next page of results for this operation. If there are no more pages, this value is null.</p>"]
    #[serde(rename="NextPageToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_page_token: Option<PageToken>,
    #[doc="<p>List of detailed product view information objects.</p>"]
    #[serde(rename="ProductViewDetails")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub product_view_details: Option<ProductViewDetails>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct SearchProductsInput {
    #[doc="<p>The language code to use for this operation. Supported language codes are as follows:</p> <p>\"en\" (English)</p> <p>\"jp\" (Japanese)</p> <p>\"zh\" (Chinese)</p> <p>If no code is specified, \"en\" is used as the default.</p>"]
    #[serde(rename="AcceptLanguage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub accept_language: Option<AcceptLanguage>,
    #[doc="<p>The list of filters with which to limit search results. If no search filters are specified, the output is all the products to which the calling user has access. </p>"]
    #[serde(rename="Filters")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub filters: Option<ProductViewFilters>,
    #[doc="<p>The maximum number of items to return in the results. If more results exist than fit in the specified <code>PageSize</code>, the value of <code>NextPageToken</code> in the response is non-null.</p>"]
    #[serde(rename="PageSize")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub page_size: Option<PageSize>,
    #[doc="<p>The page token of the first page retrieved. If null, this retrieves the first page of size <code>PageSize</code>.</p>"]
    #[serde(rename="PageToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub page_token: Option<PageToken>,
    #[doc="<p>The sort field specifier. If no value is specified, results are not sorted.</p>"]
    #[serde(rename="SortBy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sort_by: Option<ProductViewSortBy>,
    #[doc="<p>The sort order specifier. If no value is specified, results are not sorted.</p>"]
    #[serde(rename="SortOrder")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sort_order: Option<SortOrder>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct SearchProductsOutput {
    #[doc="<p>The page token to use to retrieve the next page of results for this operation. If there are no more pages, this value is null.</p>"]
    #[serde(rename="NextPageToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_page_token: Option<PageToken>,
    #[doc="<p>A list of the product view aggregation value objects.</p>"]
    #[serde(rename="ProductViewAggregations")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub product_view_aggregations: Option<ProductViewAggregations>,
    #[doc="<p>A list of the product view summary objects.</p>"]
    #[serde(rename="ProductViewSummaries")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub product_view_summaries: Option<ProductViewSummaries>,
}

pub type SortOrder = String;
pub type Status = String;
pub type SupportDescription = String;
pub type SupportEmail = String;
pub type SupportUrl = String;
#[doc="<p>Key/value pairs to associate with this provisioning. These tags are entirely discretionary and are propagated to the resources created in the provisioning.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct Tag {
    #[doc="<p>The <code>ProvisioningArtifactParameter.TagKey</code> parameter from <a>DescribeProvisioningParameters</a>.</p>"]
    #[serde(rename="Key")]
    pub key: TagKey,
    #[doc="<p>The desired value for this key.</p>"]
    #[serde(rename="Value")]
    pub value: TagValue,
}

pub type TagKey = String;
pub type TagKeys = Vec<TagKey>;
pub type TagValue = String;
pub type Tags = Vec<Tag>;
#[derive(Default,Debug,Clone,Serialize)]
pub struct TerminateProvisionedProductInput {
    #[doc="<p>The language code to use for this operation. Supported language codes are as follows:</p> <p>\"en\" (English)</p> <p>\"jp\" (Japanese)</p> <p>\"zh\" (Chinese)</p> <p>If no code is specified, \"en\" is used as the default.</p>"]
    #[serde(rename="AcceptLanguage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub accept_language: Option<AcceptLanguage>,
    #[doc="<p>If set to true, AWS Service Catalog stops managing the specified ProvisionedProduct object even if it cannot delete the underlying resources.</p>"]
    #[serde(rename="IgnoreErrors")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ignore_errors: Option<IgnoreErrors>,
    #[doc="<p>The identifier of the ProvisionedProduct object to terminate. Specify either <code>ProvisionedProductName</code> or <code>ProvisionedProductId</code>, but not both.</p>"]
    #[serde(rename="ProvisionedProductId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub provisioned_product_id: Option<Id>,
    #[doc="<p>The name of the ProvisionedProduct object to terminate. Specify either <code>ProvisionedProductName</code> or <code>ProvisionedProductId</code>, but not both.</p>"]
    #[serde(rename="ProvisionedProductName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub provisioned_product_name: Option<ProvisionedProductNameOrArn>,
    #[doc="<p>An idempotency token that uniquely identifies the termination request. This token is only valid during the termination process. After the ProvisionedProduct object is terminated, further requests to terminate the same ProvisionedProduct object always return <b>ResourceNotFound</b> regardless of the value of <code>TerminateToken</code>.</p>"]
    #[serde(rename="TerminateToken")]
    pub terminate_token: IdempotencyToken,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct TerminateProvisionedProductOutput {
    #[doc="<p>The detailed result of the <a>TerminateProvisionedProduct</a> request, containing the inputs made to that request, the current state of the request, a pointer to the ProvisionedProduct object that the request is modifying, and a list of any errors that the request encountered.</p>"]
    #[serde(rename="RecordDetail")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub record_detail: Option<RecordDetail>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateConstraintInput {
    #[doc="<p>The language code to use for this operation. Supported language codes are as follows:</p> <p>\"en\" (English)</p> <p>\"jp\" (Japanese)</p> <p>\"zh\" (Chinese)</p> <p>If no code is specified, \"en\" is used as the default.</p>"]
    #[serde(rename="AcceptLanguage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub accept_language: Option<AcceptLanguage>,
    #[doc="<p>The updated text description of the constraint.</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<ConstraintDescription>,
    #[doc="<p>The identifier of the constraint to update.</p>"]
    #[serde(rename="Id")]
    pub id: Id,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateConstraintOutput {
    #[doc="<p>The resulting detailed constraint information.</p>"]
    #[serde(rename="ConstraintDetail")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub constraint_detail: Option<ConstraintDetail>,
    #[doc="<p>The resulting updated constraint parameters.</p>"]
    #[serde(rename="ConstraintParameters")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub constraint_parameters: Option<ConstraintParameters>,
    #[doc="<p>The status of the current request.</p>"]
    #[serde(rename="Status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<Status>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdatePortfolioInput {
    #[doc="<p>The language code to use for this operation. Supported language codes are as follows:</p> <p>\"en\" (English)</p> <p>\"jp\" (Japanese)</p> <p>\"zh\" (Chinese)</p> <p>If no code is specified, \"en\" is used as the default.</p>"]
    #[serde(rename="AcceptLanguage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub accept_language: Option<AcceptLanguage>,
    #[doc="<p>Tags to add to the existing list of tags associated with the portfolio.</p>"]
    #[serde(rename="AddTags")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub add_tags: Option<AddTags>,
    #[doc="<p>The updated text description of the portfolio.</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<PortfolioDescription>,
    #[doc="<p>The name to use for display purposes.</p>"]
    #[serde(rename="DisplayName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub display_name: Option<PortfolioDisplayName>,
    #[doc="<p>The identifier of the portfolio for the update request.</p>"]
    #[serde(rename="Id")]
    pub id: Id,
    #[doc="<p>The updated name of the portfolio provider.</p>"]
    #[serde(rename="ProviderName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub provider_name: Option<ProviderName>,
    #[doc="<p>Tags to remove from the existing list of tags associated with the portfolio.</p>"]
    #[serde(rename="RemoveTags")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub remove_tags: Option<TagKeys>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdatePortfolioOutput {
    #[doc="<p>The resulting detailed portfolio information.</p>"]
    #[serde(rename="PortfolioDetail")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub portfolio_detail: Option<PortfolioDetail>,
    #[doc="<p>Tags associated with the portfolio.</p>"]
    #[serde(rename="Tags")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tags: Option<Tags>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateProductInput {
    #[doc="<p>The language code to use for this operation. Supported language codes are as follows:</p> <p>\"en\" (English)</p> <p>\"jp\" (Japanese)</p> <p>\"zh\" (Chinese)</p> <p>If no code is specified, \"en\" is used as the default.</p>"]
    #[serde(rename="AcceptLanguage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub accept_language: Option<AcceptLanguage>,
    #[doc="<p>Tags to add to the existing list of tags associated with the product.</p>"]
    #[serde(rename="AddTags")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub add_tags: Option<AddTags>,
    #[doc="<p>The updated text description of the product.</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<ProductViewShortDescription>,
    #[doc="<p>The updated distributor of the product.</p>"]
    #[serde(rename="Distributor")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub distributor: Option<ProductViewOwner>,
    #[doc="<p>The identifier of the product for the update request.</p>"]
    #[serde(rename="Id")]
    pub id: Id,
    #[doc="<p>The updated product name.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<ProductViewName>,
    #[doc="<p>The updated owner of the product.</p>"]
    #[serde(rename="Owner")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub owner: Option<ProductViewOwner>,
    #[doc="<p>Tags to remove from the existing list of tags associated with the product.</p>"]
    #[serde(rename="RemoveTags")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub remove_tags: Option<TagKeys>,
    #[doc="<p>The updated support description for the product.</p>"]
    #[serde(rename="SupportDescription")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub support_description: Option<SupportDescription>,
    #[doc="<p>The updated support email for the product.</p>"]
    #[serde(rename="SupportEmail")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub support_email: Option<SupportEmail>,
    #[doc="<p>The updated support URL for the product.</p>"]
    #[serde(rename="SupportUrl")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub support_url: Option<SupportUrl>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateProductOutput {
    #[doc="<p>The resulting detailed product view information.</p>"]
    #[serde(rename="ProductViewDetail")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub product_view_detail: Option<ProductViewDetail>,
    #[doc="<p>Tags associated with the product.</p>"]
    #[serde(rename="Tags")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tags: Option<Tags>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateProvisionedProductInput {
    #[doc="<p>The language code to use for this operation. Supported language codes are as follows:</p> <p>\"en\" (English)</p> <p>\"jp\" (Japanese)</p> <p>\"zh\" (Chinese)</p> <p>If no code is specified, \"en\" is used as the default.</p>"]
    #[serde(rename="AcceptLanguage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub accept_language: Option<AcceptLanguage>,
    #[doc="<p>The identifier of the path to use in the updated ProvisionedProduct object. This value is optional if the product has a default path, and is required if there is more than one path for the specified product.</p>"]
    #[serde(rename="PathId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub path_id: Option<Id>,
    #[doc="<p>The identifier of the ProvisionedProduct object.</p>"]
    #[serde(rename="ProductId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub product_id: Option<Id>,
    #[doc="<p>The identifier of the ProvisionedProduct object to update. Specify either <code>ProvisionedProductName</code> or <code>ProvisionedProductId</code>, but not both.</p>"]
    #[serde(rename="ProvisionedProductId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub provisioned_product_id: Option<Id>,
    #[doc="<p>The updated name of the ProvisionedProduct object. Specify either <code>ProvisionedProductName</code> or <code>ProvisionedProductId</code>, but not both.</p>"]
    #[serde(rename="ProvisionedProductName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub provisioned_product_name: Option<ProvisionedProductNameOrArn>,
    #[doc="<p>The provisioning artifact identifier for this product. This is sometimes referred to as the product version.</p>"]
    #[serde(rename="ProvisioningArtifactId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub provisioning_artifact_id: Option<Id>,
    #[doc="<p>A list of <code>ProvisioningParameter</code> objects used to update the ProvisionedProduct object.</p>"]
    #[serde(rename="ProvisioningParameters")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub provisioning_parameters: Option<UpdateProvisioningParameters>,
    #[doc="<p>The idempotency token that uniquely identifies the provisioning update request.</p>"]
    #[serde(rename="UpdateToken")]
    pub update_token: IdempotencyToken,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateProvisionedProductOutput {
    #[doc="<p>The detailed result of the <a>UpdateProvisionedProduct</a> request, containing the inputs made to that request, the current state of the request, a pointer to the ProvisionedProduct object that the request is modifying, and a list of any errors that the request encountered.</p>"]
    #[serde(rename="RecordDetail")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub record_detail: Option<RecordDetail>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateProvisioningArtifactInput {
    #[doc="<p>The language code to use for this operation. Supported language codes are as follows:</p> <p>\"en\" (English)</p> <p>\"jp\" (Japanese)</p> <p>\"zh\" (Chinese)</p> <p>If no code is specified, \"en\" is used as the default.</p>"]
    #[serde(rename="AcceptLanguage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub accept_language: Option<AcceptLanguage>,
    #[doc="<p>The updated text description of the provisioning artifact.</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<ProvisioningArtifactDescription>,
    #[doc="<p>The updated name of the provisioning artifact.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<ProvisioningArtifactName>,
    #[doc="<p>The product identifier.</p>"]
    #[serde(rename="ProductId")]
    pub product_id: Id,
    #[doc="<p>The identifier of the provisioning artifact for the update request. This is sometimes referred to as the product version.</p>"]
    #[serde(rename="ProvisioningArtifactId")]
    pub provisioning_artifact_id: Id,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateProvisioningArtifactOutput {
    #[doc="<p>Additional information about the provisioning artifact update request.</p>"]
    #[serde(rename="Info")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub info: Option<ProvisioningArtifactInfo>,
    #[doc="<p>The resulting detailed provisioning artifact information.</p>"]
    #[serde(rename="ProvisioningArtifactDetail")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub provisioning_artifact_detail: Option<ProvisioningArtifactDetail>,
    #[doc="<p>The status of the current request.</p>"]
    #[serde(rename="Status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<Status>,
}

#[doc="<p>The parameter key-value pair used to update a ProvisionedProduct object. If <code>UsePreviousValue</code> is set to true, <code>Value</code> is ignored and the value for <code>Key</code> is kept as previously set (current value).</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateProvisioningParameter {
    #[doc="<p>The <code>ProvisioningArtifactParameter.ParameterKey</code> parameter from <a>DescribeProvisioningParameters</a>.</p>"]
    #[serde(rename="Key")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub key: Option<ParameterKey>,
    #[doc="<p>If true, uses the currently set value for <code>Key</code>, ignoring <code>UpdateProvisioningParameter.Value</code>.</p>"]
    #[serde(rename="UsePreviousValue")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub use_previous_value: Option<UsePreviousValue>,
    #[doc="<p>The value to use for updating the product provisioning. Any constraints on this value can be found in the <code>ProvisioningArtifactParameter</code> parameter for <code>Key</code>.</p>"]
    #[serde(rename="Value")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub value: Option<ParameterValue>,
}

pub type UpdateProvisioningParameters = Vec<UpdateProvisioningParameter>;
pub type UpdatedTime = f64;
#[doc="<p>Additional information provided by the administrator.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct UsageInstruction {
    #[doc="<p>The usage instruction type for the value.</p>"]
    #[serde(rename="Type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub type_: Option<InstructionType>,
    #[doc="<p>The usage instruction value for this type.</p>"]
    #[serde(rename="Value")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub value: Option<InstructionValue>,
}

pub type UsageInstructions = Vec<UsageInstruction>;
pub type UsePreviousValue = bool;
pub type Verbose = bool;
/// Errors returned by AcceptPortfolioShare
#[derive(Debug, PartialEq)]
pub enum AcceptPortfolioShareError {
    ///<p>One or more parameters provided to the operation are invalid.</p>
    InvalidParameters(String),
    ///<p>The current limits of the service would have been exceeded by this operation. Reduce the resource use or increase the service limits and retry the operation.</p>
    LimitExceeded(String),
    ///<p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl AcceptPortfolioShareError {
    pub fn from_body(body: &str) -> AcceptPortfolioShareError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParametersException" => {
                        AcceptPortfolioShareError::InvalidParameters(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        AcceptPortfolioShareError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        AcceptPortfolioShareError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        AcceptPortfolioShareError::Validation(error_message.to_string())
                    }
                    _ => AcceptPortfolioShareError::Unknown(String::from(body)),
                }
            }
            Err(_) => AcceptPortfolioShareError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AcceptPortfolioShareError {
    fn from(err: serde_json::error::Error) -> AcceptPortfolioShareError {
        AcceptPortfolioShareError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AcceptPortfolioShareError {
    fn from(err: CredentialsError) -> AcceptPortfolioShareError {
        AcceptPortfolioShareError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AcceptPortfolioShareError {
    fn from(err: HttpDispatchError) -> AcceptPortfolioShareError {
        AcceptPortfolioShareError::HttpDispatch(err)
    }
}
impl fmt::Display for AcceptPortfolioShareError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AcceptPortfolioShareError {
    fn description(&self) -> &str {
        match *self {
            AcceptPortfolioShareError::InvalidParameters(ref cause) => cause,
            AcceptPortfolioShareError::LimitExceeded(ref cause) => cause,
            AcceptPortfolioShareError::ResourceNotFound(ref cause) => cause,
            AcceptPortfolioShareError::Validation(ref cause) => cause,
            AcceptPortfolioShareError::Credentials(ref err) => err.description(),
            AcceptPortfolioShareError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AcceptPortfolioShareError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AssociatePrincipalWithPortfolio
#[derive(Debug, PartialEq)]
pub enum AssociatePrincipalWithPortfolioError {
    ///<p>One or more parameters provided to the operation are invalid.</p>
    InvalidParameters(String),
    ///<p>The current limits of the service would have been exceeded by this operation. Reduce the resource use or increase the service limits and retry the operation.</p>
    LimitExceeded(String),
    ///<p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl AssociatePrincipalWithPortfolioError {
    pub fn from_body(body: &str) -> AssociatePrincipalWithPortfolioError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParametersException" => AssociatePrincipalWithPortfolioError::InvalidParameters(String::from(error_message)),
                    "LimitExceededException" => AssociatePrincipalWithPortfolioError::LimitExceeded(String::from(error_message)),
                    "ResourceNotFoundException" => AssociatePrincipalWithPortfolioError::ResourceNotFound(String::from(error_message)),
                    "ValidationException" => {
                        AssociatePrincipalWithPortfolioError::Validation(error_message.to_string())
                    }
                    _ => AssociatePrincipalWithPortfolioError::Unknown(String::from(body)),
                }
            }
            Err(_) => AssociatePrincipalWithPortfolioError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AssociatePrincipalWithPortfolioError {
    fn from(err: serde_json::error::Error) -> AssociatePrincipalWithPortfolioError {
        AssociatePrincipalWithPortfolioError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AssociatePrincipalWithPortfolioError {
    fn from(err: CredentialsError) -> AssociatePrincipalWithPortfolioError {
        AssociatePrincipalWithPortfolioError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AssociatePrincipalWithPortfolioError {
    fn from(err: HttpDispatchError) -> AssociatePrincipalWithPortfolioError {
        AssociatePrincipalWithPortfolioError::HttpDispatch(err)
    }
}
impl fmt::Display for AssociatePrincipalWithPortfolioError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AssociatePrincipalWithPortfolioError {
    fn description(&self) -> &str {
        match *self {
            AssociatePrincipalWithPortfolioError::InvalidParameters(ref cause) => cause,
            AssociatePrincipalWithPortfolioError::LimitExceeded(ref cause) => cause,
            AssociatePrincipalWithPortfolioError::ResourceNotFound(ref cause) => cause,
            AssociatePrincipalWithPortfolioError::Validation(ref cause) => cause,
            AssociatePrincipalWithPortfolioError::Credentials(ref err) => err.description(),
            AssociatePrincipalWithPortfolioError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AssociatePrincipalWithPortfolioError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AssociateProductWithPortfolio
#[derive(Debug, PartialEq)]
pub enum AssociateProductWithPortfolioError {
    ///<p>One or more parameters provided to the operation are invalid.</p>
    InvalidParameters(String),
    ///<p>The current limits of the service would have been exceeded by this operation. Reduce the resource use or increase the service limits and retry the operation.</p>
    LimitExceeded(String),
    ///<p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl AssociateProductWithPortfolioError {
    pub fn from_body(body: &str) -> AssociateProductWithPortfolioError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParametersException" => AssociateProductWithPortfolioError::InvalidParameters(String::from(error_message)),
                    "LimitExceededException" => AssociateProductWithPortfolioError::LimitExceeded(String::from(error_message)),
                    "ResourceNotFoundException" => AssociateProductWithPortfolioError::ResourceNotFound(String::from(error_message)),
                    "ValidationException" => {
                        AssociateProductWithPortfolioError::Validation(error_message.to_string())
                    }
                    _ => AssociateProductWithPortfolioError::Unknown(String::from(body)),
                }
            }
            Err(_) => AssociateProductWithPortfolioError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AssociateProductWithPortfolioError {
    fn from(err: serde_json::error::Error) -> AssociateProductWithPortfolioError {
        AssociateProductWithPortfolioError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AssociateProductWithPortfolioError {
    fn from(err: CredentialsError) -> AssociateProductWithPortfolioError {
        AssociateProductWithPortfolioError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AssociateProductWithPortfolioError {
    fn from(err: HttpDispatchError) -> AssociateProductWithPortfolioError {
        AssociateProductWithPortfolioError::HttpDispatch(err)
    }
}
impl fmt::Display for AssociateProductWithPortfolioError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AssociateProductWithPortfolioError {
    fn description(&self) -> &str {
        match *self {
            AssociateProductWithPortfolioError::InvalidParameters(ref cause) => cause,
            AssociateProductWithPortfolioError::LimitExceeded(ref cause) => cause,
            AssociateProductWithPortfolioError::ResourceNotFound(ref cause) => cause,
            AssociateProductWithPortfolioError::Validation(ref cause) => cause,
            AssociateProductWithPortfolioError::Credentials(ref err) => err.description(),
            AssociateProductWithPortfolioError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AssociateProductWithPortfolioError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateConstraint
#[derive(Debug, PartialEq)]
pub enum CreateConstraintError {
    ///<p>The specified resource is a duplicate.</p>
    DuplicateResource(String),
    ///<p>One or more parameters provided to the operation are invalid.</p>
    InvalidParameters(String),
    ///<p>The current limits of the service would have been exceeded by this operation. Reduce the resource use or increase the service limits and retry the operation.</p>
    LimitExceeded(String),
    ///<p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateConstraintError {
    pub fn from_body(body: &str) -> CreateConstraintError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DuplicateResourceException" => {
                        CreateConstraintError::DuplicateResource(String::from(error_message))
                    }
                    "InvalidParametersException" => {
                        CreateConstraintError::InvalidParameters(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateConstraintError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        CreateConstraintError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateConstraintError::Validation(error_message.to_string())
                    }
                    _ => CreateConstraintError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateConstraintError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateConstraintError {
    fn from(err: serde_json::error::Error) -> CreateConstraintError {
        CreateConstraintError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateConstraintError {
    fn from(err: CredentialsError) -> CreateConstraintError {
        CreateConstraintError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateConstraintError {
    fn from(err: HttpDispatchError) -> CreateConstraintError {
        CreateConstraintError::HttpDispatch(err)
    }
}
impl fmt::Display for CreateConstraintError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateConstraintError {
    fn description(&self) -> &str {
        match *self {
            CreateConstraintError::DuplicateResource(ref cause) => cause,
            CreateConstraintError::InvalidParameters(ref cause) => cause,
            CreateConstraintError::LimitExceeded(ref cause) => cause,
            CreateConstraintError::ResourceNotFound(ref cause) => cause,
            CreateConstraintError::Validation(ref cause) => cause,
            CreateConstraintError::Credentials(ref err) => err.description(),
            CreateConstraintError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateConstraintError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreatePortfolio
#[derive(Debug, PartialEq)]
pub enum CreatePortfolioError {
    ///<p>One or more parameters provided to the operation are invalid.</p>
    InvalidParameters(String),
    ///<p>The current limits of the service would have been exceeded by this operation. Reduce the resource use or increase the service limits and retry the operation.</p>
    LimitExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreatePortfolioError {
    pub fn from_body(body: &str) -> CreatePortfolioError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParametersException" => {
                        CreatePortfolioError::InvalidParameters(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreatePortfolioError::LimitExceeded(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreatePortfolioError::Validation(error_message.to_string())
                    }
                    _ => CreatePortfolioError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreatePortfolioError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreatePortfolioError {
    fn from(err: serde_json::error::Error) -> CreatePortfolioError {
        CreatePortfolioError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreatePortfolioError {
    fn from(err: CredentialsError) -> CreatePortfolioError {
        CreatePortfolioError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreatePortfolioError {
    fn from(err: HttpDispatchError) -> CreatePortfolioError {
        CreatePortfolioError::HttpDispatch(err)
    }
}
impl fmt::Display for CreatePortfolioError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreatePortfolioError {
    fn description(&self) -> &str {
        match *self {
            CreatePortfolioError::InvalidParameters(ref cause) => cause,
            CreatePortfolioError::LimitExceeded(ref cause) => cause,
            CreatePortfolioError::Validation(ref cause) => cause,
            CreatePortfolioError::Credentials(ref err) => err.description(),
            CreatePortfolioError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreatePortfolioError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreatePortfolioShare
#[derive(Debug, PartialEq)]
pub enum CreatePortfolioShareError {
    ///<p>One or more parameters provided to the operation are invalid.</p>
    InvalidParameters(String),
    ///<p>The current limits of the service would have been exceeded by this operation. Reduce the resource use or increase the service limits and retry the operation.</p>
    LimitExceeded(String),
    ///<p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreatePortfolioShareError {
    pub fn from_body(body: &str) -> CreatePortfolioShareError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParametersException" => {
                        CreatePortfolioShareError::InvalidParameters(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreatePortfolioShareError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        CreatePortfolioShareError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreatePortfolioShareError::Validation(error_message.to_string())
                    }
                    _ => CreatePortfolioShareError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreatePortfolioShareError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreatePortfolioShareError {
    fn from(err: serde_json::error::Error) -> CreatePortfolioShareError {
        CreatePortfolioShareError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreatePortfolioShareError {
    fn from(err: CredentialsError) -> CreatePortfolioShareError {
        CreatePortfolioShareError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreatePortfolioShareError {
    fn from(err: HttpDispatchError) -> CreatePortfolioShareError {
        CreatePortfolioShareError::HttpDispatch(err)
    }
}
impl fmt::Display for CreatePortfolioShareError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreatePortfolioShareError {
    fn description(&self) -> &str {
        match *self {
            CreatePortfolioShareError::InvalidParameters(ref cause) => cause,
            CreatePortfolioShareError::LimitExceeded(ref cause) => cause,
            CreatePortfolioShareError::ResourceNotFound(ref cause) => cause,
            CreatePortfolioShareError::Validation(ref cause) => cause,
            CreatePortfolioShareError::Credentials(ref err) => err.description(),
            CreatePortfolioShareError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreatePortfolioShareError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateProduct
#[derive(Debug, PartialEq)]
pub enum CreateProductError {
    ///<p>One or more parameters provided to the operation are invalid.</p>
    InvalidParameters(String),
    ///<p>The current limits of the service would have been exceeded by this operation. Reduce the resource use or increase the service limits and retry the operation.</p>
    LimitExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateProductError {
    pub fn from_body(body: &str) -> CreateProductError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParametersException" => {
                        CreateProductError::InvalidParameters(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateProductError::LimitExceeded(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateProductError::Validation(error_message.to_string())
                    }
                    _ => CreateProductError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateProductError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateProductError {
    fn from(err: serde_json::error::Error) -> CreateProductError {
        CreateProductError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateProductError {
    fn from(err: CredentialsError) -> CreateProductError {
        CreateProductError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateProductError {
    fn from(err: HttpDispatchError) -> CreateProductError {
        CreateProductError::HttpDispatch(err)
    }
}
impl fmt::Display for CreateProductError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateProductError {
    fn description(&self) -> &str {
        match *self {
            CreateProductError::InvalidParameters(ref cause) => cause,
            CreateProductError::LimitExceeded(ref cause) => cause,
            CreateProductError::Validation(ref cause) => cause,
            CreateProductError::Credentials(ref err) => err.description(),
            CreateProductError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateProductError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateProvisioningArtifact
#[derive(Debug, PartialEq)]
pub enum CreateProvisioningArtifactError {
    ///<p>One or more parameters provided to the operation are invalid.</p>
    InvalidParameters(String),
    ///<p>The current limits of the service would have been exceeded by this operation. Reduce the resource use or increase the service limits and retry the operation.</p>
    LimitExceeded(String),
    ///<p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateProvisioningArtifactError {
    pub fn from_body(body: &str) -> CreateProvisioningArtifactError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParametersException" => CreateProvisioningArtifactError::InvalidParameters(String::from(error_message)),
                    "LimitExceededException" => {
                        CreateProvisioningArtifactError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => CreateProvisioningArtifactError::ResourceNotFound(String::from(error_message)),
                    "ValidationException" => {
                        CreateProvisioningArtifactError::Validation(error_message.to_string())
                    }
                    _ => CreateProvisioningArtifactError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateProvisioningArtifactError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateProvisioningArtifactError {
    fn from(err: serde_json::error::Error) -> CreateProvisioningArtifactError {
        CreateProvisioningArtifactError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateProvisioningArtifactError {
    fn from(err: CredentialsError) -> CreateProvisioningArtifactError {
        CreateProvisioningArtifactError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateProvisioningArtifactError {
    fn from(err: HttpDispatchError) -> CreateProvisioningArtifactError {
        CreateProvisioningArtifactError::HttpDispatch(err)
    }
}
impl fmt::Display for CreateProvisioningArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateProvisioningArtifactError {
    fn description(&self) -> &str {
        match *self {
            CreateProvisioningArtifactError::InvalidParameters(ref cause) => cause,
            CreateProvisioningArtifactError::LimitExceeded(ref cause) => cause,
            CreateProvisioningArtifactError::ResourceNotFound(ref cause) => cause,
            CreateProvisioningArtifactError::Validation(ref cause) => cause,
            CreateProvisioningArtifactError::Credentials(ref err) => err.description(),
            CreateProvisioningArtifactError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateProvisioningArtifactError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteConstraint
#[derive(Debug, PartialEq)]
pub enum DeleteConstraintError {
    ///<p>One or more parameters provided to the operation are invalid.</p>
    InvalidParameters(String),
    ///<p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteConstraintError {
    pub fn from_body(body: &str) -> DeleteConstraintError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParametersException" => {
                        DeleteConstraintError::InvalidParameters(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteConstraintError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteConstraintError::Validation(error_message.to_string())
                    }
                    _ => DeleteConstraintError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteConstraintError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteConstraintError {
    fn from(err: serde_json::error::Error) -> DeleteConstraintError {
        DeleteConstraintError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteConstraintError {
    fn from(err: CredentialsError) -> DeleteConstraintError {
        DeleteConstraintError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteConstraintError {
    fn from(err: HttpDispatchError) -> DeleteConstraintError {
        DeleteConstraintError::HttpDispatch(err)
    }
}
impl fmt::Display for DeleteConstraintError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteConstraintError {
    fn description(&self) -> &str {
        match *self {
            DeleteConstraintError::InvalidParameters(ref cause) => cause,
            DeleteConstraintError::ResourceNotFound(ref cause) => cause,
            DeleteConstraintError::Validation(ref cause) => cause,
            DeleteConstraintError::Credentials(ref err) => err.description(),
            DeleteConstraintError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteConstraintError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeletePortfolio
#[derive(Debug, PartialEq)]
pub enum DeletePortfolioError {
    ///<p>One or more parameters provided to the operation are invalid.</p>
    InvalidParameters(String),
    ///<p>The operation was requested against a resource that is currently in use. Free the resource from use and retry the operation.</p>
    ResourceInUse(String),
    ///<p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeletePortfolioError {
    pub fn from_body(body: &str) -> DeletePortfolioError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParametersException" => {
                        DeletePortfolioError::InvalidParameters(String::from(error_message))
                    }
                    "ResourceInUseException" => {
                        DeletePortfolioError::ResourceInUse(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeletePortfolioError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeletePortfolioError::Validation(error_message.to_string())
                    }
                    _ => DeletePortfolioError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeletePortfolioError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeletePortfolioError {
    fn from(err: serde_json::error::Error) -> DeletePortfolioError {
        DeletePortfolioError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeletePortfolioError {
    fn from(err: CredentialsError) -> DeletePortfolioError {
        DeletePortfolioError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeletePortfolioError {
    fn from(err: HttpDispatchError) -> DeletePortfolioError {
        DeletePortfolioError::HttpDispatch(err)
    }
}
impl fmt::Display for DeletePortfolioError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeletePortfolioError {
    fn description(&self) -> &str {
        match *self {
            DeletePortfolioError::InvalidParameters(ref cause) => cause,
            DeletePortfolioError::ResourceInUse(ref cause) => cause,
            DeletePortfolioError::ResourceNotFound(ref cause) => cause,
            DeletePortfolioError::Validation(ref cause) => cause,
            DeletePortfolioError::Credentials(ref err) => err.description(),
            DeletePortfolioError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeletePortfolioError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeletePortfolioShare
#[derive(Debug, PartialEq)]
pub enum DeletePortfolioShareError {
    ///<p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeletePortfolioShareError {
    pub fn from_body(body: &str) -> DeletePortfolioShareError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        DeletePortfolioShareError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeletePortfolioShareError::Validation(error_message.to_string())
                    }
                    _ => DeletePortfolioShareError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeletePortfolioShareError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeletePortfolioShareError {
    fn from(err: serde_json::error::Error) -> DeletePortfolioShareError {
        DeletePortfolioShareError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeletePortfolioShareError {
    fn from(err: CredentialsError) -> DeletePortfolioShareError {
        DeletePortfolioShareError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeletePortfolioShareError {
    fn from(err: HttpDispatchError) -> DeletePortfolioShareError {
        DeletePortfolioShareError::HttpDispatch(err)
    }
}
impl fmt::Display for DeletePortfolioShareError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeletePortfolioShareError {
    fn description(&self) -> &str {
        match *self {
            DeletePortfolioShareError::ResourceNotFound(ref cause) => cause,
            DeletePortfolioShareError::Validation(ref cause) => cause,
            DeletePortfolioShareError::Credentials(ref err) => err.description(),
            DeletePortfolioShareError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeletePortfolioShareError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteProduct
#[derive(Debug, PartialEq)]
pub enum DeleteProductError {
    ///<p>One or more parameters provided to the operation are invalid.</p>
    InvalidParameters(String),
    ///<p>The operation was requested against a resource that is currently in use. Free the resource from use and retry the operation.</p>
    ResourceInUse(String),
    ///<p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteProductError {
    pub fn from_body(body: &str) -> DeleteProductError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParametersException" => {
                        DeleteProductError::InvalidParameters(String::from(error_message))
                    }
                    "ResourceInUseException" => {
                        DeleteProductError::ResourceInUse(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteProductError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteProductError::Validation(error_message.to_string())
                    }
                    _ => DeleteProductError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteProductError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteProductError {
    fn from(err: serde_json::error::Error) -> DeleteProductError {
        DeleteProductError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteProductError {
    fn from(err: CredentialsError) -> DeleteProductError {
        DeleteProductError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteProductError {
    fn from(err: HttpDispatchError) -> DeleteProductError {
        DeleteProductError::HttpDispatch(err)
    }
}
impl fmt::Display for DeleteProductError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteProductError {
    fn description(&self) -> &str {
        match *self {
            DeleteProductError::InvalidParameters(ref cause) => cause,
            DeleteProductError::ResourceInUse(ref cause) => cause,
            DeleteProductError::ResourceNotFound(ref cause) => cause,
            DeleteProductError::Validation(ref cause) => cause,
            DeleteProductError::Credentials(ref err) => err.description(),
            DeleteProductError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteProductError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteProvisioningArtifact
#[derive(Debug, PartialEq)]
pub enum DeleteProvisioningArtifactError {
    ///<p>One or more parameters provided to the operation are invalid.</p>
    InvalidParameters(String),
    ///<p>The operation was requested against a resource that is currently in use. Free the resource from use and retry the operation.</p>
    ResourceInUse(String),
    ///<p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteProvisioningArtifactError {
    pub fn from_body(body: &str) -> DeleteProvisioningArtifactError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParametersException" => DeleteProvisioningArtifactError::InvalidParameters(String::from(error_message)),
                    "ResourceInUseException" => {
                        DeleteProvisioningArtifactError::ResourceInUse(String::from(error_message))
                    }
                    "ResourceNotFoundException" => DeleteProvisioningArtifactError::ResourceNotFound(String::from(error_message)),
                    "ValidationException" => {
                        DeleteProvisioningArtifactError::Validation(error_message.to_string())
                    }
                    _ => DeleteProvisioningArtifactError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteProvisioningArtifactError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteProvisioningArtifactError {
    fn from(err: serde_json::error::Error) -> DeleteProvisioningArtifactError {
        DeleteProvisioningArtifactError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteProvisioningArtifactError {
    fn from(err: CredentialsError) -> DeleteProvisioningArtifactError {
        DeleteProvisioningArtifactError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteProvisioningArtifactError {
    fn from(err: HttpDispatchError) -> DeleteProvisioningArtifactError {
        DeleteProvisioningArtifactError::HttpDispatch(err)
    }
}
impl fmt::Display for DeleteProvisioningArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteProvisioningArtifactError {
    fn description(&self) -> &str {
        match *self {
            DeleteProvisioningArtifactError::InvalidParameters(ref cause) => cause,
            DeleteProvisioningArtifactError::ResourceInUse(ref cause) => cause,
            DeleteProvisioningArtifactError::ResourceNotFound(ref cause) => cause,
            DeleteProvisioningArtifactError::Validation(ref cause) => cause,
            DeleteProvisioningArtifactError::Credentials(ref err) => err.description(),
            DeleteProvisioningArtifactError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteProvisioningArtifactError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeConstraint
#[derive(Debug, PartialEq)]
pub enum DescribeConstraintError {
    ///<p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeConstraintError {
    pub fn from_body(body: &str) -> DescribeConstraintError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        DescribeConstraintError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeConstraintError::Validation(error_message.to_string())
                    }
                    _ => DescribeConstraintError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeConstraintError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeConstraintError {
    fn from(err: serde_json::error::Error) -> DescribeConstraintError {
        DescribeConstraintError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeConstraintError {
    fn from(err: CredentialsError) -> DescribeConstraintError {
        DescribeConstraintError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeConstraintError {
    fn from(err: HttpDispatchError) -> DescribeConstraintError {
        DescribeConstraintError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeConstraintError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeConstraintError {
    fn description(&self) -> &str {
        match *self {
            DescribeConstraintError::ResourceNotFound(ref cause) => cause,
            DescribeConstraintError::Validation(ref cause) => cause,
            DescribeConstraintError::Credentials(ref err) => err.description(),
            DescribeConstraintError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeConstraintError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribePortfolio
#[derive(Debug, PartialEq)]
pub enum DescribePortfolioError {
    ///<p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribePortfolioError {
    pub fn from_body(body: &str) -> DescribePortfolioError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        DescribePortfolioError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribePortfolioError::Validation(error_message.to_string())
                    }
                    _ => DescribePortfolioError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribePortfolioError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribePortfolioError {
    fn from(err: serde_json::error::Error) -> DescribePortfolioError {
        DescribePortfolioError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribePortfolioError {
    fn from(err: CredentialsError) -> DescribePortfolioError {
        DescribePortfolioError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribePortfolioError {
    fn from(err: HttpDispatchError) -> DescribePortfolioError {
        DescribePortfolioError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribePortfolioError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribePortfolioError {
    fn description(&self) -> &str {
        match *self {
            DescribePortfolioError::ResourceNotFound(ref cause) => cause,
            DescribePortfolioError::Validation(ref cause) => cause,
            DescribePortfolioError::Credentials(ref err) => err.description(),
            DescribePortfolioError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribePortfolioError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeProduct
#[derive(Debug, PartialEq)]
pub enum DescribeProductError {
    ///<p>One or more parameters provided to the operation are invalid.</p>
    InvalidParameters(String),
    ///<p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeProductError {
    pub fn from_body(body: &str) -> DescribeProductError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParametersException" => {
                        DescribeProductError::InvalidParameters(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DescribeProductError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeProductError::Validation(error_message.to_string())
                    }
                    _ => DescribeProductError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeProductError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeProductError {
    fn from(err: serde_json::error::Error) -> DescribeProductError {
        DescribeProductError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeProductError {
    fn from(err: CredentialsError) -> DescribeProductError {
        DescribeProductError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeProductError {
    fn from(err: HttpDispatchError) -> DescribeProductError {
        DescribeProductError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeProductError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeProductError {
    fn description(&self) -> &str {
        match *self {
            DescribeProductError::InvalidParameters(ref cause) => cause,
            DescribeProductError::ResourceNotFound(ref cause) => cause,
            DescribeProductError::Validation(ref cause) => cause,
            DescribeProductError::Credentials(ref err) => err.description(),
            DescribeProductError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeProductError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeProductAsAdmin
#[derive(Debug, PartialEq)]
pub enum DescribeProductAsAdminError {
    ///<p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeProductAsAdminError {
    pub fn from_body(body: &str) -> DescribeProductAsAdminError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        DescribeProductAsAdminError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeProductAsAdminError::Validation(error_message.to_string())
                    }
                    _ => DescribeProductAsAdminError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeProductAsAdminError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeProductAsAdminError {
    fn from(err: serde_json::error::Error) -> DescribeProductAsAdminError {
        DescribeProductAsAdminError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeProductAsAdminError {
    fn from(err: CredentialsError) -> DescribeProductAsAdminError {
        DescribeProductAsAdminError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeProductAsAdminError {
    fn from(err: HttpDispatchError) -> DescribeProductAsAdminError {
        DescribeProductAsAdminError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeProductAsAdminError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeProductAsAdminError {
    fn description(&self) -> &str {
        match *self {
            DescribeProductAsAdminError::ResourceNotFound(ref cause) => cause,
            DescribeProductAsAdminError::Validation(ref cause) => cause,
            DescribeProductAsAdminError::Credentials(ref err) => err.description(),
            DescribeProductAsAdminError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeProductAsAdminError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeProductView
#[derive(Debug, PartialEq)]
pub enum DescribeProductViewError {
    ///<p>One or more parameters provided to the operation are invalid.</p>
    InvalidParameters(String),
    ///<p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeProductViewError {
    pub fn from_body(body: &str) -> DescribeProductViewError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParametersException" => {
                        DescribeProductViewError::InvalidParameters(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DescribeProductViewError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeProductViewError::Validation(error_message.to_string())
                    }
                    _ => DescribeProductViewError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeProductViewError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeProductViewError {
    fn from(err: serde_json::error::Error) -> DescribeProductViewError {
        DescribeProductViewError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeProductViewError {
    fn from(err: CredentialsError) -> DescribeProductViewError {
        DescribeProductViewError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeProductViewError {
    fn from(err: HttpDispatchError) -> DescribeProductViewError {
        DescribeProductViewError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeProductViewError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeProductViewError {
    fn description(&self) -> &str {
        match *self {
            DescribeProductViewError::InvalidParameters(ref cause) => cause,
            DescribeProductViewError::ResourceNotFound(ref cause) => cause,
            DescribeProductViewError::Validation(ref cause) => cause,
            DescribeProductViewError::Credentials(ref err) => err.description(),
            DescribeProductViewError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeProductViewError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeProvisionedProduct
#[derive(Debug, PartialEq)]
pub enum DescribeProvisionedProductError {
    ///<p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeProvisionedProductError {
    pub fn from_body(body: &str) -> DescribeProvisionedProductError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => DescribeProvisionedProductError::ResourceNotFound(String::from(error_message)),
                    "ValidationException" => {
                        DescribeProvisionedProductError::Validation(error_message.to_string())
                    }
                    _ => DescribeProvisionedProductError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeProvisionedProductError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeProvisionedProductError {
    fn from(err: serde_json::error::Error) -> DescribeProvisionedProductError {
        DescribeProvisionedProductError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeProvisionedProductError {
    fn from(err: CredentialsError) -> DescribeProvisionedProductError {
        DescribeProvisionedProductError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeProvisionedProductError {
    fn from(err: HttpDispatchError) -> DescribeProvisionedProductError {
        DescribeProvisionedProductError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeProvisionedProductError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeProvisionedProductError {
    fn description(&self) -> &str {
        match *self {
            DescribeProvisionedProductError::ResourceNotFound(ref cause) => cause,
            DescribeProvisionedProductError::Validation(ref cause) => cause,
            DescribeProvisionedProductError::Credentials(ref err) => err.description(),
            DescribeProvisionedProductError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeProvisionedProductError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeProvisioningArtifact
#[derive(Debug, PartialEq)]
pub enum DescribeProvisioningArtifactError {
    ///<p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeProvisioningArtifactError {
    pub fn from_body(body: &str) -> DescribeProvisioningArtifactError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => DescribeProvisioningArtifactError::ResourceNotFound(String::from(error_message)),
                    "ValidationException" => {
                        DescribeProvisioningArtifactError::Validation(error_message.to_string())
                    }
                    _ => DescribeProvisioningArtifactError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeProvisioningArtifactError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeProvisioningArtifactError {
    fn from(err: serde_json::error::Error) -> DescribeProvisioningArtifactError {
        DescribeProvisioningArtifactError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeProvisioningArtifactError {
    fn from(err: CredentialsError) -> DescribeProvisioningArtifactError {
        DescribeProvisioningArtifactError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeProvisioningArtifactError {
    fn from(err: HttpDispatchError) -> DescribeProvisioningArtifactError {
        DescribeProvisioningArtifactError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeProvisioningArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeProvisioningArtifactError {
    fn description(&self) -> &str {
        match *self {
            DescribeProvisioningArtifactError::ResourceNotFound(ref cause) => cause,
            DescribeProvisioningArtifactError::Validation(ref cause) => cause,
            DescribeProvisioningArtifactError::Credentials(ref err) => err.description(),
            DescribeProvisioningArtifactError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeProvisioningArtifactError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeProvisioningParameters
#[derive(Debug, PartialEq)]
pub enum DescribeProvisioningParametersError {
    ///<p>One or more parameters provided to the operation are invalid.</p>
    InvalidParameters(String),
    ///<p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeProvisioningParametersError {
    pub fn from_body(body: &str) -> DescribeProvisioningParametersError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParametersException" => DescribeProvisioningParametersError::InvalidParameters(String::from(error_message)),
                    "ResourceNotFoundException" => DescribeProvisioningParametersError::ResourceNotFound(String::from(error_message)),
                    "ValidationException" => {
                        DescribeProvisioningParametersError::Validation(error_message.to_string())
                    }
                    _ => DescribeProvisioningParametersError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeProvisioningParametersError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeProvisioningParametersError {
    fn from(err: serde_json::error::Error) -> DescribeProvisioningParametersError {
        DescribeProvisioningParametersError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeProvisioningParametersError {
    fn from(err: CredentialsError) -> DescribeProvisioningParametersError {
        DescribeProvisioningParametersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeProvisioningParametersError {
    fn from(err: HttpDispatchError) -> DescribeProvisioningParametersError {
        DescribeProvisioningParametersError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeProvisioningParametersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeProvisioningParametersError {
    fn description(&self) -> &str {
        match *self {
            DescribeProvisioningParametersError::InvalidParameters(ref cause) => cause,
            DescribeProvisioningParametersError::ResourceNotFound(ref cause) => cause,
            DescribeProvisioningParametersError::Validation(ref cause) => cause,
            DescribeProvisioningParametersError::Credentials(ref err) => err.description(),
            DescribeProvisioningParametersError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeProvisioningParametersError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeRecord
#[derive(Debug, PartialEq)]
pub enum DescribeRecordError {
    ///<p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeRecordError {
    pub fn from_body(body: &str) -> DescribeRecordError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        DescribeRecordError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeRecordError::Validation(error_message.to_string())
                    }
                    _ => DescribeRecordError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeRecordError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeRecordError {
    fn from(err: serde_json::error::Error) -> DescribeRecordError {
        DescribeRecordError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeRecordError {
    fn from(err: CredentialsError) -> DescribeRecordError {
        DescribeRecordError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeRecordError {
    fn from(err: HttpDispatchError) -> DescribeRecordError {
        DescribeRecordError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeRecordError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeRecordError {
    fn description(&self) -> &str {
        match *self {
            DescribeRecordError::ResourceNotFound(ref cause) => cause,
            DescribeRecordError::Validation(ref cause) => cause,
            DescribeRecordError::Credentials(ref err) => err.description(),
            DescribeRecordError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeRecordError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DisassociatePrincipalFromPortfolio
#[derive(Debug, PartialEq)]
pub enum DisassociatePrincipalFromPortfolioError {
    ///<p>One or more parameters provided to the operation are invalid.</p>
    InvalidParameters(String),
    ///<p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DisassociatePrincipalFromPortfolioError {
    pub fn from_body(body: &str) -> DisassociatePrincipalFromPortfolioError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParametersException" => DisassociatePrincipalFromPortfolioError::InvalidParameters(String::from(error_message)),
                    "ResourceNotFoundException" => DisassociatePrincipalFromPortfolioError::ResourceNotFound(String::from(error_message)),
                    "ValidationException" => {
                        DisassociatePrincipalFromPortfolioError::Validation(error_message
                                                                                .to_string())
                    }
                    _ => DisassociatePrincipalFromPortfolioError::Unknown(String::from(body)),
                }
            }
            Err(_) => DisassociatePrincipalFromPortfolioError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DisassociatePrincipalFromPortfolioError {
    fn from(err: serde_json::error::Error) -> DisassociatePrincipalFromPortfolioError {
        DisassociatePrincipalFromPortfolioError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DisassociatePrincipalFromPortfolioError {
    fn from(err: CredentialsError) -> DisassociatePrincipalFromPortfolioError {
        DisassociatePrincipalFromPortfolioError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DisassociatePrincipalFromPortfolioError {
    fn from(err: HttpDispatchError) -> DisassociatePrincipalFromPortfolioError {
        DisassociatePrincipalFromPortfolioError::HttpDispatch(err)
    }
}
impl fmt::Display for DisassociatePrincipalFromPortfolioError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisassociatePrincipalFromPortfolioError {
    fn description(&self) -> &str {
        match *self {
            DisassociatePrincipalFromPortfolioError::InvalidParameters(ref cause) => cause,
            DisassociatePrincipalFromPortfolioError::ResourceNotFound(ref cause) => cause,
            DisassociatePrincipalFromPortfolioError::Validation(ref cause) => cause,
            DisassociatePrincipalFromPortfolioError::Credentials(ref err) => err.description(),
            DisassociatePrincipalFromPortfolioError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DisassociatePrincipalFromPortfolioError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DisassociateProductFromPortfolio
#[derive(Debug, PartialEq)]
pub enum DisassociateProductFromPortfolioError {
    ///<p>One or more parameters provided to the operation are invalid.</p>
    InvalidParameters(String),
    ///<p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DisassociateProductFromPortfolioError {
    pub fn from_body(body: &str) -> DisassociateProductFromPortfolioError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParametersException" => DisassociateProductFromPortfolioError::InvalidParameters(String::from(error_message)),
                    "ResourceNotFoundException" => DisassociateProductFromPortfolioError::ResourceNotFound(String::from(error_message)),
                    "ValidationException" => {
                        DisassociateProductFromPortfolioError::Validation(error_message.to_string())
                    }
                    _ => DisassociateProductFromPortfolioError::Unknown(String::from(body)),
                }
            }
            Err(_) => DisassociateProductFromPortfolioError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DisassociateProductFromPortfolioError {
    fn from(err: serde_json::error::Error) -> DisassociateProductFromPortfolioError {
        DisassociateProductFromPortfolioError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DisassociateProductFromPortfolioError {
    fn from(err: CredentialsError) -> DisassociateProductFromPortfolioError {
        DisassociateProductFromPortfolioError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DisassociateProductFromPortfolioError {
    fn from(err: HttpDispatchError) -> DisassociateProductFromPortfolioError {
        DisassociateProductFromPortfolioError::HttpDispatch(err)
    }
}
impl fmt::Display for DisassociateProductFromPortfolioError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisassociateProductFromPortfolioError {
    fn description(&self) -> &str {
        match *self {
            DisassociateProductFromPortfolioError::InvalidParameters(ref cause) => cause,
            DisassociateProductFromPortfolioError::ResourceNotFound(ref cause) => cause,
            DisassociateProductFromPortfolioError::Validation(ref cause) => cause,
            DisassociateProductFromPortfolioError::Credentials(ref err) => err.description(),
            DisassociateProductFromPortfolioError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DisassociateProductFromPortfolioError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListAcceptedPortfolioShares
#[derive(Debug, PartialEq)]
pub enum ListAcceptedPortfolioSharesError {
    ///<p>One or more parameters provided to the operation are invalid.</p>
    InvalidParameters(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListAcceptedPortfolioSharesError {
    pub fn from_body(body: &str) -> ListAcceptedPortfolioSharesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParametersException" => ListAcceptedPortfolioSharesError::InvalidParameters(String::from(error_message)),
                    "ValidationException" => {
                        ListAcceptedPortfolioSharesError::Validation(error_message.to_string())
                    }
                    _ => ListAcceptedPortfolioSharesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListAcceptedPortfolioSharesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListAcceptedPortfolioSharesError {
    fn from(err: serde_json::error::Error) -> ListAcceptedPortfolioSharesError {
        ListAcceptedPortfolioSharesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListAcceptedPortfolioSharesError {
    fn from(err: CredentialsError) -> ListAcceptedPortfolioSharesError {
        ListAcceptedPortfolioSharesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListAcceptedPortfolioSharesError {
    fn from(err: HttpDispatchError) -> ListAcceptedPortfolioSharesError {
        ListAcceptedPortfolioSharesError::HttpDispatch(err)
    }
}
impl fmt::Display for ListAcceptedPortfolioSharesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListAcceptedPortfolioSharesError {
    fn description(&self) -> &str {
        match *self {
            ListAcceptedPortfolioSharesError::InvalidParameters(ref cause) => cause,
            ListAcceptedPortfolioSharesError::Validation(ref cause) => cause,
            ListAcceptedPortfolioSharesError::Credentials(ref err) => err.description(),
            ListAcceptedPortfolioSharesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListAcceptedPortfolioSharesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListConstraintsForPortfolio
#[derive(Debug, PartialEq)]
pub enum ListConstraintsForPortfolioError {
    ///<p>One or more parameters provided to the operation are invalid.</p>
    InvalidParameters(String),
    ///<p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListConstraintsForPortfolioError {
    pub fn from_body(body: &str) -> ListConstraintsForPortfolioError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParametersException" => ListConstraintsForPortfolioError::InvalidParameters(String::from(error_message)),
                    "ResourceNotFoundException" => ListConstraintsForPortfolioError::ResourceNotFound(String::from(error_message)),
                    "ValidationException" => {
                        ListConstraintsForPortfolioError::Validation(error_message.to_string())
                    }
                    _ => ListConstraintsForPortfolioError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListConstraintsForPortfolioError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListConstraintsForPortfolioError {
    fn from(err: serde_json::error::Error) -> ListConstraintsForPortfolioError {
        ListConstraintsForPortfolioError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListConstraintsForPortfolioError {
    fn from(err: CredentialsError) -> ListConstraintsForPortfolioError {
        ListConstraintsForPortfolioError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListConstraintsForPortfolioError {
    fn from(err: HttpDispatchError) -> ListConstraintsForPortfolioError {
        ListConstraintsForPortfolioError::HttpDispatch(err)
    }
}
impl fmt::Display for ListConstraintsForPortfolioError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListConstraintsForPortfolioError {
    fn description(&self) -> &str {
        match *self {
            ListConstraintsForPortfolioError::InvalidParameters(ref cause) => cause,
            ListConstraintsForPortfolioError::ResourceNotFound(ref cause) => cause,
            ListConstraintsForPortfolioError::Validation(ref cause) => cause,
            ListConstraintsForPortfolioError::Credentials(ref err) => err.description(),
            ListConstraintsForPortfolioError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListConstraintsForPortfolioError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListLaunchPaths
#[derive(Debug, PartialEq)]
pub enum ListLaunchPathsError {
    ///<p>One or more parameters provided to the operation are invalid.</p>
    InvalidParameters(String),
    ///<p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListLaunchPathsError {
    pub fn from_body(body: &str) -> ListLaunchPathsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParametersException" => {
                        ListLaunchPathsError::InvalidParameters(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ListLaunchPathsError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListLaunchPathsError::Validation(error_message.to_string())
                    }
                    _ => ListLaunchPathsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListLaunchPathsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListLaunchPathsError {
    fn from(err: serde_json::error::Error) -> ListLaunchPathsError {
        ListLaunchPathsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListLaunchPathsError {
    fn from(err: CredentialsError) -> ListLaunchPathsError {
        ListLaunchPathsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListLaunchPathsError {
    fn from(err: HttpDispatchError) -> ListLaunchPathsError {
        ListLaunchPathsError::HttpDispatch(err)
    }
}
impl fmt::Display for ListLaunchPathsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListLaunchPathsError {
    fn description(&self) -> &str {
        match *self {
            ListLaunchPathsError::InvalidParameters(ref cause) => cause,
            ListLaunchPathsError::ResourceNotFound(ref cause) => cause,
            ListLaunchPathsError::Validation(ref cause) => cause,
            ListLaunchPathsError::Credentials(ref err) => err.description(),
            ListLaunchPathsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListLaunchPathsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListPortfolioAccess
#[derive(Debug, PartialEq)]
pub enum ListPortfolioAccessError {
    ///<p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListPortfolioAccessError {
    pub fn from_body(body: &str) -> ListPortfolioAccessError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        ListPortfolioAccessError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListPortfolioAccessError::Validation(error_message.to_string())
                    }
                    _ => ListPortfolioAccessError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListPortfolioAccessError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListPortfolioAccessError {
    fn from(err: serde_json::error::Error) -> ListPortfolioAccessError {
        ListPortfolioAccessError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListPortfolioAccessError {
    fn from(err: CredentialsError) -> ListPortfolioAccessError {
        ListPortfolioAccessError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListPortfolioAccessError {
    fn from(err: HttpDispatchError) -> ListPortfolioAccessError {
        ListPortfolioAccessError::HttpDispatch(err)
    }
}
impl fmt::Display for ListPortfolioAccessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListPortfolioAccessError {
    fn description(&self) -> &str {
        match *self {
            ListPortfolioAccessError::ResourceNotFound(ref cause) => cause,
            ListPortfolioAccessError::Validation(ref cause) => cause,
            ListPortfolioAccessError::Credentials(ref err) => err.description(),
            ListPortfolioAccessError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListPortfolioAccessError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListPortfolios
#[derive(Debug, PartialEq)]
pub enum ListPortfoliosError {
    ///<p>One or more parameters provided to the operation are invalid.</p>
    InvalidParameters(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListPortfoliosError {
    pub fn from_body(body: &str) -> ListPortfoliosError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParametersException" => {
                        ListPortfoliosError::InvalidParameters(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListPortfoliosError::Validation(error_message.to_string())
                    }
                    _ => ListPortfoliosError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListPortfoliosError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListPortfoliosError {
    fn from(err: serde_json::error::Error) -> ListPortfoliosError {
        ListPortfoliosError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListPortfoliosError {
    fn from(err: CredentialsError) -> ListPortfoliosError {
        ListPortfoliosError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListPortfoliosError {
    fn from(err: HttpDispatchError) -> ListPortfoliosError {
        ListPortfoliosError::HttpDispatch(err)
    }
}
impl fmt::Display for ListPortfoliosError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListPortfoliosError {
    fn description(&self) -> &str {
        match *self {
            ListPortfoliosError::InvalidParameters(ref cause) => cause,
            ListPortfoliosError::Validation(ref cause) => cause,
            ListPortfoliosError::Credentials(ref err) => err.description(),
            ListPortfoliosError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListPortfoliosError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListPortfoliosForProduct
#[derive(Debug, PartialEq)]
pub enum ListPortfoliosForProductError {
    ///<p>One or more parameters provided to the operation are invalid.</p>
    InvalidParameters(String),
    ///<p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListPortfoliosForProductError {
    pub fn from_body(body: &str) -> ListPortfoliosForProductError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParametersException" => ListPortfoliosForProductError::InvalidParameters(String::from(error_message)),
                    "ResourceNotFoundException" => {
                        ListPortfoliosForProductError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListPortfoliosForProductError::Validation(error_message.to_string())
                    }
                    _ => ListPortfoliosForProductError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListPortfoliosForProductError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListPortfoliosForProductError {
    fn from(err: serde_json::error::Error) -> ListPortfoliosForProductError {
        ListPortfoliosForProductError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListPortfoliosForProductError {
    fn from(err: CredentialsError) -> ListPortfoliosForProductError {
        ListPortfoliosForProductError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListPortfoliosForProductError {
    fn from(err: HttpDispatchError) -> ListPortfoliosForProductError {
        ListPortfoliosForProductError::HttpDispatch(err)
    }
}
impl fmt::Display for ListPortfoliosForProductError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListPortfoliosForProductError {
    fn description(&self) -> &str {
        match *self {
            ListPortfoliosForProductError::InvalidParameters(ref cause) => cause,
            ListPortfoliosForProductError::ResourceNotFound(ref cause) => cause,
            ListPortfoliosForProductError::Validation(ref cause) => cause,
            ListPortfoliosForProductError::Credentials(ref err) => err.description(),
            ListPortfoliosForProductError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListPortfoliosForProductError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListPrincipalsForPortfolio
#[derive(Debug, PartialEq)]
pub enum ListPrincipalsForPortfolioError {
    ///<p>One or more parameters provided to the operation are invalid.</p>
    InvalidParameters(String),
    ///<p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListPrincipalsForPortfolioError {
    pub fn from_body(body: &str) -> ListPrincipalsForPortfolioError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParametersException" => ListPrincipalsForPortfolioError::InvalidParameters(String::from(error_message)),
                    "ResourceNotFoundException" => ListPrincipalsForPortfolioError::ResourceNotFound(String::from(error_message)),
                    "ValidationException" => {
                        ListPrincipalsForPortfolioError::Validation(error_message.to_string())
                    }
                    _ => ListPrincipalsForPortfolioError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListPrincipalsForPortfolioError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListPrincipalsForPortfolioError {
    fn from(err: serde_json::error::Error) -> ListPrincipalsForPortfolioError {
        ListPrincipalsForPortfolioError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListPrincipalsForPortfolioError {
    fn from(err: CredentialsError) -> ListPrincipalsForPortfolioError {
        ListPrincipalsForPortfolioError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListPrincipalsForPortfolioError {
    fn from(err: HttpDispatchError) -> ListPrincipalsForPortfolioError {
        ListPrincipalsForPortfolioError::HttpDispatch(err)
    }
}
impl fmt::Display for ListPrincipalsForPortfolioError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListPrincipalsForPortfolioError {
    fn description(&self) -> &str {
        match *self {
            ListPrincipalsForPortfolioError::InvalidParameters(ref cause) => cause,
            ListPrincipalsForPortfolioError::ResourceNotFound(ref cause) => cause,
            ListPrincipalsForPortfolioError::Validation(ref cause) => cause,
            ListPrincipalsForPortfolioError::Credentials(ref err) => err.description(),
            ListPrincipalsForPortfolioError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListPrincipalsForPortfolioError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListProvisioningArtifacts
#[derive(Debug, PartialEq)]
pub enum ListProvisioningArtifactsError {
    ///<p>One or more parameters provided to the operation are invalid.</p>
    InvalidParameters(String),
    ///<p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListProvisioningArtifactsError {
    pub fn from_body(body: &str) -> ListProvisioningArtifactsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParametersException" => ListProvisioningArtifactsError::InvalidParameters(String::from(error_message)),
                    "ResourceNotFoundException" => ListProvisioningArtifactsError::ResourceNotFound(String::from(error_message)),
                    "ValidationException" => {
                        ListProvisioningArtifactsError::Validation(error_message.to_string())
                    }
                    _ => ListProvisioningArtifactsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListProvisioningArtifactsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListProvisioningArtifactsError {
    fn from(err: serde_json::error::Error) -> ListProvisioningArtifactsError {
        ListProvisioningArtifactsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListProvisioningArtifactsError {
    fn from(err: CredentialsError) -> ListProvisioningArtifactsError {
        ListProvisioningArtifactsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListProvisioningArtifactsError {
    fn from(err: HttpDispatchError) -> ListProvisioningArtifactsError {
        ListProvisioningArtifactsError::HttpDispatch(err)
    }
}
impl fmt::Display for ListProvisioningArtifactsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListProvisioningArtifactsError {
    fn description(&self) -> &str {
        match *self {
            ListProvisioningArtifactsError::InvalidParameters(ref cause) => cause,
            ListProvisioningArtifactsError::ResourceNotFound(ref cause) => cause,
            ListProvisioningArtifactsError::Validation(ref cause) => cause,
            ListProvisioningArtifactsError::Credentials(ref err) => err.description(),
            ListProvisioningArtifactsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListProvisioningArtifactsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListRecordHistory
#[derive(Debug, PartialEq)]
pub enum ListRecordHistoryError {
    ///<p>One or more parameters provided to the operation are invalid.</p>
    InvalidParameters(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListRecordHistoryError {
    pub fn from_body(body: &str) -> ListRecordHistoryError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParametersException" => {
                        ListRecordHistoryError::InvalidParameters(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListRecordHistoryError::Validation(error_message.to_string())
                    }
                    _ => ListRecordHistoryError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListRecordHistoryError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListRecordHistoryError {
    fn from(err: serde_json::error::Error) -> ListRecordHistoryError {
        ListRecordHistoryError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListRecordHistoryError {
    fn from(err: CredentialsError) -> ListRecordHistoryError {
        ListRecordHistoryError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListRecordHistoryError {
    fn from(err: HttpDispatchError) -> ListRecordHistoryError {
        ListRecordHistoryError::HttpDispatch(err)
    }
}
impl fmt::Display for ListRecordHistoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListRecordHistoryError {
    fn description(&self) -> &str {
        match *self {
            ListRecordHistoryError::InvalidParameters(ref cause) => cause,
            ListRecordHistoryError::Validation(ref cause) => cause,
            ListRecordHistoryError::Credentials(ref err) => err.description(),
            ListRecordHistoryError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListRecordHistoryError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ProvisionProduct
#[derive(Debug, PartialEq)]
pub enum ProvisionProductError {
    ///<p>The specified resource is a duplicate.</p>
    DuplicateResource(String),
    ///<p>One or more parameters provided to the operation are invalid.</p>
    InvalidParameters(String),
    ///<p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ProvisionProductError {
    pub fn from_body(body: &str) -> ProvisionProductError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DuplicateResourceException" => {
                        ProvisionProductError::DuplicateResource(String::from(error_message))
                    }
                    "InvalidParametersException" => {
                        ProvisionProductError::InvalidParameters(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ProvisionProductError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        ProvisionProductError::Validation(error_message.to_string())
                    }
                    _ => ProvisionProductError::Unknown(String::from(body)),
                }
            }
            Err(_) => ProvisionProductError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ProvisionProductError {
    fn from(err: serde_json::error::Error) -> ProvisionProductError {
        ProvisionProductError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ProvisionProductError {
    fn from(err: CredentialsError) -> ProvisionProductError {
        ProvisionProductError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ProvisionProductError {
    fn from(err: HttpDispatchError) -> ProvisionProductError {
        ProvisionProductError::HttpDispatch(err)
    }
}
impl fmt::Display for ProvisionProductError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ProvisionProductError {
    fn description(&self) -> &str {
        match *self {
            ProvisionProductError::DuplicateResource(ref cause) => cause,
            ProvisionProductError::InvalidParameters(ref cause) => cause,
            ProvisionProductError::ResourceNotFound(ref cause) => cause,
            ProvisionProductError::Validation(ref cause) => cause,
            ProvisionProductError::Credentials(ref err) => err.description(),
            ProvisionProductError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ProvisionProductError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RejectPortfolioShare
#[derive(Debug, PartialEq)]
pub enum RejectPortfolioShareError {
    ///<p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl RejectPortfolioShareError {
    pub fn from_body(body: &str) -> RejectPortfolioShareError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        RejectPortfolioShareError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        RejectPortfolioShareError::Validation(error_message.to_string())
                    }
                    _ => RejectPortfolioShareError::Unknown(String::from(body)),
                }
            }
            Err(_) => RejectPortfolioShareError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RejectPortfolioShareError {
    fn from(err: serde_json::error::Error) -> RejectPortfolioShareError {
        RejectPortfolioShareError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RejectPortfolioShareError {
    fn from(err: CredentialsError) -> RejectPortfolioShareError {
        RejectPortfolioShareError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RejectPortfolioShareError {
    fn from(err: HttpDispatchError) -> RejectPortfolioShareError {
        RejectPortfolioShareError::HttpDispatch(err)
    }
}
impl fmt::Display for RejectPortfolioShareError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RejectPortfolioShareError {
    fn description(&self) -> &str {
        match *self {
            RejectPortfolioShareError::ResourceNotFound(ref cause) => cause,
            RejectPortfolioShareError::Validation(ref cause) => cause,
            RejectPortfolioShareError::Credentials(ref err) => err.description(),
            RejectPortfolioShareError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RejectPortfolioShareError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ScanProvisionedProducts
#[derive(Debug, PartialEq)]
pub enum ScanProvisionedProductsError {
    ///<p>One or more parameters provided to the operation are invalid.</p>
    InvalidParameters(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ScanProvisionedProductsError {
    pub fn from_body(body: &str) -> ScanProvisionedProductsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParametersException" => {
                        ScanProvisionedProductsError::InvalidParameters(String::from(error_message))
                    }
                    "ValidationException" => {
                        ScanProvisionedProductsError::Validation(error_message.to_string())
                    }
                    _ => ScanProvisionedProductsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ScanProvisionedProductsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ScanProvisionedProductsError {
    fn from(err: serde_json::error::Error) -> ScanProvisionedProductsError {
        ScanProvisionedProductsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ScanProvisionedProductsError {
    fn from(err: CredentialsError) -> ScanProvisionedProductsError {
        ScanProvisionedProductsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ScanProvisionedProductsError {
    fn from(err: HttpDispatchError) -> ScanProvisionedProductsError {
        ScanProvisionedProductsError::HttpDispatch(err)
    }
}
impl fmt::Display for ScanProvisionedProductsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ScanProvisionedProductsError {
    fn description(&self) -> &str {
        match *self {
            ScanProvisionedProductsError::InvalidParameters(ref cause) => cause,
            ScanProvisionedProductsError::Validation(ref cause) => cause,
            ScanProvisionedProductsError::Credentials(ref err) => err.description(),
            ScanProvisionedProductsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ScanProvisionedProductsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SearchProducts
#[derive(Debug, PartialEq)]
pub enum SearchProductsError {
    ///<p>One or more parameters provided to the operation are invalid.</p>
    InvalidParameters(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl SearchProductsError {
    pub fn from_body(body: &str) -> SearchProductsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParametersException" => {
                        SearchProductsError::InvalidParameters(String::from(error_message))
                    }
                    "ValidationException" => {
                        SearchProductsError::Validation(error_message.to_string())
                    }
                    _ => SearchProductsError::Unknown(String::from(body)),
                }
            }
            Err(_) => SearchProductsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for SearchProductsError {
    fn from(err: serde_json::error::Error) -> SearchProductsError {
        SearchProductsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for SearchProductsError {
    fn from(err: CredentialsError) -> SearchProductsError {
        SearchProductsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SearchProductsError {
    fn from(err: HttpDispatchError) -> SearchProductsError {
        SearchProductsError::HttpDispatch(err)
    }
}
impl fmt::Display for SearchProductsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SearchProductsError {
    fn description(&self) -> &str {
        match *self {
            SearchProductsError::InvalidParameters(ref cause) => cause,
            SearchProductsError::Validation(ref cause) => cause,
            SearchProductsError::Credentials(ref err) => err.description(),
            SearchProductsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            SearchProductsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SearchProductsAsAdmin
#[derive(Debug, PartialEq)]
pub enum SearchProductsAsAdminError {
    ///<p>One or more parameters provided to the operation are invalid.</p>
    InvalidParameters(String),
    ///<p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl SearchProductsAsAdminError {
    pub fn from_body(body: &str) -> SearchProductsAsAdminError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParametersException" => {
                        SearchProductsAsAdminError::InvalidParameters(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        SearchProductsAsAdminError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        SearchProductsAsAdminError::Validation(error_message.to_string())
                    }
                    _ => SearchProductsAsAdminError::Unknown(String::from(body)),
                }
            }
            Err(_) => SearchProductsAsAdminError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for SearchProductsAsAdminError {
    fn from(err: serde_json::error::Error) -> SearchProductsAsAdminError {
        SearchProductsAsAdminError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for SearchProductsAsAdminError {
    fn from(err: CredentialsError) -> SearchProductsAsAdminError {
        SearchProductsAsAdminError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SearchProductsAsAdminError {
    fn from(err: HttpDispatchError) -> SearchProductsAsAdminError {
        SearchProductsAsAdminError::HttpDispatch(err)
    }
}
impl fmt::Display for SearchProductsAsAdminError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SearchProductsAsAdminError {
    fn description(&self) -> &str {
        match *self {
            SearchProductsAsAdminError::InvalidParameters(ref cause) => cause,
            SearchProductsAsAdminError::ResourceNotFound(ref cause) => cause,
            SearchProductsAsAdminError::Validation(ref cause) => cause,
            SearchProductsAsAdminError::Credentials(ref err) => err.description(),
            SearchProductsAsAdminError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            SearchProductsAsAdminError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by TerminateProvisionedProduct
#[derive(Debug, PartialEq)]
pub enum TerminateProvisionedProductError {
    ///<p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl TerminateProvisionedProductError {
    pub fn from_body(body: &str) -> TerminateProvisionedProductError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => TerminateProvisionedProductError::ResourceNotFound(String::from(error_message)),
                    "ValidationException" => {
                        TerminateProvisionedProductError::Validation(error_message.to_string())
                    }
                    _ => TerminateProvisionedProductError::Unknown(String::from(body)),
                }
            }
            Err(_) => TerminateProvisionedProductError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for TerminateProvisionedProductError {
    fn from(err: serde_json::error::Error) -> TerminateProvisionedProductError {
        TerminateProvisionedProductError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for TerminateProvisionedProductError {
    fn from(err: CredentialsError) -> TerminateProvisionedProductError {
        TerminateProvisionedProductError::Credentials(err)
    }
}
impl From<HttpDispatchError> for TerminateProvisionedProductError {
    fn from(err: HttpDispatchError) -> TerminateProvisionedProductError {
        TerminateProvisionedProductError::HttpDispatch(err)
    }
}
impl fmt::Display for TerminateProvisionedProductError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for TerminateProvisionedProductError {
    fn description(&self) -> &str {
        match *self {
            TerminateProvisionedProductError::ResourceNotFound(ref cause) => cause,
            TerminateProvisionedProductError::Validation(ref cause) => cause,
            TerminateProvisionedProductError::Credentials(ref err) => err.description(),
            TerminateProvisionedProductError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            TerminateProvisionedProductError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateConstraint
#[derive(Debug, PartialEq)]
pub enum UpdateConstraintError {
    ///<p>One or more parameters provided to the operation are invalid.</p>
    InvalidParameters(String),
    ///<p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateConstraintError {
    pub fn from_body(body: &str) -> UpdateConstraintError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParametersException" => {
                        UpdateConstraintError::InvalidParameters(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        UpdateConstraintError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateConstraintError::Validation(error_message.to_string())
                    }
                    _ => UpdateConstraintError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateConstraintError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateConstraintError {
    fn from(err: serde_json::error::Error) -> UpdateConstraintError {
        UpdateConstraintError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateConstraintError {
    fn from(err: CredentialsError) -> UpdateConstraintError {
        UpdateConstraintError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateConstraintError {
    fn from(err: HttpDispatchError) -> UpdateConstraintError {
        UpdateConstraintError::HttpDispatch(err)
    }
}
impl fmt::Display for UpdateConstraintError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateConstraintError {
    fn description(&self) -> &str {
        match *self {
            UpdateConstraintError::InvalidParameters(ref cause) => cause,
            UpdateConstraintError::ResourceNotFound(ref cause) => cause,
            UpdateConstraintError::Validation(ref cause) => cause,
            UpdateConstraintError::Credentials(ref err) => err.description(),
            UpdateConstraintError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateConstraintError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdatePortfolio
#[derive(Debug, PartialEq)]
pub enum UpdatePortfolioError {
    ///<p>One or more parameters provided to the operation are invalid.</p>
    InvalidParameters(String),
    ///<p>The current limits of the service would have been exceeded by this operation. Reduce the resource use or increase the service limits and retry the operation.</p>
    LimitExceeded(String),
    ///<p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdatePortfolioError {
    pub fn from_body(body: &str) -> UpdatePortfolioError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParametersException" => {
                        UpdatePortfolioError::InvalidParameters(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        UpdatePortfolioError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        UpdatePortfolioError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdatePortfolioError::Validation(error_message.to_string())
                    }
                    _ => UpdatePortfolioError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdatePortfolioError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdatePortfolioError {
    fn from(err: serde_json::error::Error) -> UpdatePortfolioError {
        UpdatePortfolioError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdatePortfolioError {
    fn from(err: CredentialsError) -> UpdatePortfolioError {
        UpdatePortfolioError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdatePortfolioError {
    fn from(err: HttpDispatchError) -> UpdatePortfolioError {
        UpdatePortfolioError::HttpDispatch(err)
    }
}
impl fmt::Display for UpdatePortfolioError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdatePortfolioError {
    fn description(&self) -> &str {
        match *self {
            UpdatePortfolioError::InvalidParameters(ref cause) => cause,
            UpdatePortfolioError::LimitExceeded(ref cause) => cause,
            UpdatePortfolioError::ResourceNotFound(ref cause) => cause,
            UpdatePortfolioError::Validation(ref cause) => cause,
            UpdatePortfolioError::Credentials(ref err) => err.description(),
            UpdatePortfolioError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdatePortfolioError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateProduct
#[derive(Debug, PartialEq)]
pub enum UpdateProductError {
    ///<p>One or more parameters provided to the operation are invalid.</p>
    InvalidParameters(String),
    ///<p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateProductError {
    pub fn from_body(body: &str) -> UpdateProductError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParametersException" => {
                        UpdateProductError::InvalidParameters(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        UpdateProductError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateProductError::Validation(error_message.to_string())
                    }
                    _ => UpdateProductError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateProductError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateProductError {
    fn from(err: serde_json::error::Error) -> UpdateProductError {
        UpdateProductError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateProductError {
    fn from(err: CredentialsError) -> UpdateProductError {
        UpdateProductError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateProductError {
    fn from(err: HttpDispatchError) -> UpdateProductError {
        UpdateProductError::HttpDispatch(err)
    }
}
impl fmt::Display for UpdateProductError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateProductError {
    fn description(&self) -> &str {
        match *self {
            UpdateProductError::InvalidParameters(ref cause) => cause,
            UpdateProductError::ResourceNotFound(ref cause) => cause,
            UpdateProductError::Validation(ref cause) => cause,
            UpdateProductError::Credentials(ref err) => err.description(),
            UpdateProductError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateProductError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateProvisionedProduct
#[derive(Debug, PartialEq)]
pub enum UpdateProvisionedProductError {
    ///<p>One or more parameters provided to the operation are invalid.</p>
    InvalidParameters(String),
    ///<p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateProvisionedProductError {
    pub fn from_body(body: &str) -> UpdateProvisionedProductError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParametersException" => UpdateProvisionedProductError::InvalidParameters(String::from(error_message)),
                    "ResourceNotFoundException" => {
                        UpdateProvisionedProductError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateProvisionedProductError::Validation(error_message.to_string())
                    }
                    _ => UpdateProvisionedProductError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateProvisionedProductError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateProvisionedProductError {
    fn from(err: serde_json::error::Error) -> UpdateProvisionedProductError {
        UpdateProvisionedProductError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateProvisionedProductError {
    fn from(err: CredentialsError) -> UpdateProvisionedProductError {
        UpdateProvisionedProductError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateProvisionedProductError {
    fn from(err: HttpDispatchError) -> UpdateProvisionedProductError {
        UpdateProvisionedProductError::HttpDispatch(err)
    }
}
impl fmt::Display for UpdateProvisionedProductError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateProvisionedProductError {
    fn description(&self) -> &str {
        match *self {
            UpdateProvisionedProductError::InvalidParameters(ref cause) => cause,
            UpdateProvisionedProductError::ResourceNotFound(ref cause) => cause,
            UpdateProvisionedProductError::Validation(ref cause) => cause,
            UpdateProvisionedProductError::Credentials(ref err) => err.description(),
            UpdateProvisionedProductError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateProvisionedProductError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateProvisioningArtifact
#[derive(Debug, PartialEq)]
pub enum UpdateProvisioningArtifactError {
    ///<p>One or more parameters provided to the operation are invalid.</p>
    InvalidParameters(String),
    ///<p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateProvisioningArtifactError {
    pub fn from_body(body: &str) -> UpdateProvisioningArtifactError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParametersException" => UpdateProvisioningArtifactError::InvalidParameters(String::from(error_message)),
                    "ResourceNotFoundException" => UpdateProvisioningArtifactError::ResourceNotFound(String::from(error_message)),
                    "ValidationException" => {
                        UpdateProvisioningArtifactError::Validation(error_message.to_string())
                    }
                    _ => UpdateProvisioningArtifactError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateProvisioningArtifactError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateProvisioningArtifactError {
    fn from(err: serde_json::error::Error) -> UpdateProvisioningArtifactError {
        UpdateProvisioningArtifactError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateProvisioningArtifactError {
    fn from(err: CredentialsError) -> UpdateProvisioningArtifactError {
        UpdateProvisioningArtifactError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateProvisioningArtifactError {
    fn from(err: HttpDispatchError) -> UpdateProvisioningArtifactError {
        UpdateProvisioningArtifactError::HttpDispatch(err)
    }
}
impl fmt::Display for UpdateProvisioningArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateProvisioningArtifactError {
    fn description(&self) -> &str {
        match *self {
            UpdateProvisioningArtifactError::InvalidParameters(ref cause) => cause,
            UpdateProvisioningArtifactError::ResourceNotFound(ref cause) => cause,
            UpdateProvisioningArtifactError::Validation(ref cause) => cause,
            UpdateProvisioningArtifactError::Credentials(ref err) => err.description(),
            UpdateProvisioningArtifactError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateProvisioningArtifactError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the AWS Service Catalog API. AWS Service Catalog clients implement this trait.
pub trait ServiceCatalog {
    #[doc="<p>Accepts an offer to share a portfolio.</p>"]
    fn accept_portfolio_share(&self,
                              input: &AcceptPortfolioShareInput)
                              -> Result<AcceptPortfolioShareOutput, AcceptPortfolioShareError>;


    #[doc="<p>Associates the specified principal ARN with the specified portfolio.</p>"]
    fn associate_principal_with_portfolio
        (&self,
         input: &AssociatePrincipalWithPortfolioInput)
         -> Result<AssociatePrincipalWithPortfolioOutput, AssociatePrincipalWithPortfolioError>;


    #[doc="<p>Associates a product with a portfolio.</p>"]
    fn associate_product_with_portfolio
        (&self,
         input: &AssociateProductWithPortfolioInput)
         -> Result<AssociateProductWithPortfolioOutput, AssociateProductWithPortfolioError>;


    #[doc="<p>Creates a new constraint. For more information, see <a href=\"http://docs.aws.amazon.com/servicecatalog/latest/adminguide/constraints.html\">Using Constraints</a>.</p>"]
    fn create_constraint(&self,
                         input: &CreateConstraintInput)
                         -> Result<CreateConstraintOutput, CreateConstraintError>;


    #[doc="<p>Creates a new portfolio.</p>"]
    fn create_portfolio(&self,
                        input: &CreatePortfolioInput)
                        -> Result<CreatePortfolioOutput, CreatePortfolioError>;


    #[doc="<p>Creates a new portfolio share.</p>"]
    fn create_portfolio_share(&self,
                              input: &CreatePortfolioShareInput)
                              -> Result<CreatePortfolioShareOutput, CreatePortfolioShareError>;


    #[doc="<p>Creates a new product.</p>"]
    fn create_product(&self,
                      input: &CreateProductInput)
                      -> Result<CreateProductOutput, CreateProductError>;


    #[doc="<p>Create a new provisioning artifact for the specified product. This operation does not work with a product that has been shared with you.</p> <p>See the bottom of this topic for an example JSON request.</p>"]
    fn create_provisioning_artifact
        (&self,
         input: &CreateProvisioningArtifactInput)
         -> Result<CreateProvisioningArtifactOutput, CreateProvisioningArtifactError>;


    #[doc="<p>Deletes the specified constraint.</p>"]
    fn delete_constraint(&self,
                         input: &DeleteConstraintInput)
                         -> Result<DeleteConstraintOutput, DeleteConstraintError>;


    #[doc="<p>Deletes the specified portfolio. This operation does not work with a portfolio that has been shared with you or if it has products, users, constraints, or shared accounts associated with it.</p>"]
    fn delete_portfolio(&self,
                        input: &DeletePortfolioInput)
                        -> Result<DeletePortfolioOutput, DeletePortfolioError>;


    #[doc="<p>Deletes the specified portfolio share.</p>"]
    fn delete_portfolio_share(&self,
                              input: &DeletePortfolioShareInput)
                              -> Result<DeletePortfolioShareOutput, DeletePortfolioShareError>;


    #[doc="<p>Deletes the specified product. This operation does not work with a product that has been shared with you or is associated with a portfolio. </p>"]
    fn delete_product(&self,
                      input: &DeleteProductInput)
                      -> Result<DeleteProductOutput, DeleteProductError>;


    #[doc="<p>Deletes the specified provisioning artifact. This operation does not work on a provisioning artifact associated with a product that has been shared with you, or on the last provisioning artifact associated with a product (a product must have at least one provisioning artifact).</p>"]
    fn delete_provisioning_artifact
        (&self,
         input: &DeleteProvisioningArtifactInput)
         -> Result<DeleteProvisioningArtifactOutput, DeleteProvisioningArtifactError>;


    #[doc="<p>Retrieves detailed information for a specified constraint.</p>"]
    fn describe_constraint(&self,
                           input: &DescribeConstraintInput)
                           -> Result<DescribeConstraintOutput, DescribeConstraintError>;


    #[doc="<p>Retrieves detailed information and any tags associated with the specified portfolio.</p>"]
    fn describe_portfolio(&self,
                          input: &DescribePortfolioInput)
                          -> Result<DescribePortfolioOutput, DescribePortfolioError>;


    #[doc="<p>Retrieves information about a specified product.</p> <p>This operation is functionally identical to <a>DescribeProductView</a> except that it takes as input <code>ProductId</code> instead of <code>ProductViewId</code>.</p>"]
    fn describe_product(&self,
                        input: &DescribeProductInput)
                        -> Result<DescribeProductOutput, DescribeProductError>;


    #[doc="<p>Retrieves information about a specified product, run with administrator access.</p>"]
    fn describe_product_as_admin
        (&self,
         input: &DescribeProductAsAdminInput)
         -> Result<DescribeProductAsAdminOutput, DescribeProductAsAdminError>;


    #[doc="<p>Retrieves information about a specified product.</p> <p>This operation is functionally identical to <a>DescribeProduct</a> except that it takes as input <code>ProductViewId</code> instead of <code>ProductId</code>.</p>"]
    fn describe_product_view(&self,
                             input: &DescribeProductViewInput)
                             -> Result<DescribeProductViewOutput, DescribeProductViewError>;


    #[doc="<p>Retrieve detailed information about the provisioned product.</p>"]
    fn describe_provisioned_product
        (&self,
         input: &DescribeProvisionedProductInput)
         -> Result<DescribeProvisionedProductOutput, DescribeProvisionedProductError>;


    #[doc="<p>Retrieves detailed information about the specified provisioning artifact.</p>"]
    fn describe_provisioning_artifact
        (&self,
         input: &DescribeProvisioningArtifactInput)
         -> Result<DescribeProvisioningArtifactOutput, DescribeProvisioningArtifactError>;


    #[doc="<p>Provides information about parameters required to provision a specified product in a specified manner. Use this operation to obtain the list of <code>ProvisioningArtifactParameters</code> parameters available to call the <a>ProvisionProduct</a> operation for the specified product.</p>"]
    fn describe_provisioning_parameters
        (&self,
         input: &DescribeProvisioningParametersInput)
         -> Result<DescribeProvisioningParametersOutput, DescribeProvisioningParametersError>;


    #[doc="<p>Retrieves a paginated list of the full details of a specific request. Use this operation after calling a request operation (<a>ProvisionProduct</a>, <a>TerminateProvisionedProduct</a>, or <a>UpdateProvisionedProduct</a>). </p>"]
    fn describe_record(&self,
                       input: &DescribeRecordInput)
                       -> Result<DescribeRecordOutput, DescribeRecordError>;


    #[doc="<p>Disassociates a previously associated principal ARN from a specified portfolio.</p>"]
    fn disassociate_principal_from_portfolio
        (&self,
         input: &DisassociatePrincipalFromPortfolioInput)
         -> Result<DisassociatePrincipalFromPortfolioOutput,
                   DisassociatePrincipalFromPortfolioError>;


    #[doc="<p>Disassociates the specified product from the specified portfolio. </p>"]
    fn disassociate_product_from_portfolio
        (&self,
         input: &DisassociateProductFromPortfolioInput)
         -> Result<DisassociateProductFromPortfolioOutput, DisassociateProductFromPortfolioError>;


    #[doc="<p>Lists details of all portfolios for which sharing was accepted by this account.</p>"]
    fn list_accepted_portfolio_shares
        (&self,
         input: &ListAcceptedPortfolioSharesInput)
         -> Result<ListAcceptedPortfolioSharesOutput, ListAcceptedPortfolioSharesError>;


    #[doc="<p>Retrieves detailed constraint information for the specified portfolio and product.</p>"]
    fn list_constraints_for_portfolio
        (&self,
         input: &ListConstraintsForPortfolioInput)
         -> Result<ListConstraintsForPortfolioOutput, ListConstraintsForPortfolioError>;


    #[doc="<p>Returns a paginated list of all paths to a specified product. A path is how the user has access to a specified product, and is necessary when provisioning a product. A path also determines the constraints put on the product.</p>"]
    fn list_launch_paths(&self,
                         input: &ListLaunchPathsInput)
                         -> Result<ListLaunchPathsOutput, ListLaunchPathsError>;


    #[doc="<p>Lists the account IDs that have been authorized sharing of the specified portfolio.</p>"]
    fn list_portfolio_access(&self,
                             input: &ListPortfolioAccessInput)
                             -> Result<ListPortfolioAccessOutput, ListPortfolioAccessError>;


    #[doc="<p>Lists all portfolios in the catalog.</p>"]
    fn list_portfolios(&self,
                       input: &ListPortfoliosInput)
                       -> Result<ListPortfoliosOutput, ListPortfoliosError>;


    #[doc="<p>Lists all portfolios that the specified product is associated with.</p>"]
    fn list_portfolios_for_product
        (&self,
         input: &ListPortfoliosForProductInput)
         -> Result<ListPortfoliosForProductOutput, ListPortfoliosForProductError>;


    #[doc="<p>Lists all principal ARNs associated with the specified portfolio.</p>"]
    fn list_principals_for_portfolio
        (&self,
         input: &ListPrincipalsForPortfolioInput)
         -> Result<ListPrincipalsForPortfolioOutput, ListPrincipalsForPortfolioError>;


    #[doc="<p>Lists all provisioning artifacts associated with the specified product.</p>"]
    fn list_provisioning_artifacts
        (&self,
         input: &ListProvisioningArtifactsInput)
         -> Result<ListProvisioningArtifactsOutput, ListProvisioningArtifactsError>;


    #[doc="<p>Returns a paginated list of all performed requests, in the form of RecordDetails objects that are filtered as specified.</p>"]
    fn list_record_history(&self,
                           input: &ListRecordHistoryInput)
                           -> Result<ListRecordHistoryOutput, ListRecordHistoryError>;


    #[doc="<p>Requests a <i>provision</i> of a specified product. A <i>provisioned product</i> is a resourced instance for a product. For example, provisioning a CloudFormation-template-backed product results in launching a CloudFormation stack and all the underlying resources that come with it. </p> <p>You can check the status of this request using the <a>DescribeRecord</a> operation.</p>"]
    fn provision_product(&self,
                         input: &ProvisionProductInput)
                         -> Result<ProvisionProductOutput, ProvisionProductError>;


    #[doc="<p>Rejects an offer to share a portfolio.</p>"]
    fn reject_portfolio_share(&self,
                              input: &RejectPortfolioShareInput)
                              -> Result<RejectPortfolioShareOutput, RejectPortfolioShareError>;


    #[doc="<p>Returns a paginated list of all the ProvisionedProduct objects that are currently available (not terminated). </p>"]
    fn scan_provisioned_products
        (&self,
         input: &ScanProvisionedProductsInput)
         -> Result<ScanProvisionedProductsOutput, ScanProvisionedProductsError>;


    #[doc="<p>Returns a paginated list all of the <code>Products</code> objects to which the caller has access. </p> <p>The output of this operation can be used as input for other operations, such as <a>DescribeProductView</a>.</p>"]
    fn search_products(&self,
                       input: &SearchProductsInput)
                       -> Result<SearchProductsOutput, SearchProductsError>;


    #[doc="<p>Retrieves summary and status information about all products created within the caller's account. If a portfolio ID is provided, this operation retrieves information for only those products that are associated with the specified portfolio.</p>"]
    fn search_products_as_admin
        (&self,
         input: &SearchProductsAsAdminInput)
         -> Result<SearchProductsAsAdminOutput, SearchProductsAsAdminError>;


    #[doc="<p>Requests termination of an existing ProvisionedProduct object. If there are <code>Tags</code> associated with the object, they are terminated when the ProvisionedProduct object is terminated. </p> <p>This operation does not delete any records associated with the ProvisionedProduct object.</p> <p>You can check the status of this request using the <a>DescribeRecord</a> operation.</p>"]
    fn terminate_provisioned_product
        (&self,
         input: &TerminateProvisionedProductInput)
         -> Result<TerminateProvisionedProductOutput, TerminateProvisionedProductError>;


    #[doc="<p>Updates an existing constraint.</p>"]
    fn update_constraint(&self,
                         input: &UpdateConstraintInput)
                         -> Result<UpdateConstraintOutput, UpdateConstraintError>;


    #[doc="<p>Updates the specified portfolio's details. This operation does not work with a product that has been shared with you.</p>"]
    fn update_portfolio(&self,
                        input: &UpdatePortfolioInput)
                        -> Result<UpdatePortfolioOutput, UpdatePortfolioError>;


    #[doc="<p>Updates an existing product.</p>"]
    fn update_product(&self,
                      input: &UpdateProductInput)
                      -> Result<UpdateProductOutput, UpdateProductError>;


    #[doc="<p>Requests updates to the configuration of an existing ProvisionedProduct object. If there are tags associated with the object, they cannot be updated or added with this operation. Depending on the specific updates requested, this operation may update with no interruption, with some interruption, or replace the ProvisionedProduct object entirely. </p> <p>You can check the status of this request using the <a>DescribeRecord</a> operation.</p>"]
    fn update_provisioned_product
        (&self,
         input: &UpdateProvisionedProductInput)
         -> Result<UpdateProvisionedProductOutput, UpdateProvisionedProductError>;


    #[doc="<p>Updates an existing provisioning artifact's information. This operation does not work on a provisioning artifact associated with a product that has been shared with you.</p>"]
    fn update_provisioning_artifact
        (&self,
         input: &UpdateProvisioningArtifactInput)
         -> Result<UpdateProvisioningArtifactOutput, UpdateProvisioningArtifactError>;
}
/// A client for the AWS Service Catalog API.
pub struct ServiceCatalogClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    credentials_provider: P,
    region: region::Region,
    dispatcher: D,
}

impl<P, D> ServiceCatalogClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        ServiceCatalogClient {
            credentials_provider: credentials_provider,
            region: region,
            dispatcher: request_dispatcher,
        }
    }
}

impl<P, D> ServiceCatalog for ServiceCatalogClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    #[doc="<p>Accepts an offer to share a portfolio.</p>"]
    fn accept_portfolio_share(&self,
                              input: &AcceptPortfolioShareInput)
                              -> Result<AcceptPortfolioShareOutput, AcceptPortfolioShareError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWS242ServiceCatalogService.AcceptPortfolioShare");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<AcceptPortfolioShareOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(AcceptPortfolioShareError::from_body(String::from_utf8_lossy(&response.body)
                                                             .as_ref()))
            }
        }
    }


    #[doc="<p>Associates the specified principal ARN with the specified portfolio.</p>"]
    fn associate_principal_with_portfolio
        (&self,
         input: &AssociatePrincipalWithPortfolioInput)
         -> Result<AssociatePrincipalWithPortfolioOutput, AssociatePrincipalWithPortfolioError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWS242ServiceCatalogService.AssociatePrincipalWithPortfolio");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<AssociatePrincipalWithPortfolioOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(AssociatePrincipalWithPortfolioError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Associates a product with a portfolio.</p>"]
    fn associate_product_with_portfolio
        (&self,
         input: &AssociateProductWithPortfolioInput)
         -> Result<AssociateProductWithPortfolioOutput, AssociateProductWithPortfolioError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWS242ServiceCatalogService.AssociateProductWithPortfolio");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<AssociateProductWithPortfolioOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(AssociateProductWithPortfolioError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Creates a new constraint. For more information, see <a href=\"http://docs.aws.amazon.com/servicecatalog/latest/adminguide/constraints.html\">Using Constraints</a>.</p>"]
    fn create_constraint(&self,
                         input: &CreateConstraintInput)
                         -> Result<CreateConstraintOutput, CreateConstraintError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWS242ServiceCatalogService.CreateConstraint");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<CreateConstraintOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(CreateConstraintError::from_body(String::from_utf8_lossy(&response.body)
                                                         .as_ref()))
            }
        }
    }


    #[doc="<p>Creates a new portfolio.</p>"]
    fn create_portfolio(&self,
                        input: &CreatePortfolioInput)
                        -> Result<CreatePortfolioOutput, CreatePortfolioError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWS242ServiceCatalogService.CreatePortfolio");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<CreatePortfolioOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(CreatePortfolioError::from_body(String::from_utf8_lossy(&response.body)
                                                        .as_ref()))
            }
        }
    }


    #[doc="<p>Creates a new portfolio share.</p>"]
    fn create_portfolio_share(&self,
                              input: &CreatePortfolioShareInput)
                              -> Result<CreatePortfolioShareOutput, CreatePortfolioShareError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWS242ServiceCatalogService.CreatePortfolioShare");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<CreatePortfolioShareOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(CreatePortfolioShareError::from_body(String::from_utf8_lossy(&response.body)
                                                             .as_ref()))
            }
        }
    }


    #[doc="<p>Creates a new product.</p>"]
    fn create_product(&self,
                      input: &CreateProductInput)
                      -> Result<CreateProductOutput, CreateProductError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWS242ServiceCatalogService.CreateProduct");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<CreateProductOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(CreateProductError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
            }
        }
    }


    #[doc="<p>Create a new provisioning artifact for the specified product. This operation does not work with a product that has been shared with you.</p> <p>See the bottom of this topic for an example JSON request.</p>"]
    fn create_provisioning_artifact
        (&self,
         input: &CreateProvisioningArtifactInput)
         -> Result<CreateProvisioningArtifactOutput, CreateProvisioningArtifactError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWS242ServiceCatalogService.CreateProvisioningArtifact");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<CreateProvisioningArtifactOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(CreateProvisioningArtifactError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Deletes the specified constraint.</p>"]
    fn delete_constraint(&self,
                         input: &DeleteConstraintInput)
                         -> Result<DeleteConstraintOutput, DeleteConstraintError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWS242ServiceCatalogService.DeleteConstraint");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DeleteConstraintOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DeleteConstraintError::from_body(String::from_utf8_lossy(&response.body)
                                                         .as_ref()))
            }
        }
    }


    #[doc="<p>Deletes the specified portfolio. This operation does not work with a portfolio that has been shared with you or if it has products, users, constraints, or shared accounts associated with it.</p>"]
    fn delete_portfolio(&self,
                        input: &DeletePortfolioInput)
                        -> Result<DeletePortfolioOutput, DeletePortfolioError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWS242ServiceCatalogService.DeletePortfolio");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DeletePortfolioOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DeletePortfolioError::from_body(String::from_utf8_lossy(&response.body)
                                                        .as_ref()))
            }
        }
    }


    #[doc="<p>Deletes the specified portfolio share.</p>"]
    fn delete_portfolio_share(&self,
                              input: &DeletePortfolioShareInput)
                              -> Result<DeletePortfolioShareOutput, DeletePortfolioShareError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWS242ServiceCatalogService.DeletePortfolioShare");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DeletePortfolioShareOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DeletePortfolioShareError::from_body(String::from_utf8_lossy(&response.body)
                                                             .as_ref()))
            }
        }
    }


    #[doc="<p>Deletes the specified product. This operation does not work with a product that has been shared with you or is associated with a portfolio. </p>"]
    fn delete_product(&self,
                      input: &DeleteProductInput)
                      -> Result<DeleteProductOutput, DeleteProductError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWS242ServiceCatalogService.DeleteProduct");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DeleteProductOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DeleteProductError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
            }
        }
    }


    #[doc="<p>Deletes the specified provisioning artifact. This operation does not work on a provisioning artifact associated with a product that has been shared with you, or on the last provisioning artifact associated with a product (a product must have at least one provisioning artifact).</p>"]
    fn delete_provisioning_artifact
        (&self,
         input: &DeleteProvisioningArtifactInput)
         -> Result<DeleteProvisioningArtifactOutput, DeleteProvisioningArtifactError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWS242ServiceCatalogService.DeleteProvisioningArtifact");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DeleteProvisioningArtifactOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(DeleteProvisioningArtifactError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Retrieves detailed information for a specified constraint.</p>"]
    fn describe_constraint(&self,
                           input: &DescribeConstraintInput)
                           -> Result<DescribeConstraintOutput, DescribeConstraintError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWS242ServiceCatalogService.DescribeConstraint");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeConstraintOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DescribeConstraintError::from_body(String::from_utf8_lossy(&response.body)
                                                           .as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves detailed information and any tags associated with the specified portfolio.</p>"]
    fn describe_portfolio(&self,
                          input: &DescribePortfolioInput)
                          -> Result<DescribePortfolioOutput, DescribePortfolioError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWS242ServiceCatalogService.DescribePortfolio");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribePortfolioOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DescribePortfolioError::from_body(String::from_utf8_lossy(&response.body)
                                                          .as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves information about a specified product.</p> <p>This operation is functionally identical to <a>DescribeProductView</a> except that it takes as input <code>ProductId</code> instead of <code>ProductViewId</code>.</p>"]
    fn describe_product(&self,
                        input: &DescribeProductInput)
                        -> Result<DescribeProductOutput, DescribeProductError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWS242ServiceCatalogService.DescribeProduct");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeProductOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DescribeProductError::from_body(String::from_utf8_lossy(&response.body)
                                                        .as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves information about a specified product, run with administrator access.</p>"]
    fn describe_product_as_admin
        (&self,
         input: &DescribeProductAsAdminInput)
         -> Result<DescribeProductAsAdminOutput, DescribeProductAsAdminError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWS242ServiceCatalogService.DescribeProductAsAdmin");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeProductAsAdminOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DescribeProductAsAdminError::from_body(String::from_utf8_lossy(&response.body)
                                                               .as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves information about a specified product.</p> <p>This operation is functionally identical to <a>DescribeProduct</a> except that it takes as input <code>ProductViewId</code> instead of <code>ProductId</code>.</p>"]
    fn describe_product_view(&self,
                             input: &DescribeProductViewInput)
                             -> Result<DescribeProductViewOutput, DescribeProductViewError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWS242ServiceCatalogService.DescribeProductView");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeProductViewOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DescribeProductViewError::from_body(String::from_utf8_lossy(&response.body)
                                                            .as_ref()))
            }
        }
    }


    #[doc="<p>Retrieve detailed information about the provisioned product.</p>"]
    fn describe_provisioned_product
        (&self,
         input: &DescribeProvisionedProductInput)
         -> Result<DescribeProvisionedProductOutput, DescribeProvisionedProductError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWS242ServiceCatalogService.DescribeProvisionedProduct");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeProvisionedProductOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(DescribeProvisionedProductError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Retrieves detailed information about the specified provisioning artifact.</p>"]
    fn describe_provisioning_artifact
        (&self,
         input: &DescribeProvisioningArtifactInput)
         -> Result<DescribeProvisioningArtifactOutput, DescribeProvisioningArtifactError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWS242ServiceCatalogService.DescribeProvisioningArtifact");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeProvisioningArtifactOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(DescribeProvisioningArtifactError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Provides information about parameters required to provision a specified product in a specified manner. Use this operation to obtain the list of <code>ProvisioningArtifactParameters</code> parameters available to call the <a>ProvisionProduct</a> operation for the specified product.</p>"]
    fn describe_provisioning_parameters
        (&self,
         input: &DescribeProvisioningParametersInput)
         -> Result<DescribeProvisioningParametersOutput, DescribeProvisioningParametersError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWS242ServiceCatalogService.DescribeProvisioningParameters");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeProvisioningParametersOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(DescribeProvisioningParametersError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Retrieves a paginated list of the full details of a specific request. Use this operation after calling a request operation (<a>ProvisionProduct</a>, <a>TerminateProvisionedProduct</a>, or <a>UpdateProvisionedProduct</a>). </p>"]
    fn describe_record(&self,
                       input: &DescribeRecordInput)
                       -> Result<DescribeRecordOutput, DescribeRecordError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWS242ServiceCatalogService.DescribeRecord");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeRecordOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DescribeRecordError::from_body(String::from_utf8_lossy(&response.body)
                                                       .as_ref()))
            }
        }
    }


    #[doc="<p>Disassociates a previously associated principal ARN from a specified portfolio.</p>"]
    fn disassociate_principal_from_portfolio
        (&self,
         input: &DisassociatePrincipalFromPortfolioInput)
         -> Result<DisassociatePrincipalFromPortfolioOutput,
                   DisassociatePrincipalFromPortfolioError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWS242ServiceCatalogService.DisassociatePrincipalFromPortfolio");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DisassociatePrincipalFromPortfolioOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(DisassociatePrincipalFromPortfolioError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Disassociates the specified product from the specified portfolio. </p>"]
    fn disassociate_product_from_portfolio
        (&self,
         input: &DisassociateProductFromPortfolioInput)
         -> Result<DisassociateProductFromPortfolioOutput, DisassociateProductFromPortfolioError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWS242ServiceCatalogService.DisassociateProductFromPortfolio");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DisassociateProductFromPortfolioOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(DisassociateProductFromPortfolioError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Lists details of all portfolios for which sharing was accepted by this account.</p>"]
    fn list_accepted_portfolio_shares
        (&self,
         input: &ListAcceptedPortfolioSharesInput)
         -> Result<ListAcceptedPortfolioSharesOutput, ListAcceptedPortfolioSharesError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWS242ServiceCatalogService.ListAcceptedPortfolioShares");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListAcceptedPortfolioSharesOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(ListAcceptedPortfolioSharesError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Retrieves detailed constraint information for the specified portfolio and product.</p>"]
    fn list_constraints_for_portfolio
        (&self,
         input: &ListConstraintsForPortfolioInput)
         -> Result<ListConstraintsForPortfolioOutput, ListConstraintsForPortfolioError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWS242ServiceCatalogService.ListConstraintsForPortfolio");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListConstraintsForPortfolioOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(ListConstraintsForPortfolioError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Returns a paginated list of all paths to a specified product. A path is how the user has access to a specified product, and is necessary when provisioning a product. A path also determines the constraints put on the product.</p>"]
    fn list_launch_paths(&self,
                         input: &ListLaunchPathsInput)
                         -> Result<ListLaunchPathsOutput, ListLaunchPathsError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWS242ServiceCatalogService.ListLaunchPaths");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListLaunchPathsOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(ListLaunchPathsError::from_body(String::from_utf8_lossy(&response.body)
                                                        .as_ref()))
            }
        }
    }


    #[doc="<p>Lists the account IDs that have been authorized sharing of the specified portfolio.</p>"]
    fn list_portfolio_access(&self,
                             input: &ListPortfolioAccessInput)
                             -> Result<ListPortfolioAccessOutput, ListPortfolioAccessError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWS242ServiceCatalogService.ListPortfolioAccess");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListPortfolioAccessOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(ListPortfolioAccessError::from_body(String::from_utf8_lossy(&response.body)
                                                            .as_ref()))
            }
        }
    }


    #[doc="<p>Lists all portfolios in the catalog.</p>"]
    fn list_portfolios(&self,
                       input: &ListPortfoliosInput)
                       -> Result<ListPortfoliosOutput, ListPortfoliosError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWS242ServiceCatalogService.ListPortfolios");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListPortfoliosOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(ListPortfoliosError::from_body(String::from_utf8_lossy(&response.body)
                                                       .as_ref()))
            }
        }
    }


    #[doc="<p>Lists all portfolios that the specified product is associated with.</p>"]
    fn list_portfolios_for_product
        (&self,
         input: &ListPortfoliosForProductInput)
         -> Result<ListPortfoliosForProductOutput, ListPortfoliosForProductError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWS242ServiceCatalogService.ListPortfoliosForProduct");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListPortfoliosForProductOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(ListPortfoliosForProductError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Lists all principal ARNs associated with the specified portfolio.</p>"]
    fn list_principals_for_portfolio
        (&self,
         input: &ListPrincipalsForPortfolioInput)
         -> Result<ListPrincipalsForPortfolioOutput, ListPrincipalsForPortfolioError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWS242ServiceCatalogService.ListPrincipalsForPortfolio");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListPrincipalsForPortfolioOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(ListPrincipalsForPortfolioError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Lists all provisioning artifacts associated with the specified product.</p>"]
    fn list_provisioning_artifacts
        (&self,
         input: &ListProvisioningArtifactsInput)
         -> Result<ListProvisioningArtifactsOutput, ListProvisioningArtifactsError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWS242ServiceCatalogService.ListProvisioningArtifacts");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListProvisioningArtifactsOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(ListProvisioningArtifactsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Returns a paginated list of all performed requests, in the form of RecordDetails objects that are filtered as specified.</p>"]
    fn list_record_history(&self,
                           input: &ListRecordHistoryInput)
                           -> Result<ListRecordHistoryOutput, ListRecordHistoryError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWS242ServiceCatalogService.ListRecordHistory");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListRecordHistoryOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(ListRecordHistoryError::from_body(String::from_utf8_lossy(&response.body)
                                                          .as_ref()))
            }
        }
    }


    #[doc="<p>Requests a <i>provision</i> of a specified product. A <i>provisioned product</i> is a resourced instance for a product. For example, provisioning a CloudFormation-template-backed product results in launching a CloudFormation stack and all the underlying resources that come with it. </p> <p>You can check the status of this request using the <a>DescribeRecord</a> operation.</p>"]
    fn provision_product(&self,
                         input: &ProvisionProductInput)
                         -> Result<ProvisionProductOutput, ProvisionProductError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWS242ServiceCatalogService.ProvisionProduct");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<ProvisionProductOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(ProvisionProductError::from_body(String::from_utf8_lossy(&response.body)
                                                         .as_ref()))
            }
        }
    }


    #[doc="<p>Rejects an offer to share a portfolio.</p>"]
    fn reject_portfolio_share(&self,
                              input: &RejectPortfolioShareInput)
                              -> Result<RejectPortfolioShareOutput, RejectPortfolioShareError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWS242ServiceCatalogService.RejectPortfolioShare");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<RejectPortfolioShareOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(RejectPortfolioShareError::from_body(String::from_utf8_lossy(&response.body)
                                                             .as_ref()))
            }
        }
    }


    #[doc="<p>Returns a paginated list of all the ProvisionedProduct objects that are currently available (not terminated). </p>"]
    fn scan_provisioned_products
        (&self,
         input: &ScanProvisionedProductsInput)
         -> Result<ScanProvisionedProductsOutput, ScanProvisionedProductsError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWS242ServiceCatalogService.ScanProvisionedProducts");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<ScanProvisionedProductsOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(ScanProvisionedProductsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Returns a paginated list all of the <code>Products</code> objects to which the caller has access. </p> <p>The output of this operation can be used as input for other operations, such as <a>DescribeProductView</a>.</p>"]
    fn search_products(&self,
                       input: &SearchProductsInput)
                       -> Result<SearchProductsOutput, SearchProductsError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWS242ServiceCatalogService.SearchProducts");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<SearchProductsOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(SearchProductsError::from_body(String::from_utf8_lossy(&response.body)
                                                       .as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves summary and status information about all products created within the caller's account. If a portfolio ID is provided, this operation retrieves information for only those products that are associated with the specified portfolio.</p>"]
    fn search_products_as_admin
        (&self,
         input: &SearchProductsAsAdminInput)
         -> Result<SearchProductsAsAdminOutput, SearchProductsAsAdminError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWS242ServiceCatalogService.SearchProductsAsAdmin");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<SearchProductsAsAdminOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(SearchProductsAsAdminError::from_body(String::from_utf8_lossy(&response.body)
                                                              .as_ref()))
            }
        }
    }


    #[doc="<p>Requests termination of an existing ProvisionedProduct object. If there are <code>Tags</code> associated with the object, they are terminated when the ProvisionedProduct object is terminated. </p> <p>This operation does not delete any records associated with the ProvisionedProduct object.</p> <p>You can check the status of this request using the <a>DescribeRecord</a> operation.</p>"]
    fn terminate_provisioned_product
        (&self,
         input: &TerminateProvisionedProductInput)
         -> Result<TerminateProvisionedProductOutput, TerminateProvisionedProductError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWS242ServiceCatalogService.TerminateProvisionedProduct");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<TerminateProvisionedProductOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(TerminateProvisionedProductError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Updates an existing constraint.</p>"]
    fn update_constraint(&self,
                         input: &UpdateConstraintInput)
                         -> Result<UpdateConstraintOutput, UpdateConstraintError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWS242ServiceCatalogService.UpdateConstraint");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<UpdateConstraintOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(UpdateConstraintError::from_body(String::from_utf8_lossy(&response.body)
                                                         .as_ref()))
            }
        }
    }


    #[doc="<p>Updates the specified portfolio's details. This operation does not work with a product that has been shared with you.</p>"]
    fn update_portfolio(&self,
                        input: &UpdatePortfolioInput)
                        -> Result<UpdatePortfolioOutput, UpdatePortfolioError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWS242ServiceCatalogService.UpdatePortfolio");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<UpdatePortfolioOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(UpdatePortfolioError::from_body(String::from_utf8_lossy(&response.body)
                                                        .as_ref()))
            }
        }
    }


    #[doc="<p>Updates an existing product.</p>"]
    fn update_product(&self,
                      input: &UpdateProductInput)
                      -> Result<UpdateProductOutput, UpdateProductError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWS242ServiceCatalogService.UpdateProduct");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<UpdateProductOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(UpdateProductError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
            }
        }
    }


    #[doc="<p>Requests updates to the configuration of an existing ProvisionedProduct object. If there are tags associated with the object, they cannot be updated or added with this operation. Depending on the specific updates requested, this operation may update with no interruption, with some interruption, or replace the ProvisionedProduct object entirely. </p> <p>You can check the status of this request using the <a>DescribeRecord</a> operation.</p>"]
    fn update_provisioned_product
        (&self,
         input: &UpdateProvisionedProductInput)
         -> Result<UpdateProvisionedProductOutput, UpdateProvisionedProductError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWS242ServiceCatalogService.UpdateProvisionedProduct");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<UpdateProvisionedProductOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(UpdateProvisionedProductError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Updates an existing provisioning artifact's information. This operation does not work on a provisioning artifact associated with a product that has been shared with you.</p>"]
    fn update_provisioning_artifact
        (&self,
         input: &UpdateProvisioningArtifactInput)
         -> Result<UpdateProvisioningArtifactOutput, UpdateProvisioningArtifactError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWS242ServiceCatalogService.UpdateProvisioningArtifact");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<UpdateProvisioningArtifactOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(UpdateProvisioningArtifactError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }
}

#[cfg(test)]
mod protocol_tests {}
