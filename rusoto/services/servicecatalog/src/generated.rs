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
use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::v2::{Dispatcher, Request, ServiceRequest};
use rusoto_core::{Client, RusotoError, RusotoFuture};

use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
use serde_json;
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AcceptPortfolioShareRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The portfolio identifier.</p>
    #[serde(rename = "PortfolioId")]
    pub portfolio_id: String,
    /// <p>The type of shared portfolios to accept. The default is to accept imported portfolios.</p> <ul> <li> <p> <code>AWS_ORGANIZATIONS</code> - Accept portfolios shared by the master account of your organization.</p> </li> <li> <p> <code>IMPORTED</code> - Accept imported portfolios.</p> </li> <li> <p> <code>AWS_SERVICECATALOG</code> - Not supported. (Throws ResourceNotFoundException.)</p> </li> </ul> <p>For example, <code>aws servicecatalog accept-portfolio-share --portfolio-id "port-2qwzkwxt3y5fk" --portfolio-share-type AWS_ORGANIZATIONS</code> </p>
    #[serde(rename = "PortfolioShareType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portfolio_share_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AcceptPortfolioShareResponse {}

/// <p>The access level to use to filter results.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AccessLevelFilter {
    /// <p><p>The access level.</p> <ul> <li> <p> <code>Account</code> - Filter results based on the account.</p> </li> <li> <p> <code>Role</code> - Filter results based on the federated role of the specified user.</p> </li> <li> <p> <code>User</code> - Filter results based on the specified user.</p> </li> </ul></p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The user to which the access level applies. The only supported value is <code>Self</code>.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AssociateBudgetWithResourceRequest {
    /// <p>The name of the budget you want to associate.</p>
    #[serde(rename = "BudgetName")]
    pub budget_name: String,
    /// <p> The resource identifier. Either a portfolio-id or a product-id.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AssociateBudgetWithResourceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AssociatePrincipalWithPortfolioRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The portfolio identifier.</p>
    #[serde(rename = "PortfolioId")]
    pub portfolio_id: String,
    /// <p>The ARN of the principal (IAM user, role, or group).</p>
    #[serde(rename = "PrincipalARN")]
    pub principal_arn: String,
    /// <p>The principal type. The supported value is <code>IAM</code>.</p>
    #[serde(rename = "PrincipalType")]
    pub principal_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AssociatePrincipalWithPortfolioResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AssociateProductWithPortfolioRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The portfolio identifier.</p>
    #[serde(rename = "PortfolioId")]
    pub portfolio_id: String,
    /// <p>The product identifier.</p>
    #[serde(rename = "ProductId")]
    pub product_id: String,
    /// <p>The identifier of the source portfolio.</p>
    #[serde(rename = "SourcePortfolioId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_portfolio_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AssociateProductWithPortfolioResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AssociateServiceActionWithProvisioningArtifactRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The product identifier. For example, <code>prod-abcdzk7xy33qa</code>.</p>
    #[serde(rename = "ProductId")]
    pub product_id: String,
    /// <p>The identifier of the provisioning artifact. For example, <code>pa-4abcdjnxjj6ne</code>.</p>
    #[serde(rename = "ProvisioningArtifactId")]
    pub provisioning_artifact_id: String,
    /// <p>The self-service action identifier. For example, <code>act-fs7abcd89wxyz</code>.</p>
    #[serde(rename = "ServiceActionId")]
    pub service_action_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AssociateServiceActionWithProvisioningArtifactResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AssociateTagOptionWithResourceRequest {
    /// <p>The resource identifier.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p>The TagOption identifier.</p>
    #[serde(rename = "TagOptionId")]
    pub tag_option_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AssociateTagOptionWithResourceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchAssociateServiceActionWithProvisioningArtifactRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>One or more associations, each consisting of the Action ID, the Product ID, and the Provisioning Artifact ID.</p>
    #[serde(rename = "ServiceActionAssociations")]
    pub service_action_associations: Vec<ServiceActionAssociation>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct BatchAssociateServiceActionWithProvisioningArtifactResponse {
    /// <p>An object that contains a list of errors, along with information to help you identify the self-service action.</p>
    #[serde(rename = "FailedServiceActionAssociations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_service_action_associations: Option<Vec<FailedServiceActionAssociation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchDisassociateServiceActionFromProvisioningArtifactRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>One or more associations, each consisting of the Action ID, the Product ID, and the Provisioning Artifact ID.</p>
    #[serde(rename = "ServiceActionAssociations")]
    pub service_action_associations: Vec<ServiceActionAssociation>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct BatchDisassociateServiceActionFromProvisioningArtifactResponse {
    /// <p>An object that contains a list of errors, along with information to help you identify the self-service action.</p>
    #[serde(rename = "FailedServiceActionAssociations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_service_action_associations: Option<Vec<FailedServiceActionAssociation>>,
}

/// <p>Information about a budget.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct BudgetDetail {
    /// <p>Name of the associated budget.</p>
    #[serde(rename = "BudgetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_name: Option<String>,
}

/// <p>Information about a CloudWatch dashboard.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CloudWatchDashboard {
    /// <p>The name of the CloudWatch dashboard.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Information about a constraint.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ConstraintDetail {
    /// <p>The identifier of the constraint.</p>
    #[serde(rename = "ConstraintId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraint_id: Option<String>,
    /// <p>The description of the constraint.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The owner of the constraint.</p>
    #[serde(rename = "Owner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// <p><p>The type of constraint.</p> <ul> <li> <p> <code>LAUNCH</code> </p> </li> <li> <p> <code>NOTIFICATION</code> </p> </li> <li> <p>STACKSET</p> </li> <li> <p> <code>TEMPLATE</code> </p> </li> </ul></p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Summary information about a constraint.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ConstraintSummary {
    /// <p>The description of the constraint.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p><p>The type of constraint.</p> <ul> <li> <p> <code>LAUNCH</code> </p> </li> <li> <p> <code>NOTIFICATION</code> </p> </li> <li> <p>STACKSET</p> </li> <li> <p> <code>TEMPLATE</code> </p> </li> </ul></p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CopyProductRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The copy options. If the value is <code>CopyTags</code>, the tags from the source product are copied to the target product.</p>
    #[serde(rename = "CopyOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_options: Option<Vec<String>>,
    /// <p> A unique identifier that you provide to ensure idempotency. If multiple requests differ only by the idempotency token, the same response is returned for each repeated request. </p>
    #[serde(rename = "IdempotencyToken")]
    pub idempotency_token: String,
    /// <p>The Amazon Resource Name (ARN) of the source product.</p>
    #[serde(rename = "SourceProductArn")]
    pub source_product_arn: String,
    /// <p>The identifiers of the provisioning artifacts (also known as versions) of the product to copy. By default, all provisioning artifacts are copied.</p>
    #[serde(rename = "SourceProvisioningArtifactIdentifiers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_provisioning_artifact_identifiers:
        Option<Vec<::std::collections::HashMap<String, String>>>,
    /// <p>The identifier of the target product. By default, a new product is created.</p>
    #[serde(rename = "TargetProductId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_product_id: Option<String>,
    /// <p>A name for the target product. The default is the name of the source product.</p>
    #[serde(rename = "TargetProductName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_product_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CopyProductResponse {
    /// <p>The token to use to track the progress of the operation.</p>
    #[serde(rename = "CopyProductToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_product_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateConstraintRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The description of the constraint.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A unique identifier that you provide to ensure idempotency. If multiple requests differ only by the idempotency token, the same response is returned for each repeated request.</p>
    #[serde(rename = "IdempotencyToken")]
    pub idempotency_token: String,
    /// <p><p>The constraint parameters, in JSON format. The syntax depends on the constraint type as follows:</p> <dl> <dt>LAUNCH</dt> <dd> <p>Specify the <code>RoleArn</code> property as follows:</p> <p> <code>{&quot;RoleArn&quot; : &quot;arn:aws:iam::123456789012:role/LaunchRole&quot;}</code> </p> <p>You cannot have both a <code>LAUNCH</code> and a <code>STACKSET</code> constraint.</p> <p>You also cannot have more than one <code>LAUNCH</code> constraint on a product and portfolio.</p> </dd> <dt>NOTIFICATION</dt> <dd> <p>Specify the <code>NotificationArns</code> property as follows:</p> <p> <code>{&quot;NotificationArns&quot; : [&quot;arn:aws:sns:us-east-1:123456789012:Topic&quot;]}</code> </p> </dd> <dt>RESOURCE<em>UPDATE</dt> <dd> <p>Specify the <code>TagUpdatesOnProvisionedProduct</code> property as follows:</p> <p> <code>{&quot;Version&quot;:&quot;2.0&quot;,&quot;Properties&quot;:{&quot;TagUpdateOnProvisionedProduct&quot;:&quot;String&quot;}}</code> </p> <p>The <code>TagUpdatesOnProvisionedProduct</code> property accepts a string value of <code>ALLOWED</code> or <code>NOT</em>ALLOWED</code>.</p> </dd> <dt>STACKSET</dt> <dd> <p>Specify the <code>Parameters</code> property as follows:</p> <p> <code>{&quot;Version&quot;: &quot;String&quot;, &quot;Properties&quot;: {&quot;AccountList&quot;: [ &quot;String&quot; ], &quot;RegionList&quot;: [ &quot;String&quot; ], &quot;AdminRole&quot;: &quot;String&quot;, &quot;ExecutionRole&quot;: &quot;String&quot;}}</code> </p> <p>You cannot have both a <code>LAUNCH</code> and a <code>STACKSET</code> constraint.</p> <p>You also cannot have more than one <code>STACKSET</code> constraint on a product and portfolio.</p> <p>Products with a <code>STACKSET</code> constraint will launch an AWS CloudFormation stack set.</p> </dd> <dt>TEMPLATE</dt> <dd> <p>Specify the <code>Rules</code> property. For more information, see <a href="http://docs.aws.amazon.com/servicecatalog/latest/adminguide/reference-template_constraint_rules.html">Template Constraint Rules</a>.</p> </dd> </dl></p>
    #[serde(rename = "Parameters")]
    pub parameters: String,
    /// <p>The portfolio identifier.</p>
    #[serde(rename = "PortfolioId")]
    pub portfolio_id: String,
    /// <p>The product identifier.</p>
    #[serde(rename = "ProductId")]
    pub product_id: String,
    /// <p><p>The type of constraint.</p> <ul> <li> <p> <code>LAUNCH</code> </p> </li> <li> <p> <code>NOTIFICATION</code> </p> </li> <li> <p> <code>RESOURCE_UPDATE</code> </p> </li> <li> <p> <code>STACKSET</code> </p> </li> <li> <p> <code>TEMPLATE</code> </p> </li> </ul></p>
    #[serde(rename = "Type")]
    pub type_: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateConstraintResponse {
    /// <p>Information about the constraint.</p>
    #[serde(rename = "ConstraintDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraint_detail: Option<ConstraintDetail>,
    /// <p>The constraint parameters.</p>
    #[serde(rename = "ConstraintParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraint_parameters: Option<String>,
    /// <p>The status of the current request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreatePortfolioRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The description of the portfolio.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name to use for display purposes.</p>
    #[serde(rename = "DisplayName")]
    pub display_name: String,
    /// <p>A unique identifier that you provide to ensure idempotency. If multiple requests differ only by the idempotency token, the same response is returned for each repeated request.</p>
    #[serde(rename = "IdempotencyToken")]
    pub idempotency_token: String,
    /// <p>The name of the portfolio provider.</p>
    #[serde(rename = "ProviderName")]
    pub provider_name: String,
    /// <p>One or more tags.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreatePortfolioResponse {
    /// <p>Information about the portfolio.</p>
    #[serde(rename = "PortfolioDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portfolio_detail: Option<PortfolioDetail>,
    /// <p>Information about the tags associated with the portfolio.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreatePortfolioShareRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The AWS account ID. For example, <code>123456789012</code>.</p>
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>The organization node to whom you are going to share. If <code>OrganizationNode</code> is passed in, <code>PortfolioShare</code> will be created for the node and its children (when applies), and a <code>PortfolioShareToken</code> will be returned in the output in order for the administrator to monitor the status of the <code>PortfolioShare</code> creation process.</p>
    #[serde(rename = "OrganizationNode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_node: Option<OrganizationNode>,
    /// <p>The portfolio identifier.</p>
    #[serde(rename = "PortfolioId")]
    pub portfolio_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreatePortfolioShareResponse {
    /// <p>The portfolio share unique identifier. This will only be returned if portfolio is shared to an organization node.</p>
    #[serde(rename = "PortfolioShareToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portfolio_share_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateProductRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The description of the product.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The distributor of the product.</p>
    #[serde(rename = "Distributor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distributor: Option<String>,
    /// <p>A unique identifier that you provide to ensure idempotency. If multiple requests differ only by the idempotency token, the same response is returned for each repeated request.</p>
    #[serde(rename = "IdempotencyToken")]
    pub idempotency_token: String,
    /// <p>The name of the product.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The owner of the product.</p>
    #[serde(rename = "Owner")]
    pub owner: String,
    /// <p>The type of product.</p>
    #[serde(rename = "ProductType")]
    pub product_type: String,
    /// <p>The configuration of the provisioning artifact.</p>
    #[serde(rename = "ProvisioningArtifactParameters")]
    pub provisioning_artifact_parameters: ProvisioningArtifactProperties,
    /// <p>The support information about the product.</p>
    #[serde(rename = "SupportDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_description: Option<String>,
    /// <p>The contact email for product support.</p>
    #[serde(rename = "SupportEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_email: Option<String>,
    /// <p>The contact URL for product support.</p>
    #[serde(rename = "SupportUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_url: Option<String>,
    /// <p>One or more tags.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateProductResponse {
    /// <p>Information about the product view.</p>
    #[serde(rename = "ProductViewDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_view_detail: Option<ProductViewDetail>,
    /// <p>Information about the provisioning artifact.</p>
    #[serde(rename = "ProvisioningArtifactDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_detail: Option<ProvisioningArtifactDetail>,
    /// <p>Information about the tags associated with the product.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateProvisionedProductPlanRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>A unique identifier that you provide to ensure idempotency. If multiple requests differ only by the idempotency token, the same response is returned for each repeated request.</p>
    #[serde(rename = "IdempotencyToken")]
    pub idempotency_token: String,
    /// <p>Passed to CloudFormation. The SNS topic ARNs to which to publish stack-related events.</p>
    #[serde(rename = "NotificationArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_arns: Option<Vec<String>>,
    /// <p>The path identifier of the product. This value is optional if the product has a default path, and required if the product has more than one path. To list the paths for a product, use <a>ListLaunchPaths</a>.</p>
    #[serde(rename = "PathId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_id: Option<String>,
    /// <p>The name of the plan.</p>
    #[serde(rename = "PlanName")]
    pub plan_name: String,
    /// <p>The plan type.</p>
    #[serde(rename = "PlanType")]
    pub plan_type: String,
    /// <p>The product identifier.</p>
    #[serde(rename = "ProductId")]
    pub product_id: String,
    /// <p>A user-friendly name for the provisioned product. This value must be unique for the AWS account and cannot be updated after the product is provisioned.</p>
    #[serde(rename = "ProvisionedProductName")]
    pub provisioned_product_name: String,
    /// <p>The identifier of the provisioning artifact.</p>
    #[serde(rename = "ProvisioningArtifactId")]
    pub provisioning_artifact_id: String,
    /// <p>Parameters specified by the administrator that are required for provisioning the product.</p>
    #[serde(rename = "ProvisioningParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_parameters: Option<Vec<UpdateProvisioningParameter>>,
    /// <p>One or more tags.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateProvisionedProductPlanResponse {
    /// <p>The plan identifier.</p>
    #[serde(rename = "PlanId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    /// <p>The name of the plan.</p>
    #[serde(rename = "PlanName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_name: Option<String>,
    /// <p>The product identifier.</p>
    #[serde(rename = "ProvisionProductId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provision_product_id: Option<String>,
    /// <p>The user-friendly name of the provisioned product.</p>
    #[serde(rename = "ProvisionedProductName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_product_name: Option<String>,
    /// <p>The identifier of the provisioning artifact.</p>
    #[serde(rename = "ProvisioningArtifactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateProvisioningArtifactRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>A unique identifier that you provide to ensure idempotency. If multiple requests differ only by the idempotency token, the same response is returned for each repeated request.</p>
    #[serde(rename = "IdempotencyToken")]
    pub idempotency_token: String,
    /// <p>The configuration for the provisioning artifact.</p>
    #[serde(rename = "Parameters")]
    pub parameters: ProvisioningArtifactProperties,
    /// <p>The product identifier.</p>
    #[serde(rename = "ProductId")]
    pub product_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateProvisioningArtifactResponse {
    /// <p>The URL of the CloudFormation template in Amazon S3, in JSON format.</p>
    #[serde(rename = "Info")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<::std::collections::HashMap<String, String>>,
    /// <p>Information about the provisioning artifact.</p>
    #[serde(rename = "ProvisioningArtifactDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_detail: Option<ProvisioningArtifactDetail>,
    /// <p>The status of the current request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateServiceActionRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p><p>The self-service action definition. Can be one of the following:</p> <dl> <dt>Name</dt> <dd> <p>The name of the AWS Systems Manager Document. For example, <code>AWS-RestartEC2Instance</code>.</p> </dd> <dt>Version</dt> <dd> <p>The AWS Systems Manager automation document version. For example, <code>&quot;Version&quot;: &quot;1&quot;</code> </p> </dd> <dt>AssumeRole</dt> <dd> <p>The Amazon Resource Name (ARN) of the role that performs the self-service actions on your behalf. For example, <code>&quot;AssumeRole&quot;: &quot;arn:aws:iam::12345678910:role/ActionRole&quot;</code>.</p> <p>To reuse the provisioned product launch role, set to <code>&quot;AssumeRole&quot;: &quot;LAUNCH_ROLE&quot;</code>.</p> </dd> <dt>Parameters</dt> <dd> <p>The list of parameters in JSON format.</p> <p>For example: <code>[{&quot;Name&quot;:&quot;InstanceId&quot;,&quot;Type&quot;:&quot;TARGET&quot;}]</code>.</p> </dd> </dl></p>
    #[serde(rename = "Definition")]
    pub definition: ::std::collections::HashMap<String, String>,
    /// <p>The service action definition type. For example, <code>SSM_AUTOMATION</code>.</p>
    #[serde(rename = "DefinitionType")]
    pub definition_type: String,
    /// <p>The self-service action description.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A unique identifier that you provide to ensure idempotency. If multiple requests differ only by the idempotency token, the same response is returned for each repeated request.</p>
    #[serde(rename = "IdempotencyToken")]
    pub idempotency_token: String,
    /// <p>The self-service action name.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateServiceActionResponse {
    /// <p>An object containing information about the self-service action.</p>
    #[serde(rename = "ServiceActionDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_action_detail: Option<ServiceActionDetail>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateTagOptionRequest {
    /// <p>The TagOption key.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The TagOption value.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateTagOptionResponse {
    /// <p>Information about the TagOption.</p>
    #[serde(rename = "TagOptionDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_option_detail: Option<TagOptionDetail>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteConstraintRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The identifier of the constraint.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteConstraintResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeletePortfolioRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The portfolio identifier.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeletePortfolioResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeletePortfolioShareRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The AWS account ID.</p>
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>The organization node to whom you are going to stop sharing.</p>
    #[serde(rename = "OrganizationNode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_node: Option<OrganizationNode>,
    /// <p>The portfolio identifier.</p>
    #[serde(rename = "PortfolioId")]
    pub portfolio_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeletePortfolioShareResponse {
    /// <p>The portfolio share unique identifier. This will only be returned if delete is made to an organization node.</p>
    #[serde(rename = "PortfolioShareToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portfolio_share_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteProductRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The product identifier.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteProductResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteProvisionedProductPlanRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>If set to true, AWS Service Catalog stops managing the specified provisioned product even if it cannot delete the underlying resources.</p>
    #[serde(rename = "IgnoreErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_errors: Option<bool>,
    /// <p>The plan identifier.</p>
    #[serde(rename = "PlanId")]
    pub plan_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteProvisionedProductPlanResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteProvisioningArtifactRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The product identifier.</p>
    #[serde(rename = "ProductId")]
    pub product_id: String,
    /// <p>The identifier of the provisioning artifact.</p>
    #[serde(rename = "ProvisioningArtifactId")]
    pub provisioning_artifact_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteProvisioningArtifactResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteServiceActionRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The self-service action identifier. For example, <code>act-fs7abcd89wxyz</code>.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteServiceActionResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteTagOptionRequest {
    /// <p>The TagOption identifier.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteTagOptionResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeConstraintRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The identifier of the constraint.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeConstraintResponse {
    /// <p>Information about the constraint.</p>
    #[serde(rename = "ConstraintDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraint_detail: Option<ConstraintDetail>,
    /// <p>The constraint parameters.</p>
    #[serde(rename = "ConstraintParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraint_parameters: Option<String>,
    /// <p>The status of the current request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeCopyProductStatusRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The token for the copy product operation. This token is returned by <a>CopyProduct</a>.</p>
    #[serde(rename = "CopyProductToken")]
    pub copy_product_token: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeCopyProductStatusResponse {
    /// <p>The status of the copy product operation.</p>
    #[serde(rename = "CopyProductStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_product_status: Option<String>,
    /// <p>The status message.</p>
    #[serde(rename = "StatusDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_detail: Option<String>,
    /// <p>The identifier of the copied product.</p>
    #[serde(rename = "TargetProductId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_product_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribePortfolioRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The portfolio identifier.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribePortfolioResponse {
    /// <p>Information about the associated budgets.</p>
    #[serde(rename = "Budgets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budgets: Option<Vec<BudgetDetail>>,
    /// <p>Information about the portfolio.</p>
    #[serde(rename = "PortfolioDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portfolio_detail: Option<PortfolioDetail>,
    /// <p>Information about the TagOptions associated with the portfolio.</p>
    #[serde(rename = "TagOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_options: Option<Vec<TagOptionDetail>>,
    /// <p>Information about the tags associated with the portfolio.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribePortfolioShareStatusRequest {
    /// <p>The token for the portfolio share operation. This token is returned either by CreatePortfolioShare or by DeletePortfolioShare.</p>
    #[serde(rename = "PortfolioShareToken")]
    pub portfolio_share_token: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribePortfolioShareStatusResponse {
    /// <p>Organization node identifier. It can be either account id, organizational unit id or organization id.</p>
    #[serde(rename = "OrganizationNodeValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_node_value: Option<String>,
    /// <p>The portfolio identifier.</p>
    #[serde(rename = "PortfolioId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portfolio_id: Option<String>,
    /// <p>The token for the portfolio share operation. For example, <code>share-6v24abcdefghi</code>.</p>
    #[serde(rename = "PortfolioShareToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portfolio_share_token: Option<String>,
    /// <p>Information about the portfolio share operation.</p>
    #[serde(rename = "ShareDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_details: Option<ShareDetails>,
    /// <p>Status of the portfolio share operation.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeProductAsAdminRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The product identifier.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeProductAsAdminResponse {
    /// <p>Information about the associated budgets.</p>
    #[serde(rename = "Budgets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budgets: Option<Vec<BudgetDetail>>,
    /// <p>Information about the product view.</p>
    #[serde(rename = "ProductViewDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_view_detail: Option<ProductViewDetail>,
    /// <p>Information about the provisioning artifacts (also known as versions) for the specified product.</p>
    #[serde(rename = "ProvisioningArtifactSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_summaries: Option<Vec<ProvisioningArtifactSummary>>,
    /// <p>Information about the TagOptions associated with the product.</p>
    #[serde(rename = "TagOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_options: Option<Vec<TagOptionDetail>>,
    /// <p>Information about the tags associated with the product.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeProductRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The product identifier.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeProductResponse {
    /// <p>Information about the associated budgets.</p>
    #[serde(rename = "Budgets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budgets: Option<Vec<BudgetDetail>>,
    /// <p>Summary information about the product view.</p>
    #[serde(rename = "ProductViewSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_view_summary: Option<ProductViewSummary>,
    /// <p>Information about the provisioning artifacts for the specified product.</p>
    #[serde(rename = "ProvisioningArtifacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifacts: Option<Vec<ProvisioningArtifact>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeProductViewRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The product view identifier.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeProductViewResponse {
    /// <p>Summary information about the product.</p>
    #[serde(rename = "ProductViewSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_view_summary: Option<ProductViewSummary>,
    /// <p>Information about the provisioning artifacts for the product.</p>
    #[serde(rename = "ProvisioningArtifacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifacts: Option<Vec<ProvisioningArtifact>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeProvisionedProductPlanRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The maximum number of items to return with this call.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// <p>The page token for the next set of results. To retrieve the first set of results, use null.</p>
    #[serde(rename = "PageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// <p>The plan identifier.</p>
    #[serde(rename = "PlanId")]
    pub plan_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeProvisionedProductPlanResponse {
    /// <p>The page token to use to retrieve the next set of results. If there are no additional results, this value is null.</p>
    #[serde(rename = "NextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>Information about the plan.</p>
    #[serde(rename = "ProvisionedProductPlanDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_product_plan_details: Option<ProvisionedProductPlanDetails>,
    /// <p>Information about the resource changes that will occur when the plan is executed.</p>
    #[serde(rename = "ResourceChanges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_changes: Option<Vec<ResourceChange>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeProvisionedProductRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The provisioned product identifier.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeProvisionedProductResponse {
    /// <p>Any CloudWatch dashboards that were created when provisioning the product.</p>
    #[serde(rename = "CloudWatchDashboards")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_dashboards: Option<Vec<CloudWatchDashboard>>,
    /// <p>Information about the provisioned product.</p>
    #[serde(rename = "ProvisionedProductDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_product_detail: Option<ProvisionedProductDetail>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeProvisioningArtifactRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The product identifier.</p>
    #[serde(rename = "ProductId")]
    pub product_id: String,
    /// <p>The identifier of the provisioning artifact.</p>
    #[serde(rename = "ProvisioningArtifactId")]
    pub provisioning_artifact_id: String,
    /// <p>Indicates whether a verbose level of detail is enabled.</p>
    #[serde(rename = "Verbose")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbose: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeProvisioningArtifactResponse {
    /// <p>The URL of the CloudFormation template in Amazon S3.</p>
    #[serde(rename = "Info")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<::std::collections::HashMap<String, String>>,
    /// <p>Information about the provisioning artifact.</p>
    #[serde(rename = "ProvisioningArtifactDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_detail: Option<ProvisioningArtifactDetail>,
    /// <p>The status of the current request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeProvisioningParametersRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The path identifier of the product. This value is optional if the product has a default path, and required if the product has more than one path. To list the paths for a product, use <a>ListLaunchPaths</a>.</p>
    #[serde(rename = "PathId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_id: Option<String>,
    /// <p>The product identifier.</p>
    #[serde(rename = "ProductId")]
    pub product_id: String,
    /// <p>The identifier of the provisioning artifact.</p>
    #[serde(rename = "ProvisioningArtifactId")]
    pub provisioning_artifact_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeProvisioningParametersResponse {
    /// <p>Information about the constraints used to provision the product.</p>
    #[serde(rename = "ConstraintSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraint_summaries: Option<Vec<ConstraintSummary>>,
    /// <p>Information about the parameters used to provision the product.</p>
    #[serde(rename = "ProvisioningArtifactParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_parameters: Option<Vec<ProvisioningArtifactParameter>>,
    /// <p>An object that contains information about preferences, such as regions and accounts, for the provisioning artifact.</p>
    #[serde(rename = "ProvisioningArtifactPreferences")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_preferences: Option<ProvisioningArtifactPreferences>,
    /// <p>Information about the TagOptions associated with the resource.</p>
    #[serde(rename = "TagOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_options: Option<Vec<TagOptionSummary>>,
    /// <p>Any additional metadata specifically related to the provisioning of the product. For example, see the <code>Version</code> field of the CloudFormation template.</p>
    #[serde(rename = "UsageInstructions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_instructions: Option<Vec<UsageInstruction>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeRecordRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The record identifier of the provisioned product. This identifier is returned by the request operation.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The maximum number of items to return with this call.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// <p>The page token for the next set of results. To retrieve the first set of results, use null.</p>
    #[serde(rename = "PageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeRecordResponse {
    /// <p>The page token to use to retrieve the next set of results. If there are no additional results, this value is null.</p>
    #[serde(rename = "NextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>Information about the product.</p>
    #[serde(rename = "RecordDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_detail: Option<RecordDetail>,
    /// <p>Information about the product created as the result of a request. For example, the output for a CloudFormation-backed product that creates an S3 bucket would include the S3 bucket URL.</p>
    #[serde(rename = "RecordOutputs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_outputs: Option<Vec<RecordOutput>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeServiceActionRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The self-service action identifier.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeServiceActionResponse {
    /// <p>Detailed information about the self-service action.</p>
    #[serde(rename = "ServiceActionDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_action_detail: Option<ServiceActionDetail>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeTagOptionRequest {
    /// <p>The TagOption identifier.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeTagOptionResponse {
    /// <p>Information about the TagOption.</p>
    #[serde(rename = "TagOptionDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_option_detail: Option<TagOptionDetail>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisableAWSOrganizationsAccessRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DisableAWSOrganizationsAccessResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisassociateBudgetFromResourceRequest {
    /// <p>The name of the budget you want to disassociate.</p>
    #[serde(rename = "BudgetName")]
    pub budget_name: String,
    /// <p>The resource identifier you want to disassociate from. Either a portfolio-id or a product-id.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DisassociateBudgetFromResourceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisassociatePrincipalFromPortfolioRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The portfolio identifier.</p>
    #[serde(rename = "PortfolioId")]
    pub portfolio_id: String,
    /// <p>The ARN of the principal (IAM user, role, or group).</p>
    #[serde(rename = "PrincipalARN")]
    pub principal_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DisassociatePrincipalFromPortfolioResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisassociateProductFromPortfolioRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The portfolio identifier.</p>
    #[serde(rename = "PortfolioId")]
    pub portfolio_id: String,
    /// <p>The product identifier.</p>
    #[serde(rename = "ProductId")]
    pub product_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DisassociateProductFromPortfolioResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisassociateServiceActionFromProvisioningArtifactRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The product identifier. For example, <code>prod-abcdzk7xy33qa</code>.</p>
    #[serde(rename = "ProductId")]
    pub product_id: String,
    /// <p>The identifier of the provisioning artifact. For example, <code>pa-4abcdjnxjj6ne</code>.</p>
    #[serde(rename = "ProvisioningArtifactId")]
    pub provisioning_artifact_id: String,
    /// <p>The self-service action identifier. For example, <code>act-fs7abcd89wxyz</code>.</p>
    #[serde(rename = "ServiceActionId")]
    pub service_action_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DisassociateServiceActionFromProvisioningArtifactResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisassociateTagOptionFromResourceRequest {
    /// <p>The resource identifier.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p>The TagOption identifier.</p>
    #[serde(rename = "TagOptionId")]
    pub tag_option_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DisassociateTagOptionFromResourceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct EnableAWSOrganizationsAccessRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct EnableAWSOrganizationsAccessResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ExecuteProvisionedProductPlanRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>A unique identifier that you provide to ensure idempotency. If multiple requests differ only by the idempotency token, the same response is returned for each repeated request.</p>
    #[serde(rename = "IdempotencyToken")]
    pub idempotency_token: String,
    /// <p>The plan identifier.</p>
    #[serde(rename = "PlanId")]
    pub plan_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ExecuteProvisionedProductPlanResponse {
    /// <p>Information about the result of provisioning the product.</p>
    #[serde(rename = "RecordDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_detail: Option<RecordDetail>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ExecuteProvisionedProductServiceActionRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>An idempotency token that uniquely identifies the execute request.</p>
    #[serde(rename = "ExecuteToken")]
    pub execute_token: String,
    /// <p>The identifier of the provisioned product.</p>
    #[serde(rename = "ProvisionedProductId")]
    pub provisioned_product_id: String,
    /// <p>The self-service action identifier. For example, <code>act-fs7abcd89wxyz</code>.</p>
    #[serde(rename = "ServiceActionId")]
    pub service_action_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ExecuteProvisionedProductServiceActionResponse {
    /// <p>An object containing detailed information about the result of provisioning the product.</p>
    #[serde(rename = "RecordDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_detail: Option<RecordDetail>,
}

/// <p>An object containing information about the error, along with identifying information about the self-service action and its associations.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct FailedServiceActionAssociation {
    /// <p>The error code. Valid values are listed below.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>A text description of the error.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The product identifier. For example, <code>prod-abcdzk7xy33qa</code>.</p>
    #[serde(rename = "ProductId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// <p>The identifier of the provisioning artifact. For example, <code>pa-4abcdjnxjj6ne</code>.</p>
    #[serde(rename = "ProvisioningArtifactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_id: Option<String>,
    /// <p>The self-service action identifier. For example, <code>act-fs7abcd89wxyz</code>.</p>
    #[serde(rename = "ServiceActionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_action_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetAWSOrganizationsAccessStatusRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetAWSOrganizationsAccessStatusResponse {
    /// <p>The status of the portfolio share feature.</p>
    #[serde(rename = "AccessStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_status: Option<String>,
}

/// <p>Summary information about a product path for a user.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct LaunchPathSummary {
    /// <p>The constraints on the portfolio-product relationship.</p>
    #[serde(rename = "ConstraintSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraint_summaries: Option<Vec<ConstraintSummary>>,
    /// <p>The identifier of the product path.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the portfolio to which the user was assigned.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The tags associated with this product path.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListAcceptedPortfolioSharesRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The maximum number of items to return with this call.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// <p>The page token for the next set of results. To retrieve the first set of results, use null.</p>
    #[serde(rename = "PageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// <p><p>The type of shared portfolios to list. The default is to list imported portfolios.</p> <ul> <li> <p> <code>AWS<em>ORGANIZATIONS</code> - List portfolios shared by the master account of your organization</p> </li> <li> <p> <code>AWS</em>SERVICECATALOG</code> - List default portfolios</p> </li> <li> <p> <code>IMPORTED</code> - List imported portfolios</p> </li> </ul></p>
    #[serde(rename = "PortfolioShareType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portfolio_share_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListAcceptedPortfolioSharesResponse {
    /// <p>The page token to use to retrieve the next set of results. If there are no additional results, this value is null.</p>
    #[serde(rename = "NextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>Information about the portfolios.</p>
    #[serde(rename = "PortfolioDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portfolio_details: Option<Vec<PortfolioDetail>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListBudgetsForResourceRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The maximum number of items to return with this call.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// <p>The page token for the next set of results. To retrieve the first set of results, use null.</p>
    #[serde(rename = "PageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// <p>The resource identifier.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListBudgetsForResourceResponse {
    /// <p>Information about the associated budgets.</p>
    #[serde(rename = "Budgets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budgets: Option<Vec<BudgetDetail>>,
    /// <p>The page token to use to retrieve the next set of results. If there are no additional results, this value is null.</p>
    #[serde(rename = "NextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListConstraintsForPortfolioRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The maximum number of items to return with this call.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// <p>The page token for the next set of results. To retrieve the first set of results, use null.</p>
    #[serde(rename = "PageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// <p>The portfolio identifier.</p>
    #[serde(rename = "PortfolioId")]
    pub portfolio_id: String,
    /// <p>The product identifier.</p>
    #[serde(rename = "ProductId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListConstraintsForPortfolioResponse {
    /// <p>Information about the constraints.</p>
    #[serde(rename = "ConstraintDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraint_details: Option<Vec<ConstraintDetail>>,
    /// <p>The page token to use to retrieve the next set of results. If there are no additional results, this value is null.</p>
    #[serde(rename = "NextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListLaunchPathsRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The maximum number of items to return with this call.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// <p>The page token for the next set of results. To retrieve the first set of results, use null.</p>
    #[serde(rename = "PageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// <p>The product identifier.</p>
    #[serde(rename = "ProductId")]
    pub product_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListLaunchPathsResponse {
    /// <p>Information about the launch path.</p>
    #[serde(rename = "LaunchPathSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_path_summaries: Option<Vec<LaunchPathSummary>>,
    /// <p>The page token to use to retrieve the next set of results. If there are no additional results, this value is null.</p>
    #[serde(rename = "NextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListOrganizationPortfolioAccessRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p><p>The organization node type that will be returned in the output.</p> <ul> <li> <p> <code>ORGANIZATION</code> - Organization that has access to the portfolio. </p> </li> <li> <p> <code>ORGANIZATIONAL_UNIT</code> - Organizational unit that has access to the portfolio within your organization.</p> </li> <li> <p> <code>ACCOUNT</code> - Account that has access to the portfolio within your organization.</p> </li> </ul></p>
    #[serde(rename = "OrganizationNodeType")]
    pub organization_node_type: String,
    /// <p>The maximum number of items to return with this call.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// <p>The page token for the next set of results. To retrieve the first set of results, use null.</p>
    #[serde(rename = "PageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// <p>The portfolio identifier. For example, <code>port-2abcdext3y5fk</code>.</p>
    #[serde(rename = "PortfolioId")]
    pub portfolio_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListOrganizationPortfolioAccessResponse {
    /// <p>The page token to use to retrieve the next set of results. If there are no additional results, this value is null.</p>
    #[serde(rename = "NextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>Displays information about the organization nodes.</p>
    #[serde(rename = "OrganizationNodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_nodes: Option<Vec<OrganizationNode>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListPortfolioAccessRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The portfolio identifier.</p>
    #[serde(rename = "PortfolioId")]
    pub portfolio_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListPortfolioAccessResponse {
    /// <p>Information about the AWS accounts with access to the portfolio.</p>
    #[serde(rename = "AccountIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    /// <p>The page token to use to retrieve the next set of results. If there are no additional results, this value is null.</p>
    #[serde(rename = "NextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListPortfoliosForProductRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The maximum number of items to return with this call.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// <p>The page token for the next set of results. To retrieve the first set of results, use null.</p>
    #[serde(rename = "PageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// <p>The product identifier.</p>
    #[serde(rename = "ProductId")]
    pub product_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListPortfoliosForProductResponse {
    /// <p>The page token to use to retrieve the next set of results. If there are no additional results, this value is null.</p>
    #[serde(rename = "NextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>Information about the portfolios.</p>
    #[serde(rename = "PortfolioDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portfolio_details: Option<Vec<PortfolioDetail>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListPortfoliosRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The maximum number of items to return with this call.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// <p>The page token for the next set of results. To retrieve the first set of results, use null.</p>
    #[serde(rename = "PageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListPortfoliosResponse {
    /// <p>The page token to use to retrieve the next set of results. If there are no additional results, this value is null.</p>
    #[serde(rename = "NextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>Information about the portfolios.</p>
    #[serde(rename = "PortfolioDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portfolio_details: Option<Vec<PortfolioDetail>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListPrincipalsForPortfolioRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The maximum number of items to return with this call.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// <p>The page token for the next set of results. To retrieve the first set of results, use null.</p>
    #[serde(rename = "PageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// <p>The portfolio identifier.</p>
    #[serde(rename = "PortfolioId")]
    pub portfolio_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListPrincipalsForPortfolioResponse {
    /// <p>The page token to use to retrieve the next set of results. If there are no additional results, this value is null.</p>
    #[serde(rename = "NextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>The IAM principals (users or roles) associated with the portfolio.</p>
    #[serde(rename = "Principals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principals: Option<Vec<Principal>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListProvisionedProductPlansRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The access level to use to obtain results. The default is <code>User</code>.</p>
    #[serde(rename = "AccessLevelFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_level_filter: Option<AccessLevelFilter>,
    /// <p>The maximum number of items to return with this call.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// <p>The page token for the next set of results. To retrieve the first set of results, use null.</p>
    #[serde(rename = "PageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// <p>The product identifier.</p>
    #[serde(rename = "ProvisionProductId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provision_product_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListProvisionedProductPlansResponse {
    /// <p>The page token to use to retrieve the next set of results. If there are no additional results, this value is null.</p>
    #[serde(rename = "NextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>Information about the plans.</p>
    #[serde(rename = "ProvisionedProductPlans")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_product_plans: Option<Vec<ProvisionedProductPlanSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListProvisioningArtifactsForServiceActionRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The maximum number of items to return with this call.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// <p>The page token for the next set of results. To retrieve the first set of results, use null.</p>
    #[serde(rename = "PageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// <p>The self-service action identifier. For example, <code>act-fs7abcd89wxyz</code>.</p>
    #[serde(rename = "ServiceActionId")]
    pub service_action_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListProvisioningArtifactsForServiceActionResponse {
    /// <p>The page token to use to retrieve the next set of results. If there are no additional results, this value is null.</p>
    #[serde(rename = "NextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>An array of objects with information about product views and provisioning artifacts.</p>
    #[serde(rename = "ProvisioningArtifactViews")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_views: Option<Vec<ProvisioningArtifactView>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListProvisioningArtifactsRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The product identifier.</p>
    #[serde(rename = "ProductId")]
    pub product_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListProvisioningArtifactsResponse {
    /// <p>The page token to use to retrieve the next set of results. If there are no additional results, this value is null.</p>
    #[serde(rename = "NextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>Information about the provisioning artifacts.</p>
    #[serde(rename = "ProvisioningArtifactDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_details: Option<Vec<ProvisioningArtifactDetail>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListRecordHistoryRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The access level to use to obtain results. The default is <code>User</code>.</p>
    #[serde(rename = "AccessLevelFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_level_filter: Option<AccessLevelFilter>,
    /// <p>The maximum number of items to return with this call.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// <p>The page token for the next set of results. To retrieve the first set of results, use null.</p>
    #[serde(rename = "PageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// <p>The search filter to scope the results.</p>
    #[serde(rename = "SearchFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_filter: Option<ListRecordHistorySearchFilter>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListRecordHistoryResponse {
    /// <p>The page token to use to retrieve the next set of results. If there are no additional results, this value is null.</p>
    #[serde(rename = "NextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>The records, in reverse chronological order.</p>
    #[serde(rename = "RecordDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_details: Option<Vec<RecordDetail>>,
}

/// <p>The search filter to use when listing history records.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListRecordHistorySearchFilter {
    /// <p><p>The filter key.</p> <ul> <li> <p> <code>product</code> - Filter results based on the specified product identifier.</p> </li> <li> <p> <code>provisionedproduct</code> - Filter results based on the provisioned product identifier.</p> </li> </ul></p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The filter value.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListResourcesForTagOptionRequest {
    /// <p>The maximum number of items to return with this call.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// <p>The page token for the next set of results. To retrieve the first set of results, use null.</p>
    #[serde(rename = "PageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// <p><p>The resource type.</p> <ul> <li> <p> <code>Portfolio</code> </p> </li> <li> <p> <code>Product</code> </p> </li> </ul></p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>The TagOption identifier.</p>
    #[serde(rename = "TagOptionId")]
    pub tag_option_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListResourcesForTagOptionResponse {
    /// <p>The page token for the next set of results. To retrieve the first set of results, use null.</p>
    #[serde(rename = "PageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// <p>Information about the resources.</p>
    #[serde(rename = "ResourceDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_details: Option<Vec<ResourceDetail>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListServiceActionsForProvisioningArtifactRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The maximum number of items to return with this call.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// <p>The page token for the next set of results. To retrieve the first set of results, use null.</p>
    #[serde(rename = "PageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// <p>The product identifier. For example, <code>prod-abcdzk7xy33qa</code>.</p>
    #[serde(rename = "ProductId")]
    pub product_id: String,
    /// <p>The identifier of the provisioning artifact. For example, <code>pa-4abcdjnxjj6ne</code>.</p>
    #[serde(rename = "ProvisioningArtifactId")]
    pub provisioning_artifact_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListServiceActionsForProvisioningArtifactResponse {
    /// <p>The page token to use to retrieve the next set of results. If there are no additional results, this value is null.</p>
    #[serde(rename = "NextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>An object containing information about the self-service actions associated with the provisioning artifact.</p>
    #[serde(rename = "ServiceActionSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_action_summaries: Option<Vec<ServiceActionSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListServiceActionsRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The maximum number of items to return with this call.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// <p>The page token for the next set of results. To retrieve the first set of results, use null.</p>
    #[serde(rename = "PageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListServiceActionsResponse {
    /// <p>The page token to use to retrieve the next set of results. If there are no additional results, this value is null.</p>
    #[serde(rename = "NextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>An object containing information about the service actions associated with the provisioning artifact.</p>
    #[serde(rename = "ServiceActionSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_action_summaries: Option<Vec<ServiceActionSummary>>,
}

/// <p>Filters to use when listing TagOptions.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListTagOptionsFilters {
    /// <p>The active state.</p>
    #[serde(rename = "Active")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// <p>The TagOption key.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The TagOption value.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListTagOptionsRequest {
    /// <p>The search filters. If no search filters are specified, the output includes all TagOptions.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<ListTagOptionsFilters>,
    /// <p>The maximum number of items to return with this call.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// <p>The page token for the next set of results. To retrieve the first set of results, use null.</p>
    #[serde(rename = "PageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListTagOptionsResponse {
    /// <p>The page token for the next set of results. To retrieve the first set of results, use null.</p>
    #[serde(rename = "PageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// <p>Information about the TagOptions.</p>
    #[serde(rename = "TagOptionDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_option_details: Option<Vec<TagOptionDetail>>,
}

/// <p>Information about the organization node.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrganizationNode {
    /// <p>The organization node type.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The identifier of the organization node.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>The constraints that the administrator has put on the parameter.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ParameterConstraints {
    /// <p>The values that the administrator has allowed for the parameter.</p>
    #[serde(rename = "AllowedValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_values: Option<Vec<String>>,
}

/// <p>Information about a portfolio.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PortfolioDetail {
    /// <p>The ARN assigned to the portfolio.</p>
    #[serde(rename = "ARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The UTC time stamp of the creation time.</p>
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    /// <p>The description of the portfolio.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name to use for display purposes.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>The portfolio identifier.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the portfolio provider.</p>
    #[serde(rename = "ProviderName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
}

/// <p>Information about a principal.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Principal {
    /// <p>The ARN of the principal (IAM user, role, or group).</p>
    #[serde(rename = "PrincipalARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_arn: Option<String>,
    /// <p>The principal type. The supported value is <code>IAM</code>.</p>
    #[serde(rename = "PrincipalType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_type: Option<String>,
}

/// <p>A single product view aggregation value/count pair, containing metadata about each product to which the calling user has access.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ProductViewAggregationValue {
    /// <p>An approximate count of the products that match the value.</p>
    #[serde(rename = "ApproximateCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_count: Option<i64>,
    /// <p>The value of the product view aggregation.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>Information about a product view.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ProductViewDetail {
    /// <p>The UTC time stamp of the creation time.</p>
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    /// <p>The ARN of the product.</p>
    #[serde(rename = "ProductARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_arn: Option<String>,
    /// <p>Summary information about the product view.</p>
    #[serde(rename = "ProductViewSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_view_summary: Option<ProductViewSummary>,
    /// <p><p>The status of the product.</p> <ul> <li> <p> <code>AVAILABLE</code> - The product is ready for use.</p> </li> <li> <p> <code>CREATING</code> - Product creation has started; the product is not ready for use.</p> </li> <li> <p> <code>FAILED</code> - An action failed.</p> </li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Summary information about a product view.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ProductViewSummary {
    /// <p>The distributor of the product. Contact the product administrator for the significance of this value.</p>
    #[serde(rename = "Distributor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distributor: Option<String>,
    /// <p>Indicates whether the product has a default path. If the product does not have a default path, call <a>ListLaunchPaths</a> to disambiguate between paths. Otherwise, <a>ListLaunchPaths</a> is not required, and the output of <a>ProductViewSummary</a> can be used directly with <a>DescribeProvisioningParameters</a>.</p>
    #[serde(rename = "HasDefaultPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_default_path: Option<bool>,
    /// <p>The product view identifier.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the product.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The owner of the product. Contact the product administrator for the significance of this value.</p>
    #[serde(rename = "Owner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// <p>The product identifier.</p>
    #[serde(rename = "ProductId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// <p>Short description of the product.</p>
    #[serde(rename = "ShortDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,
    /// <p>The description of the support for this Product.</p>
    #[serde(rename = "SupportDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_description: Option<String>,
    /// <p>The email contact information to obtain support for this Product.</p>
    #[serde(rename = "SupportEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_email: Option<String>,
    /// <p>The URL information to obtain support for this Product.</p>
    #[serde(rename = "SupportUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_url: Option<String>,
    /// <p>The product type. Contact the product administrator for the significance of this value. If this value is <code>MARKETPLACE</code>, the product was created by AWS Marketplace.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ProvisionProductRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>Passed to CloudFormation. The SNS topic ARNs to which to publish stack-related events.</p>
    #[serde(rename = "NotificationArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_arns: Option<Vec<String>>,
    /// <p>The path identifier of the product. This value is optional if the product has a default path, and required if the product has more than one path. To list the paths for a product, use <a>ListLaunchPaths</a>.</p>
    #[serde(rename = "PathId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_id: Option<String>,
    /// <p>The product identifier.</p>
    #[serde(rename = "ProductId")]
    pub product_id: String,
    /// <p>An idempotency token that uniquely identifies the provisioning request.</p>
    #[serde(rename = "ProvisionToken")]
    pub provision_token: String,
    /// <p>A user-friendly name for the provisioned product. This value must be unique for the AWS account and cannot be updated after the product is provisioned.</p>
    #[serde(rename = "ProvisionedProductName")]
    pub provisioned_product_name: String,
    /// <p>The identifier of the provisioning artifact.</p>
    #[serde(rename = "ProvisioningArtifactId")]
    pub provisioning_artifact_id: String,
    /// <p>Parameters specified by the administrator that are required for provisioning the product.</p>
    #[serde(rename = "ProvisioningParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_parameters: Option<Vec<ProvisioningParameter>>,
    /// <p>An object that contains information about the provisioning preferences for a stack set.</p>
    #[serde(rename = "ProvisioningPreferences")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_preferences: Option<ProvisioningPreferences>,
    /// <p>One or more tags.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ProvisionProductResponse {
    /// <p>Information about the result of provisioning the product.</p>
    #[serde(rename = "RecordDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_detail: Option<RecordDetail>,
}

/// <p>Information about a provisioned product.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ProvisionedProductAttribute {
    /// <p>The ARN of the provisioned product.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The UTC time stamp of the creation time.</p>
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    /// <p>The identifier of the provisioned product.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>A unique identifier that you provide to ensure idempotency. If multiple requests differ only by the idempotency token, the same response is returned for each repeated request.</p>
    #[serde(rename = "IdempotencyToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_token: Option<String>,
    /// <p>The record identifier of the last request performed on this provisioned product.</p>
    #[serde(rename = "LastRecordId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_record_id: Option<String>,
    /// <p>The user-friendly name of the provisioned product.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The assigned identifier for the resource, such as an EC2 instance ID or an S3 bucket name.</p>
    #[serde(rename = "PhysicalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_id: Option<String>,
    /// <p>The product identifier.</p>
    #[serde(rename = "ProductId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// <p>The identifier of the provisioning artifact.</p>
    #[serde(rename = "ProvisioningArtifactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_id: Option<String>,
    /// <p><p>The current status of the provisioned product.</p> <ul> <li> <p> <code>AVAILABLE</code> - Stable state, ready to perform any operation. The most recent operation succeeded and completed.</p> </li> <li> <p> <code>UNDER<em>CHANGE</code> - Transitive state. Operations performed might not have valid results. Wait for an <code>AVAILABLE</code> status before performing operations.</p> </li> <li> <p> <code>TAINTED</code> - Stable state, ready to perform any operation. The stack has completed the requested operation but is not exactly what was requested. For example, a request to update to a new version failed and the stack rolled back to the current version.</p> </li> <li> <p> <code>ERROR</code> - An unexpected error occurred. The provisioned product exists but the stack is not running. For example, CloudFormation received a parameter value that was not valid and could not launch the stack.</p> </li> <li> <p> <code>PLAN</em>IN_PROGRESS</code> - Transitive state. The plan operations were performed to provision a new product, but resources have not yet been created. After reviewing the list of resources to be created, execute the plan. Wait for an <code>AVAILABLE</code> status before performing operations.</p> </li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The current status message of the provisioned product.</p>
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// <p>One or more tags.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The type of provisioned product. The supported values are <code>CFN_STACK</code> and <code>CFN_STACKSET</code>.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the IAM user.</p>
    #[serde(rename = "UserArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_arn: Option<String>,
    /// <p>The ARN of the IAM user in the session. This ARN might contain a session ID.</p>
    #[serde(rename = "UserArnSession")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_arn_session: Option<String>,
}

/// <p>Information about a provisioned product.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ProvisionedProductDetail {
    /// <p>The ARN of the provisioned product.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The UTC time stamp of the creation time.</p>
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    /// <p>The identifier of the provisioned product.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>A unique identifier that you provide to ensure idempotency. If multiple requests differ only by the idempotency token, the same response is returned for each repeated request.</p>
    #[serde(rename = "IdempotencyToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_token: Option<String>,
    /// <p>The record identifier of the last request performed on this provisioned product.</p>
    #[serde(rename = "LastRecordId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_record_id: Option<String>,
    /// <p>The user-friendly name of the provisioned product.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The product identifier. For example, <code>prod-abcdzk7xy33qa</code>.</p>
    #[serde(rename = "ProductId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// <p>The identifier of the provisioning artifact. For example, <code>pa-4abcdjnxjj6ne</code>.</p>
    #[serde(rename = "ProvisioningArtifactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_id: Option<String>,
    /// <p><p>The current status of the provisioned product.</p> <ul> <li> <p> <code>AVAILABLE</code> - Stable state, ready to perform any operation. The most recent operation succeeded and completed.</p> </li> <li> <p> <code>UNDER<em>CHANGE</code> - Transitive state. Operations performed might not have valid results. Wait for an <code>AVAILABLE</code> status before performing operations.</p> </li> <li> <p> <code>TAINTED</code> - Stable state, ready to perform any operation. The stack has completed the requested operation but is not exactly what was requested. For example, a request to update to a new version failed and the stack rolled back to the current version.</p> </li> <li> <p> <code>ERROR</code> - An unexpected error occurred. The provisioned product exists but the stack is not running. For example, CloudFormation received a parameter value that was not valid and could not launch the stack.</p> </li> <li> <p> <code>PLAN</em>IN_PROGRESS</code> - Transitive state. The plan operations were performed to provision a new product, but resources have not yet been created. After reviewing the list of resources to be created, execute the plan. Wait for an <code>AVAILABLE</code> status before performing operations.</p> </li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The current status message of the provisioned product.</p>
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// <p>The type of provisioned product. The supported values are <code>CFN_STACK</code> and <code>CFN_STACKSET</code>.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Information about a plan.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ProvisionedProductPlanDetails {
    /// <p>The UTC time stamp of the creation time.</p>
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    /// <p>Passed to CloudFormation. The SNS topic ARNs to which to publish stack-related events.</p>
    #[serde(rename = "NotificationArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_arns: Option<Vec<String>>,
    /// <p>The path identifier of the product. This value is optional if the product has a default path, and required if the product has more than one path. To list the paths for a product, use <a>ListLaunchPaths</a>.</p>
    #[serde(rename = "PathId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_id: Option<String>,
    /// <p>The plan identifier.</p>
    #[serde(rename = "PlanId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    /// <p>The name of the plan.</p>
    #[serde(rename = "PlanName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_name: Option<String>,
    /// <p>The plan type.</p>
    #[serde(rename = "PlanType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_type: Option<String>,
    /// <p>The product identifier.</p>
    #[serde(rename = "ProductId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// <p>The product identifier.</p>
    #[serde(rename = "ProvisionProductId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provision_product_id: Option<String>,
    /// <p>The user-friendly name of the provisioned product.</p>
    #[serde(rename = "ProvisionProductName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provision_product_name: Option<String>,
    /// <p>The identifier of the provisioning artifact.</p>
    #[serde(rename = "ProvisioningArtifactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_id: Option<String>,
    /// <p>Parameters specified by the administrator that are required for provisioning the product.</p>
    #[serde(rename = "ProvisioningParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_parameters: Option<Vec<UpdateProvisioningParameter>>,
    /// <p>The status.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The status message.</p>
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// <p>One or more tags.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The time when the plan was last updated.</p>
    #[serde(rename = "UpdatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<f64>,
}

/// <p>Summary information about a plan.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ProvisionedProductPlanSummary {
    /// <p>The plan identifier.</p>
    #[serde(rename = "PlanId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    /// <p>The name of the plan.</p>
    #[serde(rename = "PlanName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_name: Option<String>,
    /// <p>The plan type.</p>
    #[serde(rename = "PlanType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_type: Option<String>,
    /// <p>The product identifier.</p>
    #[serde(rename = "ProvisionProductId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provision_product_id: Option<String>,
    /// <p>The user-friendly name of the provisioned product.</p>
    #[serde(rename = "ProvisionProductName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provision_product_name: Option<String>,
    /// <p>The identifier of the provisioning artifact.</p>
    #[serde(rename = "ProvisioningArtifactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_id: Option<String>,
}

/// <p>Information about a provisioning artifact. A provisioning artifact is also known as a product version.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ProvisioningArtifact {
    /// <p>The UTC time stamp of the creation time.</p>
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    /// <p>The description of the provisioning artifact.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The identifier of the provisioning artifact.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the provisioning artifact.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Information about a provisioning artifact (also known as a version) for a product.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ProvisioningArtifactDetail {
    /// <p>Indicates whether the product version is active.</p>
    #[serde(rename = "Active")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// <p>The UTC time stamp of the creation time.</p>
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    /// <p>The description of the provisioning artifact.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The identifier of the provisioning artifact.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the provisioning artifact.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p><p>The type of provisioning artifact.</p> <ul> <li> <p> <code>CLOUD<em>FORMATION</em>TEMPLATE</code> - AWS CloudFormation template</p> </li> <li> <p> <code>MARKETPLACE<em>AMI</code> - AWS Marketplace AMI</p> </li> <li> <p> <code>MARKETPLACE</em>CAR</code> - AWS Marketplace Clusters and AWS Resources</p> </li> </ul></p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Information about a parameter used to provision a product.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ProvisioningArtifactParameter {
    /// <p>The default value.</p>
    #[serde(rename = "DefaultValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    /// <p>The description of the parameter.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>If this value is true, the value for this parameter is obfuscated from view when the parameter is retrieved. This parameter is used to hide sensitive information.</p>
    #[serde(rename = "IsNoEcho")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_no_echo: Option<bool>,
    /// <p>Constraints that the administrator has put on a parameter.</p>
    #[serde(rename = "ParameterConstraints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_constraints: Option<ParameterConstraints>,
    /// <p>The parameter key.</p>
    #[serde(rename = "ParameterKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_key: Option<String>,
    /// <p>The parameter type.</p>
    #[serde(rename = "ParameterType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_type: Option<String>,
}

/// <p>The user-defined preferences that will be applied during product provisioning, unless overridden by <code>ProvisioningPreferences</code> or <code>UpdateProvisioningPreferences</code>.</p> <p>For more information on maximum concurrent accounts and failure tolerance, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/stacksets-concepts.html#stackset-ops-options">Stack set operation options</a> in the <i>AWS CloudFormation User Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ProvisioningArtifactPreferences {
    /// <p>One or more AWS accounts where stack instances are deployed from the stack set. These accounts can be scoped in <code>ProvisioningPreferences$StackSetAccounts</code> and <code>UpdateProvisioningPreferences$StackSetAccounts</code>.</p> <p>Applicable only to a <code>CFN_STACKSET</code> provisioned product type.</p>
    #[serde(rename = "StackSetAccounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_accounts: Option<Vec<String>>,
    /// <p>One or more AWS Regions where stack instances are deployed from the stack set. These regions can be scoped in <code>ProvisioningPreferences$StackSetRegions</code> and <code>UpdateProvisioningPreferences$StackSetRegions</code>.</p> <p>Applicable only to a <code>CFN_STACKSET</code> provisioned product type.</p>
    #[serde(rename = "StackSetRegions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_regions: Option<Vec<String>>,
}

/// <p>Information about a provisioning artifact (also known as a version) for a product.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ProvisioningArtifactProperties {
    /// <p>The description of the provisioning artifact, including how it differs from the previous provisioning artifact.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>If set to true, AWS Service Catalog stops validating the specified provisioning artifact even if it is invalid.</p>
    #[serde(rename = "DisableTemplateValidation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_template_validation: Option<bool>,
    /// <p>The URL of the CloudFormation template in Amazon S3. Specify the URL in JSON format as follows:</p> <p> <code>"LoadTemplateFromURL": "https://s3.amazonaws.com/cf-templates-ozkq9d3hgiq2-us-east-1/..."</code> </p>
    #[serde(rename = "Info")]
    pub info: ::std::collections::HashMap<String, String>,
    /// <p>The name of the provisioning artifact (for example, v1 v2beta). No spaces are allowed.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p><p>The type of provisioning artifact.</p> <ul> <li> <p> <code>CLOUD<em>FORMATION</em>TEMPLATE</code> - AWS CloudFormation template</p> </li> <li> <p> <code>MARKETPLACE<em>AMI</code> - AWS Marketplace AMI</p> </li> <li> <p> <code>MARKETPLACE</em>CAR</code> - AWS Marketplace Clusters and AWS Resources</p> </li> </ul></p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Summary information about a provisioning artifact (also known as a version) for a product.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ProvisioningArtifactSummary {
    /// <p>The UTC time stamp of the creation time.</p>
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    /// <p>The description of the provisioning artifact.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The identifier of the provisioning artifact.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the provisioning artifact.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The metadata for the provisioning artifact. This is used with AWS Marketplace products.</p>
    #[serde(rename = "ProvisioningArtifactMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_metadata: Option<::std::collections::HashMap<String, String>>,
}

/// <p>An object that contains summary information about a product view and a provisioning artifact.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ProvisioningArtifactView {
    /// <p>Summary information about a product view.</p>
    #[serde(rename = "ProductViewSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_view_summary: Option<ProductViewSummary>,
    /// <p>Information about a provisioning artifact. A provisioning artifact is also known as a product version.</p>
    #[serde(rename = "ProvisioningArtifact")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact: Option<ProvisioningArtifact>,
}

/// <p>Information about a parameter used to provision a product.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ProvisioningParameter {
    /// <p>The parameter key.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The parameter value.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>The user-defined preferences that will be applied when updating a provisioned product. Not all preferences are applicable to all provisioned product types.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ProvisioningPreferences {
    /// <p>One or more AWS accounts that will have access to the provisioned product.</p> <p>Applicable only to a <code>CFN_STACKSET</code> provisioned product type.</p> <p>The AWS accounts specified should be within the list of accounts in the <code>STACKSET</code> constraint. To get the list of accounts in the <code>STACKSET</code> constraint, use the <code>DescribeProvisioningParameters</code> operation.</p> <p>If no values are specified, the default value is all accounts from the <code>STACKSET</code> constraint.</p>
    #[serde(rename = "StackSetAccounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_accounts: Option<Vec<String>>,
    /// <p>The number of accounts, per region, for which this operation can fail before AWS Service Catalog stops the operation in that region. If the operation is stopped in a region, AWS Service Catalog doesn't attempt the operation in any subsequent regions.</p> <p>Applicable only to a <code>CFN_STACKSET</code> provisioned product type.</p> <p>Conditional: You must specify either <code>StackSetFailureToleranceCount</code> or <code>StackSetFailureTolerancePercentage</code>, but not both.</p> <p>The default value is <code>0</code> if no value is specified.</p>
    #[serde(rename = "StackSetFailureToleranceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_failure_tolerance_count: Option<i64>,
    /// <p>The percentage of accounts, per region, for which this stack operation can fail before AWS Service Catalog stops the operation in that region. If the operation is stopped in a region, AWS Service Catalog doesn't attempt the operation in any subsequent regions.</p> <p>When calculating the number of accounts based on the specified percentage, AWS Service Catalog rounds down to the next whole number.</p> <p>Applicable only to a <code>CFN_STACKSET</code> provisioned product type.</p> <p>Conditional: You must specify either <code>StackSetFailureToleranceCount</code> or <code>StackSetFailureTolerancePercentage</code>, but not both.</p>
    #[serde(rename = "StackSetFailureTolerancePercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_failure_tolerance_percentage: Option<i64>,
    /// <p>The maximum number of accounts in which to perform this operation at one time. This is dependent on the value of <code>StackSetFailureToleranceCount</code>. <code>StackSetMaxConcurrentCount</code> is at most one more than the <code>StackSetFailureToleranceCount</code>.</p> <p>Note that this setting lets you specify the maximum for operations. For large deployments, under certain circumstances the actual number of accounts acted upon concurrently may be lower due to service throttling.</p> <p>Applicable only to a <code>CFN_STACKSET</code> provisioned product type.</p> <p>Conditional: You must specify either <code>StackSetMaxConcurrentCount</code> or <code>StackSetMaxConcurrentPercentage</code>, but not both.</p>
    #[serde(rename = "StackSetMaxConcurrencyCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_max_concurrency_count: Option<i64>,
    /// <p>The maximum percentage of accounts in which to perform this operation at one time.</p> <p>When calculating the number of accounts based on the specified percentage, AWS Service Catalog rounds down to the next whole number. This is true except in cases where rounding down would result is zero. In this case, AWS Service Catalog sets the number as <code>1</code> instead.</p> <p>Note that this setting lets you specify the maximum for operations. For large deployments, under certain circumstances the actual number of accounts acted upon concurrently may be lower due to service throttling.</p> <p>Applicable only to a <code>CFN_STACKSET</code> provisioned product type.</p> <p>Conditional: You must specify either <code>StackSetMaxConcurrentCount</code> or <code>StackSetMaxConcurrentPercentage</code>, but not both.</p>
    #[serde(rename = "StackSetMaxConcurrencyPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_max_concurrency_percentage: Option<i64>,
    /// <p>One or more AWS Regions where the provisioned product will be available.</p> <p>Applicable only to a <code>CFN_STACKSET</code> provisioned product type.</p> <p>The specified regions should be within the list of regions from the <code>STACKSET</code> constraint. To get the list of regions in the <code>STACKSET</code> constraint, use the <code>DescribeProvisioningParameters</code> operation.</p> <p>If no values are specified, the default value is all regions from the <code>STACKSET</code> constraint.</p>
    #[serde(rename = "StackSetRegions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_regions: Option<Vec<String>>,
}

/// <p>Information about a request operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RecordDetail {
    /// <p>The UTC time stamp of the creation time.</p>
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    /// <p>The path identifier.</p>
    #[serde(rename = "PathId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_id: Option<String>,
    /// <p>The product identifier.</p>
    #[serde(rename = "ProductId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// <p>The identifier of the provisioned product.</p>
    #[serde(rename = "ProvisionedProductId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_product_id: Option<String>,
    /// <p>The user-friendly name of the provisioned product.</p>
    #[serde(rename = "ProvisionedProductName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_product_name: Option<String>,
    /// <p>The type of provisioned product. The supported values are <code>CFN_STACK</code> and <code>CFN_STACKSET</code>.</p>
    #[serde(rename = "ProvisionedProductType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_product_type: Option<String>,
    /// <p>The identifier of the provisioning artifact.</p>
    #[serde(rename = "ProvisioningArtifactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_id: Option<String>,
    /// <p>The errors that occurred.</p>
    #[serde(rename = "RecordErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_errors: Option<Vec<RecordError>>,
    /// <p>The identifier of the record.</p>
    #[serde(rename = "RecordId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_id: Option<String>,
    /// <p>One or more tags.</p>
    #[serde(rename = "RecordTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_tags: Option<Vec<RecordTag>>,
    /// <p><p>The record type.</p> <ul> <li> <p> <code>PROVISION<em>PRODUCT</code> </p> </li> <li> <p> <code>UPDATE</em>PROVISIONED<em>PRODUCT</code> </p> </li> <li> <p> <code>TERMINATE</em>PROVISIONED_PRODUCT</code> </p> </li> </ul></p>
    #[serde(rename = "RecordType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_type: Option<String>,
    /// <p><p>The status of the provisioned product.</p> <ul> <li> <p> <code>CREATED</code> - The request was created but the operation has not started.</p> </li> <li> <p> <code>IN<em>PROGRESS</code> - The requested operation is in progress.</p> </li> <li> <p> <code>IN</em>PROGRESS<em>IN</em>ERROR</code> - The provisioned product is under change but the requested operation failed and some remediation is occurring. For example, a rollback.</p> </li> <li> <p> <code>SUCCEEDED</code> - The requested operation has successfully completed.</p> </li> <li> <p> <code>FAILED</code> - The requested operation has unsuccessfully completed. Investigate using the error messages returned.</p> </li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The time when the record was last updated.</p>
    #[serde(rename = "UpdatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<f64>,
}

/// <p>The error code and description resulting from an operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RecordError {
    /// <p>The numeric value of the error.</p>
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// <p>The description of the error.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// <p>The output for the product created as the result of a request. For example, the output for a CloudFormation-backed product that creates an S3 bucket would include the S3 bucket URL.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RecordOutput {
    /// <p>The description of the output.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The output key.</p>
    #[serde(rename = "OutputKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_key: Option<String>,
    /// <p>The output value.</p>
    #[serde(rename = "OutputValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_value: Option<String>,
}

/// <p>Information about a tag, which is a key-value pair.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RecordTag {
    /// <p>The key for this tag.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The value for this tag.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RejectPortfolioShareRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The portfolio identifier.</p>
    #[serde(rename = "PortfolioId")]
    pub portfolio_id: String,
    /// <p>The type of shared portfolios to reject. The default is to reject imported portfolios.</p> <ul> <li> <p> <code>AWS_ORGANIZATIONS</code> - Reject portfolios shared by the master account of your organization.</p> </li> <li> <p> <code>IMPORTED</code> - Reject imported portfolios.</p> </li> <li> <p> <code>AWS_SERVICECATALOG</code> - Not supported. (Throws ResourceNotFoundException.)</p> </li> </ul> <p>For example, <code>aws servicecatalog reject-portfolio-share --portfolio-id "port-2qwzkwxt3y5fk" --portfolio-share-type AWS_ORGANIZATIONS</code> </p>
    #[serde(rename = "PortfolioShareType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portfolio_share_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RejectPortfolioShareResponse {}

/// <p>Information about a resource change that will occur when a plan is executed.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ResourceChange {
    /// <p>The change action.</p>
    #[serde(rename = "Action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// <p>Information about the resource changes.</p>
    #[serde(rename = "Details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<ResourceChangeDetail>>,
    /// <p>The ID of the resource, as defined in the CloudFormation template.</p>
    #[serde(rename = "LogicalResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical_resource_id: Option<String>,
    /// <p>The ID of the resource, if it was already created.</p>
    #[serde(rename = "PhysicalResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_resource_id: Option<String>,
    /// <p>If the change type is <code>Modify</code>, indicates whether the existing resource is deleted and replaced with a new one.</p>
    #[serde(rename = "Replacement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replacement: Option<String>,
    /// <p>The type of resource.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>The change scope.</p>
    #[serde(rename = "Scope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<Vec<String>>,
}

/// <p>Information about a change to a resource attribute.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ResourceChangeDetail {
    /// <p>The ID of the entity that caused the change.</p>
    #[serde(rename = "CausingEntity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub causing_entity: Option<String>,
    /// <p>For static evaluations, the value of the resource attribute will change and the new value is known. For dynamic evaluations, the value might change, and any new value will be determined when the plan is updated.</p>
    #[serde(rename = "Evaluation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation: Option<String>,
    /// <p>Information about the resource attribute to be modified.</p>
    #[serde(rename = "Target")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<ResourceTargetDefinition>,
}

/// <p>Information about a resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ResourceDetail {
    /// <p>The ARN of the resource.</p>
    #[serde(rename = "ARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The creation time of the resource.</p>
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    /// <p>The description of the resource.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The identifier of the resource.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the resource.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Information about a change to a resource attribute.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ResourceTargetDefinition {
    /// <p>The attribute to be changed.</p>
    #[serde(rename = "Attribute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute: Option<String>,
    /// <p>If the attribute is <code>Properties</code>, the value is the name of the property. Otherwise, the value is null.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>If the attribute is <code>Properties</code>, indicates whether a change to this property causes the resource to be re-created.</p>
    #[serde(rename = "RequiresRecreation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requires_recreation: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ScanProvisionedProductsRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The access level to use to obtain results. The default is <code>User</code>.</p>
    #[serde(rename = "AccessLevelFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_level_filter: Option<AccessLevelFilter>,
    /// <p>The maximum number of items to return with this call.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// <p>The page token for the next set of results. To retrieve the first set of results, use null.</p>
    #[serde(rename = "PageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ScanProvisionedProductsResponse {
    /// <p>The page token to use to retrieve the next set of results. If there are no additional results, this value is null.</p>
    #[serde(rename = "NextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>Information about the provisioned products.</p>
    #[serde(rename = "ProvisionedProducts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_products: Option<Vec<ProvisionedProductDetail>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SearchProductsAsAdminRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The search filters. If no search filters are specified, the output includes all products to which the administrator has access.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The maximum number of items to return with this call.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// <p>The page token for the next set of results. To retrieve the first set of results, use null.</p>
    #[serde(rename = "PageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// <p>The portfolio identifier.</p>
    #[serde(rename = "PortfolioId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portfolio_id: Option<String>,
    /// <p>Access level of the source of the product.</p>
    #[serde(rename = "ProductSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_source: Option<String>,
    /// <p>The sort field. If no value is specified, the results are not sorted.</p>
    #[serde(rename = "SortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// <p>The sort order. If no value is specified, the results are not sorted.</p>
    #[serde(rename = "SortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SearchProductsAsAdminResponse {
    /// <p>The page token to use to retrieve the next set of results. If there are no additional results, this value is null.</p>
    #[serde(rename = "NextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>Information about the product views.</p>
    #[serde(rename = "ProductViewDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_view_details: Option<Vec<ProductViewDetail>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SearchProductsRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The search filters. If no search filters are specified, the output includes all products to which the caller has access.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The maximum number of items to return with this call.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// <p>The page token for the next set of results. To retrieve the first set of results, use null.</p>
    #[serde(rename = "PageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// <p>The sort field. If no value is specified, the results are not sorted.</p>
    #[serde(rename = "SortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// <p>The sort order. If no value is specified, the results are not sorted.</p>
    #[serde(rename = "SortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SearchProductsResponse {
    /// <p>The page token to use to retrieve the next set of results. If there are no additional results, this value is null.</p>
    #[serde(rename = "NextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>The product view aggregations.</p>
    #[serde(rename = "ProductViewAggregations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_view_aggregations:
        Option<::std::collections::HashMap<String, Vec<ProductViewAggregationValue>>>,
    /// <p>Information about the product views.</p>
    #[serde(rename = "ProductViewSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_view_summaries: Option<Vec<ProductViewSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SearchProvisionedProductsRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The access level to use to obtain results. The default is <code>User</code>.</p>
    #[serde(rename = "AccessLevelFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_level_filter: Option<AccessLevelFilter>,
    /// <p>The search filters.</p> <p>When the key is <code>SearchQuery</code>, the searchable fields are <code>arn</code>, <code>createdTime</code>, <code>id</code>, <code>lastRecordId</code>, <code>idempotencyToken</code>, <code>name</code>, <code>physicalId</code>, <code>productId</code>, <code>provisioningArtifact</code>, <code>type</code>, <code>status</code>, <code>tags</code>, <code>userArn</code>, and <code>userArnSession</code>.</p> <p>Example: <code>"SearchQuery":["status:AVAILABLE"]</code> </p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The maximum number of items to return with this call.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// <p>The page token for the next set of results. To retrieve the first set of results, use null.</p>
    #[serde(rename = "PageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// <p>The sort field. If no value is specified, the results are not sorted. The valid values are <code>arn</code>, <code>id</code>, <code>name</code>, and <code>lastRecordId</code>.</p>
    #[serde(rename = "SortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// <p>The sort order. If no value is specified, the results are not sorted.</p>
    #[serde(rename = "SortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SearchProvisionedProductsResponse {
    /// <p>The page token to use to retrieve the next set of results. If there are no additional results, this value is null.</p>
    #[serde(rename = "NextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>Information about the provisioned products.</p>
    #[serde(rename = "ProvisionedProducts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_products: Option<Vec<ProvisionedProductAttribute>>,
    /// <p>The number of provisioned products found.</p>
    #[serde(rename = "TotalResultsCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_results_count: Option<i64>,
}

/// <p>A self-service action association consisting of the Action ID, the Product ID, and the Provisioning Artifact ID.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ServiceActionAssociation {
    /// <p>The product identifier. For example, <code>prod-abcdzk7xy33qa</code>.</p>
    #[serde(rename = "ProductId")]
    pub product_id: String,
    /// <p>The identifier of the provisioning artifact. For example, <code>pa-4abcdjnxjj6ne</code>.</p>
    #[serde(rename = "ProvisioningArtifactId")]
    pub provisioning_artifact_id: String,
    /// <p>The self-service action identifier. For example, <code>act-fs7abcd89wxyz</code>.</p>
    #[serde(rename = "ServiceActionId")]
    pub service_action_id: String,
}

/// <p>An object containing detailed information about the self-service action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ServiceActionDetail {
    /// <p>A map that defines the self-service action.</p>
    #[serde(rename = "Definition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<::std::collections::HashMap<String, String>>,
    /// <p>Summary information about the self-service action.</p>
    #[serde(rename = "ServiceActionSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_action_summary: Option<ServiceActionSummary>,
}

/// <p>Detailed information about the self-service action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ServiceActionSummary {
    /// <p>The self-service action definition type. For example, <code>SSM_AUTOMATION</code>.</p>
    #[serde(rename = "DefinitionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition_type: Option<String>,
    /// <p>The self-service action description.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The self-service action identifier.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The self-service action name.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Information about the portfolio share operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ShareDetails {
    /// <p>List of errors.</p>
    #[serde(rename = "ShareErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_errors: Option<Vec<ShareError>>,
    /// <p>List of accounts for whom the operation succeeded.</p>
    #[serde(rename = "SuccessfulShares")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful_shares: Option<Vec<String>>,
}

/// <p>Errors that occurred during the portfolio share operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ShareError {
    /// <p>List of accounts impacted by the error.</p>
    #[serde(rename = "Accounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<String>>,
    /// <p>Error type that happened when processing the operation.</p>
    #[serde(rename = "Error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// <p>Information about the error.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// <p>Information about a tag. A tag is a key-value pair. Tags are propagated to the resources created when provisioning a product.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>The tag key.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The value for this key.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

/// <p>Information about a TagOption.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct TagOptionDetail {
    /// <p>The TagOption active state.</p>
    #[serde(rename = "Active")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// <p>The TagOption identifier.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The TagOption key.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The TagOption value.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>Summary information about a TagOption.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct TagOptionSummary {
    /// <p>The TagOption key.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The TagOption value.</p>
    #[serde(rename = "Values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TerminateProvisionedProductRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>If set to true, AWS Service Catalog stops managing the specified provisioned product even if it cannot delete the underlying resources.</p>
    #[serde(rename = "IgnoreErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_errors: Option<bool>,
    /// <p>The identifier of the provisioned product. You cannot specify both <code>ProvisionedProductName</code> and <code>ProvisionedProductId</code>.</p>
    #[serde(rename = "ProvisionedProductId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_product_id: Option<String>,
    /// <p>The name of the provisioned product. You cannot specify both <code>ProvisionedProductName</code> and <code>ProvisionedProductId</code>.</p>
    #[serde(rename = "ProvisionedProductName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_product_name: Option<String>,
    /// <p>An idempotency token that uniquely identifies the termination request. This token is only valid during the termination process. After the provisioned product is terminated, subsequent requests to terminate the same provisioned product always return <b>ResourceNotFound</b>.</p>
    #[serde(rename = "TerminateToken")]
    pub terminate_token: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct TerminateProvisionedProductResponse {
    /// <p>Information about the result of this request.</p>
    #[serde(rename = "RecordDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_detail: Option<RecordDetail>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateConstraintRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The updated description of the constraint.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The identifier of the constraint.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p><p>The constraint parameters, in JSON format. The syntax depends on the constraint type as follows:</p> <dl> <dt>LAUNCH</dt> <dd> <p>Specify the <code>RoleArn</code> property as follows:</p> <p> <code>{&quot;RoleArn&quot; : &quot;arn:aws:iam::123456789012:role/LaunchRole&quot;}</code> </p> <p>You cannot have both a <code>LAUNCH</code> and a <code>STACKSET</code> constraint.</p> <p>You also cannot have more than one <code>LAUNCH</code> constraint on a product and portfolio.</p> </dd> <dt>NOTIFICATION</dt> <dd> <p>Specify the <code>NotificationArns</code> property as follows:</p> <p> <code>{&quot;NotificationArns&quot; : [&quot;arn:aws:sns:us-east-1:123456789012:Topic&quot;]}</code> </p> </dd> <dt>RESOURCE<em>UPDATE</dt> <dd> <p>Specify the <code>TagUpdatesOnProvisionedProduct</code> property as follows:</p> <p> <code>{&quot;Version&quot;:&quot;2.0&quot;,&quot;Properties&quot;:{&quot;TagUpdateOnProvisionedProduct&quot;:&quot;String&quot;}}</code> </p> <p>The <code>TagUpdatesOnProvisionedProduct</code> property accepts a string value of <code>ALLOWED</code> or <code>NOT</em>ALLOWED</code>.</p> </dd> <dt>STACKSET</dt> <dd> <p>Specify the <code>Parameters</code> property as follows:</p> <p> <code>{&quot;Version&quot;: &quot;String&quot;, &quot;Properties&quot;: {&quot;AccountList&quot;: [ &quot;String&quot; ], &quot;RegionList&quot;: [ &quot;String&quot; ], &quot;AdminRole&quot;: &quot;String&quot;, &quot;ExecutionRole&quot;: &quot;String&quot;}}</code> </p> <p>You cannot have both a <code>LAUNCH</code> and a <code>STACKSET</code> constraint.</p> <p>You also cannot have more than one <code>STACKSET</code> constraint on a product and portfolio.</p> <p>Products with a <code>STACKSET</code> constraint will launch an AWS CloudFormation stack set.</p> </dd> <dt>TEMPLATE</dt> <dd> <p>Specify the <code>Rules</code> property. For more information, see <a href="http://docs.aws.amazon.com/servicecatalog/latest/adminguide/reference-template_constraint_rules.html">Template Constraint Rules</a>.</p> </dd> </dl></p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateConstraintResponse {
    /// <p>Information about the constraint.</p>
    #[serde(rename = "ConstraintDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraint_detail: Option<ConstraintDetail>,
    /// <p>The constraint parameters.</p>
    #[serde(rename = "ConstraintParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraint_parameters: Option<String>,
    /// <p>The status of the current request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdatePortfolioRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The tags to add.</p>
    #[serde(rename = "AddTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_tags: Option<Vec<Tag>>,
    /// <p>The updated description of the portfolio.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name to use for display purposes.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>The portfolio identifier.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The updated name of the portfolio provider.</p>
    #[serde(rename = "ProviderName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
    /// <p>The tags to remove.</p>
    #[serde(rename = "RemoveTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_tags: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdatePortfolioResponse {
    /// <p>Information about the portfolio.</p>
    #[serde(rename = "PortfolioDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portfolio_detail: Option<PortfolioDetail>,
    /// <p>Information about the tags associated with the portfolio.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateProductRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The tags to add to the product.</p>
    #[serde(rename = "AddTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_tags: Option<Vec<Tag>>,
    /// <p>The updated description of the product.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The updated distributor of the product.</p>
    #[serde(rename = "Distributor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distributor: Option<String>,
    /// <p>The product identifier.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The updated product name.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The updated owner of the product.</p>
    #[serde(rename = "Owner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// <p>The tags to remove from the product.</p>
    #[serde(rename = "RemoveTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_tags: Option<Vec<String>>,
    /// <p>The updated support description for the product.</p>
    #[serde(rename = "SupportDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_description: Option<String>,
    /// <p>The updated support email for the product.</p>
    #[serde(rename = "SupportEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_email: Option<String>,
    /// <p>The updated support URL for the product.</p>
    #[serde(rename = "SupportUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateProductResponse {
    /// <p>Information about the product view.</p>
    #[serde(rename = "ProductViewDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_view_detail: Option<ProductViewDetail>,
    /// <p>Information about the tags associated with the product.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateProvisionedProductPropertiesRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The idempotency token that uniquely identifies the provisioning product update request.</p>
    #[serde(rename = "IdempotencyToken")]
    pub idempotency_token: String,
    /// <p>The identifier of the provisioned product.</p>
    #[serde(rename = "ProvisionedProductId")]
    pub provisioned_product_id: String,
    /// <p>A map that contains the provisioned product properties to be updated.</p> <p>The <code>OWNER</code> key only accepts user ARNs. The owner is the user that is allowed to see, update, terminate, and execute service actions in the provisioned product.</p> <p>The administrator can change the owner of a provisioned product to another IAM user within the same account. Both end user owners and administrators can see ownership history of the provisioned product using the <code>ListRecordHistory</code> API. The new owner can describe all past records for the provisioned product using the <code>DescribeRecord</code> API. The previous owner can no longer use <code>DescribeRecord</code>, but can still see the product's history from when he was an owner using <code>ListRecordHistory</code>.</p> <p>If a provisioned product ownership is assigned to an end user, they can see and perform any action through the API or Service Catalog console such as update, terminate, and execute service actions. If an end user provisions a product and the owner is updated to someone else, they will no longer be able to see or perform any actions through API or the Service Catalog console on that provisioned product.</p>
    #[serde(rename = "ProvisionedProductProperties")]
    pub provisioned_product_properties: ::std::collections::HashMap<String, String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateProvisionedProductPropertiesResponse {
    /// <p>The provisioned product identifier.</p>
    #[serde(rename = "ProvisionedProductId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_product_id: Option<String>,
    /// <p>A map that contains the properties updated.</p>
    #[serde(rename = "ProvisionedProductProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_product_properties: Option<::std::collections::HashMap<String, String>>,
    /// <p>The identifier of the record.</p>
    #[serde(rename = "RecordId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_id: Option<String>,
    /// <p>The status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateProvisionedProductRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The new path identifier. This value is optional if the product has a default path, and required if the product has more than one path.</p>
    #[serde(rename = "PathId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_id: Option<String>,
    /// <p>The identifier of the product.</p>
    #[serde(rename = "ProductId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// <p>The identifier of the provisioned product. You cannot specify both <code>ProvisionedProductName</code> and <code>ProvisionedProductId</code>.</p>
    #[serde(rename = "ProvisionedProductId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_product_id: Option<String>,
    /// <p>The name of the provisioned product. You cannot specify both <code>ProvisionedProductName</code> and <code>ProvisionedProductId</code>.</p>
    #[serde(rename = "ProvisionedProductName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_product_name: Option<String>,
    /// <p>The identifier of the provisioning artifact.</p>
    #[serde(rename = "ProvisioningArtifactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_id: Option<String>,
    /// <p>The new parameters.</p>
    #[serde(rename = "ProvisioningParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_parameters: Option<Vec<UpdateProvisioningParameter>>,
    /// <p>An object that contains information about the provisioning preferences for a stack set.</p>
    #[serde(rename = "ProvisioningPreferences")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_preferences: Option<UpdateProvisioningPreferences>,
    /// <p>One or more tags. Requires the product to have <code>RESOURCE_UPDATE</code> constraint with <code>TagUpdatesOnProvisionedProduct</code> set to <code>ALLOWED</code> to allow tag updates.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The idempotency token that uniquely identifies the provisioning update request.</p>
    #[serde(rename = "UpdateToken")]
    pub update_token: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateProvisionedProductResponse {
    /// <p>Information about the result of the request.</p>
    #[serde(rename = "RecordDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_detail: Option<RecordDetail>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateProvisioningArtifactRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>Indicates whether the product version is active.</p>
    #[serde(rename = "Active")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// <p>The updated description of the provisioning artifact.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The updated name of the provisioning artifact.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The product identifier.</p>
    #[serde(rename = "ProductId")]
    pub product_id: String,
    /// <p>The identifier of the provisioning artifact.</p>
    #[serde(rename = "ProvisioningArtifactId")]
    pub provisioning_artifact_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateProvisioningArtifactResponse {
    /// <p>The URL of the CloudFormation template in Amazon S3.</p>
    #[serde(rename = "Info")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<::std::collections::HashMap<String, String>>,
    /// <p>Information about the provisioning artifact.</p>
    #[serde(rename = "ProvisioningArtifactDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_detail: Option<ProvisioningArtifactDetail>,
    /// <p>The status of the current request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>The parameter key-value pair used to update a provisioned product.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateProvisioningParameter {
    /// <p>The parameter key.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>If set to true, <code>Value</code> is ignored and the previous parameter value is kept.</p>
    #[serde(rename = "UsePreviousValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_previous_value: Option<bool>,
    /// <p>The parameter value.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>The user-defined preferences that will be applied when updating a provisioned product. Not all preferences are applicable to all provisioned product types.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateProvisioningPreferences {
    /// <p>One or more AWS accounts that will have access to the provisioned product.</p> <p>Applicable only to a <code>CFN_STACKSET</code> provisioned product type.</p> <p>The AWS accounts specified should be within the list of accounts in the <code>STACKSET</code> constraint. To get the list of accounts in the <code>STACKSET</code> constraint, use the <code>DescribeProvisioningParameters</code> operation.</p> <p>If no values are specified, the default value is all accounts from the <code>STACKSET</code> constraint.</p>
    #[serde(rename = "StackSetAccounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_accounts: Option<Vec<String>>,
    /// <p>The number of accounts, per region, for which this operation can fail before AWS Service Catalog stops the operation in that region. If the operation is stopped in a region, AWS Service Catalog doesn't attempt the operation in any subsequent regions.</p> <p>Applicable only to a <code>CFN_STACKSET</code> provisioned product type.</p> <p>Conditional: You must specify either <code>StackSetFailureToleranceCount</code> or <code>StackSetFailureTolerancePercentage</code>, but not both.</p> <p>The default value is <code>0</code> if no value is specified.</p>
    #[serde(rename = "StackSetFailureToleranceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_failure_tolerance_count: Option<i64>,
    /// <p>The percentage of accounts, per region, for which this stack operation can fail before AWS Service Catalog stops the operation in that region. If the operation is stopped in a region, AWS Service Catalog doesn't attempt the operation in any subsequent regions.</p> <p>When calculating the number of accounts based on the specified percentage, AWS Service Catalog rounds down to the next whole number.</p> <p>Applicable only to a <code>CFN_STACKSET</code> provisioned product type.</p> <p>Conditional: You must specify either <code>StackSetFailureToleranceCount</code> or <code>StackSetFailureTolerancePercentage</code>, but not both.</p>
    #[serde(rename = "StackSetFailureTolerancePercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_failure_tolerance_percentage: Option<i64>,
    /// <p>The maximum number of accounts in which to perform this operation at one time. This is dependent on the value of <code>StackSetFailureToleranceCount</code>. <code>StackSetMaxConcurrentCount</code> is at most one more than the <code>StackSetFailureToleranceCount</code>.</p> <p>Note that this setting lets you specify the maximum for operations. For large deployments, under certain circumstances the actual number of accounts acted upon concurrently may be lower due to service throttling.</p> <p>Applicable only to a <code>CFN_STACKSET</code> provisioned product type.</p> <p>Conditional: You must specify either <code>StackSetMaxConcurrentCount</code> or <code>StackSetMaxConcurrentPercentage</code>, but not both.</p>
    #[serde(rename = "StackSetMaxConcurrencyCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_max_concurrency_count: Option<i64>,
    /// <p>The maximum percentage of accounts in which to perform this operation at one time.</p> <p>When calculating the number of accounts based on the specified percentage, AWS Service Catalog rounds down to the next whole number. This is true except in cases where rounding down would result is zero. In this case, AWS Service Catalog sets the number as <code>1</code> instead.</p> <p>Note that this setting lets you specify the maximum for operations. For large deployments, under certain circumstances the actual number of accounts acted upon concurrently may be lower due to service throttling.</p> <p>Applicable only to a <code>CFN_STACKSET</code> provisioned product type.</p> <p>Conditional: You must specify either <code>StackSetMaxConcurrentCount</code> or <code>StackSetMaxConcurrentPercentage</code>, but not both.</p>
    #[serde(rename = "StackSetMaxConcurrencyPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_max_concurrency_percentage: Option<i64>,
    /// <p><p>Determines what action AWS Service Catalog performs to a stack set or a stack instance represented by the provisioned product. The default value is <code>UPDATE</code> if nothing is specified.</p> <p>Applicable only to a <code>CFN_STACKSET</code> provisioned product type.</p> <dl> <dt>CREATE</dt> <dd> <p>Creates a new stack instance in the stack set represented by the provisioned product. In this case, only new stack instances are created based on accounts and regions; if new ProductId or ProvisioningArtifactID are passed, they will be ignored.</p> </dd> <dt>UPDATE</dt> <dd> <p>Updates the stack set represented by the provisioned product and also its stack instances.</p> </dd> <dt>DELETE</dt> <dd> <p>Deletes a stack instance in the stack set represented by the provisioned product.</p> </dd> </dl></p>
    #[serde(rename = "StackSetOperationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_operation_type: Option<String>,
    /// <p>One or more AWS Regions where the provisioned product will be available.</p> <p>Applicable only to a <code>CFN_STACKSET</code> provisioned product type.</p> <p>The specified regions should be within the list of regions from the <code>STACKSET</code> constraint. To get the list of regions in the <code>STACKSET</code> constraint, use the <code>DescribeProvisioningParameters</code> operation.</p> <p>If no values are specified, the default value is all regions from the <code>STACKSET</code> constraint.</p>
    #[serde(rename = "StackSetRegions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_regions: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateServiceActionRequest {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>A map that defines the self-service action.</p>
    #[serde(rename = "Definition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<::std::collections::HashMap<String, String>>,
    /// <p>The self-service action description.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The self-service action identifier.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The self-service action name.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateServiceActionResponse {
    /// <p>Detailed information about the self-service action.</p>
    #[serde(rename = "ServiceActionDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_action_detail: Option<ServiceActionDetail>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateTagOptionRequest {
    /// <p>The updated active state.</p>
    #[serde(rename = "Active")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// <p>The TagOption identifier.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The updated value.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateTagOptionResponse {
    /// <p>Information about the TagOption.</p>
    #[serde(rename = "TagOptionDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_option_detail: Option<TagOptionDetail>,
}

/// <p>Additional information provided by the administrator.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UsageInstruction {
    /// <p>The usage instruction type for the value.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The usage instruction value for this type.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// Errors returned by AcceptPortfolioShare
#[derive(Debug, PartialEq)]
pub enum AcceptPortfolioShareError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The current limits of the service would have been exceeded by this operation. Decrease your resource use or increase your service limits and retry the operation.</p>
    LimitExceeded(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl AcceptPortfolioShareError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AcceptPortfolioShareError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(AcceptPortfolioShareError::InvalidParameters(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(AcceptPortfolioShareError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AcceptPortfolioShareError::ResourceNotFound(
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
        }
    }
}
/// Errors returned by AssociateBudgetWithResource
#[derive(Debug, PartialEq)]
pub enum AssociateBudgetWithResourceError {
    /// <p>The specified resource is a duplicate.</p>
    DuplicateResource(String),
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The current limits of the service would have been exceeded by this operation. Decrease your resource use or increase your service limits and retry the operation.</p>
    LimitExceeded(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl AssociateBudgetWithResourceError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AssociateBudgetWithResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DuplicateResourceException" => {
                    return RusotoError::Service(
                        AssociateBudgetWithResourceError::DuplicateResource(err.msg),
                    )
                }
                "InvalidParametersException" => {
                    return RusotoError::Service(
                        AssociateBudgetWithResourceError::InvalidParameters(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(AssociateBudgetWithResourceError::LimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        AssociateBudgetWithResourceError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for AssociateBudgetWithResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AssociateBudgetWithResourceError {
    fn description(&self) -> &str {
        match *self {
            AssociateBudgetWithResourceError::DuplicateResource(ref cause) => cause,
            AssociateBudgetWithResourceError::InvalidParameters(ref cause) => cause,
            AssociateBudgetWithResourceError::LimitExceeded(ref cause) => cause,
            AssociateBudgetWithResourceError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by AssociatePrincipalWithPortfolio
#[derive(Debug, PartialEq)]
pub enum AssociatePrincipalWithPortfolioError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The current limits of the service would have been exceeded by this operation. Decrease your resource use or increase your service limits and retry the operation.</p>
    LimitExceeded(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl AssociatePrincipalWithPortfolioError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AssociatePrincipalWithPortfolioError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(
                        AssociatePrincipalWithPortfolioError::InvalidParameters(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(
                        AssociatePrincipalWithPortfolioError::LimitExceeded(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        AssociatePrincipalWithPortfolioError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by AssociateProductWithPortfolio
#[derive(Debug, PartialEq)]
pub enum AssociateProductWithPortfolioError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The current limits of the service would have been exceeded by this operation. Decrease your resource use or increase your service limits and retry the operation.</p>
    LimitExceeded(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl AssociateProductWithPortfolioError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AssociateProductWithPortfolioError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(
                        AssociateProductWithPortfolioError::InvalidParameters(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(AssociateProductWithPortfolioError::LimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        AssociateProductWithPortfolioError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by AssociateServiceActionWithProvisioningArtifact
#[derive(Debug, PartialEq)]
pub enum AssociateServiceActionWithProvisioningArtifactError {
    /// <p>The specified resource is a duplicate.</p>
    DuplicateResource(String),
    /// <p>The current limits of the service would have been exceeded by this operation. Decrease your resource use or increase your service limits and retry the operation.</p>
    LimitExceeded(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl AssociateServiceActionWithProvisioningArtifactError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AssociateServiceActionWithProvisioningArtifactError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DuplicateResourceException" => {
                    return RusotoError::Service(
                        AssociateServiceActionWithProvisioningArtifactError::DuplicateResource(
                            err.msg,
                        ),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(
                        AssociateServiceActionWithProvisioningArtifactError::LimitExceeded(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        AssociateServiceActionWithProvisioningArtifactError::ResourceNotFound(
                            err.msg,
                        ),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for AssociateServiceActionWithProvisioningArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AssociateServiceActionWithProvisioningArtifactError {
    fn description(&self) -> &str {
        match *self {
            AssociateServiceActionWithProvisioningArtifactError::DuplicateResource(ref cause) => {
                cause
            }
            AssociateServiceActionWithProvisioningArtifactError::LimitExceeded(ref cause) => cause,
            AssociateServiceActionWithProvisioningArtifactError::ResourceNotFound(ref cause) => {
                cause
            }
        }
    }
}
/// Errors returned by AssociateTagOptionWithResource
#[derive(Debug, PartialEq)]
pub enum AssociateTagOptionWithResourceError {
    /// <p>The specified resource is a duplicate.</p>
    DuplicateResource(String),
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>An attempt was made to modify a resource that is in a state that is not valid. Check your resources to ensure that they are in valid states before retrying the operation.</p>
    InvalidState(String),
    /// <p>The current limits of the service would have been exceeded by this operation. Decrease your resource use or increase your service limits and retry the operation.</p>
    LimitExceeded(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>An operation requiring TagOptions failed because the TagOptions migration process has not been performed for this account. Please use the AWS console to perform the migration process before retrying the operation.</p>
    TagOptionNotMigrated(String),
}

impl AssociateTagOptionWithResourceError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AssociateTagOptionWithResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DuplicateResourceException" => {
                    return RusotoError::Service(
                        AssociateTagOptionWithResourceError::DuplicateResource(err.msg),
                    )
                }
                "InvalidParametersException" => {
                    return RusotoError::Service(
                        AssociateTagOptionWithResourceError::InvalidParameters(err.msg),
                    )
                }
                "InvalidStateException" => {
                    return RusotoError::Service(AssociateTagOptionWithResourceError::InvalidState(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(
                        AssociateTagOptionWithResourceError::LimitExceeded(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        AssociateTagOptionWithResourceError::ResourceNotFound(err.msg),
                    )
                }
                "TagOptionNotMigratedException" => {
                    return RusotoError::Service(
                        AssociateTagOptionWithResourceError::TagOptionNotMigrated(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for AssociateTagOptionWithResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AssociateTagOptionWithResourceError {
    fn description(&self) -> &str {
        match *self {
            AssociateTagOptionWithResourceError::DuplicateResource(ref cause) => cause,
            AssociateTagOptionWithResourceError::InvalidParameters(ref cause) => cause,
            AssociateTagOptionWithResourceError::InvalidState(ref cause) => cause,
            AssociateTagOptionWithResourceError::LimitExceeded(ref cause) => cause,
            AssociateTagOptionWithResourceError::ResourceNotFound(ref cause) => cause,
            AssociateTagOptionWithResourceError::TagOptionNotMigrated(ref cause) => cause,
        }
    }
}
/// Errors returned by BatchAssociateServiceActionWithProvisioningArtifact
#[derive(Debug, PartialEq)]
pub enum BatchAssociateServiceActionWithProvisioningArtifactError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
}

impl BatchAssociateServiceActionWithProvisioningArtifactError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<BatchAssociateServiceActionWithProvisioningArtifactError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(
                        BatchAssociateServiceActionWithProvisioningArtifactError::InvalidParameters(
                            err.msg,
                        ),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for BatchAssociateServiceActionWithProvisioningArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchAssociateServiceActionWithProvisioningArtifactError {
    fn description(&self) -> &str {
        match *self {
            BatchAssociateServiceActionWithProvisioningArtifactError::InvalidParameters(
                ref cause,
            ) => cause,
        }
    }
}
/// Errors returned by BatchDisassociateServiceActionFromProvisioningArtifact
#[derive(Debug, PartialEq)]
pub enum BatchDisassociateServiceActionFromProvisioningArtifactError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
}

impl BatchDisassociateServiceActionFromProvisioningArtifactError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<BatchDisassociateServiceActionFromProvisioningArtifactError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => return RusotoError::Service(
                    BatchDisassociateServiceActionFromProvisioningArtifactError::InvalidParameters(
                        err.msg,
                    ),
                ),
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for BatchDisassociateServiceActionFromProvisioningArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchDisassociateServiceActionFromProvisioningArtifactError {
    fn description(&self) -> &str {
        match *self {
            BatchDisassociateServiceActionFromProvisioningArtifactError::InvalidParameters(
                ref cause,
            ) => cause,
        }
    }
}
/// Errors returned by CopyProduct
#[derive(Debug, PartialEq)]
pub enum CopyProductError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl CopyProductError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CopyProductError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(CopyProductError::InvalidParameters(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CopyProductError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CopyProductError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CopyProductError {
    fn description(&self) -> &str {
        match *self {
            CopyProductError::InvalidParameters(ref cause) => cause,
            CopyProductError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateConstraint
#[derive(Debug, PartialEq)]
pub enum CreateConstraintError {
    /// <p>The specified resource is a duplicate.</p>
    DuplicateResource(String),
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The current limits of the service would have been exceeded by this operation. Decrease your resource use or increase your service limits and retry the operation.</p>
    LimitExceeded(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl CreateConstraintError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateConstraintError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DuplicateResourceException" => {
                    return RusotoError::Service(CreateConstraintError::DuplicateResource(err.msg))
                }
                "InvalidParametersException" => {
                    return RusotoError::Service(CreateConstraintError::InvalidParameters(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateConstraintError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateConstraintError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by CreatePortfolio
#[derive(Debug, PartialEq)]
pub enum CreatePortfolioError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The current limits of the service would have been exceeded by this operation. Decrease your resource use or increase your service limits and retry the operation.</p>
    LimitExceeded(String),
    /// <p>An operation requiring TagOptions failed because the TagOptions migration process has not been performed for this account. Please use the AWS console to perform the migration process before retrying the operation.</p>
    TagOptionNotMigrated(String),
}

impl CreatePortfolioError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreatePortfolioError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(CreatePortfolioError::InvalidParameters(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreatePortfolioError::LimitExceeded(err.msg))
                }
                "TagOptionNotMigratedException" => {
                    return RusotoError::Service(CreatePortfolioError::TagOptionNotMigrated(
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
            CreatePortfolioError::TagOptionNotMigrated(ref cause) => cause,
        }
    }
}
/// Errors returned by CreatePortfolioShare
#[derive(Debug, PartialEq)]
pub enum CreatePortfolioShareError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The current limits of the service would have been exceeded by this operation. Decrease your resource use or increase your service limits and retry the operation.</p>
    LimitExceeded(String),
    /// <p>The operation is not supported.</p>
    OperationNotSupported(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl CreatePortfolioShareError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreatePortfolioShareError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(CreatePortfolioShareError::InvalidParameters(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreatePortfolioShareError::LimitExceeded(err.msg))
                }
                "OperationNotSupportedException" => {
                    return RusotoError::Service(CreatePortfolioShareError::OperationNotSupported(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreatePortfolioShareError::ResourceNotFound(
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
            CreatePortfolioShareError::OperationNotSupported(ref cause) => cause,
            CreatePortfolioShareError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateProduct
#[derive(Debug, PartialEq)]
pub enum CreateProductError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The current limits of the service would have been exceeded by this operation. Decrease your resource use or increase your service limits and retry the operation.</p>
    LimitExceeded(String),
    /// <p>An operation requiring TagOptions failed because the TagOptions migration process has not been performed for this account. Please use the AWS console to perform the migration process before retrying the operation.</p>
    TagOptionNotMigrated(String),
}

impl CreateProductError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateProductError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(CreateProductError::InvalidParameters(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateProductError::LimitExceeded(err.msg))
                }
                "TagOptionNotMigratedException" => {
                    return RusotoError::Service(CreateProductError::TagOptionNotMigrated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            CreateProductError::TagOptionNotMigrated(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateProvisionedProductPlan
#[derive(Debug, PartialEq)]
pub enum CreateProvisionedProductPlanError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>An attempt was made to modify a resource that is in a state that is not valid. Check your resources to ensure that they are in valid states before retrying the operation.</p>
    InvalidState(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl CreateProvisionedProductPlanError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateProvisionedProductPlanError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(
                        CreateProvisionedProductPlanError::InvalidParameters(err.msg),
                    )
                }
                "InvalidStateException" => {
                    return RusotoError::Service(CreateProvisionedProductPlanError::InvalidState(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        CreateProvisionedProductPlanError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateProvisionedProductPlanError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateProvisionedProductPlanError {
    fn description(&self) -> &str {
        match *self {
            CreateProvisionedProductPlanError::InvalidParameters(ref cause) => cause,
            CreateProvisionedProductPlanError::InvalidState(ref cause) => cause,
            CreateProvisionedProductPlanError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateProvisioningArtifact
#[derive(Debug, PartialEq)]
pub enum CreateProvisioningArtifactError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The current limits of the service would have been exceeded by this operation. Decrease your resource use or increase your service limits and retry the operation.</p>
    LimitExceeded(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl CreateProvisioningArtifactError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateProvisioningArtifactError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(
                        CreateProvisioningArtifactError::InvalidParameters(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateProvisioningArtifactError::LimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateProvisioningArtifactError::ResourceNotFound(
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
        }
    }
}
/// Errors returned by CreateServiceAction
#[derive(Debug, PartialEq)]
pub enum CreateServiceActionError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The current limits of the service would have been exceeded by this operation. Decrease your resource use or increase your service limits and retry the operation.</p>
    LimitExceeded(String),
}

impl CreateServiceActionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateServiceActionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(CreateServiceActionError::InvalidParameters(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateServiceActionError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateServiceActionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateServiceActionError {
    fn description(&self) -> &str {
        match *self {
            CreateServiceActionError::InvalidParameters(ref cause) => cause,
            CreateServiceActionError::LimitExceeded(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateTagOption
#[derive(Debug, PartialEq)]
pub enum CreateTagOptionError {
    /// <p>The specified resource is a duplicate.</p>
    DuplicateResource(String),
    /// <p>The current limits of the service would have been exceeded by this operation. Decrease your resource use or increase your service limits and retry the operation.</p>
    LimitExceeded(String),
    /// <p>An operation requiring TagOptions failed because the TagOptions migration process has not been performed for this account. Please use the AWS console to perform the migration process before retrying the operation.</p>
    TagOptionNotMigrated(String),
}

impl CreateTagOptionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateTagOptionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DuplicateResourceException" => {
                    return RusotoError::Service(CreateTagOptionError::DuplicateResource(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateTagOptionError::LimitExceeded(err.msg))
                }
                "TagOptionNotMigratedException" => {
                    return RusotoError::Service(CreateTagOptionError::TagOptionNotMigrated(
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
impl fmt::Display for CreateTagOptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateTagOptionError {
    fn description(&self) -> &str {
        match *self {
            CreateTagOptionError::DuplicateResource(ref cause) => cause,
            CreateTagOptionError::LimitExceeded(ref cause) => cause,
            CreateTagOptionError::TagOptionNotMigrated(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteConstraint
#[derive(Debug, PartialEq)]
pub enum DeleteConstraintError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DeleteConstraintError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteConstraintError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(DeleteConstraintError::InvalidParameters(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteConstraintError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by DeletePortfolio
#[derive(Debug, PartialEq)]
pub enum DeletePortfolioError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>A resource that is currently in use. Ensure that the resource is not in use and retry the operation.</p>
    ResourceInUse(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>An operation requiring TagOptions failed because the TagOptions migration process has not been performed for this account. Please use the AWS console to perform the migration process before retrying the operation.</p>
    TagOptionNotMigrated(String),
}

impl DeletePortfolioError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeletePortfolioError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(DeletePortfolioError::InvalidParameters(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeletePortfolioError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeletePortfolioError::ResourceNotFound(err.msg))
                }
                "TagOptionNotMigratedException" => {
                    return RusotoError::Service(DeletePortfolioError::TagOptionNotMigrated(
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
            DeletePortfolioError::TagOptionNotMigrated(ref cause) => cause,
        }
    }
}
/// Errors returned by DeletePortfolioShare
#[derive(Debug, PartialEq)]
pub enum DeletePortfolioShareError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The operation is not supported.</p>
    OperationNotSupported(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DeletePortfolioShareError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeletePortfolioShareError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(DeletePortfolioShareError::InvalidParameters(
                        err.msg,
                    ))
                }
                "OperationNotSupportedException" => {
                    return RusotoError::Service(DeletePortfolioShareError::OperationNotSupported(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeletePortfolioShareError::ResourceNotFound(
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
impl fmt::Display for DeletePortfolioShareError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeletePortfolioShareError {
    fn description(&self) -> &str {
        match *self {
            DeletePortfolioShareError::InvalidParameters(ref cause) => cause,
            DeletePortfolioShareError::OperationNotSupported(ref cause) => cause,
            DeletePortfolioShareError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteProduct
#[derive(Debug, PartialEq)]
pub enum DeleteProductError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>A resource that is currently in use. Ensure that the resource is not in use and retry the operation.</p>
    ResourceInUse(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>An operation requiring TagOptions failed because the TagOptions migration process has not been performed for this account. Please use the AWS console to perform the migration process before retrying the operation.</p>
    TagOptionNotMigrated(String),
}

impl DeleteProductError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteProductError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(DeleteProductError::InvalidParameters(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteProductError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteProductError::ResourceNotFound(err.msg))
                }
                "TagOptionNotMigratedException" => {
                    return RusotoError::Service(DeleteProductError::TagOptionNotMigrated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            DeleteProductError::TagOptionNotMigrated(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteProvisionedProductPlan
#[derive(Debug, PartialEq)]
pub enum DeleteProvisionedProductPlanError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DeleteProvisionedProductPlanError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteProvisionedProductPlanError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(
                        DeleteProvisionedProductPlanError::InvalidParameters(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DeleteProvisionedProductPlanError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteProvisionedProductPlanError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteProvisionedProductPlanError {
    fn description(&self) -> &str {
        match *self {
            DeleteProvisionedProductPlanError::InvalidParameters(ref cause) => cause,
            DeleteProvisionedProductPlanError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteProvisioningArtifact
#[derive(Debug, PartialEq)]
pub enum DeleteProvisioningArtifactError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>A resource that is currently in use. Ensure that the resource is not in use and retry the operation.</p>
    ResourceInUse(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DeleteProvisioningArtifactError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteProvisioningArtifactError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(
                        DeleteProvisioningArtifactError::InvalidParameters(err.msg),
                    )
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteProvisioningArtifactError::ResourceInUse(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteProvisioningArtifactError::ResourceNotFound(
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
        }
    }
}
/// Errors returned by DeleteServiceAction
#[derive(Debug, PartialEq)]
pub enum DeleteServiceActionError {
    /// <p>A resource that is currently in use. Ensure that the resource is not in use and retry the operation.</p>
    ResourceInUse(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DeleteServiceActionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteServiceActionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteServiceActionError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteServiceActionError::ResourceNotFound(
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
impl fmt::Display for DeleteServiceActionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteServiceActionError {
    fn description(&self) -> &str {
        match *self {
            DeleteServiceActionError::ResourceInUse(ref cause) => cause,
            DeleteServiceActionError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteTagOption
#[derive(Debug, PartialEq)]
pub enum DeleteTagOptionError {
    /// <p>A resource that is currently in use. Ensure that the resource is not in use and retry the operation.</p>
    ResourceInUse(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>An operation requiring TagOptions failed because the TagOptions migration process has not been performed for this account. Please use the AWS console to perform the migration process before retrying the operation.</p>
    TagOptionNotMigrated(String),
}

impl DeleteTagOptionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteTagOptionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteTagOptionError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteTagOptionError::ResourceNotFound(err.msg))
                }
                "TagOptionNotMigratedException" => {
                    return RusotoError::Service(DeleteTagOptionError::TagOptionNotMigrated(
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
impl fmt::Display for DeleteTagOptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteTagOptionError {
    fn description(&self) -> &str {
        match *self {
            DeleteTagOptionError::ResourceInUse(ref cause) => cause,
            DeleteTagOptionError::ResourceNotFound(ref cause) => cause,
            DeleteTagOptionError::TagOptionNotMigrated(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeConstraint
#[derive(Debug, PartialEq)]
pub enum DescribeConstraintError {
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DescribeConstraintError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeConstraintError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeConstraintError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by DescribeCopyProductStatus
#[derive(Debug, PartialEq)]
pub enum DescribeCopyProductStatusError {
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DescribeCopyProductStatusError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeCopyProductStatusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeCopyProductStatusError::ResourceNotFound(
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
impl fmt::Display for DescribeCopyProductStatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeCopyProductStatusError {
    fn description(&self) -> &str {
        match *self {
            DescribeCopyProductStatusError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribePortfolio
#[derive(Debug, PartialEq)]
pub enum DescribePortfolioError {
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DescribePortfolioError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribePortfolioError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribePortfolioError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by DescribePortfolioShareStatus
#[derive(Debug, PartialEq)]
pub enum DescribePortfolioShareStatusError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The operation is not supported.</p>
    OperationNotSupported(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DescribePortfolioShareStatusError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribePortfolioShareStatusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(
                        DescribePortfolioShareStatusError::InvalidParameters(err.msg),
                    )
                }
                "OperationNotSupportedException" => {
                    return RusotoError::Service(
                        DescribePortfolioShareStatusError::OperationNotSupported(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DescribePortfolioShareStatusError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribePortfolioShareStatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribePortfolioShareStatusError {
    fn description(&self) -> &str {
        match *self {
            DescribePortfolioShareStatusError::InvalidParameters(ref cause) => cause,
            DescribePortfolioShareStatusError::OperationNotSupported(ref cause) => cause,
            DescribePortfolioShareStatusError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeProduct
#[derive(Debug, PartialEq)]
pub enum DescribeProductError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DescribeProductError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeProductError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(DescribeProductError::InvalidParameters(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeProductError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by DescribeProductAsAdmin
#[derive(Debug, PartialEq)]
pub enum DescribeProductAsAdminError {
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DescribeProductAsAdminError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeProductAsAdminError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeProductAsAdminError::ResourceNotFound(
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
impl fmt::Display for DescribeProductAsAdminError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeProductAsAdminError {
    fn description(&self) -> &str {
        match *self {
            DescribeProductAsAdminError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeProductView
#[derive(Debug, PartialEq)]
pub enum DescribeProductViewError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DescribeProductViewError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeProductViewError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(DescribeProductViewError::InvalidParameters(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeProductViewError::ResourceNotFound(
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
        }
    }
}
/// Errors returned by DescribeProvisionedProduct
#[derive(Debug, PartialEq)]
pub enum DescribeProvisionedProductError {
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DescribeProvisionedProductError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeProvisionedProductError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeProvisionedProductError::ResourceNotFound(
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
impl fmt::Display for DescribeProvisionedProductError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeProvisionedProductError {
    fn description(&self) -> &str {
        match *self {
            DescribeProvisionedProductError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeProvisionedProductPlan
#[derive(Debug, PartialEq)]
pub enum DescribeProvisionedProductPlanError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DescribeProvisionedProductPlanError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeProvisionedProductPlanError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(
                        DescribeProvisionedProductPlanError::InvalidParameters(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DescribeProvisionedProductPlanError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeProvisionedProductPlanError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeProvisionedProductPlanError {
    fn description(&self) -> &str {
        match *self {
            DescribeProvisionedProductPlanError::InvalidParameters(ref cause) => cause,
            DescribeProvisionedProductPlanError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeProvisioningArtifact
#[derive(Debug, PartialEq)]
pub enum DescribeProvisioningArtifactError {
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DescribeProvisioningArtifactError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeProvisioningArtifactError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DescribeProvisioningArtifactError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by DescribeProvisioningParameters
#[derive(Debug, PartialEq)]
pub enum DescribeProvisioningParametersError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DescribeProvisioningParametersError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeProvisioningParametersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(
                        DescribeProvisioningParametersError::InvalidParameters(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DescribeProvisioningParametersError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by DescribeRecord
#[derive(Debug, PartialEq)]
pub enum DescribeRecordError {
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DescribeRecordError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeRecordError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeRecordError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by DescribeServiceAction
#[derive(Debug, PartialEq)]
pub enum DescribeServiceActionError {
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DescribeServiceActionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeServiceActionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeServiceActionError::ResourceNotFound(
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
impl fmt::Display for DescribeServiceActionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeServiceActionError {
    fn description(&self) -> &str {
        match *self {
            DescribeServiceActionError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeTagOption
#[derive(Debug, PartialEq)]
pub enum DescribeTagOptionError {
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>An operation requiring TagOptions failed because the TagOptions migration process has not been performed for this account. Please use the AWS console to perform the migration process before retrying the operation.</p>
    TagOptionNotMigrated(String),
}

impl DescribeTagOptionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeTagOptionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeTagOptionError::ResourceNotFound(err.msg))
                }
                "TagOptionNotMigratedException" => {
                    return RusotoError::Service(DescribeTagOptionError::TagOptionNotMigrated(
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
impl fmt::Display for DescribeTagOptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeTagOptionError {
    fn description(&self) -> &str {
        match *self {
            DescribeTagOptionError::ResourceNotFound(ref cause) => cause,
            DescribeTagOptionError::TagOptionNotMigrated(ref cause) => cause,
        }
    }
}
/// Errors returned by DisableAWSOrganizationsAccess
#[derive(Debug, PartialEq)]
pub enum DisableAWSOrganizationsAccessError {
    /// <p>An attempt was made to modify a resource that is in a state that is not valid. Check your resources to ensure that they are in valid states before retrying the operation.</p>
    InvalidState(String),
    /// <p>The operation is not supported.</p>
    OperationNotSupported(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DisableAWSOrganizationsAccessError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisableAWSOrganizationsAccessError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidStateException" => {
                    return RusotoError::Service(DisableAWSOrganizationsAccessError::InvalidState(
                        err.msg,
                    ))
                }
                "OperationNotSupportedException" => {
                    return RusotoError::Service(
                        DisableAWSOrganizationsAccessError::OperationNotSupported(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DisableAWSOrganizationsAccessError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DisableAWSOrganizationsAccessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisableAWSOrganizationsAccessError {
    fn description(&self) -> &str {
        match *self {
            DisableAWSOrganizationsAccessError::InvalidState(ref cause) => cause,
            DisableAWSOrganizationsAccessError::OperationNotSupported(ref cause) => cause,
            DisableAWSOrganizationsAccessError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by DisassociateBudgetFromResource
#[derive(Debug, PartialEq)]
pub enum DisassociateBudgetFromResourceError {
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DisassociateBudgetFromResourceError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisassociateBudgetFromResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DisassociateBudgetFromResourceError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DisassociateBudgetFromResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisassociateBudgetFromResourceError {
    fn description(&self) -> &str {
        match *self {
            DisassociateBudgetFromResourceError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by DisassociatePrincipalFromPortfolio
#[derive(Debug, PartialEq)]
pub enum DisassociatePrincipalFromPortfolioError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DisassociatePrincipalFromPortfolioError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisassociatePrincipalFromPortfolioError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(
                        DisassociatePrincipalFromPortfolioError::InvalidParameters(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DisassociatePrincipalFromPortfolioError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by DisassociateProductFromPortfolio
#[derive(Debug, PartialEq)]
pub enum DisassociateProductFromPortfolioError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>A resource that is currently in use. Ensure that the resource is not in use and retry the operation.</p>
    ResourceInUse(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DisassociateProductFromPortfolioError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisassociateProductFromPortfolioError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(
                        DisassociateProductFromPortfolioError::InvalidParameters(err.msg),
                    )
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(
                        DisassociateProductFromPortfolioError::ResourceInUse(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DisassociateProductFromPortfolioError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            DisassociateProductFromPortfolioError::ResourceInUse(ref cause) => cause,
            DisassociateProductFromPortfolioError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by DisassociateServiceActionFromProvisioningArtifact
#[derive(Debug, PartialEq)]
pub enum DisassociateServiceActionFromProvisioningArtifactError {
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DisassociateServiceActionFromProvisioningArtifactError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisassociateServiceActionFromProvisioningArtifactError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DisassociateServiceActionFromProvisioningArtifactError::ResourceNotFound(
                            err.msg,
                        ),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DisassociateServiceActionFromProvisioningArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisassociateServiceActionFromProvisioningArtifactError {
    fn description(&self) -> &str {
        match *self {
            DisassociateServiceActionFromProvisioningArtifactError::ResourceNotFound(ref cause) => {
                cause
            }
        }
    }
}
/// Errors returned by DisassociateTagOptionFromResource
#[derive(Debug, PartialEq)]
pub enum DisassociateTagOptionFromResourceError {
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>An operation requiring TagOptions failed because the TagOptions migration process has not been performed for this account. Please use the AWS console to perform the migration process before retrying the operation.</p>
    TagOptionNotMigrated(String),
}

impl DisassociateTagOptionFromResourceError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisassociateTagOptionFromResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DisassociateTagOptionFromResourceError::ResourceNotFound(err.msg),
                    )
                }
                "TagOptionNotMigratedException" => {
                    return RusotoError::Service(
                        DisassociateTagOptionFromResourceError::TagOptionNotMigrated(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DisassociateTagOptionFromResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisassociateTagOptionFromResourceError {
    fn description(&self) -> &str {
        match *self {
            DisassociateTagOptionFromResourceError::ResourceNotFound(ref cause) => cause,
            DisassociateTagOptionFromResourceError::TagOptionNotMigrated(ref cause) => cause,
        }
    }
}
/// Errors returned by EnableAWSOrganizationsAccess
#[derive(Debug, PartialEq)]
pub enum EnableAWSOrganizationsAccessError {
    /// <p>An attempt was made to modify a resource that is in a state that is not valid. Check your resources to ensure that they are in valid states before retrying the operation.</p>
    InvalidState(String),
    /// <p>The operation is not supported.</p>
    OperationNotSupported(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl EnableAWSOrganizationsAccessError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<EnableAWSOrganizationsAccessError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidStateException" => {
                    return RusotoError::Service(EnableAWSOrganizationsAccessError::InvalidState(
                        err.msg,
                    ))
                }
                "OperationNotSupportedException" => {
                    return RusotoError::Service(
                        EnableAWSOrganizationsAccessError::OperationNotSupported(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        EnableAWSOrganizationsAccessError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for EnableAWSOrganizationsAccessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for EnableAWSOrganizationsAccessError {
    fn description(&self) -> &str {
        match *self {
            EnableAWSOrganizationsAccessError::InvalidState(ref cause) => cause,
            EnableAWSOrganizationsAccessError::OperationNotSupported(ref cause) => cause,
            EnableAWSOrganizationsAccessError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by ExecuteProvisionedProductPlan
#[derive(Debug, PartialEq)]
pub enum ExecuteProvisionedProductPlanError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>An attempt was made to modify a resource that is in a state that is not valid. Check your resources to ensure that they are in valid states before retrying the operation.</p>
    InvalidState(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl ExecuteProvisionedProductPlanError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ExecuteProvisionedProductPlanError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(
                        ExecuteProvisionedProductPlanError::InvalidParameters(err.msg),
                    )
                }
                "InvalidStateException" => {
                    return RusotoError::Service(ExecuteProvisionedProductPlanError::InvalidState(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        ExecuteProvisionedProductPlanError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ExecuteProvisionedProductPlanError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ExecuteProvisionedProductPlanError {
    fn description(&self) -> &str {
        match *self {
            ExecuteProvisionedProductPlanError::InvalidParameters(ref cause) => cause,
            ExecuteProvisionedProductPlanError::InvalidState(ref cause) => cause,
            ExecuteProvisionedProductPlanError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by ExecuteProvisionedProductServiceAction
#[derive(Debug, PartialEq)]
pub enum ExecuteProvisionedProductServiceActionError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>An attempt was made to modify a resource that is in a state that is not valid. Check your resources to ensure that they are in valid states before retrying the operation.</p>
    InvalidState(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl ExecuteProvisionedProductServiceActionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ExecuteProvisionedProductServiceActionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(
                        ExecuteProvisionedProductServiceActionError::InvalidParameters(err.msg),
                    )
                }
                "InvalidStateException" => {
                    return RusotoError::Service(
                        ExecuteProvisionedProductServiceActionError::InvalidState(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        ExecuteProvisionedProductServiceActionError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ExecuteProvisionedProductServiceActionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ExecuteProvisionedProductServiceActionError {
    fn description(&self) -> &str {
        match *self {
            ExecuteProvisionedProductServiceActionError::InvalidParameters(ref cause) => cause,
            ExecuteProvisionedProductServiceActionError::InvalidState(ref cause) => cause,
            ExecuteProvisionedProductServiceActionError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by GetAWSOrganizationsAccessStatus
#[derive(Debug, PartialEq)]
pub enum GetAWSOrganizationsAccessStatusError {
    /// <p>The operation is not supported.</p>
    OperationNotSupported(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl GetAWSOrganizationsAccessStatusError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetAWSOrganizationsAccessStatusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "OperationNotSupportedException" => {
                    return RusotoError::Service(
                        GetAWSOrganizationsAccessStatusError::OperationNotSupported(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        GetAWSOrganizationsAccessStatusError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetAWSOrganizationsAccessStatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetAWSOrganizationsAccessStatusError {
    fn description(&self) -> &str {
        match *self {
            GetAWSOrganizationsAccessStatusError::OperationNotSupported(ref cause) => cause,
            GetAWSOrganizationsAccessStatusError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by ListAcceptedPortfolioShares
#[derive(Debug, PartialEq)]
pub enum ListAcceptedPortfolioSharesError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The operation is not supported.</p>
    OperationNotSupported(String),
}

impl ListAcceptedPortfolioSharesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListAcceptedPortfolioSharesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(
                        ListAcceptedPortfolioSharesError::InvalidParameters(err.msg),
                    )
                }
                "OperationNotSupportedException" => {
                    return RusotoError::Service(
                        ListAcceptedPortfolioSharesError::OperationNotSupported(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            ListAcceptedPortfolioSharesError::OperationNotSupported(ref cause) => cause,
        }
    }
}
/// Errors returned by ListBudgetsForResource
#[derive(Debug, PartialEq)]
pub enum ListBudgetsForResourceError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl ListBudgetsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListBudgetsForResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(ListBudgetsForResourceError::InvalidParameters(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListBudgetsForResourceError::ResourceNotFound(
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
impl fmt::Display for ListBudgetsForResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListBudgetsForResourceError {
    fn description(&self) -> &str {
        match *self {
            ListBudgetsForResourceError::InvalidParameters(ref cause) => cause,
            ListBudgetsForResourceError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by ListConstraintsForPortfolio
#[derive(Debug, PartialEq)]
pub enum ListConstraintsForPortfolioError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl ListConstraintsForPortfolioError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListConstraintsForPortfolioError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(
                        ListConstraintsForPortfolioError::InvalidParameters(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        ListConstraintsForPortfolioError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by ListLaunchPaths
#[derive(Debug, PartialEq)]
pub enum ListLaunchPathsError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl ListLaunchPathsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListLaunchPathsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(ListLaunchPathsError::InvalidParameters(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListLaunchPathsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by ListOrganizationPortfolioAccess
#[derive(Debug, PartialEq)]
pub enum ListOrganizationPortfolioAccessError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The operation is not supported.</p>
    OperationNotSupported(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl ListOrganizationPortfolioAccessError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListOrganizationPortfolioAccessError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(
                        ListOrganizationPortfolioAccessError::InvalidParameters(err.msg),
                    )
                }
                "OperationNotSupportedException" => {
                    return RusotoError::Service(
                        ListOrganizationPortfolioAccessError::OperationNotSupported(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        ListOrganizationPortfolioAccessError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListOrganizationPortfolioAccessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListOrganizationPortfolioAccessError {
    fn description(&self) -> &str {
        match *self {
            ListOrganizationPortfolioAccessError::InvalidParameters(ref cause) => cause,
            ListOrganizationPortfolioAccessError::OperationNotSupported(ref cause) => cause,
            ListOrganizationPortfolioAccessError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by ListPortfolioAccess
#[derive(Debug, PartialEq)]
pub enum ListPortfolioAccessError {
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl ListPortfolioAccessError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListPortfolioAccessError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListPortfolioAccessError::ResourceNotFound(
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
impl fmt::Display for ListPortfolioAccessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListPortfolioAccessError {
    fn description(&self) -> &str {
        match *self {
            ListPortfolioAccessError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by ListPortfolios
#[derive(Debug, PartialEq)]
pub enum ListPortfoliosError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
}

impl ListPortfoliosError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListPortfoliosError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(ListPortfoliosError::InvalidParameters(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by ListPortfoliosForProduct
#[derive(Debug, PartialEq)]
pub enum ListPortfoliosForProductError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl ListPortfoliosForProductError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListPortfoliosForProductError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(ListPortfoliosForProductError::InvalidParameters(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListPortfoliosForProductError::ResourceNotFound(
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
        }
    }
}
/// Errors returned by ListPrincipalsForPortfolio
#[derive(Debug, PartialEq)]
pub enum ListPrincipalsForPortfolioError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl ListPrincipalsForPortfolioError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListPrincipalsForPortfolioError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(
                        ListPrincipalsForPortfolioError::InvalidParameters(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListPrincipalsForPortfolioError::ResourceNotFound(
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
        }
    }
}
/// Errors returned by ListProvisionedProductPlans
#[derive(Debug, PartialEq)]
pub enum ListProvisionedProductPlansError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl ListProvisionedProductPlansError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListProvisionedProductPlansError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(
                        ListProvisionedProductPlansError::InvalidParameters(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        ListProvisionedProductPlansError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListProvisionedProductPlansError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListProvisionedProductPlansError {
    fn description(&self) -> &str {
        match *self {
            ListProvisionedProductPlansError::InvalidParameters(ref cause) => cause,
            ListProvisionedProductPlansError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by ListProvisioningArtifacts
#[derive(Debug, PartialEq)]
pub enum ListProvisioningArtifactsError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl ListProvisioningArtifactsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListProvisioningArtifactsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(ListProvisioningArtifactsError::InvalidParameters(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListProvisioningArtifactsError::ResourceNotFound(
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
        }
    }
}
/// Errors returned by ListProvisioningArtifactsForServiceAction
#[derive(Debug, PartialEq)]
pub enum ListProvisioningArtifactsForServiceActionError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl ListProvisioningArtifactsForServiceActionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListProvisioningArtifactsForServiceActionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(
                        ListProvisioningArtifactsForServiceActionError::InvalidParameters(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        ListProvisioningArtifactsForServiceActionError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListProvisioningArtifactsForServiceActionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListProvisioningArtifactsForServiceActionError {
    fn description(&self) -> &str {
        match *self {
            ListProvisioningArtifactsForServiceActionError::InvalidParameters(ref cause) => cause,
            ListProvisioningArtifactsForServiceActionError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by ListRecordHistory
#[derive(Debug, PartialEq)]
pub enum ListRecordHistoryError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
}

impl ListRecordHistoryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListRecordHistoryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(ListRecordHistoryError::InvalidParameters(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by ListResourcesForTagOption
#[derive(Debug, PartialEq)]
pub enum ListResourcesForTagOptionError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>An operation requiring TagOptions failed because the TagOptions migration process has not been performed for this account. Please use the AWS console to perform the migration process before retrying the operation.</p>
    TagOptionNotMigrated(String),
}

impl ListResourcesForTagOptionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListResourcesForTagOptionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(ListResourcesForTagOptionError::InvalidParameters(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListResourcesForTagOptionError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "TagOptionNotMigratedException" => {
                    return RusotoError::Service(
                        ListResourcesForTagOptionError::TagOptionNotMigrated(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListResourcesForTagOptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListResourcesForTagOptionError {
    fn description(&self) -> &str {
        match *self {
            ListResourcesForTagOptionError::InvalidParameters(ref cause) => cause,
            ListResourcesForTagOptionError::ResourceNotFound(ref cause) => cause,
            ListResourcesForTagOptionError::TagOptionNotMigrated(ref cause) => cause,
        }
    }
}
/// Errors returned by ListServiceActions
#[derive(Debug, PartialEq)]
pub enum ListServiceActionsError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
}

impl ListServiceActionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListServiceActionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(ListServiceActionsError::InvalidParameters(
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
impl fmt::Display for ListServiceActionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListServiceActionsError {
    fn description(&self) -> &str {
        match *self {
            ListServiceActionsError::InvalidParameters(ref cause) => cause,
        }
    }
}
/// Errors returned by ListServiceActionsForProvisioningArtifact
#[derive(Debug, PartialEq)]
pub enum ListServiceActionsForProvisioningArtifactError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl ListServiceActionsForProvisioningArtifactError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListServiceActionsForProvisioningArtifactError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(
                        ListServiceActionsForProvisioningArtifactError::InvalidParameters(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        ListServiceActionsForProvisioningArtifactError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListServiceActionsForProvisioningArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListServiceActionsForProvisioningArtifactError {
    fn description(&self) -> &str {
        match *self {
            ListServiceActionsForProvisioningArtifactError::InvalidParameters(ref cause) => cause,
            ListServiceActionsForProvisioningArtifactError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTagOptions
#[derive(Debug, PartialEq)]
pub enum ListTagOptionsError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>An operation requiring TagOptions failed because the TagOptions migration process has not been performed for this account. Please use the AWS console to perform the migration process before retrying the operation.</p>
    TagOptionNotMigrated(String),
}

impl ListTagOptionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagOptionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(ListTagOptionsError::InvalidParameters(err.msg))
                }
                "TagOptionNotMigratedException" => {
                    return RusotoError::Service(ListTagOptionsError::TagOptionNotMigrated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListTagOptionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTagOptionsError {
    fn description(&self) -> &str {
        match *self {
            ListTagOptionsError::InvalidParameters(ref cause) => cause,
            ListTagOptionsError::TagOptionNotMigrated(ref cause) => cause,
        }
    }
}
/// Errors returned by ProvisionProduct
#[derive(Debug, PartialEq)]
pub enum ProvisionProductError {
    /// <p>The specified resource is a duplicate.</p>
    DuplicateResource(String),
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl ProvisionProductError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ProvisionProductError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DuplicateResourceException" => {
                    return RusotoError::Service(ProvisionProductError::DuplicateResource(err.msg))
                }
                "InvalidParametersException" => {
                    return RusotoError::Service(ProvisionProductError::InvalidParameters(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ProvisionProductError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by RejectPortfolioShare
#[derive(Debug, PartialEq)]
pub enum RejectPortfolioShareError {
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl RejectPortfolioShareError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RejectPortfolioShareError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(RejectPortfolioShareError::ResourceNotFound(
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
impl fmt::Display for RejectPortfolioShareError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RejectPortfolioShareError {
    fn description(&self) -> &str {
        match *self {
            RejectPortfolioShareError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by ScanProvisionedProducts
#[derive(Debug, PartialEq)]
pub enum ScanProvisionedProductsError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
}

impl ScanProvisionedProductsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ScanProvisionedProductsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(ScanProvisionedProductsError::InvalidParameters(
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
impl fmt::Display for ScanProvisionedProductsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ScanProvisionedProductsError {
    fn description(&self) -> &str {
        match *self {
            ScanProvisionedProductsError::InvalidParameters(ref cause) => cause,
        }
    }
}
/// Errors returned by SearchProducts
#[derive(Debug, PartialEq)]
pub enum SearchProductsError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
}

impl SearchProductsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SearchProductsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(SearchProductsError::InvalidParameters(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by SearchProductsAsAdmin
#[derive(Debug, PartialEq)]
pub enum SearchProductsAsAdminError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl SearchProductsAsAdminError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SearchProductsAsAdminError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(SearchProductsAsAdminError::InvalidParameters(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(SearchProductsAsAdminError::ResourceNotFound(
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
        }
    }
}
/// Errors returned by SearchProvisionedProducts
#[derive(Debug, PartialEq)]
pub enum SearchProvisionedProductsError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
}

impl SearchProvisionedProductsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SearchProvisionedProductsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(SearchProvisionedProductsError::InvalidParameters(
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
impl fmt::Display for SearchProvisionedProductsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SearchProvisionedProductsError {
    fn description(&self) -> &str {
        match *self {
            SearchProvisionedProductsError::InvalidParameters(ref cause) => cause,
        }
    }
}
/// Errors returned by TerminateProvisionedProduct
#[derive(Debug, PartialEq)]
pub enum TerminateProvisionedProductError {
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl TerminateProvisionedProductError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<TerminateProvisionedProductError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        TerminateProvisionedProductError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by UpdateConstraint
#[derive(Debug, PartialEq)]
pub enum UpdateConstraintError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl UpdateConstraintError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateConstraintError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(UpdateConstraintError::InvalidParameters(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateConstraintError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by UpdatePortfolio
#[derive(Debug, PartialEq)]
pub enum UpdatePortfolioError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The current limits of the service would have been exceeded by this operation. Decrease your resource use or increase your service limits and retry the operation.</p>
    LimitExceeded(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>An operation requiring TagOptions failed because the TagOptions migration process has not been performed for this account. Please use the AWS console to perform the migration process before retrying the operation.</p>
    TagOptionNotMigrated(String),
}

impl UpdatePortfolioError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdatePortfolioError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(UpdatePortfolioError::InvalidParameters(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdatePortfolioError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdatePortfolioError::ResourceNotFound(err.msg))
                }
                "TagOptionNotMigratedException" => {
                    return RusotoError::Service(UpdatePortfolioError::TagOptionNotMigrated(
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
            UpdatePortfolioError::TagOptionNotMigrated(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateProduct
#[derive(Debug, PartialEq)]
pub enum UpdateProductError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>An operation requiring TagOptions failed because the TagOptions migration process has not been performed for this account. Please use the AWS console to perform the migration process before retrying the operation.</p>
    TagOptionNotMigrated(String),
}

impl UpdateProductError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateProductError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(UpdateProductError::InvalidParameters(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateProductError::ResourceNotFound(err.msg))
                }
                "TagOptionNotMigratedException" => {
                    return RusotoError::Service(UpdateProductError::TagOptionNotMigrated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            UpdateProductError::TagOptionNotMigrated(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateProvisionedProduct
#[derive(Debug, PartialEq)]
pub enum UpdateProvisionedProductError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl UpdateProvisionedProductError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateProvisionedProductError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(UpdateProvisionedProductError::InvalidParameters(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateProvisionedProductError::ResourceNotFound(
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
        }
    }
}
/// Errors returned by UpdateProvisionedProductProperties
#[derive(Debug, PartialEq)]
pub enum UpdateProvisionedProductPropertiesError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>An attempt was made to modify a resource that is in a state that is not valid. Check your resources to ensure that they are in valid states before retrying the operation.</p>
    InvalidState(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl UpdateProvisionedProductPropertiesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateProvisionedProductPropertiesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(
                        UpdateProvisionedProductPropertiesError::InvalidParameters(err.msg),
                    )
                }
                "InvalidStateException" => {
                    return RusotoError::Service(
                        UpdateProvisionedProductPropertiesError::InvalidState(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        UpdateProvisionedProductPropertiesError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateProvisionedProductPropertiesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateProvisionedProductPropertiesError {
    fn description(&self) -> &str {
        match *self {
            UpdateProvisionedProductPropertiesError::InvalidParameters(ref cause) => cause,
            UpdateProvisionedProductPropertiesError::InvalidState(ref cause) => cause,
            UpdateProvisionedProductPropertiesError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateProvisioningArtifact
#[derive(Debug, PartialEq)]
pub enum UpdateProvisioningArtifactError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl UpdateProvisioningArtifactError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateProvisioningArtifactError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(
                        UpdateProvisioningArtifactError::InvalidParameters(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateProvisioningArtifactError::ResourceNotFound(
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
        }
    }
}
/// Errors returned by UpdateServiceAction
#[derive(Debug, PartialEq)]
pub enum UpdateServiceActionError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl UpdateServiceActionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateServiceActionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(UpdateServiceActionError::InvalidParameters(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateServiceActionError::ResourceNotFound(
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
impl fmt::Display for UpdateServiceActionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateServiceActionError {
    fn description(&self) -> &str {
        match *self {
            UpdateServiceActionError::InvalidParameters(ref cause) => cause,
            UpdateServiceActionError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateTagOption
#[derive(Debug, PartialEq)]
pub enum UpdateTagOptionError {
    /// <p>The specified resource is a duplicate.</p>
    DuplicateResource(String),
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>An operation requiring TagOptions failed because the TagOptions migration process has not been performed for this account. Please use the AWS console to perform the migration process before retrying the operation.</p>
    TagOptionNotMigrated(String),
}

impl UpdateTagOptionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateTagOptionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DuplicateResourceException" => {
                    return RusotoError::Service(UpdateTagOptionError::DuplicateResource(err.msg))
                }
                "InvalidParametersException" => {
                    return RusotoError::Service(UpdateTagOptionError::InvalidParameters(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateTagOptionError::ResourceNotFound(err.msg))
                }
                "TagOptionNotMigratedException" => {
                    return RusotoError::Service(UpdateTagOptionError::TagOptionNotMigrated(
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
impl fmt::Display for UpdateTagOptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateTagOptionError {
    fn description(&self) -> &str {
        match *self {
            UpdateTagOptionError::DuplicateResource(ref cause) => cause,
            UpdateTagOptionError::InvalidParameters(ref cause) => cause,
            UpdateTagOptionError::ResourceNotFound(ref cause) => cause,
            UpdateTagOptionError::TagOptionNotMigrated(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the AWS Service Catalog API. AWS Service Catalog clients implement this trait.
pub trait ServiceCatalog {
    /// <p>Accepts an offer to share the specified portfolio.</p>
    fn accept_portfolio_share(
        &self,
        input: AcceptPortfolioShareRequest,
    ) -> Request<AcceptPortfolioShareRequest>;

    /// <p>Associates the specified budget with the specified resource.</p>
    fn associate_budget_with_resource(
        &self,
        input: AssociateBudgetWithResourceRequest,
    ) -> Request<AssociateBudgetWithResourceRequest>;

    /// <p>Associates the specified principal ARN with the specified portfolio.</p>
    fn associate_principal_with_portfolio(
        &self,
        input: AssociatePrincipalWithPortfolioRequest,
    ) -> Request<AssociatePrincipalWithPortfolioRequest>;

    /// <p>Associates the specified product with the specified portfolio.</p>
    fn associate_product_with_portfolio(
        &self,
        input: AssociateProductWithPortfolioRequest,
    ) -> Request<AssociateProductWithPortfolioRequest>;

    /// <p>Associates a self-service action with a provisioning artifact.</p>
    fn associate_service_action_with_provisioning_artifact(
        &self,
        input: AssociateServiceActionWithProvisioningArtifactRequest,
    ) -> Request<AssociateServiceActionWithProvisioningArtifactRequest>;

    /// <p>Associate the specified TagOption with the specified portfolio or product.</p>
    fn associate_tag_option_with_resource(
        &self,
        input: AssociateTagOptionWithResourceRequest,
    ) -> Request<AssociateTagOptionWithResourceRequest>;

    /// <p>Associates multiple self-service actions with provisioning artifacts.</p>
    fn batch_associate_service_action_with_provisioning_artifact(
        &self,
        input: BatchAssociateServiceActionWithProvisioningArtifactRequest,
    ) -> Request<BatchAssociateServiceActionWithProvisioningArtifactRequest>;

    /// <p>Disassociates a batch of self-service actions from the specified provisioning artifact.</p>
    fn batch_disassociate_service_action_from_provisioning_artifact(
        &self,
        input: BatchDisassociateServiceActionFromProvisioningArtifactRequest,
    ) -> Request<BatchDisassociateServiceActionFromProvisioningArtifactRequest>;

    /// <p>Copies the specified source product to the specified target product or a new product.</p> <p>You can copy a product to the same account or another account. You can copy a product to the same region or another region.</p> <p>This operation is performed asynchronously. To track the progress of the operation, use <a>DescribeCopyProductStatus</a>.</p>
    fn copy_product(&self, input: CopyProductRequest) -> Request<CopyProductRequest>;

    /// <p>Creates a constraint.</p>
    fn create_constraint(&self, input: CreateConstraintRequest)
        -> Request<CreateConstraintRequest>;

    /// <p>Creates a portfolio.</p>
    fn create_portfolio(&self, input: CreatePortfolioRequest) -> Request<CreatePortfolioRequest>;

    /// <p>Shares the specified portfolio with the specified account or organization node. Shares to an organization node can only be created by the master account of an Organization. AWSOrganizationsAccess must be enabled in order to create a portfolio share to an organization node.</p>
    fn create_portfolio_share(
        &self,
        input: CreatePortfolioShareRequest,
    ) -> Request<CreatePortfolioShareRequest>;

    /// <p>Creates a product.</p>
    fn create_product(&self, input: CreateProductRequest) -> Request<CreateProductRequest>;

    /// <p>Creates a plan. A plan includes the list of resources to be created (when provisioning a new product) or modified (when updating a provisioned product) when the plan is executed.</p> <p>You can create one plan per provisioned product. To create a plan for an existing provisioned product, the product status must be AVAILBLE or TAINTED.</p> <p>To view the resource changes in the change set, use <a>DescribeProvisionedProductPlan</a>. To create or modify the provisioned product, use <a>ExecuteProvisionedProductPlan</a>.</p>
    fn create_provisioned_product_plan(
        &self,
        input: CreateProvisionedProductPlanRequest,
    ) -> Request<CreateProvisionedProductPlanRequest>;

    /// <p>Creates a provisioning artifact (also known as a version) for the specified product.</p> <p>You cannot create a provisioning artifact for a product that was shared with you.</p>
    fn create_provisioning_artifact(
        &self,
        input: CreateProvisioningArtifactRequest,
    ) -> Request<CreateProvisioningArtifactRequest>;

    /// <p>Creates a self-service action.</p>
    fn create_service_action(
        &self,
        input: CreateServiceActionRequest,
    ) -> Request<CreateServiceActionRequest>;

    /// <p>Creates a TagOption.</p>
    fn create_tag_option(&self, input: CreateTagOptionRequest) -> Request<CreateTagOptionRequest>;

    /// <p>Deletes the specified constraint.</p>
    fn delete_constraint(&self, input: DeleteConstraintRequest)
        -> Request<DeleteConstraintRequest>;

    /// <p>Deletes the specified portfolio.</p> <p>You cannot delete a portfolio if it was shared with you or if it has associated products, users, constraints, or shared accounts.</p>
    fn delete_portfolio(&self, input: DeletePortfolioRequest) -> Request<DeletePortfolioRequest>;

    /// <p>Stops sharing the specified portfolio with the specified account or organization node. Shares to an organization node can only be deleted by the master account of an Organization.</p>
    fn delete_portfolio_share(
        &self,
        input: DeletePortfolioShareRequest,
    ) -> Request<DeletePortfolioShareRequest>;

    /// <p>Deletes the specified product.</p> <p>You cannot delete a product if it was shared with you or is associated with a portfolio.</p>
    fn delete_product(&self, input: DeleteProductRequest) -> Request<DeleteProductRequest>;

    /// <p>Deletes the specified plan.</p>
    fn delete_provisioned_product_plan(
        &self,
        input: DeleteProvisionedProductPlanRequest,
    ) -> Request<DeleteProvisionedProductPlanRequest>;

    /// <p>Deletes the specified provisioning artifact (also known as a version) for the specified product.</p> <p>You cannot delete a provisioning artifact associated with a product that was shared with you. You cannot delete the last provisioning artifact for a product, because a product must have at least one provisioning artifact.</p>
    fn delete_provisioning_artifact(
        &self,
        input: DeleteProvisioningArtifactRequest,
    ) -> Request<DeleteProvisioningArtifactRequest>;

    /// <p>Deletes a self-service action.</p>
    fn delete_service_action(
        &self,
        input: DeleteServiceActionRequest,
    ) -> Request<DeleteServiceActionRequest>;

    /// <p>Deletes the specified TagOption.</p> <p>You cannot delete a TagOption if it is associated with a product or portfolio.</p>
    fn delete_tag_option(&self, input: DeleteTagOptionRequest) -> Request<DeleteTagOptionRequest>;

    /// <p>Gets information about the specified constraint.</p>
    fn describe_constraint(
        &self,
        input: DescribeConstraintRequest,
    ) -> Request<DescribeConstraintRequest>;

    /// <p>Gets the status of the specified copy product operation.</p>
    fn describe_copy_product_status(
        &self,
        input: DescribeCopyProductStatusRequest,
    ) -> Request<DescribeCopyProductStatusRequest>;

    /// <p>Gets information about the specified portfolio.</p>
    fn describe_portfolio(
        &self,
        input: DescribePortfolioRequest,
    ) -> Request<DescribePortfolioRequest>;

    /// <p>Gets the status of the specified portfolio share operation. This API can only be called by the master account in the organization.</p>
    fn describe_portfolio_share_status(
        &self,
        input: DescribePortfolioShareStatusRequest,
    ) -> Request<DescribePortfolioShareStatusRequest>;

    /// <p>Gets information about the specified product.</p>
    fn describe_product(&self, input: DescribeProductRequest) -> Request<DescribeProductRequest>;

    /// <p>Gets information about the specified product. This operation is run with administrator access.</p>
    fn describe_product_as_admin(
        &self,
        input: DescribeProductAsAdminRequest,
    ) -> Request<DescribeProductAsAdminRequest>;

    /// <p>Gets information about the specified product.</p>
    fn describe_product_view(
        &self,
        input: DescribeProductViewRequest,
    ) -> Request<DescribeProductViewRequest>;

    /// <p>Gets information about the specified provisioned product.</p>
    fn describe_provisioned_product(
        &self,
        input: DescribeProvisionedProductRequest,
    ) -> Request<DescribeProvisionedProductRequest>;

    /// <p>Gets information about the resource changes for the specified plan.</p>
    fn describe_provisioned_product_plan(
        &self,
        input: DescribeProvisionedProductPlanRequest,
    ) -> Request<DescribeProvisionedProductPlanRequest>;

    /// <p>Gets information about the specified provisioning artifact (also known as a version) for the specified product.</p>
    fn describe_provisioning_artifact(
        &self,
        input: DescribeProvisioningArtifactRequest,
    ) -> Request<DescribeProvisioningArtifactRequest>;

    /// <p>Gets information about the configuration required to provision the specified product using the specified provisioning artifact.</p> <p>If the output contains a TagOption key with an empty list of values, there is a TagOption conflict for that key. The end user cannot take action to fix the conflict, and launch is not blocked. In subsequent calls to <a>ProvisionProduct</a>, do not include conflicted TagOption keys as tags, or this causes the error "Parameter validation failed: Missing required parameter in Tags[<i>N</i>]:<i>Value</i>". Tag the provisioned product with the value <code>sc-tagoption-conflict-portfolioId-productId</code>.</p>
    fn describe_provisioning_parameters(
        &self,
        input: DescribeProvisioningParametersRequest,
    ) -> Request<DescribeProvisioningParametersRequest>;

    /// <p><p>Gets information about the specified request operation.</p> <p>Use this operation after calling a request operation (for example, <a>ProvisionProduct</a>, <a>TerminateProvisionedProduct</a>, or <a>UpdateProvisionedProduct</a>). </p> <note> <p>If a provisioned product was transferred to a new owner using <a>UpdateProvisionedProductProperties</a>, the new owner will be able to describe all past records for that product. The previous owner will no longer be able to describe the records, but will be able to use <a>ListRecordHistory</a> to see the product&#39;s history from when he was the owner.</p> </note></p>
    fn describe_record(&self, input: DescribeRecordRequest) -> Request<DescribeRecordRequest>;

    /// <p>Describes a self-service action.</p>
    fn describe_service_action(
        &self,
        input: DescribeServiceActionRequest,
    ) -> Request<DescribeServiceActionRequest>;

    /// <p>Gets information about the specified TagOption.</p>
    fn describe_tag_option(
        &self,
        input: DescribeTagOptionRequest,
    ) -> Request<DescribeTagOptionRequest>;

    /// <p>Disable portfolio sharing through AWS Organizations feature. This feature will not delete your current shares but it will prevent you from creating new shares throughout your organization. Current shares will not be in sync with your organization structure if it changes after calling this API. This API can only be called by the master account in the organization.</p>
    fn disable_aws_organizations_access(&self) -> Request<DisableAWSOrganizationsAccessRequest>;

    /// <p>Disassociates the specified budget from the specified resource.</p>
    fn disassociate_budget_from_resource(
        &self,
        input: DisassociateBudgetFromResourceRequest,
    ) -> Request<DisassociateBudgetFromResourceRequest>;

    /// <p>Disassociates a previously associated principal ARN from a specified portfolio.</p>
    fn disassociate_principal_from_portfolio(
        &self,
        input: DisassociatePrincipalFromPortfolioRequest,
    ) -> Request<DisassociatePrincipalFromPortfolioRequest>;

    /// <p>Disassociates the specified product from the specified portfolio. </p>
    fn disassociate_product_from_portfolio(
        &self,
        input: DisassociateProductFromPortfolioRequest,
    ) -> Request<DisassociateProductFromPortfolioRequest>;

    /// <p>Disassociates the specified self-service action association from the specified provisioning artifact.</p>
    fn disassociate_service_action_from_provisioning_artifact(
        &self,
        input: DisassociateServiceActionFromProvisioningArtifactRequest,
    ) -> Request<DisassociateServiceActionFromProvisioningArtifactRequest>;

    /// <p>Disassociates the specified TagOption from the specified resource.</p>
    fn disassociate_tag_option_from_resource(
        &self,
        input: DisassociateTagOptionFromResourceRequest,
    ) -> Request<DisassociateTagOptionFromResourceRequest>;

    /// <p>Enable portfolio sharing feature through AWS Organizations. This API will allow Service Catalog to receive updates on your organization in order to sync your shares with the current structure. This API can only be called by the master account in the organization.</p> <p>By calling this API Service Catalog will make a call to organizations:EnableAWSServiceAccess on your behalf so that your shares can be in sync with any changes in your AWS Organizations structure.</p>
    fn enable_aws_organizations_access(&self) -> Request<EnableAWSOrganizationsAccessRequest>;

    /// <p>Provisions or modifies a product based on the resource changes for the specified plan.</p>
    fn execute_provisioned_product_plan(
        &self,
        input: ExecuteProvisionedProductPlanRequest,
    ) -> Request<ExecuteProvisionedProductPlanRequest>;

    /// <p>Executes a self-service action against a provisioned product.</p>
    fn execute_provisioned_product_service_action(
        &self,
        input: ExecuteProvisionedProductServiceActionRequest,
    ) -> Request<ExecuteProvisionedProductServiceActionRequest>;

    /// <p>Get the Access Status for AWS Organization portfolio share feature. This API can only be called by the master account in the organization.</p>
    fn get_aws_organizations_access_status(
        &self,
    ) -> Request<GetAWSOrganizationsAccessStatusRequest>;

    /// <p>Lists all portfolios for which sharing was accepted by this account.</p>
    fn list_accepted_portfolio_shares(
        &self,
        input: ListAcceptedPortfolioSharesRequest,
    ) -> Request<ListAcceptedPortfolioSharesRequest>;

    /// <p>Lists all the budgets associated to the specified resource.</p>
    fn list_budgets_for_resource(
        &self,
        input: ListBudgetsForResourceRequest,
    ) -> Request<ListBudgetsForResourceRequest>;

    /// <p>Lists the constraints for the specified portfolio and product.</p>
    fn list_constraints_for_portfolio(
        &self,
        input: ListConstraintsForPortfolioRequest,
    ) -> Request<ListConstraintsForPortfolioRequest>;

    /// <p>Lists the paths to the specified product. A path is how the user has access to a specified product, and is necessary when provisioning a product. A path also determines the constraints put on the product.</p>
    fn list_launch_paths(&self, input: ListLaunchPathsRequest) -> Request<ListLaunchPathsRequest>;

    /// <p>Lists the organization nodes that have access to the specified portfolio. This API can only be called by the master account in the organization.</p>
    fn list_organization_portfolio_access(
        &self,
        input: ListOrganizationPortfolioAccessRequest,
    ) -> Request<ListOrganizationPortfolioAccessRequest>;

    /// <p>Lists the account IDs that have access to the specified portfolio.</p>
    fn list_portfolio_access(
        &self,
        input: ListPortfolioAccessRequest,
    ) -> Request<ListPortfolioAccessRequest>;

    /// <p>Lists all portfolios in the catalog.</p>
    fn list_portfolios(&self, input: ListPortfoliosRequest) -> Request<ListPortfoliosRequest>;

    /// <p>Lists all portfolios that the specified product is associated with.</p>
    fn list_portfolios_for_product(
        &self,
        input: ListPortfoliosForProductRequest,
    ) -> Request<ListPortfoliosForProductRequest>;

    /// <p>Lists all principal ARNs associated with the specified portfolio.</p>
    fn list_principals_for_portfolio(
        &self,
        input: ListPrincipalsForPortfolioRequest,
    ) -> Request<ListPrincipalsForPortfolioRequest>;

    /// <p>Lists the plans for the specified provisioned product or all plans to which the user has access.</p>
    fn list_provisioned_product_plans(
        &self,
        input: ListProvisionedProductPlansRequest,
    ) -> Request<ListProvisionedProductPlansRequest>;

    /// <p>Lists all provisioning artifacts (also known as versions) for the specified product.</p>
    fn list_provisioning_artifacts(
        &self,
        input: ListProvisioningArtifactsRequest,
    ) -> Request<ListProvisioningArtifactsRequest>;

    /// <p>Lists all provisioning artifacts (also known as versions) for the specified self-service action.</p>
    fn list_provisioning_artifacts_for_service_action(
        &self,
        input: ListProvisioningArtifactsForServiceActionRequest,
    ) -> Request<ListProvisioningArtifactsForServiceActionRequest>;

    /// <p>Lists the specified requests or all performed requests.</p>
    fn list_record_history(
        &self,
        input: ListRecordHistoryRequest,
    ) -> Request<ListRecordHistoryRequest>;

    /// <p>Lists the resources associated with the specified TagOption.</p>
    fn list_resources_for_tag_option(
        &self,
        input: ListResourcesForTagOptionRequest,
    ) -> Request<ListResourcesForTagOptionRequest>;

    /// <p>Lists all self-service actions.</p>
    fn list_service_actions(
        &self,
        input: ListServiceActionsRequest,
    ) -> Request<ListServiceActionsRequest>;

    /// <p>Returns a paginated list of self-service actions associated with the specified Product ID and Provisioning Artifact ID.</p>
    fn list_service_actions_for_provisioning_artifact(
        &self,
        input: ListServiceActionsForProvisioningArtifactRequest,
    ) -> Request<ListServiceActionsForProvisioningArtifactRequest>;

    /// <p>Lists the specified TagOptions or all TagOptions.</p>
    fn list_tag_options(&self, input: ListTagOptionsRequest) -> Request<ListTagOptionsRequest>;

    /// <p>Provisions the specified product.</p> <p>A provisioned product is a resourced instance of a product. For example, provisioning a product based on a CloudFormation template launches a CloudFormation stack and its underlying resources. You can check the status of this request using <a>DescribeRecord</a>.</p> <p>If the request contains a tag key with an empty list of values, there is a tag conflict for that key. Do not include conflicted keys as tags, or this causes the error "Parameter validation failed: Missing required parameter in Tags[<i>N</i>]:<i>Value</i>".</p>
    fn provision_product(&self, input: ProvisionProductRequest)
        -> Request<ProvisionProductRequest>;

    /// <p>Rejects an offer to share the specified portfolio.</p>
    fn reject_portfolio_share(
        &self,
        input: RejectPortfolioShareRequest,
    ) -> Request<RejectPortfolioShareRequest>;

    /// <p>Lists the provisioned products that are available (not terminated).</p> <p>To use additional filtering, see <a>SearchProvisionedProducts</a>.</p>
    fn scan_provisioned_products(
        &self,
        input: ScanProvisionedProductsRequest,
    ) -> Request<ScanProvisionedProductsRequest>;

    /// <p>Gets information about the products to which the caller has access.</p>
    fn search_products(&self, input: SearchProductsRequest) -> Request<SearchProductsRequest>;

    /// <p>Gets information about the products for the specified portfolio or all products.</p>
    fn search_products_as_admin(
        &self,
        input: SearchProductsAsAdminRequest,
    ) -> Request<SearchProductsAsAdminRequest>;

    /// <p>Gets information about the provisioned products that meet the specified criteria.</p>
    fn search_provisioned_products(
        &self,
        input: SearchProvisionedProductsRequest,
    ) -> Request<SearchProvisionedProductsRequest>;

    /// <p>Terminates the specified provisioned product.</p> <p>This operation does not delete any records associated with the provisioned product.</p> <p>You can check the status of this request using <a>DescribeRecord</a>.</p>
    fn terminate_provisioned_product(
        &self,
        input: TerminateProvisionedProductRequest,
    ) -> Request<TerminateProvisionedProductRequest>;

    /// <p>Updates the specified constraint.</p>
    fn update_constraint(&self, input: UpdateConstraintRequest)
        -> Request<UpdateConstraintRequest>;

    /// <p>Updates the specified portfolio.</p> <p>You cannot update a product that was shared with you.</p>
    fn update_portfolio(&self, input: UpdatePortfolioRequest) -> Request<UpdatePortfolioRequest>;

    /// <p>Updates the specified product.</p>
    fn update_product(&self, input: UpdateProductRequest) -> Request<UpdateProductRequest>;

    /// <p>Requests updates to the configuration of the specified provisioned product.</p> <p>If there are tags associated with the object, they cannot be updated or added. Depending on the specific updates requested, this operation can update with no interruption, with some interruption, or replace the provisioned product entirely.</p> <p>You can check the status of this request using <a>DescribeRecord</a>.</p>
    fn update_provisioned_product(
        &self,
        input: UpdateProvisionedProductRequest,
    ) -> Request<UpdateProvisionedProductRequest>;

    /// <p>Requests updates to the properties of the specified provisioned product.</p>
    fn update_provisioned_product_properties(
        &self,
        input: UpdateProvisionedProductPropertiesRequest,
    ) -> Request<UpdateProvisionedProductPropertiesRequest>;

    /// <p>Updates the specified provisioning artifact (also known as a version) for the specified product.</p> <p>You cannot update a provisioning artifact for a product that was shared with you.</p>
    fn update_provisioning_artifact(
        &self,
        input: UpdateProvisioningArtifactRequest,
    ) -> Request<UpdateProvisioningArtifactRequest>;

    /// <p>Updates a self-service action.</p>
    fn update_service_action(
        &self,
        input: UpdateServiceActionRequest,
    ) -> Request<UpdateServiceActionRequest>;

    /// <p>Updates the specified TagOption.</p>
    fn update_tag_option(&self, input: UpdateTagOptionRequest) -> Request<UpdateTagOptionRequest>;
}
/// A client for the AWS Service Catalog API.
#[derive(Clone)]
pub struct ServiceCatalogClient {
    client: Client,
    region: region::Region,
}

impl ServiceCatalogClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> ServiceCatalogClient {
        ServiceCatalogClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> ServiceCatalogClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        ServiceCatalogClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }
}

impl ServiceCatalog for ServiceCatalogClient {
    /// <p>Accepts an offer to share the specified portfolio.</p>
    fn accept_portfolio_share(
        &self,
        input: AcceptPortfolioShareRequest,
    ) -> Request<AcceptPortfolioShareRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Associates the specified budget with the specified resource.</p>
    fn associate_budget_with_resource(
        &self,
        input: AssociateBudgetWithResourceRequest,
    ) -> Request<AssociateBudgetWithResourceRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Associates the specified principal ARN with the specified portfolio.</p>
    fn associate_principal_with_portfolio(
        &self,
        input: AssociatePrincipalWithPortfolioRequest,
    ) -> Request<AssociatePrincipalWithPortfolioRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Associates the specified product with the specified portfolio.</p>
    fn associate_product_with_portfolio(
        &self,
        input: AssociateProductWithPortfolioRequest,
    ) -> Request<AssociateProductWithPortfolioRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Associates a self-service action with a provisioning artifact.</p>
    fn associate_service_action_with_provisioning_artifact(
        &self,
        input: AssociateServiceActionWithProvisioningArtifactRequest,
    ) -> Request<AssociateServiceActionWithProvisioningArtifactRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Associate the specified TagOption with the specified portfolio or product.</p>
    fn associate_tag_option_with_resource(
        &self,
        input: AssociateTagOptionWithResourceRequest,
    ) -> Request<AssociateTagOptionWithResourceRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Associates multiple self-service actions with provisioning artifacts.</p>
    fn batch_associate_service_action_with_provisioning_artifact(
        &self,
        input: BatchAssociateServiceActionWithProvisioningArtifactRequest,
    ) -> Request<BatchAssociateServiceActionWithProvisioningArtifactRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Disassociates a batch of self-service actions from the specified provisioning artifact.</p>
    fn batch_disassociate_service_action_from_provisioning_artifact(
        &self,
        input: BatchDisassociateServiceActionFromProvisioningArtifactRequest,
    ) -> Request<BatchDisassociateServiceActionFromProvisioningArtifactRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Copies the specified source product to the specified target product or a new product.</p> <p>You can copy a product to the same account or another account. You can copy a product to the same region or another region.</p> <p>This operation is performed asynchronously. To track the progress of the operation, use <a>DescribeCopyProductStatus</a>.</p>
    fn copy_product(&self, input: CopyProductRequest) -> Request<CopyProductRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Creates a constraint.</p>
    fn create_constraint(
        &self,
        input: CreateConstraintRequest,
    ) -> Request<CreateConstraintRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Creates a portfolio.</p>
    fn create_portfolio(&self, input: CreatePortfolioRequest) -> Request<CreatePortfolioRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Shares the specified portfolio with the specified account or organization node. Shares to an organization node can only be created by the master account of an Organization. AWSOrganizationsAccess must be enabled in order to create a portfolio share to an organization node.</p>
    fn create_portfolio_share(
        &self,
        input: CreatePortfolioShareRequest,
    ) -> Request<CreatePortfolioShareRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Creates a product.</p>
    fn create_product(&self, input: CreateProductRequest) -> Request<CreateProductRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Creates a plan. A plan includes the list of resources to be created (when provisioning a new product) or modified (when updating a provisioned product) when the plan is executed.</p> <p>You can create one plan per provisioned product. To create a plan for an existing provisioned product, the product status must be AVAILBLE or TAINTED.</p> <p>To view the resource changes in the change set, use <a>DescribeProvisionedProductPlan</a>. To create or modify the provisioned product, use <a>ExecuteProvisionedProductPlan</a>.</p>
    fn create_provisioned_product_plan(
        &self,
        input: CreateProvisionedProductPlanRequest,
    ) -> Request<CreateProvisionedProductPlanRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Creates a provisioning artifact (also known as a version) for the specified product.</p> <p>You cannot create a provisioning artifact for a product that was shared with you.</p>
    fn create_provisioning_artifact(
        &self,
        input: CreateProvisioningArtifactRequest,
    ) -> Request<CreateProvisioningArtifactRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Creates a self-service action.</p>
    fn create_service_action(
        &self,
        input: CreateServiceActionRequest,
    ) -> Request<CreateServiceActionRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Creates a TagOption.</p>
    fn create_tag_option(&self, input: CreateTagOptionRequest) -> Request<CreateTagOptionRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Deletes the specified constraint.</p>
    fn delete_constraint(
        &self,
        input: DeleteConstraintRequest,
    ) -> Request<DeleteConstraintRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Deletes the specified portfolio.</p> <p>You cannot delete a portfolio if it was shared with you or if it has associated products, users, constraints, or shared accounts.</p>
    fn delete_portfolio(&self, input: DeletePortfolioRequest) -> Request<DeletePortfolioRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Stops sharing the specified portfolio with the specified account or organization node. Shares to an organization node can only be deleted by the master account of an Organization.</p>
    fn delete_portfolio_share(
        &self,
        input: DeletePortfolioShareRequest,
    ) -> Request<DeletePortfolioShareRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Deletes the specified product.</p> <p>You cannot delete a product if it was shared with you or is associated with a portfolio.</p>
    fn delete_product(&self, input: DeleteProductRequest) -> Request<DeleteProductRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Deletes the specified plan.</p>
    fn delete_provisioned_product_plan(
        &self,
        input: DeleteProvisionedProductPlanRequest,
    ) -> Request<DeleteProvisionedProductPlanRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Deletes the specified provisioning artifact (also known as a version) for the specified product.</p> <p>You cannot delete a provisioning artifact associated with a product that was shared with you. You cannot delete the last provisioning artifact for a product, because a product must have at least one provisioning artifact.</p>
    fn delete_provisioning_artifact(
        &self,
        input: DeleteProvisioningArtifactRequest,
    ) -> Request<DeleteProvisioningArtifactRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Deletes a self-service action.</p>
    fn delete_service_action(
        &self,
        input: DeleteServiceActionRequest,
    ) -> Request<DeleteServiceActionRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Deletes the specified TagOption.</p> <p>You cannot delete a TagOption if it is associated with a product or portfolio.</p>
    fn delete_tag_option(&self, input: DeleteTagOptionRequest) -> Request<DeleteTagOptionRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Gets information about the specified constraint.</p>
    fn describe_constraint(
        &self,
        input: DescribeConstraintRequest,
    ) -> Request<DescribeConstraintRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Gets the status of the specified copy product operation.</p>
    fn describe_copy_product_status(
        &self,
        input: DescribeCopyProductStatusRequest,
    ) -> Request<DescribeCopyProductStatusRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Gets information about the specified portfolio.</p>
    fn describe_portfolio(
        &self,
        input: DescribePortfolioRequest,
    ) -> Request<DescribePortfolioRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Gets the status of the specified portfolio share operation. This API can only be called by the master account in the organization.</p>
    fn describe_portfolio_share_status(
        &self,
        input: DescribePortfolioShareStatusRequest,
    ) -> Request<DescribePortfolioShareStatusRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Gets information about the specified product.</p>
    fn describe_product(&self, input: DescribeProductRequest) -> Request<DescribeProductRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Gets information about the specified product. This operation is run with administrator access.</p>
    fn describe_product_as_admin(
        &self,
        input: DescribeProductAsAdminRequest,
    ) -> Request<DescribeProductAsAdminRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Gets information about the specified product.</p>
    fn describe_product_view(
        &self,
        input: DescribeProductViewRequest,
    ) -> Request<DescribeProductViewRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Gets information about the specified provisioned product.</p>
    fn describe_provisioned_product(
        &self,
        input: DescribeProvisionedProductRequest,
    ) -> Request<DescribeProvisionedProductRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Gets information about the resource changes for the specified plan.</p>
    fn describe_provisioned_product_plan(
        &self,
        input: DescribeProvisionedProductPlanRequest,
    ) -> Request<DescribeProvisionedProductPlanRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Gets information about the specified provisioning artifact (also known as a version) for the specified product.</p>
    fn describe_provisioning_artifact(
        &self,
        input: DescribeProvisioningArtifactRequest,
    ) -> Request<DescribeProvisioningArtifactRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Gets information about the configuration required to provision the specified product using the specified provisioning artifact.</p> <p>If the output contains a TagOption key with an empty list of values, there is a TagOption conflict for that key. The end user cannot take action to fix the conflict, and launch is not blocked. In subsequent calls to <a>ProvisionProduct</a>, do not include conflicted TagOption keys as tags, or this causes the error "Parameter validation failed: Missing required parameter in Tags[<i>N</i>]:<i>Value</i>". Tag the provisioned product with the value <code>sc-tagoption-conflict-portfolioId-productId</code>.</p>
    fn describe_provisioning_parameters(
        &self,
        input: DescribeProvisioningParametersRequest,
    ) -> Request<DescribeProvisioningParametersRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p><p>Gets information about the specified request operation.</p> <p>Use this operation after calling a request operation (for example, <a>ProvisionProduct</a>, <a>TerminateProvisionedProduct</a>, or <a>UpdateProvisionedProduct</a>). </p> <note> <p>If a provisioned product was transferred to a new owner using <a>UpdateProvisionedProductProperties</a>, the new owner will be able to describe all past records for that product. The previous owner will no longer be able to describe the records, but will be able to use <a>ListRecordHistory</a> to see the product&#39;s history from when he was the owner.</p> </note></p>
    fn describe_record(&self, input: DescribeRecordRequest) -> Request<DescribeRecordRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Describes a self-service action.</p>
    fn describe_service_action(
        &self,
        input: DescribeServiceActionRequest,
    ) -> Request<DescribeServiceActionRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Gets information about the specified TagOption.</p>
    fn describe_tag_option(
        &self,
        input: DescribeTagOptionRequest,
    ) -> Request<DescribeTagOptionRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Disable portfolio sharing through AWS Organizations feature. This feature will not delete your current shares but it will prevent you from creating new shares throughout your organization. Current shares will not be in sync with your organization structure if it changes after calling this API. This API can only be called by the master account in the organization.</p>
    fn disable_aws_organizations_access(&self) -> Request<DisableAWSOrganizationsAccessRequest> {
        Request::new(
            DisableAWSOrganizationsAccessRequest {},
            self.region.clone(),
            self.client.clone(),
        )
    }

    /// <p>Disassociates the specified budget from the specified resource.</p>
    fn disassociate_budget_from_resource(
        &self,
        input: DisassociateBudgetFromResourceRequest,
    ) -> Request<DisassociateBudgetFromResourceRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Disassociates a previously associated principal ARN from a specified portfolio.</p>
    fn disassociate_principal_from_portfolio(
        &self,
        input: DisassociatePrincipalFromPortfolioRequest,
    ) -> Request<DisassociatePrincipalFromPortfolioRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Disassociates the specified product from the specified portfolio. </p>
    fn disassociate_product_from_portfolio(
        &self,
        input: DisassociateProductFromPortfolioRequest,
    ) -> Request<DisassociateProductFromPortfolioRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Disassociates the specified self-service action association from the specified provisioning artifact.</p>
    fn disassociate_service_action_from_provisioning_artifact(
        &self,
        input: DisassociateServiceActionFromProvisioningArtifactRequest,
    ) -> Request<DisassociateServiceActionFromProvisioningArtifactRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Disassociates the specified TagOption from the specified resource.</p>
    fn disassociate_tag_option_from_resource(
        &self,
        input: DisassociateTagOptionFromResourceRequest,
    ) -> Request<DisassociateTagOptionFromResourceRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Enable portfolio sharing feature through AWS Organizations. This API will allow Service Catalog to receive updates on your organization in order to sync your shares with the current structure. This API can only be called by the master account in the organization.</p> <p>By calling this API Service Catalog will make a call to organizations:EnableAWSServiceAccess on your behalf so that your shares can be in sync with any changes in your AWS Organizations structure.</p>
    fn enable_aws_organizations_access(&self) -> Request<EnableAWSOrganizationsAccessRequest> {
        Request::new(
            EnableAWSOrganizationsAccessRequest {},
            self.region.clone(),
            self.client.clone(),
        )
    }

    /// <p>Provisions or modifies a product based on the resource changes for the specified plan.</p>
    fn execute_provisioned_product_plan(
        &self,
        input: ExecuteProvisionedProductPlanRequest,
    ) -> Request<ExecuteProvisionedProductPlanRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Executes a self-service action against a provisioned product.</p>
    fn execute_provisioned_product_service_action(
        &self,
        input: ExecuteProvisionedProductServiceActionRequest,
    ) -> Request<ExecuteProvisionedProductServiceActionRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Get the Access Status for AWS Organization portfolio share feature. This API can only be called by the master account in the organization.</p>
    fn get_aws_organizations_access_status(
        &self,
    ) -> Request<GetAWSOrganizationsAccessStatusRequest> {
        Request::new(
            GetAWSOrganizationsAccessStatusRequest {},
            self.region.clone(),
            self.client.clone(),
        )
    }

    /// <p>Lists all portfolios for which sharing was accepted by this account.</p>
    fn list_accepted_portfolio_shares(
        &self,
        input: ListAcceptedPortfolioSharesRequest,
    ) -> Request<ListAcceptedPortfolioSharesRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Lists all the budgets associated to the specified resource.</p>
    fn list_budgets_for_resource(
        &self,
        input: ListBudgetsForResourceRequest,
    ) -> Request<ListBudgetsForResourceRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Lists the constraints for the specified portfolio and product.</p>
    fn list_constraints_for_portfolio(
        &self,
        input: ListConstraintsForPortfolioRequest,
    ) -> Request<ListConstraintsForPortfolioRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Lists the paths to the specified product. A path is how the user has access to a specified product, and is necessary when provisioning a product. A path also determines the constraints put on the product.</p>
    fn list_launch_paths(&self, input: ListLaunchPathsRequest) -> Request<ListLaunchPathsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Lists the organization nodes that have access to the specified portfolio. This API can only be called by the master account in the organization.</p>
    fn list_organization_portfolio_access(
        &self,
        input: ListOrganizationPortfolioAccessRequest,
    ) -> Request<ListOrganizationPortfolioAccessRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Lists the account IDs that have access to the specified portfolio.</p>
    fn list_portfolio_access(
        &self,
        input: ListPortfolioAccessRequest,
    ) -> Request<ListPortfolioAccessRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Lists all portfolios in the catalog.</p>
    fn list_portfolios(&self, input: ListPortfoliosRequest) -> Request<ListPortfoliosRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Lists all portfolios that the specified product is associated with.</p>
    fn list_portfolios_for_product(
        &self,
        input: ListPortfoliosForProductRequest,
    ) -> Request<ListPortfoliosForProductRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Lists all principal ARNs associated with the specified portfolio.</p>
    fn list_principals_for_portfolio(
        &self,
        input: ListPrincipalsForPortfolioRequest,
    ) -> Request<ListPrincipalsForPortfolioRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Lists the plans for the specified provisioned product or all plans to which the user has access.</p>
    fn list_provisioned_product_plans(
        &self,
        input: ListProvisionedProductPlansRequest,
    ) -> Request<ListProvisionedProductPlansRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Lists all provisioning artifacts (also known as versions) for the specified product.</p>
    fn list_provisioning_artifacts(
        &self,
        input: ListProvisioningArtifactsRequest,
    ) -> Request<ListProvisioningArtifactsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Lists all provisioning artifacts (also known as versions) for the specified self-service action.</p>
    fn list_provisioning_artifacts_for_service_action(
        &self,
        input: ListProvisioningArtifactsForServiceActionRequest,
    ) -> Request<ListProvisioningArtifactsForServiceActionRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Lists the specified requests or all performed requests.</p>
    fn list_record_history(
        &self,
        input: ListRecordHistoryRequest,
    ) -> Request<ListRecordHistoryRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Lists the resources associated with the specified TagOption.</p>
    fn list_resources_for_tag_option(
        &self,
        input: ListResourcesForTagOptionRequest,
    ) -> Request<ListResourcesForTagOptionRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Lists all self-service actions.</p>
    fn list_service_actions(
        &self,
        input: ListServiceActionsRequest,
    ) -> Request<ListServiceActionsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Returns a paginated list of self-service actions associated with the specified Product ID and Provisioning Artifact ID.</p>
    fn list_service_actions_for_provisioning_artifact(
        &self,
        input: ListServiceActionsForProvisioningArtifactRequest,
    ) -> Request<ListServiceActionsForProvisioningArtifactRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Lists the specified TagOptions or all TagOptions.</p>
    fn list_tag_options(&self, input: ListTagOptionsRequest) -> Request<ListTagOptionsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Provisions the specified product.</p> <p>A provisioned product is a resourced instance of a product. For example, provisioning a product based on a CloudFormation template launches a CloudFormation stack and its underlying resources. You can check the status of this request using <a>DescribeRecord</a>.</p> <p>If the request contains a tag key with an empty list of values, there is a tag conflict for that key. Do not include conflicted keys as tags, or this causes the error "Parameter validation failed: Missing required parameter in Tags[<i>N</i>]:<i>Value</i>".</p>
    fn provision_product(
        &self,
        input: ProvisionProductRequest,
    ) -> Request<ProvisionProductRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Rejects an offer to share the specified portfolio.</p>
    fn reject_portfolio_share(
        &self,
        input: RejectPortfolioShareRequest,
    ) -> Request<RejectPortfolioShareRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Lists the provisioned products that are available (not terminated).</p> <p>To use additional filtering, see <a>SearchProvisionedProducts</a>.</p>
    fn scan_provisioned_products(
        &self,
        input: ScanProvisionedProductsRequest,
    ) -> Request<ScanProvisionedProductsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Gets information about the products to which the caller has access.</p>
    fn search_products(&self, input: SearchProductsRequest) -> Request<SearchProductsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Gets information about the products for the specified portfolio or all products.</p>
    fn search_products_as_admin(
        &self,
        input: SearchProductsAsAdminRequest,
    ) -> Request<SearchProductsAsAdminRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Gets information about the provisioned products that meet the specified criteria.</p>
    fn search_provisioned_products(
        &self,
        input: SearchProvisionedProductsRequest,
    ) -> Request<SearchProvisionedProductsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Terminates the specified provisioned product.</p> <p>This operation does not delete any records associated with the provisioned product.</p> <p>You can check the status of this request using <a>DescribeRecord</a>.</p>
    fn terminate_provisioned_product(
        &self,
        input: TerminateProvisionedProductRequest,
    ) -> Request<TerminateProvisionedProductRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Updates the specified constraint.</p>
    fn update_constraint(
        &self,
        input: UpdateConstraintRequest,
    ) -> Request<UpdateConstraintRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Updates the specified portfolio.</p> <p>You cannot update a product that was shared with you.</p>
    fn update_portfolio(&self, input: UpdatePortfolioRequest) -> Request<UpdatePortfolioRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Updates the specified product.</p>
    fn update_product(&self, input: UpdateProductRequest) -> Request<UpdateProductRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Requests updates to the configuration of the specified provisioned product.</p> <p>If there are tags associated with the object, they cannot be updated or added. Depending on the specific updates requested, this operation can update with no interruption, with some interruption, or replace the provisioned product entirely.</p> <p>You can check the status of this request using <a>DescribeRecord</a>.</p>
    fn update_provisioned_product(
        &self,
        input: UpdateProvisionedProductRequest,
    ) -> Request<UpdateProvisionedProductRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Requests updates to the properties of the specified provisioned product.</p>
    fn update_provisioned_product_properties(
        &self,
        input: UpdateProvisionedProductPropertiesRequest,
    ) -> Request<UpdateProvisionedProductPropertiesRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Updates the specified provisioning artifact (also known as a version) for the specified product.</p> <p>You cannot update a provisioning artifact for a product that was shared with you.</p>
    fn update_provisioning_artifact(
        &self,
        input: UpdateProvisioningArtifactRequest,
    ) -> Request<UpdateProvisioningArtifactRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Updates a self-service action.</p>
    fn update_service_action(
        &self,
        input: UpdateServiceActionRequest,
    ) -> Request<UpdateServiceActionRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Updates the specified TagOption.</p>
    fn update_tag_option(&self, input: UpdateTagOptionRequest) -> Request<UpdateTagOptionRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }
}

impl ServiceRequest for AcceptPortfolioShareRequest {
    type Output = AcceptPortfolioShareResponse;
    type Error = AcceptPortfolioShareError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.AcceptPortfolioShare",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<AcceptPortfolioShareResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(AcceptPortfolioShareError::from_response(response))
                    }),
                )
            }
        })
    }
}

impl ServiceRequest for AssociateBudgetWithResourceRequest {
    type Output = AssociateBudgetWithResourceResponse;
    type Error = AssociateBudgetWithResourceError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.AssociateBudgetWithResource",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<AssociateBudgetWithResourceResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AssociateBudgetWithResourceError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for AssociatePrincipalWithPortfolioRequest {
    type Output = AssociatePrincipalWithPortfolioResponse;
    type Error = AssociatePrincipalWithPortfolioError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.AssociatePrincipalWithPortfolio",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<AssociatePrincipalWithPortfolioResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AssociatePrincipalWithPortfolioError::from_response(
                        response,
                    ))
                }))
            }
        })
    }
}

impl ServiceRequest for AssociateProductWithPortfolioRequest {
    type Output = AssociateProductWithPortfolioResponse;
    type Error = AssociateProductWithPortfolioError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.AssociateProductWithPortfolio",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<AssociateProductWithPortfolioResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AssociateProductWithPortfolioError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for AssociateServiceActionWithProvisioningArtifactRequest {
    type Output = AssociateServiceActionWithProvisioningArtifactResponse;
    type Error = AssociateServiceActionWithProvisioningArtifactError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.AssociateServiceActionWithProvisioningArtifact",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
                            if response.status.is_success() {
                                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response).deserialize::<AssociateServiceActionWithProvisioningArtifactResponse, _>()
                }))
                            } else {
                                Box::new(response.buffer().from_err().and_then(|response| {
                                    Err(AssociateServiceActionWithProvisioningArtifactError::from_response(response))
                                }))
                            }
                        })
    }
}

impl ServiceRequest for AssociateTagOptionWithResourceRequest {
    type Output = AssociateTagOptionWithResourceResponse;
    type Error = AssociateTagOptionWithResourceError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.AssociateTagOptionWithResource",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<AssociateTagOptionWithResourceResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AssociateTagOptionWithResourceError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for BatchAssociateServiceActionWithProvisioningArtifactRequest {
    type Output = BatchAssociateServiceActionWithProvisioningArtifactResponse;
    type Error = BatchAssociateServiceActionWithProvisioningArtifactError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.BatchAssociateServiceActionWithProvisioningArtifact",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
                            if response.status.is_success() {
                                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response).deserialize::<BatchAssociateServiceActionWithProvisioningArtifactResponse, _>()
                }))
                            } else {
                                Box::new(response.buffer().from_err().and_then(|response| {
                                    Err(BatchAssociateServiceActionWithProvisioningArtifactError::from_response(response))
                                }))
                            }
                        })
    }
}

impl ServiceRequest for BatchDisassociateServiceActionFromProvisioningArtifactRequest {
    type Output = BatchDisassociateServiceActionFromProvisioningArtifactResponse;
    type Error = BatchDisassociateServiceActionFromProvisioningArtifactError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.BatchDisassociateServiceActionFromProvisioningArtifact",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
                            if response.status.is_success() {
                                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response).deserialize::<BatchDisassociateServiceActionFromProvisioningArtifactResponse, _>()
                }))
                            } else {
                                Box::new(response.buffer().from_err().and_then(|response| {
                                    Err(BatchDisassociateServiceActionFromProvisioningArtifactError::from_response(response))
                                }))
                            }
                        })
    }
}

impl ServiceRequest for CopyProductRequest {
    type Output = CopyProductResponse;
    type Error = CopyProductError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWS242ServiceCatalogService.CopyProduct");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CopyProductResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CopyProductError::from_response(response))),
                )
            }
        })
    }
}

impl ServiceRequest for CreateConstraintRequest {
    type Output = CreateConstraintResponse;
    type Error = CreateConstraintError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.CreateConstraint",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateConstraintResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateConstraintError::from_response(response))),
                )
            }
        })
    }
}

impl ServiceRequest for CreatePortfolioRequest {
    type Output = CreatePortfolioResponse;
    type Error = CreatePortfolioError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.CreatePortfolio",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreatePortfolioResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreatePortfolioError::from_response(response))),
                )
            }
        })
    }
}

impl ServiceRequest for CreatePortfolioShareRequest {
    type Output = CreatePortfolioShareResponse;
    type Error = CreatePortfolioShareError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.CreatePortfolioShare",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreatePortfolioShareResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(CreatePortfolioShareError::from_response(response))
                    }),
                )
            }
        })
    }
}

impl ServiceRequest for CreateProductRequest {
    type Output = CreateProductResponse;
    type Error = CreateProductError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWS242ServiceCatalogService.CreateProduct");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateProductResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateProductError::from_response(response))),
                )
            }
        })
    }
}

impl ServiceRequest for CreateProvisionedProductPlanRequest {
    type Output = CreateProvisionedProductPlanResponse;
    type Error = CreateProvisionedProductPlanError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.CreateProvisionedProductPlan",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateProvisionedProductPlanResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateProvisionedProductPlanError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for CreateProvisioningArtifactRequest {
    type Output = CreateProvisioningArtifactResponse;
    type Error = CreateProvisioningArtifactError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.CreateProvisioningArtifact",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateProvisioningArtifactResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateProvisioningArtifactError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for CreateServiceActionRequest {
    type Output = CreateServiceActionResponse;
    type Error = CreateServiceActionError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.CreateServiceAction",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateServiceActionResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(CreateServiceActionError::from_response(response))
                    }),
                )
            }
        })
    }
}

impl ServiceRequest for CreateTagOptionRequest {
    type Output = CreateTagOptionResponse;
    type Error = CreateTagOptionError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.CreateTagOption",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateTagOptionResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateTagOptionError::from_response(response))),
                )
            }
        })
    }
}

impl ServiceRequest for DeleteConstraintRequest {
    type Output = DeleteConstraintResponse;
    type Error = DeleteConstraintError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DeleteConstraint",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteConstraintResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteConstraintError::from_response(response))),
                )
            }
        })
    }
}

impl ServiceRequest for DeletePortfolioRequest {
    type Output = DeletePortfolioResponse;
    type Error = DeletePortfolioError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DeletePortfolio",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeletePortfolioResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeletePortfolioError::from_response(response))),
                )
            }
        })
    }
}

impl ServiceRequest for DeletePortfolioShareRequest {
    type Output = DeletePortfolioShareResponse;
    type Error = DeletePortfolioShareError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DeletePortfolioShare",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeletePortfolioShareResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeletePortfolioShareError::from_response(response))
                    }),
                )
            }
        })
    }
}

impl ServiceRequest for DeleteProductRequest {
    type Output = DeleteProductResponse;
    type Error = DeleteProductError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWS242ServiceCatalogService.DeleteProduct");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteProductResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteProductError::from_response(response))),
                )
            }
        })
    }
}

impl ServiceRequest for DeleteProvisionedProductPlanRequest {
    type Output = DeleteProvisionedProductPlanResponse;
    type Error = DeleteProvisionedProductPlanError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DeleteProvisionedProductPlan",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteProvisionedProductPlanResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteProvisionedProductPlanError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for DeleteProvisioningArtifactRequest {
    type Output = DeleteProvisioningArtifactResponse;
    type Error = DeleteProvisioningArtifactError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DeleteProvisioningArtifact",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteProvisioningArtifactResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteProvisioningArtifactError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for DeleteServiceActionRequest {
    type Output = DeleteServiceActionResponse;
    type Error = DeleteServiceActionError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DeleteServiceAction",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteServiceActionResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteServiceActionError::from_response(response))
                    }),
                )
            }
        })
    }
}

impl ServiceRequest for DeleteTagOptionRequest {
    type Output = DeleteTagOptionResponse;
    type Error = DeleteTagOptionError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DeleteTagOption",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteTagOptionResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteTagOptionError::from_response(response))),
                )
            }
        })
    }
}

impl ServiceRequest for DescribeConstraintRequest {
    type Output = DescribeConstraintResponse;
    type Error = DescribeConstraintError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DescribeConstraint",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeConstraintResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeConstraintError::from_response(response))),
                )
            }
        })
    }
}

impl ServiceRequest for DescribeCopyProductStatusRequest {
    type Output = DescribeCopyProductStatusResponse;
    type Error = DescribeCopyProductStatusError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DescribeCopyProductStatus",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeCopyProductStatusResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeCopyProductStatusError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for DescribePortfolioRequest {
    type Output = DescribePortfolioResponse;
    type Error = DescribePortfolioError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DescribePortfolio",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribePortfolioResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribePortfolioError::from_response(response))),
                )
            }
        })
    }
}

impl ServiceRequest for DescribePortfolioShareStatusRequest {
    type Output = DescribePortfolioShareStatusResponse;
    type Error = DescribePortfolioShareStatusError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DescribePortfolioShareStatus",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribePortfolioShareStatusResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribePortfolioShareStatusError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for DescribeProductRequest {
    type Output = DescribeProductResponse;
    type Error = DescribeProductError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DescribeProduct",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeProductResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeProductError::from_response(response))),
                )
            }
        })
    }
}

impl ServiceRequest for DescribeProductAsAdminRequest {
    type Output = DescribeProductAsAdminResponse;
    type Error = DescribeProductAsAdminError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DescribeProductAsAdmin",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeProductAsAdminResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DescribeProductAsAdminError::from_response(response))
                    }),
                )
            }
        })
    }
}

impl ServiceRequest for DescribeProductViewRequest {
    type Output = DescribeProductViewResponse;
    type Error = DescribeProductViewError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DescribeProductView",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeProductViewResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DescribeProductViewError::from_response(response))
                    }),
                )
            }
        })
    }
}

impl ServiceRequest for DescribeProvisionedProductRequest {
    type Output = DescribeProvisionedProductResponse;
    type Error = DescribeProvisionedProductError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DescribeProvisionedProduct",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeProvisionedProductResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeProvisionedProductError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for DescribeProvisionedProductPlanRequest {
    type Output = DescribeProvisionedProductPlanResponse;
    type Error = DescribeProvisionedProductPlanError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DescribeProvisionedProductPlan",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeProvisionedProductPlanResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeProvisionedProductPlanError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for DescribeProvisioningArtifactRequest {
    type Output = DescribeProvisioningArtifactResponse;
    type Error = DescribeProvisioningArtifactError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DescribeProvisioningArtifact",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeProvisioningArtifactResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeProvisioningArtifactError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for DescribeProvisioningParametersRequest {
    type Output = DescribeProvisioningParametersResponse;
    type Error = DescribeProvisioningParametersError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DescribeProvisioningParameters",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeProvisioningParametersResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeProvisioningParametersError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for DescribeRecordRequest {
    type Output = DescribeRecordResponse;
    type Error = DescribeRecordError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWS242ServiceCatalogService.DescribeRecord");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeRecordResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeRecordError::from_response(response))),
                )
            }
        })
    }
}

impl ServiceRequest for DescribeServiceActionRequest {
    type Output = DescribeServiceActionResponse;
    type Error = DescribeServiceActionError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DescribeServiceAction",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeServiceActionResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DescribeServiceActionError::from_response(response))
                    }),
                )
            }
        })
    }
}

impl ServiceRequest for DescribeTagOptionRequest {
    type Output = DescribeTagOptionResponse;
    type Error = DescribeTagOptionError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DescribeTagOption",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeTagOptionResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeTagOptionError::from_response(response))),
                )
            }
        })
    }
}

impl ServiceRequest for DisableAWSOrganizationsAccessRequest {
    type Output = DisableAWSOrganizationsAccessResponse;
    type Error = DisableAWSOrganizationsAccessError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DisableAWSOrganizationsAccess",
        );
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DisableAWSOrganizationsAccessResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DisableAWSOrganizationsAccessError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for DisassociateBudgetFromResourceRequest {
    type Output = DisassociateBudgetFromResourceResponse;
    type Error = DisassociateBudgetFromResourceError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DisassociateBudgetFromResource",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DisassociateBudgetFromResourceResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DisassociateBudgetFromResourceError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for DisassociatePrincipalFromPortfolioRequest {
    type Output = DisassociatePrincipalFromPortfolioResponse;
    type Error = DisassociatePrincipalFromPortfolioError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DisassociatePrincipalFromPortfolio",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DisassociatePrincipalFromPortfolioResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DisassociatePrincipalFromPortfolioError::from_response(
                        response,
                    ))
                }))
            }
        })
    }
}

impl ServiceRequest for DisassociateProductFromPortfolioRequest {
    type Output = DisassociateProductFromPortfolioResponse;
    type Error = DisassociateProductFromPortfolioError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DisassociateProductFromPortfolio",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DisassociateProductFromPortfolioResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DisassociateProductFromPortfolioError::from_response(
                        response,
                    ))
                }))
            }
        })
    }
}

impl ServiceRequest for DisassociateServiceActionFromProvisioningArtifactRequest {
    type Output = DisassociateServiceActionFromProvisioningArtifactResponse;
    type Error = DisassociateServiceActionFromProvisioningArtifactError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DisassociateServiceActionFromProvisioningArtifact",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
                            if response.status.is_success() {
                                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response).deserialize::<DisassociateServiceActionFromProvisioningArtifactResponse, _>()
                }))
                            } else {
                                Box::new(response.buffer().from_err().and_then(|response| {
                                    Err(DisassociateServiceActionFromProvisioningArtifactError::from_response(response))
                                }))
                            }
                        })
    }
}

impl ServiceRequest for DisassociateTagOptionFromResourceRequest {
    type Output = DisassociateTagOptionFromResourceResponse;
    type Error = DisassociateTagOptionFromResourceError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DisassociateTagOptionFromResource",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DisassociateTagOptionFromResourceResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DisassociateTagOptionFromResourceError::from_response(
                        response,
                    ))
                }))
            }
        })
    }
}

impl ServiceRequest for EnableAWSOrganizationsAccessRequest {
    type Output = EnableAWSOrganizationsAccessResponse;
    type Error = EnableAWSOrganizationsAccessError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.EnableAWSOrganizationsAccess",
        );
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<EnableAWSOrganizationsAccessResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(EnableAWSOrganizationsAccessError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for ExecuteProvisionedProductPlanRequest {
    type Output = ExecuteProvisionedProductPlanResponse;
    type Error = ExecuteProvisionedProductPlanError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.ExecuteProvisionedProductPlan",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ExecuteProvisionedProductPlanResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ExecuteProvisionedProductPlanError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for ExecuteProvisionedProductServiceActionRequest {
    type Output = ExecuteProvisionedProductServiceActionResponse;
    type Error = ExecuteProvisionedProductServiceActionError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.ExecuteProvisionedProductServiceAction",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ExecuteProvisionedProductServiceActionResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ExecuteProvisionedProductServiceActionError::from_response(
                        response,
                    ))
                }))
            }
        })
    }
}

impl ServiceRequest for GetAWSOrganizationsAccessStatusRequest {
    type Output = GetAWSOrganizationsAccessStatusResponse;
    type Error = GetAWSOrganizationsAccessStatusError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.GetAWSOrganizationsAccessStatus",
        );
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetAWSOrganizationsAccessStatusResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetAWSOrganizationsAccessStatusError::from_response(
                        response,
                    ))
                }))
            }
        })
    }
}

impl ServiceRequest for ListAcceptedPortfolioSharesRequest {
    type Output = ListAcceptedPortfolioSharesResponse;
    type Error = ListAcceptedPortfolioSharesError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.ListAcceptedPortfolioShares",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListAcceptedPortfolioSharesResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListAcceptedPortfolioSharesError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for ListBudgetsForResourceRequest {
    type Output = ListBudgetsForResourceResponse;
    type Error = ListBudgetsForResourceError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.ListBudgetsForResource",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListBudgetsForResourceResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListBudgetsForResourceError::from_response(response))
                    }),
                )
            }
        })
    }
}

impl ServiceRequest for ListConstraintsForPortfolioRequest {
    type Output = ListConstraintsForPortfolioResponse;
    type Error = ListConstraintsForPortfolioError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.ListConstraintsForPortfolio",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListConstraintsForPortfolioResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListConstraintsForPortfolioError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for ListLaunchPathsRequest {
    type Output = ListLaunchPathsResponse;
    type Error = ListLaunchPathsError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.ListLaunchPaths",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListLaunchPathsResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListLaunchPathsError::from_response(response))),
                )
            }
        })
    }
}

impl ServiceRequest for ListOrganizationPortfolioAccessRequest {
    type Output = ListOrganizationPortfolioAccessResponse;
    type Error = ListOrganizationPortfolioAccessError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.ListOrganizationPortfolioAccess",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListOrganizationPortfolioAccessResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListOrganizationPortfolioAccessError::from_response(
                        response,
                    ))
                }))
            }
        })
    }
}

impl ServiceRequest for ListPortfolioAccessRequest {
    type Output = ListPortfolioAccessResponse;
    type Error = ListPortfolioAccessError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.ListPortfolioAccess",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListPortfolioAccessResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListPortfolioAccessError::from_response(response))
                    }),
                )
            }
        })
    }
}

impl ServiceRequest for ListPortfoliosRequest {
    type Output = ListPortfoliosResponse;
    type Error = ListPortfoliosError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWS242ServiceCatalogService.ListPortfolios");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListPortfoliosResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListPortfoliosError::from_response(response))),
                )
            }
        })
    }
}

impl ServiceRequest for ListPortfoliosForProductRequest {
    type Output = ListPortfoliosForProductResponse;
    type Error = ListPortfoliosForProductError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.ListPortfoliosForProduct",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListPortfoliosForProductResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListPortfoliosForProductError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for ListPrincipalsForPortfolioRequest {
    type Output = ListPrincipalsForPortfolioResponse;
    type Error = ListPrincipalsForPortfolioError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.ListPrincipalsForPortfolio",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListPrincipalsForPortfolioResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListPrincipalsForPortfolioError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for ListProvisionedProductPlansRequest {
    type Output = ListProvisionedProductPlansResponse;
    type Error = ListProvisionedProductPlansError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.ListProvisionedProductPlans",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListProvisionedProductPlansResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListProvisionedProductPlansError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for ListProvisioningArtifactsRequest {
    type Output = ListProvisioningArtifactsResponse;
    type Error = ListProvisioningArtifactsError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.ListProvisioningArtifacts",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListProvisioningArtifactsResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListProvisioningArtifactsError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for ListProvisioningArtifactsForServiceActionRequest {
    type Output = ListProvisioningArtifactsForServiceActionResponse;
    type Error = ListProvisioningArtifactsForServiceActionError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.ListProvisioningArtifactsForServiceAction",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListProvisioningArtifactsForServiceActionResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListProvisioningArtifactsForServiceActionError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for ListRecordHistoryRequest {
    type Output = ListRecordHistoryResponse;
    type Error = ListRecordHistoryError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.ListRecordHistory",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListRecordHistoryResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListRecordHistoryError::from_response(response))),
                )
            }
        })
    }
}

impl ServiceRequest for ListResourcesForTagOptionRequest {
    type Output = ListResourcesForTagOptionResponse;
    type Error = ListResourcesForTagOptionError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.ListResourcesForTagOption",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListResourcesForTagOptionResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListResourcesForTagOptionError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for ListServiceActionsRequest {
    type Output = ListServiceActionsResponse;
    type Error = ListServiceActionsError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.ListServiceActions",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListServiceActionsResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListServiceActionsError::from_response(response))),
                )
            }
        })
    }
}

impl ServiceRequest for ListServiceActionsForProvisioningArtifactRequest {
    type Output = ListServiceActionsForProvisioningArtifactResponse;
    type Error = ListServiceActionsForProvisioningArtifactError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.ListServiceActionsForProvisioningArtifact",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListServiceActionsForProvisioningArtifactResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListServiceActionsForProvisioningArtifactError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for ListTagOptionsRequest {
    type Output = ListTagOptionsResponse;
    type Error = ListTagOptionsError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWS242ServiceCatalogService.ListTagOptions");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListTagOptionsResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListTagOptionsError::from_response(response))),
                )
            }
        })
    }
}

impl ServiceRequest for ProvisionProductRequest {
    type Output = ProvisionProductResponse;
    type Error = ProvisionProductError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.ProvisionProduct",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ProvisionProductResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ProvisionProductError::from_response(response))),
                )
            }
        })
    }
}

impl ServiceRequest for RejectPortfolioShareRequest {
    type Output = RejectPortfolioShareResponse;
    type Error = RejectPortfolioShareError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.RejectPortfolioShare",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<RejectPortfolioShareResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(RejectPortfolioShareError::from_response(response))
                    }),
                )
            }
        })
    }
}

impl ServiceRequest for ScanProvisionedProductsRequest {
    type Output = ScanProvisionedProductsResponse;
    type Error = ScanProvisionedProductsError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.ScanProvisionedProducts",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ScanProvisionedProductsResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ScanProvisionedProductsError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for SearchProductsRequest {
    type Output = SearchProductsResponse;
    type Error = SearchProductsError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWS242ServiceCatalogService.SearchProducts");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<SearchProductsResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(SearchProductsError::from_response(response))),
                )
            }
        })
    }
}

impl ServiceRequest for SearchProductsAsAdminRequest {
    type Output = SearchProductsAsAdminResponse;
    type Error = SearchProductsAsAdminError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.SearchProductsAsAdmin",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<SearchProductsAsAdminResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(SearchProductsAsAdminError::from_response(response))
                    }),
                )
            }
        })
    }
}

impl ServiceRequest for SearchProvisionedProductsRequest {
    type Output = SearchProvisionedProductsResponse;
    type Error = SearchProvisionedProductsError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.SearchProvisionedProducts",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<SearchProvisionedProductsResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(SearchProvisionedProductsError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for TerminateProvisionedProductRequest {
    type Output = TerminateProvisionedProductResponse;
    type Error = TerminateProvisionedProductError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.TerminateProvisionedProduct",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<TerminateProvisionedProductResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(TerminateProvisionedProductError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for UpdateConstraintRequest {
    type Output = UpdateConstraintResponse;
    type Error = UpdateConstraintError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.UpdateConstraint",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateConstraintResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateConstraintError::from_response(response))),
                )
            }
        })
    }
}

impl ServiceRequest for UpdatePortfolioRequest {
    type Output = UpdatePortfolioResponse;
    type Error = UpdatePortfolioError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.UpdatePortfolio",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdatePortfolioResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdatePortfolioError::from_response(response))),
                )
            }
        })
    }
}

impl ServiceRequest for UpdateProductRequest {
    type Output = UpdateProductResponse;
    type Error = UpdateProductError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWS242ServiceCatalogService.UpdateProduct");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateProductResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateProductError::from_response(response))),
                )
            }
        })
    }
}

impl ServiceRequest for UpdateProvisionedProductRequest {
    type Output = UpdateProvisionedProductResponse;
    type Error = UpdateProvisionedProductError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.UpdateProvisionedProduct",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateProvisionedProductResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateProvisionedProductError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for UpdateProvisionedProductPropertiesRequest {
    type Output = UpdateProvisionedProductPropertiesResponse;
    type Error = UpdateProvisionedProductPropertiesError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.UpdateProvisionedProductProperties",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateProvisionedProductPropertiesResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateProvisionedProductPropertiesError::from_response(
                        response,
                    ))
                }))
            }
        })
    }
}

impl ServiceRequest for UpdateProvisioningArtifactRequest {
    type Output = UpdateProvisioningArtifactResponse;
    type Error = UpdateProvisioningArtifactError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.UpdateProvisioningArtifact",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateProvisioningArtifactResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateProvisioningArtifactError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for UpdateServiceActionRequest {
    type Output = UpdateServiceActionResponse;
    type Error = UpdateServiceActionError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.UpdateServiceAction",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateServiceActionResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateServiceActionError::from_response(response))
                    }),
                )
            }
        })
    }
}

impl ServiceRequest for UpdateTagOptionRequest {
    type Output = UpdateTagOptionResponse;
    type Error = UpdateTagOptionError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "servicecatalog", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.UpdateTagOption",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateTagOptionResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateTagOptionError::from_response(response))),
                )
            }
        })
    }
}
