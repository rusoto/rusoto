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
use rusoto_core::reactor::{CredentialsProvider, RequestDispatcher};
use rusoto_core::region;
use rusoto_core::request::DispatchSignedRequest;
use rusoto_core::{ClientInner, RusotoFuture};

use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};
use rusoto_core::request::HttpDispatchError;

use hyper::StatusCode;
use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_str;
use serde_json::Value as SerdeJsonValue;
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AcceptPortfolioShareInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The portfolio identifier.</p>
    #[serde(rename = "PortfolioId")]
    pub portfolio_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AcceptPortfolioShareOutput {}

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
pub struct AssociatePrincipalWithPortfolioInput {
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
pub struct AssociatePrincipalWithPortfolioOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AssociateProductWithPortfolioInput {
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
pub struct AssociateProductWithPortfolioOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AssociateTagOptionWithResourceInput {
    /// <p>The resource identifier.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p>The TagOption identifier.</p>
    #[serde(rename = "TagOptionId")]
    pub tag_option_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AssociateTagOptionWithResourceOutput {}

/// <p>Information about a CloudWatch dashboard.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CloudWatchDashboard {
    /// <p>The name of the CloudWatch dashboard.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Information about a constraint.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
    /// <p><p>The type of constraint.</p> <ul> <li> <p> <code>LAUNCH</code> </p> </li> <li> <p> <code>NOTIFICATION</code> </p> </li> <li> <p> <code>TEMPLATE</code> </p> </li> </ul></p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Summary information about a constraint.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ConstraintSummary {
    /// <p>The description of the constraint.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p><p>The type of constraint.</p> <ul> <li> <p> <code>LAUNCH</code> </p> </li> <li> <p> <code>NOTIFICATION</code> </p> </li> <li> <p> <code>TEMPLATE</code> </p> </li> </ul></p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CopyProductInput {
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
pub struct CopyProductOutput {
    /// <p>The token to use to track the progress of the operation.</p>
    #[serde(rename = "CopyProductToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_product_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateConstraintInput {
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
    /// <p><p>The constraint parameters, in JSON format. The syntax depends on the constraint type as follows:</p> <dl> <dt>LAUNCH</dt> <dd> <p>Specify the <code>RoleArn</code> property as follows:</p> <p>&quot;RoleArn&quot; : &quot;arn:aws:iam::123456789012:role/LaunchRole&quot;</p> </dd> <dt>NOTIFICATION</dt> <dd> <p>Specify the <code>NotificationArns</code> property as follows:</p> <p>&quot;NotificationArns&quot; : [&quot;arn:aws:sns:us-east-1:123456789012:Topic&quot;]</p> </dd> <dt>TEMPLATE</dt> <dd> <p>Specify the <code>Rules</code> property. For more information, see <a href="http://docs.aws.amazon.com/servicecatalog/latest/adminguide/reference-template_constraint_rules.html">Template Constraint Rules</a>.</p> </dd> </dl></p>
    #[serde(rename = "Parameters")]
    pub parameters: String,
    /// <p>The portfolio identifier.</p>
    #[serde(rename = "PortfolioId")]
    pub portfolio_id: String,
    /// <p>The product identifier.</p>
    #[serde(rename = "ProductId")]
    pub product_id: String,
    /// <p><p>The type of constraint.</p> <ul> <li> <p> <code>LAUNCH</code> </p> </li> <li> <p> <code>NOTIFICATION</code> </p> </li> <li> <p> <code>TEMPLATE</code> </p> </li> </ul></p>
    #[serde(rename = "Type")]
    pub type_: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateConstraintOutput {
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
pub struct CreatePortfolioInput {
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
pub struct CreatePortfolioOutput {
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
pub struct CreatePortfolioShareInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The AWS account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The portfolio identifier.</p>
    #[serde(rename = "PortfolioId")]
    pub portfolio_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreatePortfolioShareOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateProductInput {
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
pub struct CreateProductOutput {
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
pub struct CreateProvisionedProductPlanInput {
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
pub struct CreateProvisionedProductPlanOutput {
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
pub struct CreateProvisioningArtifactInput {
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
pub struct CreateProvisioningArtifactOutput {
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
pub struct CreateTagOptionInput {
    /// <p>The TagOption key.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The TagOption value.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateTagOptionOutput {
    /// <p>Information about the TagOption.</p>
    #[serde(rename = "TagOptionDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_option_detail: Option<TagOptionDetail>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteConstraintInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The identifier of the constraint.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteConstraintOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeletePortfolioInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The portfolio identifier.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeletePortfolioOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeletePortfolioShareInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The AWS account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The portfolio identifier.</p>
    #[serde(rename = "PortfolioId")]
    pub portfolio_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeletePortfolioShareOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteProductInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The product identifier.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteProductOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteProvisionedProductPlanInput {
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
pub struct DeleteProvisionedProductPlanOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteProvisioningArtifactInput {
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
pub struct DeleteProvisioningArtifactOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteTagOptionInput {
    /// <p>The TagOption identifier.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteTagOptionOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeConstraintInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The identifier of the constraint.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeConstraintOutput {
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
pub struct DescribeCopyProductStatusInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The token for the copy product operation. This token is returned by <a>CopyProduct</a>.</p>
    #[serde(rename = "CopyProductToken")]
    pub copy_product_token: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeCopyProductStatusOutput {
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
pub struct DescribePortfolioInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The portfolio identifier.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribePortfolioOutput {
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
pub struct DescribeProductAsAdminInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The product identifier.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeProductAsAdminOutput {
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
pub struct DescribeProductInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The product identifier.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeProductOutput {
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
pub struct DescribeProductViewInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The product view identifier.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeProductViewOutput {
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
pub struct DescribeProvisionedProductInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The provisioned product identifier.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeProvisionedProductOutput {
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
pub struct DescribeProvisionedProductPlanInput {
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
pub struct DescribeProvisionedProductPlanOutput {
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
pub struct DescribeProvisioningArtifactInput {
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
pub struct DescribeProvisioningArtifactOutput {
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
pub struct DescribeProvisioningParametersInput {
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
pub struct DescribeProvisioningParametersOutput {
    /// <p>Information about the constraints used to provision the product.</p>
    #[serde(rename = "ConstraintSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraint_summaries: Option<Vec<ConstraintSummary>>,
    /// <p>Information about the parameters used to provision the product.</p>
    #[serde(rename = "ProvisioningArtifactParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_parameters: Option<Vec<ProvisioningArtifactParameter>>,
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
pub struct DescribeRecordInput {
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
pub struct DescribeRecordOutput {
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
pub struct DescribeTagOptionInput {
    /// <p>The TagOption identifier.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeTagOptionOutput {
    /// <p>Information about the TagOption.</p>
    #[serde(rename = "TagOptionDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_option_detail: Option<TagOptionDetail>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisassociatePrincipalFromPortfolioInput {
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
pub struct DisassociatePrincipalFromPortfolioOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisassociateProductFromPortfolioInput {
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
pub struct DisassociateProductFromPortfolioOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisassociateTagOptionFromResourceInput {
    /// <p>The resource identifier.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p>The TagOption identifier.</p>
    #[serde(rename = "TagOptionId")]
    pub tag_option_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DisassociateTagOptionFromResourceOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ExecuteProvisionedProductPlanInput {
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
pub struct ExecuteProvisionedProductPlanOutput {
    /// <p>Information about the result of provisioning the product.</p>
    #[serde(rename = "RecordDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_detail: Option<RecordDetail>,
}

/// <p>Summary information about a product path for a user.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
pub struct ListAcceptedPortfolioSharesInput {
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
pub struct ListAcceptedPortfolioSharesOutput {
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
pub struct ListConstraintsForPortfolioInput {
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
pub struct ListConstraintsForPortfolioOutput {
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
pub struct ListLaunchPathsInput {
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
pub struct ListLaunchPathsOutput {
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
pub struct ListPortfolioAccessInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The portfolio identifier.</p>
    #[serde(rename = "PortfolioId")]
    pub portfolio_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListPortfolioAccessOutput {
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
pub struct ListPortfoliosForProductInput {
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
pub struct ListPortfoliosForProductOutput {
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
pub struct ListPortfoliosInput {
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
pub struct ListPortfoliosOutput {
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
pub struct ListPrincipalsForPortfolioInput {
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
pub struct ListPrincipalsForPortfolioOutput {
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
pub struct ListProvisionedProductPlansInput {
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
pub struct ListProvisionedProductPlansOutput {
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
pub struct ListProvisioningArtifactsInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The product identifier.</p>
    #[serde(rename = "ProductId")]
    pub product_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListProvisioningArtifactsOutput {
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
pub struct ListRecordHistoryInput {
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
pub struct ListRecordHistoryOutput {
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
pub struct ListResourcesForTagOptionInput {
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
pub struct ListResourcesForTagOptionOutput {
    /// <p>The page token for the next set of results. To retrieve the first set of results, use null.</p>
    #[serde(rename = "PageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// <p>Information about the resources.</p>
    #[serde(rename = "ResourceDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_details: Option<Vec<ResourceDetail>>,
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
pub struct ListTagOptionsInput {
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
pub struct ListTagOptionsOutput {
    /// <p>The page token for the next set of results. To retrieve the first set of results, use null.</p>
    #[serde(rename = "PageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// <p>Information about the TagOptions.</p>
    #[serde(rename = "TagOptionDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_option_details: Option<Vec<TagOptionDetail>>,
}

/// <p>The constraints that the administrator has put on the parameter.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ParameterConstraints {
    /// <p>The values that the administrator has allowed for the parameter.</p>
    #[serde(rename = "AllowedValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_values: Option<Vec<String>>,
}

/// <p>Information about a portfolio.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
pub struct ProvisionProductInput {
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
    /// <p>One or more tags.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ProvisionProductOutput {
    /// <p>Information about the result of provisioning the product.</p>
    #[serde(rename = "RecordDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_detail: Option<RecordDetail>,
}

/// <p>Information about a provisioned product.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
    /// <p><p>The current status of the provisioned product.</p> <ul> <li> <p> <code>AVAILABLE</code> - Stable state, ready to perform any operation. The most recent operation succeeded and completed.</p> </li> <li> <p> <code>UNDER_CHANGE</code> - Transitive state, operations performed might not have valid results. Wait for an <code>AVAILABLE</code> status before performing operations.</p> </li> <li> <p> <code>TAINTED</code> - Stable state, ready to perform any operation. The stack has completed the requested operation but is not exactly what was requested. For example, a request to update to a new version failed and the stack rolled back to the current version.</p> </li> <li> <p> <code>ERROR</code> - An unexpected error occurred, the provisioned product exists but the stack is not running. For example, CloudFormation received a parameter value that was not valid and could not launch the stack.</p> </li> </ul></p>
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
    /// <p>The type of provisioned product. The supported value is <code>CFN_STACK</code>.</p>
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
    /// <p><p>The current status of the provisioned product.</p> <ul> <li> <p> <code>AVAILABLE</code> - Stable state, ready to perform any operation. The most recent operation succeeded and completed.</p> </li> <li> <p> <code>UNDER_CHANGE</code> - Transitive state, operations performed might not have valid results. Wait for an <code>AVAILABLE</code> status before performing operations.</p> </li> <li> <p> <code>TAINTED</code> - Stable state, ready to perform any operation. The stack has completed the requested operation but is not exactly what was requested. For example, a request to update to a new version failed and the stack rolled back to the current version.</p> </li> <li> <p> <code>ERROR</code> - An unexpected error occurred, the provisioned product exists but the stack is not running. For example, CloudFormation received a parameter value that was not valid and could not launch the stack.</p> </li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The current status message of the provisioned product.</p>
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// <p>The type of provisioned product. The supported value is <code>CFN_STACK</code>.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Information about a plan.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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

/// <p>Information about a provisioning artifact (also known as a version) for a product.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ProvisioningArtifactProperties {
    /// <p>The description of the provisioning artifact, including how it differs from the previous provisioning artifact.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
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

/// <p>Information about a request operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
    /// <p>The type of provisioned product. The supported value is <code>CFN_STACK</code>.</p>
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
pub struct RejectPortfolioShareInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The portfolio identifier.</p>
    #[serde(rename = "PortfolioId")]
    pub portfolio_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct RejectPortfolioShareOutput {}

/// <p>Information about a resource change that will occur when a plan is executed.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
pub struct ScanProvisionedProductsInput {
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
pub struct ScanProvisionedProductsOutput {
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
pub struct SearchProductsAsAdminInput {
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
pub struct SearchProductsAsAdminOutput {
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
pub struct SearchProductsInput {
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
pub struct SearchProductsOutput {
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
pub struct SearchProvisionedProductsInput {
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
pub struct SearchProvisionedProductsOutput {
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
pub struct TerminateProvisionedProductInput {
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
pub struct TerminateProvisionedProductOutput {
    /// <p>Information about the result of this request.</p>
    #[serde(rename = "RecordDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_detail: Option<RecordDetail>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateConstraintInput {
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
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateConstraintOutput {
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
pub struct UpdatePortfolioInput {
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
pub struct UpdatePortfolioOutput {
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
pub struct UpdateProductInput {
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
pub struct UpdateProductOutput {
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
pub struct UpdateProvisionedProductInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The new path identifier. This value is optional if the product has a default path, and required if the product has more than one path.</p>
    #[serde(rename = "PathId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_id: Option<String>,
    /// <p>The identifier of the provisioned product.</p>
    #[serde(rename = "ProductId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// <p>The identifier of the provisioned product. You cannot specify both <code>ProvisionedProductName</code> and <code>ProvisionedProductId</code>.</p>
    #[serde(rename = "ProvisionedProductId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_product_id: Option<String>,
    /// <p>The updated name of the provisioned product. You cannot specify both <code>ProvisionedProductName</code> and <code>ProvisionedProductId</code>.</p>
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
    /// <p>The idempotency token that uniquely identifies the provisioning update request.</p>
    #[serde(rename = "UpdateToken")]
    pub update_token: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateProvisionedProductOutput {
    /// <p>Information about the result of the request.</p>
    #[serde(rename = "RecordDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_detail: Option<RecordDetail>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateProvisioningArtifactInput {
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
pub struct UpdateProvisioningArtifactOutput {
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateTagOptionInput {
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
pub struct UpdateTagOptionOutput {
    /// <p>Information about the TagOption.</p>
    #[serde(rename = "TagOptionDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_option_detail: Option<TagOptionDetail>,
}

/// <p>Additional information provided by the administrator.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
                let raw_error_type = json
                    .get("__type")
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
impl From<io::Error> for AcceptPortfolioShareError {
    fn from(err: io::Error) -> AcceptPortfolioShareError {
        AcceptPortfolioShareError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The current limits of the service would have been exceeded by this operation. Decrease your resource use or increase your service limits and retry the operation.</p>
    LimitExceeded(String),
    /// <p>The specified resource was not found.</p>
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
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParametersException" => {
                        AssociatePrincipalWithPortfolioError::InvalidParameters(String::from(
                            error_message,
                        ))
                    }
                    "LimitExceededException" => {
                        AssociatePrincipalWithPortfolioError::LimitExceeded(String::from(
                            error_message,
                        ))
                    }
                    "ResourceNotFoundException" => {
                        AssociatePrincipalWithPortfolioError::ResourceNotFound(String::from(
                            error_message,
                        ))
                    }
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
impl From<io::Error> for AssociatePrincipalWithPortfolioError {
    fn from(err: io::Error) -> AssociatePrincipalWithPortfolioError {
        AssociatePrincipalWithPortfolioError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The current limits of the service would have been exceeded by this operation. Decrease your resource use or increase your service limits and retry the operation.</p>
    LimitExceeded(String),
    /// <p>The specified resource was not found.</p>
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
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParametersException" => {
                        AssociateProductWithPortfolioError::InvalidParameters(String::from(
                            error_message,
                        ))
                    }
                    "LimitExceededException" => AssociateProductWithPortfolioError::LimitExceeded(
                        String::from(error_message),
                    ),
                    "ResourceNotFoundException" => {
                        AssociateProductWithPortfolioError::ResourceNotFound(String::from(
                            error_message,
                        ))
                    }
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
impl From<io::Error> for AssociateProductWithPortfolioError {
    fn from(err: io::Error) -> AssociateProductWithPortfolioError {
        AssociateProductWithPortfolioError::HttpDispatch(HttpDispatchError::from(err))
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl AssociateTagOptionWithResourceError {
    pub fn from_body(body: &str) -> AssociateTagOptionWithResourceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DuplicateResourceException" => {
                        AssociateTagOptionWithResourceError::DuplicateResource(String::from(
                            error_message,
                        ))
                    }
                    "InvalidParametersException" => {
                        AssociateTagOptionWithResourceError::InvalidParameters(String::from(
                            error_message,
                        ))
                    }
                    "InvalidStateException" => AssociateTagOptionWithResourceError::InvalidState(
                        String::from(error_message),
                    ),
                    "LimitExceededException" => AssociateTagOptionWithResourceError::LimitExceeded(
                        String::from(error_message),
                    ),
                    "ResourceNotFoundException" => {
                        AssociateTagOptionWithResourceError::ResourceNotFound(String::from(
                            error_message,
                        ))
                    }
                    "TagOptionNotMigratedException" => {
                        AssociateTagOptionWithResourceError::TagOptionNotMigrated(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        AssociateTagOptionWithResourceError::Validation(error_message.to_string())
                    }
                    _ => AssociateTagOptionWithResourceError::Unknown(String::from(body)),
                }
            }
            Err(_) => AssociateTagOptionWithResourceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AssociateTagOptionWithResourceError {
    fn from(err: serde_json::error::Error) -> AssociateTagOptionWithResourceError {
        AssociateTagOptionWithResourceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AssociateTagOptionWithResourceError {
    fn from(err: CredentialsError) -> AssociateTagOptionWithResourceError {
        AssociateTagOptionWithResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AssociateTagOptionWithResourceError {
    fn from(err: HttpDispatchError) -> AssociateTagOptionWithResourceError {
        AssociateTagOptionWithResourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for AssociateTagOptionWithResourceError {
    fn from(err: io::Error) -> AssociateTagOptionWithResourceError {
        AssociateTagOptionWithResourceError::HttpDispatch(HttpDispatchError::from(err))
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
            AssociateTagOptionWithResourceError::Validation(ref cause) => cause,
            AssociateTagOptionWithResourceError::Credentials(ref err) => err.description(),
            AssociateTagOptionWithResourceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AssociateTagOptionWithResourceError::Unknown(ref cause) => cause,
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CopyProductError {
    pub fn from_body(body: &str) -> CopyProductError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParametersException" => {
                        CopyProductError::InvalidParameters(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        CopyProductError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        CopyProductError::Validation(error_message.to_string())
                    }
                    _ => CopyProductError::Unknown(String::from(body)),
                }
            }
            Err(_) => CopyProductError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CopyProductError {
    fn from(err: serde_json::error::Error) -> CopyProductError {
        CopyProductError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CopyProductError {
    fn from(err: CredentialsError) -> CopyProductError {
        CopyProductError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CopyProductError {
    fn from(err: HttpDispatchError) -> CopyProductError {
        CopyProductError::HttpDispatch(err)
    }
}
impl From<io::Error> for CopyProductError {
    fn from(err: io::Error) -> CopyProductError {
        CopyProductError::HttpDispatch(HttpDispatchError::from(err))
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
            CopyProductError::Validation(ref cause) => cause,
            CopyProductError::Credentials(ref err) => err.description(),
            CopyProductError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CopyProductError::Unknown(ref cause) => cause,
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
                let raw_error_type = json
                    .get("__type")
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
impl From<io::Error> for CreateConstraintError {
    fn from(err: io::Error) -> CreateConstraintError {
        CreateConstraintError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The current limits of the service would have been exceeded by this operation. Decrease your resource use or increase your service limits and retry the operation.</p>
    LimitExceeded(String),
    /// <p>An operation requiring TagOptions failed because the TagOptions migration process has not been performed for this account. Please use the AWS console to perform the migration process before retrying the operation.</p>
    TagOptionNotMigrated(String),
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
                let raw_error_type = json
                    .get("__type")
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
                    "TagOptionNotMigratedException" => {
                        CreatePortfolioError::TagOptionNotMigrated(String::from(error_message))
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
impl From<io::Error> for CreatePortfolioError {
    fn from(err: io::Error) -> CreatePortfolioError {
        CreatePortfolioError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The current limits of the service would have been exceeded by this operation. Decrease your resource use or increase your service limits and retry the operation.</p>
    LimitExceeded(String),
    /// <p>The specified resource was not found.</p>
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
                let raw_error_type = json
                    .get("__type")
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
impl From<io::Error> for CreatePortfolioShareError {
    fn from(err: io::Error) -> CreatePortfolioShareError {
        CreatePortfolioShareError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The current limits of the service would have been exceeded by this operation. Decrease your resource use or increase your service limits and retry the operation.</p>
    LimitExceeded(String),
    /// <p>An operation requiring TagOptions failed because the TagOptions migration process has not been performed for this account. Please use the AWS console to perform the migration process before retrying the operation.</p>
    TagOptionNotMigrated(String),
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
                let raw_error_type = json
                    .get("__type")
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
                    "TagOptionNotMigratedException" => {
                        CreateProductError::TagOptionNotMigrated(String::from(error_message))
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
impl From<io::Error> for CreateProductError {
    fn from(err: io::Error) -> CreateProductError {
        CreateProductError::HttpDispatch(HttpDispatchError::from(err))
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
            CreateProductError::Validation(ref cause) => cause,
            CreateProductError::Credentials(ref err) => err.description(),
            CreateProductError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateProductError::Unknown(ref cause) => cause,
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateProvisionedProductPlanError {
    pub fn from_body(body: &str) -> CreateProvisionedProductPlanError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParametersException" => {
                        CreateProvisionedProductPlanError::InvalidParameters(String::from(
                            error_message,
                        ))
                    }
                    "InvalidStateException" => {
                        CreateProvisionedProductPlanError::InvalidState(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        CreateProvisionedProductPlanError::ResourceNotFound(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        CreateProvisionedProductPlanError::Validation(error_message.to_string())
                    }
                    _ => CreateProvisionedProductPlanError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateProvisionedProductPlanError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateProvisionedProductPlanError {
    fn from(err: serde_json::error::Error) -> CreateProvisionedProductPlanError {
        CreateProvisionedProductPlanError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateProvisionedProductPlanError {
    fn from(err: CredentialsError) -> CreateProvisionedProductPlanError {
        CreateProvisionedProductPlanError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateProvisionedProductPlanError {
    fn from(err: HttpDispatchError) -> CreateProvisionedProductPlanError {
        CreateProvisionedProductPlanError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateProvisionedProductPlanError {
    fn from(err: io::Error) -> CreateProvisionedProductPlanError {
        CreateProvisionedProductPlanError::HttpDispatch(HttpDispatchError::from(err))
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
            CreateProvisionedProductPlanError::Validation(ref cause) => cause,
            CreateProvisionedProductPlanError::Credentials(ref err) => err.description(),
            CreateProvisionedProductPlanError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateProvisionedProductPlanError::Unknown(ref cause) => cause,
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
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParametersException" => {
                        CreateProvisioningArtifactError::InvalidParameters(String::from(
                            error_message,
                        ))
                    }
                    "LimitExceededException" => {
                        CreateProvisioningArtifactError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        CreateProvisioningArtifactError::ResourceNotFound(String::from(
                            error_message,
                        ))
                    }
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
impl From<io::Error> for CreateProvisioningArtifactError {
    fn from(err: io::Error) -> CreateProvisioningArtifactError {
        CreateProvisioningArtifactError::HttpDispatch(HttpDispatchError::from(err))
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
/// Errors returned by CreateTagOption
#[derive(Debug, PartialEq)]
pub enum CreateTagOptionError {
    /// <p>The specified resource is a duplicate.</p>
    DuplicateResource(String),
    /// <p>The current limits of the service would have been exceeded by this operation. Decrease your resource use or increase your service limits and retry the operation.</p>
    LimitExceeded(String),
    /// <p>An operation requiring TagOptions failed because the TagOptions migration process has not been performed for this account. Please use the AWS console to perform the migration process before retrying the operation.</p>
    TagOptionNotMigrated(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateTagOptionError {
    pub fn from_body(body: &str) -> CreateTagOptionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DuplicateResourceException" => {
                        CreateTagOptionError::DuplicateResource(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateTagOptionError::LimitExceeded(String::from(error_message))
                    }
                    "TagOptionNotMigratedException" => {
                        CreateTagOptionError::TagOptionNotMigrated(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateTagOptionError::Validation(error_message.to_string())
                    }
                    _ => CreateTagOptionError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateTagOptionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateTagOptionError {
    fn from(err: serde_json::error::Error) -> CreateTagOptionError {
        CreateTagOptionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateTagOptionError {
    fn from(err: CredentialsError) -> CreateTagOptionError {
        CreateTagOptionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateTagOptionError {
    fn from(err: HttpDispatchError) -> CreateTagOptionError {
        CreateTagOptionError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateTagOptionError {
    fn from(err: io::Error) -> CreateTagOptionError {
        CreateTagOptionError::HttpDispatch(HttpDispatchError::from(err))
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
            CreateTagOptionError::Validation(ref cause) => cause,
            CreateTagOptionError::Credentials(ref err) => err.description(),
            CreateTagOptionError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateTagOptionError::Unknown(ref cause) => cause,
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
                let raw_error_type = json
                    .get("__type")
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
impl From<io::Error> for DeleteConstraintError {
    fn from(err: io::Error) -> DeleteConstraintError {
        DeleteConstraintError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>A resource that is currently in use. Ensure that the resource is not in use and retry the operation.</p>
    ResourceInUse(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>An operation requiring TagOptions failed because the TagOptions migration process has not been performed for this account. Please use the AWS console to perform the migration process before retrying the operation.</p>
    TagOptionNotMigrated(String),
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
                let raw_error_type = json
                    .get("__type")
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
                    "TagOptionNotMigratedException" => {
                        DeletePortfolioError::TagOptionNotMigrated(String::from(error_message))
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
impl From<io::Error> for DeletePortfolioError {
    fn from(err: io::Error) -> DeletePortfolioError {
        DeletePortfolioError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>The specified resource was not found.</p>
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
                let raw_error_type = json
                    .get("__type")
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
impl From<io::Error> for DeletePortfolioShareError {
    fn from(err: io::Error) -> DeletePortfolioShareError {
        DeletePortfolioShareError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>A resource that is currently in use. Ensure that the resource is not in use and retry the operation.</p>
    ResourceInUse(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>An operation requiring TagOptions failed because the TagOptions migration process has not been performed for this account. Please use the AWS console to perform the migration process before retrying the operation.</p>
    TagOptionNotMigrated(String),
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
                let raw_error_type = json
                    .get("__type")
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
                    "TagOptionNotMigratedException" => {
                        DeleteProductError::TagOptionNotMigrated(String::from(error_message))
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
impl From<io::Error> for DeleteProductError {
    fn from(err: io::Error) -> DeleteProductError {
        DeleteProductError::HttpDispatch(HttpDispatchError::from(err))
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
            DeleteProductError::Validation(ref cause) => cause,
            DeleteProductError::Credentials(ref err) => err.description(),
            DeleteProductError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteProductError::Unknown(ref cause) => cause,
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteProvisionedProductPlanError {
    pub fn from_body(body: &str) -> DeleteProvisionedProductPlanError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParametersException" => {
                        DeleteProvisionedProductPlanError::InvalidParameters(String::from(
                            error_message,
                        ))
                    }
                    "ResourceNotFoundException" => {
                        DeleteProvisionedProductPlanError::ResourceNotFound(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        DeleteProvisionedProductPlanError::Validation(error_message.to_string())
                    }
                    _ => DeleteProvisionedProductPlanError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteProvisionedProductPlanError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteProvisionedProductPlanError {
    fn from(err: serde_json::error::Error) -> DeleteProvisionedProductPlanError {
        DeleteProvisionedProductPlanError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteProvisionedProductPlanError {
    fn from(err: CredentialsError) -> DeleteProvisionedProductPlanError {
        DeleteProvisionedProductPlanError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteProvisionedProductPlanError {
    fn from(err: HttpDispatchError) -> DeleteProvisionedProductPlanError {
        DeleteProvisionedProductPlanError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteProvisionedProductPlanError {
    fn from(err: io::Error) -> DeleteProvisionedProductPlanError {
        DeleteProvisionedProductPlanError::HttpDispatch(HttpDispatchError::from(err))
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
            DeleteProvisionedProductPlanError::Validation(ref cause) => cause,
            DeleteProvisionedProductPlanError::Credentials(ref err) => err.description(),
            DeleteProvisionedProductPlanError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteProvisionedProductPlanError::Unknown(ref cause) => cause,
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
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParametersException" => {
                        DeleteProvisioningArtifactError::InvalidParameters(String::from(
                            error_message,
                        ))
                    }
                    "ResourceInUseException" => {
                        DeleteProvisioningArtifactError::ResourceInUse(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteProvisioningArtifactError::ResourceNotFound(String::from(
                            error_message,
                        ))
                    }
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
impl From<io::Error> for DeleteProvisioningArtifactError {
    fn from(err: io::Error) -> DeleteProvisioningArtifactError {
        DeleteProvisioningArtifactError::HttpDispatch(HttpDispatchError::from(err))
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
/// Errors returned by DeleteTagOption
#[derive(Debug, PartialEq)]
pub enum DeleteTagOptionError {
    /// <p>A resource that is currently in use. Ensure that the resource is not in use and retry the operation.</p>
    ResourceInUse(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>An operation requiring TagOptions failed because the TagOptions migration process has not been performed for this account. Please use the AWS console to perform the migration process before retrying the operation.</p>
    TagOptionNotMigrated(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteTagOptionError {
    pub fn from_body(body: &str) -> DeleteTagOptionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceInUseException" => {
                        DeleteTagOptionError::ResourceInUse(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteTagOptionError::ResourceNotFound(String::from(error_message))
                    }
                    "TagOptionNotMigratedException" => {
                        DeleteTagOptionError::TagOptionNotMigrated(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteTagOptionError::Validation(error_message.to_string())
                    }
                    _ => DeleteTagOptionError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteTagOptionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteTagOptionError {
    fn from(err: serde_json::error::Error) -> DeleteTagOptionError {
        DeleteTagOptionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteTagOptionError {
    fn from(err: CredentialsError) -> DeleteTagOptionError {
        DeleteTagOptionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteTagOptionError {
    fn from(err: HttpDispatchError) -> DeleteTagOptionError {
        DeleteTagOptionError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteTagOptionError {
    fn from(err: io::Error) -> DeleteTagOptionError {
        DeleteTagOptionError::HttpDispatch(HttpDispatchError::from(err))
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
            DeleteTagOptionError::Validation(ref cause) => cause,
            DeleteTagOptionError::Credentials(ref err) => err.description(),
            DeleteTagOptionError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteTagOptionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeConstraint
#[derive(Debug, PartialEq)]
pub enum DescribeConstraintError {
    /// <p>The specified resource was not found.</p>
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
                let raw_error_type = json
                    .get("__type")
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
impl From<io::Error> for DescribeConstraintError {
    fn from(err: io::Error) -> DescribeConstraintError {
        DescribeConstraintError::HttpDispatch(HttpDispatchError::from(err))
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
/// Errors returned by DescribeCopyProductStatus
#[derive(Debug, PartialEq)]
pub enum DescribeCopyProductStatusError {
    /// <p>The specified resource was not found.</p>
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

impl DescribeCopyProductStatusError {
    pub fn from_body(body: &str) -> DescribeCopyProductStatusError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        DescribeCopyProductStatusError::ResourceNotFound(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        DescribeCopyProductStatusError::Validation(error_message.to_string())
                    }
                    _ => DescribeCopyProductStatusError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeCopyProductStatusError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeCopyProductStatusError {
    fn from(err: serde_json::error::Error) -> DescribeCopyProductStatusError {
        DescribeCopyProductStatusError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeCopyProductStatusError {
    fn from(err: CredentialsError) -> DescribeCopyProductStatusError {
        DescribeCopyProductStatusError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeCopyProductStatusError {
    fn from(err: HttpDispatchError) -> DescribeCopyProductStatusError {
        DescribeCopyProductStatusError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeCopyProductStatusError {
    fn from(err: io::Error) -> DescribeCopyProductStatusError {
        DescribeCopyProductStatusError::HttpDispatch(HttpDispatchError::from(err))
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
            DescribeCopyProductStatusError::Validation(ref cause) => cause,
            DescribeCopyProductStatusError::Credentials(ref err) => err.description(),
            DescribeCopyProductStatusError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeCopyProductStatusError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribePortfolio
#[derive(Debug, PartialEq)]
pub enum DescribePortfolioError {
    /// <p>The specified resource was not found.</p>
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
                let raw_error_type = json
                    .get("__type")
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
impl From<io::Error> for DescribePortfolioError {
    fn from(err: io::Error) -> DescribePortfolioError {
        DescribePortfolioError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
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
                let raw_error_type = json
                    .get("__type")
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
impl From<io::Error> for DescribeProductError {
    fn from(err: io::Error) -> DescribeProductError {
        DescribeProductError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>The specified resource was not found.</p>
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
                let raw_error_type = json
                    .get("__type")
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
impl From<io::Error> for DescribeProductAsAdminError {
    fn from(err: io::Error) -> DescribeProductAsAdminError {
        DescribeProductAsAdminError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
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
                let raw_error_type = json
                    .get("__type")
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
impl From<io::Error> for DescribeProductViewError {
    fn from(err: io::Error) -> DescribeProductViewError {
        DescribeProductViewError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>The specified resource was not found.</p>
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
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        DescribeProvisionedProductError::ResourceNotFound(String::from(
                            error_message,
                        ))
                    }
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
impl From<io::Error> for DescribeProvisionedProductError {
    fn from(err: io::Error) -> DescribeProvisionedProductError {
        DescribeProvisionedProductError::HttpDispatch(HttpDispatchError::from(err))
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
/// Errors returned by DescribeProvisionedProductPlan
#[derive(Debug, PartialEq)]
pub enum DescribeProvisionedProductPlanError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
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

impl DescribeProvisionedProductPlanError {
    pub fn from_body(body: &str) -> DescribeProvisionedProductPlanError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParametersException" => {
                        DescribeProvisionedProductPlanError::InvalidParameters(String::from(
                            error_message,
                        ))
                    }
                    "ResourceNotFoundException" => {
                        DescribeProvisionedProductPlanError::ResourceNotFound(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        DescribeProvisionedProductPlanError::Validation(error_message.to_string())
                    }
                    _ => DescribeProvisionedProductPlanError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeProvisionedProductPlanError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeProvisionedProductPlanError {
    fn from(err: serde_json::error::Error) -> DescribeProvisionedProductPlanError {
        DescribeProvisionedProductPlanError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeProvisionedProductPlanError {
    fn from(err: CredentialsError) -> DescribeProvisionedProductPlanError {
        DescribeProvisionedProductPlanError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeProvisionedProductPlanError {
    fn from(err: HttpDispatchError) -> DescribeProvisionedProductPlanError {
        DescribeProvisionedProductPlanError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeProvisionedProductPlanError {
    fn from(err: io::Error) -> DescribeProvisionedProductPlanError {
        DescribeProvisionedProductPlanError::HttpDispatch(HttpDispatchError::from(err))
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
            DescribeProvisionedProductPlanError::Validation(ref cause) => cause,
            DescribeProvisionedProductPlanError::Credentials(ref err) => err.description(),
            DescribeProvisionedProductPlanError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeProvisionedProductPlanError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeProvisioningArtifact
#[derive(Debug, PartialEq)]
pub enum DescribeProvisioningArtifactError {
    /// <p>The specified resource was not found.</p>
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
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        DescribeProvisioningArtifactError::ResourceNotFound(String::from(
                            error_message,
                        ))
                    }
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
impl From<io::Error> for DescribeProvisioningArtifactError {
    fn from(err: io::Error) -> DescribeProvisioningArtifactError {
        DescribeProvisioningArtifactError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
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
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParametersException" => {
                        DescribeProvisioningParametersError::InvalidParameters(String::from(
                            error_message,
                        ))
                    }
                    "ResourceNotFoundException" => {
                        DescribeProvisioningParametersError::ResourceNotFound(String::from(
                            error_message,
                        ))
                    }
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
impl From<io::Error> for DescribeProvisioningParametersError {
    fn from(err: io::Error) -> DescribeProvisioningParametersError {
        DescribeProvisioningParametersError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>The specified resource was not found.</p>
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
                let raw_error_type = json
                    .get("__type")
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
impl From<io::Error> for DescribeRecordError {
    fn from(err: io::Error) -> DescribeRecordError {
        DescribeRecordError::HttpDispatch(HttpDispatchError::from(err))
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
/// Errors returned by DescribeTagOption
#[derive(Debug, PartialEq)]
pub enum DescribeTagOptionError {
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>An operation requiring TagOptions failed because the TagOptions migration process has not been performed for this account. Please use the AWS console to perform the migration process before retrying the operation.</p>
    TagOptionNotMigrated(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeTagOptionError {
    pub fn from_body(body: &str) -> DescribeTagOptionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        DescribeTagOptionError::ResourceNotFound(String::from(error_message))
                    }
                    "TagOptionNotMigratedException" => {
                        DescribeTagOptionError::TagOptionNotMigrated(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeTagOptionError::Validation(error_message.to_string())
                    }
                    _ => DescribeTagOptionError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeTagOptionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeTagOptionError {
    fn from(err: serde_json::error::Error) -> DescribeTagOptionError {
        DescribeTagOptionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeTagOptionError {
    fn from(err: CredentialsError) -> DescribeTagOptionError {
        DescribeTagOptionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeTagOptionError {
    fn from(err: HttpDispatchError) -> DescribeTagOptionError {
        DescribeTagOptionError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeTagOptionError {
    fn from(err: io::Error) -> DescribeTagOptionError {
        DescribeTagOptionError::HttpDispatch(HttpDispatchError::from(err))
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
            DescribeTagOptionError::Validation(ref cause) => cause,
            DescribeTagOptionError::Credentials(ref err) => err.description(),
            DescribeTagOptionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeTagOptionError::Unknown(ref cause) => cause,
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
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParametersException" => {
                        DisassociatePrincipalFromPortfolioError::InvalidParameters(String::from(
                            error_message,
                        ))
                    }
                    "ResourceNotFoundException" => {
                        DisassociatePrincipalFromPortfolioError::ResourceNotFound(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => DisassociatePrincipalFromPortfolioError::Validation(
                        error_message.to_string(),
                    ),
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
impl From<io::Error> for DisassociatePrincipalFromPortfolioError {
    fn from(err: io::Error) -> DisassociatePrincipalFromPortfolioError {
        DisassociatePrincipalFromPortfolioError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>A resource that is currently in use. Ensure that the resource is not in use and retry the operation.</p>
    ResourceInUse(String),
    /// <p>The specified resource was not found.</p>
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
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParametersException" => {
                        DisassociateProductFromPortfolioError::InvalidParameters(String::from(
                            error_message,
                        ))
                    }
                    "ResourceInUseException" => {
                        DisassociateProductFromPortfolioError::ResourceInUse(String::from(
                            error_message,
                        ))
                    }
                    "ResourceNotFoundException" => {
                        DisassociateProductFromPortfolioError::ResourceNotFound(String::from(
                            error_message,
                        ))
                    }
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
impl From<io::Error> for DisassociateProductFromPortfolioError {
    fn from(err: io::Error) -> DisassociateProductFromPortfolioError {
        DisassociateProductFromPortfolioError::HttpDispatch(HttpDispatchError::from(err))
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
            DisassociateProductFromPortfolioError::Validation(ref cause) => cause,
            DisassociateProductFromPortfolioError::Credentials(ref err) => err.description(),
            DisassociateProductFromPortfolioError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DisassociateProductFromPortfolioError::Unknown(ref cause) => cause,
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DisassociateTagOptionFromResourceError {
    pub fn from_body(body: &str) -> DisassociateTagOptionFromResourceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        DisassociateTagOptionFromResourceError::ResourceNotFound(String::from(
                            error_message,
                        ))
                    }
                    "TagOptionNotMigratedException" => {
                        DisassociateTagOptionFromResourceError::TagOptionNotMigrated(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => DisassociateTagOptionFromResourceError::Validation(
                        error_message.to_string(),
                    ),
                    _ => DisassociateTagOptionFromResourceError::Unknown(String::from(body)),
                }
            }
            Err(_) => DisassociateTagOptionFromResourceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DisassociateTagOptionFromResourceError {
    fn from(err: serde_json::error::Error) -> DisassociateTagOptionFromResourceError {
        DisassociateTagOptionFromResourceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DisassociateTagOptionFromResourceError {
    fn from(err: CredentialsError) -> DisassociateTagOptionFromResourceError {
        DisassociateTagOptionFromResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DisassociateTagOptionFromResourceError {
    fn from(err: HttpDispatchError) -> DisassociateTagOptionFromResourceError {
        DisassociateTagOptionFromResourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for DisassociateTagOptionFromResourceError {
    fn from(err: io::Error) -> DisassociateTagOptionFromResourceError {
        DisassociateTagOptionFromResourceError::HttpDispatch(HttpDispatchError::from(err))
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
            DisassociateTagOptionFromResourceError::Validation(ref cause) => cause,
            DisassociateTagOptionFromResourceError::Credentials(ref err) => err.description(),
            DisassociateTagOptionFromResourceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DisassociateTagOptionFromResourceError::Unknown(ref cause) => cause,
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ExecuteProvisionedProductPlanError {
    pub fn from_body(body: &str) -> ExecuteProvisionedProductPlanError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParametersException" => {
                        ExecuteProvisionedProductPlanError::InvalidParameters(String::from(
                            error_message,
                        ))
                    }
                    "InvalidStateException" => ExecuteProvisionedProductPlanError::InvalidState(
                        String::from(error_message),
                    ),
                    "ResourceNotFoundException" => {
                        ExecuteProvisionedProductPlanError::ResourceNotFound(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        ExecuteProvisionedProductPlanError::Validation(error_message.to_string())
                    }
                    _ => ExecuteProvisionedProductPlanError::Unknown(String::from(body)),
                }
            }
            Err(_) => ExecuteProvisionedProductPlanError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ExecuteProvisionedProductPlanError {
    fn from(err: serde_json::error::Error) -> ExecuteProvisionedProductPlanError {
        ExecuteProvisionedProductPlanError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ExecuteProvisionedProductPlanError {
    fn from(err: CredentialsError) -> ExecuteProvisionedProductPlanError {
        ExecuteProvisionedProductPlanError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ExecuteProvisionedProductPlanError {
    fn from(err: HttpDispatchError) -> ExecuteProvisionedProductPlanError {
        ExecuteProvisionedProductPlanError::HttpDispatch(err)
    }
}
impl From<io::Error> for ExecuteProvisionedProductPlanError {
    fn from(err: io::Error) -> ExecuteProvisionedProductPlanError {
        ExecuteProvisionedProductPlanError::HttpDispatch(HttpDispatchError::from(err))
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
            ExecuteProvisionedProductPlanError::Validation(ref cause) => cause,
            ExecuteProvisionedProductPlanError::Credentials(ref err) => err.description(),
            ExecuteProvisionedProductPlanError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ExecuteProvisionedProductPlanError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListAcceptedPortfolioShares
#[derive(Debug, PartialEq)]
pub enum ListAcceptedPortfolioSharesError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
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
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParametersException" => {
                        ListAcceptedPortfolioSharesError::InvalidParameters(String::from(
                            error_message,
                        ))
                    }
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
impl From<io::Error> for ListAcceptedPortfolioSharesError {
    fn from(err: io::Error) -> ListAcceptedPortfolioSharesError {
        ListAcceptedPortfolioSharesError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
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
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParametersException" => {
                        ListConstraintsForPortfolioError::InvalidParameters(String::from(
                            error_message,
                        ))
                    }
                    "ResourceNotFoundException" => {
                        ListConstraintsForPortfolioError::ResourceNotFound(String::from(
                            error_message,
                        ))
                    }
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
impl From<io::Error> for ListConstraintsForPortfolioError {
    fn from(err: io::Error) -> ListConstraintsForPortfolioError {
        ListConstraintsForPortfolioError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
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
                let raw_error_type = json
                    .get("__type")
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
impl From<io::Error> for ListLaunchPathsError {
    fn from(err: io::Error) -> ListLaunchPathsError {
        ListLaunchPathsError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>The specified resource was not found.</p>
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
                let raw_error_type = json
                    .get("__type")
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
impl From<io::Error> for ListPortfolioAccessError {
    fn from(err: io::Error) -> ListPortfolioAccessError {
        ListPortfolioAccessError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>One or more parameters provided to the operation are not valid.</p>
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
                let raw_error_type = json
                    .get("__type")
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
impl From<io::Error> for ListPortfoliosError {
    fn from(err: io::Error) -> ListPortfoliosError {
        ListPortfoliosError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
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
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParametersException" => {
                        ListPortfoliosForProductError::InvalidParameters(String::from(
                            error_message,
                        ))
                    }
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
impl From<io::Error> for ListPortfoliosForProductError {
    fn from(err: io::Error) -> ListPortfoliosForProductError {
        ListPortfoliosForProductError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
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
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParametersException" => {
                        ListPrincipalsForPortfolioError::InvalidParameters(String::from(
                            error_message,
                        ))
                    }
                    "ResourceNotFoundException" => {
                        ListPrincipalsForPortfolioError::ResourceNotFound(String::from(
                            error_message,
                        ))
                    }
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
impl From<io::Error> for ListPrincipalsForPortfolioError {
    fn from(err: io::Error) -> ListPrincipalsForPortfolioError {
        ListPrincipalsForPortfolioError::HttpDispatch(HttpDispatchError::from(err))
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
/// Errors returned by ListProvisionedProductPlans
#[derive(Debug, PartialEq)]
pub enum ListProvisionedProductPlansError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
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

impl ListProvisionedProductPlansError {
    pub fn from_body(body: &str) -> ListProvisionedProductPlansError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParametersException" => {
                        ListProvisionedProductPlansError::InvalidParameters(String::from(
                            error_message,
                        ))
                    }
                    "ResourceNotFoundException" => {
                        ListProvisionedProductPlansError::ResourceNotFound(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        ListProvisionedProductPlansError::Validation(error_message.to_string())
                    }
                    _ => ListProvisionedProductPlansError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListProvisionedProductPlansError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListProvisionedProductPlansError {
    fn from(err: serde_json::error::Error) -> ListProvisionedProductPlansError {
        ListProvisionedProductPlansError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListProvisionedProductPlansError {
    fn from(err: CredentialsError) -> ListProvisionedProductPlansError {
        ListProvisionedProductPlansError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListProvisionedProductPlansError {
    fn from(err: HttpDispatchError) -> ListProvisionedProductPlansError {
        ListProvisionedProductPlansError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListProvisionedProductPlansError {
    fn from(err: io::Error) -> ListProvisionedProductPlansError {
        ListProvisionedProductPlansError::HttpDispatch(HttpDispatchError::from(err))
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
            ListProvisionedProductPlansError::Validation(ref cause) => cause,
            ListProvisionedProductPlansError::Credentials(ref err) => err.description(),
            ListProvisionedProductPlansError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListProvisionedProductPlansError::Unknown(ref cause) => cause,
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
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParametersException" => {
                        ListProvisioningArtifactsError::InvalidParameters(String::from(
                            error_message,
                        ))
                    }
                    "ResourceNotFoundException" => {
                        ListProvisioningArtifactsError::ResourceNotFound(String::from(
                            error_message,
                        ))
                    }
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
impl From<io::Error> for ListProvisioningArtifactsError {
    fn from(err: io::Error) -> ListProvisioningArtifactsError {
        ListProvisioningArtifactsError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>One or more parameters provided to the operation are not valid.</p>
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
                let raw_error_type = json
                    .get("__type")
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
impl From<io::Error> for ListRecordHistoryError {
    fn from(err: io::Error) -> ListRecordHistoryError {
        ListRecordHistoryError::HttpDispatch(HttpDispatchError::from(err))
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
/// Errors returned by ListResourcesForTagOption
#[derive(Debug, PartialEq)]
pub enum ListResourcesForTagOptionError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>An operation requiring TagOptions failed because the TagOptions migration process has not been performed for this account. Please use the AWS console to perform the migration process before retrying the operation.</p>
    TagOptionNotMigrated(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListResourcesForTagOptionError {
    pub fn from_body(body: &str) -> ListResourcesForTagOptionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParametersException" => {
                        ListResourcesForTagOptionError::InvalidParameters(String::from(
                            error_message,
                        ))
                    }
                    "ResourceNotFoundException" => {
                        ListResourcesForTagOptionError::ResourceNotFound(String::from(
                            error_message,
                        ))
                    }
                    "TagOptionNotMigratedException" => {
                        ListResourcesForTagOptionError::TagOptionNotMigrated(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        ListResourcesForTagOptionError::Validation(error_message.to_string())
                    }
                    _ => ListResourcesForTagOptionError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListResourcesForTagOptionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListResourcesForTagOptionError {
    fn from(err: serde_json::error::Error) -> ListResourcesForTagOptionError {
        ListResourcesForTagOptionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListResourcesForTagOptionError {
    fn from(err: CredentialsError) -> ListResourcesForTagOptionError {
        ListResourcesForTagOptionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListResourcesForTagOptionError {
    fn from(err: HttpDispatchError) -> ListResourcesForTagOptionError {
        ListResourcesForTagOptionError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListResourcesForTagOptionError {
    fn from(err: io::Error) -> ListResourcesForTagOptionError {
        ListResourcesForTagOptionError::HttpDispatch(HttpDispatchError::from(err))
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
            ListResourcesForTagOptionError::Validation(ref cause) => cause,
            ListResourcesForTagOptionError::Credentials(ref err) => err.description(),
            ListResourcesForTagOptionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListResourcesForTagOptionError::Unknown(ref cause) => cause,
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListTagOptionsError {
    pub fn from_body(body: &str) -> ListTagOptionsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParametersException" => {
                        ListTagOptionsError::InvalidParameters(String::from(error_message))
                    }
                    "TagOptionNotMigratedException" => {
                        ListTagOptionsError::TagOptionNotMigrated(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListTagOptionsError::Validation(error_message.to_string())
                    }
                    _ => ListTagOptionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListTagOptionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListTagOptionsError {
    fn from(err: serde_json::error::Error) -> ListTagOptionsError {
        ListTagOptionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListTagOptionsError {
    fn from(err: CredentialsError) -> ListTagOptionsError {
        ListTagOptionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListTagOptionsError {
    fn from(err: HttpDispatchError) -> ListTagOptionsError {
        ListTagOptionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListTagOptionsError {
    fn from(err: io::Error) -> ListTagOptionsError {
        ListTagOptionsError::HttpDispatch(HttpDispatchError::from(err))
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
            ListTagOptionsError::Validation(ref cause) => cause,
            ListTagOptionsError::Credentials(ref err) => err.description(),
            ListTagOptionsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListTagOptionsError::Unknown(ref cause) => cause,
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
                let raw_error_type = json
                    .get("__type")
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
impl From<io::Error> for ProvisionProductError {
    fn from(err: io::Error) -> ProvisionProductError {
        ProvisionProductError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>The specified resource was not found.</p>
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
                let raw_error_type = json
                    .get("__type")
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
impl From<io::Error> for RejectPortfolioShareError {
    fn from(err: io::Error) -> RejectPortfolioShareError {
        RejectPortfolioShareError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>One or more parameters provided to the operation are not valid.</p>
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
                let raw_error_type = json
                    .get("__type")
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
impl From<io::Error> for ScanProvisionedProductsError {
    fn from(err: io::Error) -> ScanProvisionedProductsError {
        ScanProvisionedProductsError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>One or more parameters provided to the operation are not valid.</p>
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
                let raw_error_type = json
                    .get("__type")
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
impl From<io::Error> for SearchProductsError {
    fn from(err: io::Error) -> SearchProductsError {
        SearchProductsError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
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
                let raw_error_type = json
                    .get("__type")
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
impl From<io::Error> for SearchProductsAsAdminError {
    fn from(err: io::Error) -> SearchProductsAsAdminError {
        SearchProductsAsAdminError::HttpDispatch(HttpDispatchError::from(err))
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
/// Errors returned by SearchProvisionedProducts
#[derive(Debug, PartialEq)]
pub enum SearchProvisionedProductsError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
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

impl SearchProvisionedProductsError {
    pub fn from_body(body: &str) -> SearchProvisionedProductsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParametersException" => {
                        SearchProvisionedProductsError::InvalidParameters(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        SearchProvisionedProductsError::Validation(error_message.to_string())
                    }
                    _ => SearchProvisionedProductsError::Unknown(String::from(body)),
                }
            }
            Err(_) => SearchProvisionedProductsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for SearchProvisionedProductsError {
    fn from(err: serde_json::error::Error) -> SearchProvisionedProductsError {
        SearchProvisionedProductsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for SearchProvisionedProductsError {
    fn from(err: CredentialsError) -> SearchProvisionedProductsError {
        SearchProvisionedProductsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SearchProvisionedProductsError {
    fn from(err: HttpDispatchError) -> SearchProvisionedProductsError {
        SearchProvisionedProductsError::HttpDispatch(err)
    }
}
impl From<io::Error> for SearchProvisionedProductsError {
    fn from(err: io::Error) -> SearchProvisionedProductsError {
        SearchProvisionedProductsError::HttpDispatch(HttpDispatchError::from(err))
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
            SearchProvisionedProductsError::Validation(ref cause) => cause,
            SearchProvisionedProductsError::Credentials(ref err) => err.description(),
            SearchProvisionedProductsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            SearchProvisionedProductsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by TerminateProvisionedProduct
#[derive(Debug, PartialEq)]
pub enum TerminateProvisionedProductError {
    /// <p>The specified resource was not found.</p>
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
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        TerminateProvisionedProductError::ResourceNotFound(String::from(
                            error_message,
                        ))
                    }
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
impl From<io::Error> for TerminateProvisionedProductError {
    fn from(err: io::Error) -> TerminateProvisionedProductError {
        TerminateProvisionedProductError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
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
                let raw_error_type = json
                    .get("__type")
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
impl From<io::Error> for UpdateConstraintError {
    fn from(err: io::Error) -> UpdateConstraintError {
        UpdateConstraintError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The current limits of the service would have been exceeded by this operation. Decrease your resource use or increase your service limits and retry the operation.</p>
    LimitExceeded(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>An operation requiring TagOptions failed because the TagOptions migration process has not been performed for this account. Please use the AWS console to perform the migration process before retrying the operation.</p>
    TagOptionNotMigrated(String),
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
                let raw_error_type = json
                    .get("__type")
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
                    "TagOptionNotMigratedException" => {
                        UpdatePortfolioError::TagOptionNotMigrated(String::from(error_message))
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
impl From<io::Error> for UpdatePortfolioError {
    fn from(err: io::Error) -> UpdatePortfolioError {
        UpdatePortfolioError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>An operation requiring TagOptions failed because the TagOptions migration process has not been performed for this account. Please use the AWS console to perform the migration process before retrying the operation.</p>
    TagOptionNotMigrated(String),
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
                let raw_error_type = json
                    .get("__type")
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
                    "TagOptionNotMigratedException" => {
                        UpdateProductError::TagOptionNotMigrated(String::from(error_message))
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
impl From<io::Error> for UpdateProductError {
    fn from(err: io::Error) -> UpdateProductError {
        UpdateProductError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
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
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParametersException" => {
                        UpdateProvisionedProductError::InvalidParameters(String::from(
                            error_message,
                        ))
                    }
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
impl From<io::Error> for UpdateProvisionedProductError {
    fn from(err: io::Error) -> UpdateProvisionedProductError {
        UpdateProvisionedProductError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
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
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParametersException" => {
                        UpdateProvisioningArtifactError::InvalidParameters(String::from(
                            error_message,
                        ))
                    }
                    "ResourceNotFoundException" => {
                        UpdateProvisioningArtifactError::ResourceNotFound(String::from(
                            error_message,
                        ))
                    }
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
impl From<io::Error> for UpdateProvisioningArtifactError {
    fn from(err: io::Error) -> UpdateProvisioningArtifactError {
        UpdateProvisioningArtifactError::HttpDispatch(HttpDispatchError::from(err))
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateTagOptionError {
    pub fn from_body(body: &str) -> UpdateTagOptionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DuplicateResourceException" => {
                        UpdateTagOptionError::DuplicateResource(String::from(error_message))
                    }
                    "InvalidParametersException" => {
                        UpdateTagOptionError::InvalidParameters(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        UpdateTagOptionError::ResourceNotFound(String::from(error_message))
                    }
                    "TagOptionNotMigratedException" => {
                        UpdateTagOptionError::TagOptionNotMigrated(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateTagOptionError::Validation(error_message.to_string())
                    }
                    _ => UpdateTagOptionError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateTagOptionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateTagOptionError {
    fn from(err: serde_json::error::Error) -> UpdateTagOptionError {
        UpdateTagOptionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateTagOptionError {
    fn from(err: CredentialsError) -> UpdateTagOptionError {
        UpdateTagOptionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateTagOptionError {
    fn from(err: HttpDispatchError) -> UpdateTagOptionError {
        UpdateTagOptionError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateTagOptionError {
    fn from(err: io::Error) -> UpdateTagOptionError {
        UpdateTagOptionError::HttpDispatch(HttpDispatchError::from(err))
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
            UpdateTagOptionError::Validation(ref cause) => cause,
            UpdateTagOptionError::Credentials(ref err) => err.description(),
            UpdateTagOptionError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateTagOptionError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the AWS Service Catalog API. AWS Service Catalog clients implement this trait.
pub trait ServiceCatalog {
    /// <p>Accepts an offer to share the specified portfolio.</p>
    fn accept_portfolio_share(
        &self,
        input: AcceptPortfolioShareInput,
    ) -> RusotoFuture<AcceptPortfolioShareOutput, AcceptPortfolioShareError>;

    /// <p>Associates the specified principal ARN with the specified portfolio.</p>
    fn associate_principal_with_portfolio(
        &self,
        input: AssociatePrincipalWithPortfolioInput,
    ) -> RusotoFuture<AssociatePrincipalWithPortfolioOutput, AssociatePrincipalWithPortfolioError>;

    /// <p>Associates the specified product with the specified portfolio.</p>
    fn associate_product_with_portfolio(
        &self,
        input: AssociateProductWithPortfolioInput,
    ) -> RusotoFuture<AssociateProductWithPortfolioOutput, AssociateProductWithPortfolioError>;

    /// <p>Associate the specified TagOption with the specified portfolio or product.</p>
    fn associate_tag_option_with_resource(
        &self,
        input: AssociateTagOptionWithResourceInput,
    ) -> RusotoFuture<AssociateTagOptionWithResourceOutput, AssociateTagOptionWithResourceError>;

    /// <p>Copies the specified source product to the specified target product or a new product.</p> <p>You can copy a product to the same account or another account. You can copy a product to the same region or another region.</p> <p>This operation is performed asynchronously. To track the progress of the operation, use <a>DescribeCopyProductStatus</a>.</p>
    fn copy_product(
        &self,
        input: CopyProductInput,
    ) -> RusotoFuture<CopyProductOutput, CopyProductError>;

    /// <p>Creates a constraint.</p>
    fn create_constraint(
        &self,
        input: CreateConstraintInput,
    ) -> RusotoFuture<CreateConstraintOutput, CreateConstraintError>;

    /// <p>Creates a portfolio.</p>
    fn create_portfolio(
        &self,
        input: CreatePortfolioInput,
    ) -> RusotoFuture<CreatePortfolioOutput, CreatePortfolioError>;

    /// <p>Shares the specified portfolio with the specified account.</p>
    fn create_portfolio_share(
        &self,
        input: CreatePortfolioShareInput,
    ) -> RusotoFuture<CreatePortfolioShareOutput, CreatePortfolioShareError>;

    /// <p>Creates a product.</p>
    fn create_product(
        &self,
        input: CreateProductInput,
    ) -> RusotoFuture<CreateProductOutput, CreateProductError>;

    /// <p>Creates a plan. A plan includes the list of resources to be created (when provisioning a new product) or modified (when updating a provisioned product) when the plan is executed.</p> <p>You can create one plan per provisioned product. To create a plan for an existing provisioned product, the product status must be AVAILBLE or TAINTED.</p> <p>To view the resource changes in the change set, use <a>DescribeProvisionedProductPlan</a>. To create or modify the provisioned product, use <a>ExecuteProvisionedProductPlan</a>.</p>
    fn create_provisioned_product_plan(
        &self,
        input: CreateProvisionedProductPlanInput,
    ) -> RusotoFuture<CreateProvisionedProductPlanOutput, CreateProvisionedProductPlanError>;

    /// <p>Creates a provisioning artifact (also known as a version) for the specified product.</p> <p>You cannot create a provisioning artifact for a product that was shared with you.</p>
    fn create_provisioning_artifact(
        &self,
        input: CreateProvisioningArtifactInput,
    ) -> RusotoFuture<CreateProvisioningArtifactOutput, CreateProvisioningArtifactError>;

    /// <p>Creates a TagOption.</p>
    fn create_tag_option(
        &self,
        input: CreateTagOptionInput,
    ) -> RusotoFuture<CreateTagOptionOutput, CreateTagOptionError>;

    /// <p>Deletes the specified constraint.</p>
    fn delete_constraint(
        &self,
        input: DeleteConstraintInput,
    ) -> RusotoFuture<DeleteConstraintOutput, DeleteConstraintError>;

    /// <p>Deletes the specified portfolio.</p> <p>You cannot delete a portfolio if it was shared with you or if it has associated products, users, constraints, or shared accounts.</p>
    fn delete_portfolio(
        &self,
        input: DeletePortfolioInput,
    ) -> RusotoFuture<DeletePortfolioOutput, DeletePortfolioError>;

    /// <p>Stops sharing the specified portfolio with the specified account.</p>
    fn delete_portfolio_share(
        &self,
        input: DeletePortfolioShareInput,
    ) -> RusotoFuture<DeletePortfolioShareOutput, DeletePortfolioShareError>;

    /// <p>Deletes the specified product.</p> <p>You cannot delete a product if it was shared with you or is associated with a portfolio.</p>
    fn delete_product(
        &self,
        input: DeleteProductInput,
    ) -> RusotoFuture<DeleteProductOutput, DeleteProductError>;

    /// <p>Deletes the specified plan.</p>
    fn delete_provisioned_product_plan(
        &self,
        input: DeleteProvisionedProductPlanInput,
    ) -> RusotoFuture<DeleteProvisionedProductPlanOutput, DeleteProvisionedProductPlanError>;

    /// <p>Deletes the specified provisioning artifact (also known as a version) for the specified product.</p> <p>You cannot delete a provisioning artifact associated with a product that was shared with you. You cannot delete the last provisioning artifact for a product, because a product must have at least one provisioning artifact.</p>
    fn delete_provisioning_artifact(
        &self,
        input: DeleteProvisioningArtifactInput,
    ) -> RusotoFuture<DeleteProvisioningArtifactOutput, DeleteProvisioningArtifactError>;

    /// <p>Deletes the specified TagOption.</p> <p>You cannot delete a TagOption if it is associated with a product or portfolio.</p>
    fn delete_tag_option(
        &self,
        input: DeleteTagOptionInput,
    ) -> RusotoFuture<DeleteTagOptionOutput, DeleteTagOptionError>;

    /// <p>Gets information about the specified constraint.</p>
    fn describe_constraint(
        &self,
        input: DescribeConstraintInput,
    ) -> RusotoFuture<DescribeConstraintOutput, DescribeConstraintError>;

    /// <p>Gets the status of the specified copy product operation.</p>
    fn describe_copy_product_status(
        &self,
        input: DescribeCopyProductStatusInput,
    ) -> RusotoFuture<DescribeCopyProductStatusOutput, DescribeCopyProductStatusError>;

    /// <p>Gets information about the specified portfolio.</p>
    fn describe_portfolio(
        &self,
        input: DescribePortfolioInput,
    ) -> RusotoFuture<DescribePortfolioOutput, DescribePortfolioError>;

    /// <p>Gets information about the specified product.</p>
    fn describe_product(
        &self,
        input: DescribeProductInput,
    ) -> RusotoFuture<DescribeProductOutput, DescribeProductError>;

    /// <p>Gets information about the specified product. This operation is run with administrator access.</p>
    fn describe_product_as_admin(
        &self,
        input: DescribeProductAsAdminInput,
    ) -> RusotoFuture<DescribeProductAsAdminOutput, DescribeProductAsAdminError>;

    /// <p>Gets information about the specified product.</p>
    fn describe_product_view(
        &self,
        input: DescribeProductViewInput,
    ) -> RusotoFuture<DescribeProductViewOutput, DescribeProductViewError>;

    /// <p>Gets information about the specified provisioned product.</p>
    fn describe_provisioned_product(
        &self,
        input: DescribeProvisionedProductInput,
    ) -> RusotoFuture<DescribeProvisionedProductOutput, DescribeProvisionedProductError>;

    /// <p>Gets information about the resource changes for the specified plan.</p>
    fn describe_provisioned_product_plan(
        &self,
        input: DescribeProvisionedProductPlanInput,
    ) -> RusotoFuture<DescribeProvisionedProductPlanOutput, DescribeProvisionedProductPlanError>;

    /// <p>Gets information about the specified provisioning artifact (also known as a version) for the specified product.</p>
    fn describe_provisioning_artifact(
        &self,
        input: DescribeProvisioningArtifactInput,
    ) -> RusotoFuture<DescribeProvisioningArtifactOutput, DescribeProvisioningArtifactError>;

    /// <p>Gets information about the configuration required to provision the specified product using the specified provisioning artifact.</p> <p>If the output contains a TagOption key with an empty list of values, there is a TagOption conflict for that key. The end user cannot take action to fix the conflict, and launch is not blocked. In subsequent calls to <a>ProvisionProduct</a>, do not include conflicted TagOption keys as tags, or this causes the error "Parameter validation failed: Missing required parameter in Tags[<i>N</i>]:<i>Value</i>". Tag the provisioned product with the value <code>sc-tagoption-conflict-portfolioId-productId</code>.</p>
    fn describe_provisioning_parameters(
        &self,
        input: DescribeProvisioningParametersInput,
    ) -> RusotoFuture<DescribeProvisioningParametersOutput, DescribeProvisioningParametersError>;

    /// <p>Gets information about the specified request operation.</p> <p>Use this operation after calling a request operation (for example, <a>ProvisionProduct</a>, <a>TerminateProvisionedProduct</a>, or <a>UpdateProvisionedProduct</a>). </p>
    fn describe_record(
        &self,
        input: DescribeRecordInput,
    ) -> RusotoFuture<DescribeRecordOutput, DescribeRecordError>;

    /// <p>Gets information about the specified TagOption.</p>
    fn describe_tag_option(
        &self,
        input: DescribeTagOptionInput,
    ) -> RusotoFuture<DescribeTagOptionOutput, DescribeTagOptionError>;

    /// <p>Disassociates a previously associated principal ARN from a specified portfolio.</p>
    fn disassociate_principal_from_portfolio(
        &self,
        input: DisassociatePrincipalFromPortfolioInput,
    ) -> RusotoFuture<
        DisassociatePrincipalFromPortfolioOutput,
        DisassociatePrincipalFromPortfolioError,
    >;

    /// <p>Disassociates the specified product from the specified portfolio. </p>
    fn disassociate_product_from_portfolio(
        &self,
        input: DisassociateProductFromPortfolioInput,
    ) -> RusotoFuture<DisassociateProductFromPortfolioOutput, DisassociateProductFromPortfolioError>;

    /// <p>Disassociates the specified TagOption from the specified resource.</p>
    fn disassociate_tag_option_from_resource(
        &self,
        input: DisassociateTagOptionFromResourceInput,
    ) -> RusotoFuture<DisassociateTagOptionFromResourceOutput, DisassociateTagOptionFromResourceError>;

    /// <p>Provisions or modifies a product based on the resource changes for the specified plan.</p>
    fn execute_provisioned_product_plan(
        &self,
        input: ExecuteProvisionedProductPlanInput,
    ) -> RusotoFuture<ExecuteProvisionedProductPlanOutput, ExecuteProvisionedProductPlanError>;

    /// <p>Lists all portfolios for which sharing was accepted by this account.</p>
    fn list_accepted_portfolio_shares(
        &self,
        input: ListAcceptedPortfolioSharesInput,
    ) -> RusotoFuture<ListAcceptedPortfolioSharesOutput, ListAcceptedPortfolioSharesError>;

    /// <p>Lists the constraints for the specified portfolio and product.</p>
    fn list_constraints_for_portfolio(
        &self,
        input: ListConstraintsForPortfolioInput,
    ) -> RusotoFuture<ListConstraintsForPortfolioOutput, ListConstraintsForPortfolioError>;

    /// <p>Lists the paths to the specified product. A path is how the user has access to a specified product, and is necessary when provisioning a product. A path also determines the constraints put on the product.</p>
    fn list_launch_paths(
        &self,
        input: ListLaunchPathsInput,
    ) -> RusotoFuture<ListLaunchPathsOutput, ListLaunchPathsError>;

    /// <p>Lists the account IDs that have access to the specified portfolio.</p>
    fn list_portfolio_access(
        &self,
        input: ListPortfolioAccessInput,
    ) -> RusotoFuture<ListPortfolioAccessOutput, ListPortfolioAccessError>;

    /// <p>Lists all portfolios in the catalog.</p>
    fn list_portfolios(
        &self,
        input: ListPortfoliosInput,
    ) -> RusotoFuture<ListPortfoliosOutput, ListPortfoliosError>;

    /// <p>Lists all portfolios that the specified product is associated with.</p>
    fn list_portfolios_for_product(
        &self,
        input: ListPortfoliosForProductInput,
    ) -> RusotoFuture<ListPortfoliosForProductOutput, ListPortfoliosForProductError>;

    /// <p>Lists all principal ARNs associated with the specified portfolio.</p>
    fn list_principals_for_portfolio(
        &self,
        input: ListPrincipalsForPortfolioInput,
    ) -> RusotoFuture<ListPrincipalsForPortfolioOutput, ListPrincipalsForPortfolioError>;

    /// <p>Lists the plans for the specified provisioned product or all plans to which the user has access.</p>
    fn list_provisioned_product_plans(
        &self,
        input: ListProvisionedProductPlansInput,
    ) -> RusotoFuture<ListProvisionedProductPlansOutput, ListProvisionedProductPlansError>;

    /// <p>Lists all provisioning artifacts (also known as versions) for the specified product.</p>
    fn list_provisioning_artifacts(
        &self,
        input: ListProvisioningArtifactsInput,
    ) -> RusotoFuture<ListProvisioningArtifactsOutput, ListProvisioningArtifactsError>;

    /// <p>Lists the specified requests or all performed requests.</p>
    fn list_record_history(
        &self,
        input: ListRecordHistoryInput,
    ) -> RusotoFuture<ListRecordHistoryOutput, ListRecordHistoryError>;

    /// <p>Lists the resources associated with the specified TagOption.</p>
    fn list_resources_for_tag_option(
        &self,
        input: ListResourcesForTagOptionInput,
    ) -> RusotoFuture<ListResourcesForTagOptionOutput, ListResourcesForTagOptionError>;

    /// <p>Lists the specified TagOptions or all TagOptions.</p>
    fn list_tag_options(
        &self,
        input: ListTagOptionsInput,
    ) -> RusotoFuture<ListTagOptionsOutput, ListTagOptionsError>;

    /// <p>Provisions the specified product.</p> <p>A provisioned product is a resourced instance of a product. For example, provisioning a product based on a CloudFormation template launches a CloudFormation stack and its underlying resources. You can check the status of this request using <a>DescribeRecord</a>.</p> <p>If the request contains a tag key with an empty list of values, there is a tag conflict for that key. Do not include conflicted keys as tags, or this causes the error "Parameter validation failed: Missing required parameter in Tags[<i>N</i>]:<i>Value</i>".</p>
    fn provision_product(
        &self,
        input: ProvisionProductInput,
    ) -> RusotoFuture<ProvisionProductOutput, ProvisionProductError>;

    /// <p>Rejects an offer to share the specified portfolio.</p>
    fn reject_portfolio_share(
        &self,
        input: RejectPortfolioShareInput,
    ) -> RusotoFuture<RejectPortfolioShareOutput, RejectPortfolioShareError>;

    /// <p>Lists the provisioned products that are available (not terminated).</p> <p>To use additional filtering, see <a>SearchProvisionedProducts</a>.</p>
    fn scan_provisioned_products(
        &self,
        input: ScanProvisionedProductsInput,
    ) -> RusotoFuture<ScanProvisionedProductsOutput, ScanProvisionedProductsError>;

    /// <p>Gets information about the products to which the caller has access.</p>
    fn search_products(
        &self,
        input: SearchProductsInput,
    ) -> RusotoFuture<SearchProductsOutput, SearchProductsError>;

    /// <p>Gets information about the products for the specified portfolio or all products.</p>
    fn search_products_as_admin(
        &self,
        input: SearchProductsAsAdminInput,
    ) -> RusotoFuture<SearchProductsAsAdminOutput, SearchProductsAsAdminError>;

    /// <p>Gets information about the provisioned products that meet the specified criteria.</p>
    fn search_provisioned_products(
        &self,
        input: SearchProvisionedProductsInput,
    ) -> RusotoFuture<SearchProvisionedProductsOutput, SearchProvisionedProductsError>;

    /// <p>Terminates the specified provisioned product.</p> <p>This operation does not delete any records associated with the provisioned product.</p> <p>You can check the status of this request using <a>DescribeRecord</a>.</p>
    fn terminate_provisioned_product(
        &self,
        input: TerminateProvisionedProductInput,
    ) -> RusotoFuture<TerminateProvisionedProductOutput, TerminateProvisionedProductError>;

    /// <p>Updates the specified constraint.</p>
    fn update_constraint(
        &self,
        input: UpdateConstraintInput,
    ) -> RusotoFuture<UpdateConstraintOutput, UpdateConstraintError>;

    /// <p>Updates the specified portfolio.</p> <p>You cannot update a product that was shared with you.</p>
    fn update_portfolio(
        &self,
        input: UpdatePortfolioInput,
    ) -> RusotoFuture<UpdatePortfolioOutput, UpdatePortfolioError>;

    /// <p>Updates the specified product.</p>
    fn update_product(
        &self,
        input: UpdateProductInput,
    ) -> RusotoFuture<UpdateProductOutput, UpdateProductError>;

    /// <p>Requests updates to the configuration of the specified provisioned product.</p> <p>If there are tags associated with the object, they cannot be updated or added. Depending on the specific updates requested, this operation can update with no interruption, with some interruption, or replace the provisioned product entirely.</p> <p>You can check the status of this request using <a>DescribeRecord</a>.</p>
    fn update_provisioned_product(
        &self,
        input: UpdateProvisionedProductInput,
    ) -> RusotoFuture<UpdateProvisionedProductOutput, UpdateProvisionedProductError>;

    /// <p>Updates the specified provisioning artifact (also known as a version) for the specified product.</p> <p>You cannot update a provisioning artifact for a product that was shared with you.</p>
    fn update_provisioning_artifact(
        &self,
        input: UpdateProvisioningArtifactInput,
    ) -> RusotoFuture<UpdateProvisioningArtifactOutput, UpdateProvisioningArtifactError>;

    /// <p>Updates the specified TagOption.</p>
    fn update_tag_option(
        &self,
        input: UpdateTagOptionInput,
    ) -> RusotoFuture<UpdateTagOptionOutput, UpdateTagOptionError>;
}
/// A client for the AWS Service Catalog API.
pub struct ServiceCatalogClient<P = CredentialsProvider, D = RequestDispatcher>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    inner: ClientInner<P, D>,
    region: region::Region,
}

impl ServiceCatalogClient {
    /// Creates a simple client backed by an implicit event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    ///
    /// See the `rusoto_core::reactor` module for more details.
    pub fn simple(region: region::Region) -> ServiceCatalogClient {
        ServiceCatalogClient::new(
            RequestDispatcher::default(),
            CredentialsProvider::default(),
            region,
        )
    }
}

impl<P, D> ServiceCatalogClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        ServiceCatalogClient {
            inner: ClientInner::new(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl<P, D> ServiceCatalog for ServiceCatalogClient<P, D>
where
    P: ProvideAwsCredentials + 'static,
    D: DispatchSignedRequest + 'static,
{
    /// <p>Accepts an offer to share the specified portfolio.</p>
    fn accept_portfolio_share(
        &self,
        input: AcceptPortfolioShareInput,
    ) -> RusotoFuture<AcceptPortfolioShareOutput, AcceptPortfolioShareError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.AcceptPortfolioShare",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<AcceptPortfolioShareOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(AcceptPortfolioShareError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Associates the specified principal ARN with the specified portfolio.</p>
    fn associate_principal_with_portfolio(
        &self,
        input: AssociatePrincipalWithPortfolioInput,
    ) -> RusotoFuture<AssociatePrincipalWithPortfolioOutput, AssociatePrincipalWithPortfolioError>
    {
        let mut request = SignedRequest::new("POST", "servicecatalog", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.AssociatePrincipalWithPortfolio",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<AssociatePrincipalWithPortfolioOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(AssociatePrincipalWithPortfolioError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Associates the specified product with the specified portfolio.</p>
    fn associate_product_with_portfolio(
        &self,
        input: AssociateProductWithPortfolioInput,
    ) -> RusotoFuture<AssociateProductWithPortfolioOutput, AssociateProductWithPortfolioError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.AssociateProductWithPortfolio",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<AssociateProductWithPortfolioOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(AssociateProductWithPortfolioError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Associate the specified TagOption with the specified portfolio or product.</p>
    fn associate_tag_option_with_resource(
        &self,
        input: AssociateTagOptionWithResourceInput,
    ) -> RusotoFuture<AssociateTagOptionWithResourceOutput, AssociateTagOptionWithResourceError>
    {
        let mut request = SignedRequest::new("POST", "servicecatalog", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.AssociateTagOptionWithResource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<AssociateTagOptionWithResourceOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(AssociateTagOptionWithResourceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Copies the specified source product to the specified target product or a new product.</p> <p>You can copy a product to the same account or another account. You can copy a product to the same region or another region.</p> <p>This operation is performed asynchronously. To track the progress of the operation, use <a>DescribeCopyProductStatus</a>.</p>
    fn copy_product(
        &self,
        input: CopyProductInput,
    ) -> RusotoFuture<CopyProductOutput, CopyProductError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWS242ServiceCatalogService.CopyProduct");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CopyProductOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CopyProductError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a constraint.</p>
    fn create_constraint(
        &self,
        input: CreateConstraintInput,
    ) -> RusotoFuture<CreateConstraintOutput, CreateConstraintError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.CreateConstraint",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateConstraintOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateConstraintError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a portfolio.</p>
    fn create_portfolio(
        &self,
        input: CreatePortfolioInput,
    ) -> RusotoFuture<CreatePortfolioOutput, CreatePortfolioError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.CreatePortfolio",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreatePortfolioOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreatePortfolioError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Shares the specified portfolio with the specified account.</p>
    fn create_portfolio_share(
        &self,
        input: CreatePortfolioShareInput,
    ) -> RusotoFuture<CreatePortfolioShareOutput, CreatePortfolioShareError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.CreatePortfolioShare",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreatePortfolioShareOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreatePortfolioShareError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a product.</p>
    fn create_product(
        &self,
        input: CreateProductInput,
    ) -> RusotoFuture<CreateProductOutput, CreateProductError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWS242ServiceCatalogService.CreateProduct");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateProductOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateProductError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a plan. A plan includes the list of resources to be created (when provisioning a new product) or modified (when updating a provisioned product) when the plan is executed.</p> <p>You can create one plan per provisioned product. To create a plan for an existing provisioned product, the product status must be AVAILBLE or TAINTED.</p> <p>To view the resource changes in the change set, use <a>DescribeProvisionedProductPlan</a>. To create or modify the provisioned product, use <a>ExecuteProvisionedProductPlan</a>.</p>
    fn create_provisioned_product_plan(
        &self,
        input: CreateProvisionedProductPlanInput,
    ) -> RusotoFuture<CreateProvisionedProductPlanOutput, CreateProvisionedProductPlanError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.CreateProvisionedProductPlan",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateProvisionedProductPlanOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateProvisionedProductPlanError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a provisioning artifact (also known as a version) for the specified product.</p> <p>You cannot create a provisioning artifact for a product that was shared with you.</p>
    fn create_provisioning_artifact(
        &self,
        input: CreateProvisioningArtifactInput,
    ) -> RusotoFuture<CreateProvisioningArtifactOutput, CreateProvisioningArtifactError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.CreateProvisioningArtifact",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateProvisioningArtifactOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateProvisioningArtifactError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a TagOption.</p>
    fn create_tag_option(
        &self,
        input: CreateTagOptionInput,
    ) -> RusotoFuture<CreateTagOptionOutput, CreateTagOptionError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.CreateTagOption",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateTagOptionOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateTagOptionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the specified constraint.</p>
    fn delete_constraint(
        &self,
        input: DeleteConstraintInput,
    ) -> RusotoFuture<DeleteConstraintOutput, DeleteConstraintError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DeleteConstraint",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteConstraintOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteConstraintError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the specified portfolio.</p> <p>You cannot delete a portfolio if it was shared with you or if it has associated products, users, constraints, or shared accounts.</p>
    fn delete_portfolio(
        &self,
        input: DeletePortfolioInput,
    ) -> RusotoFuture<DeletePortfolioOutput, DeletePortfolioError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DeletePortfolio",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeletePortfolioOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeletePortfolioError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Stops sharing the specified portfolio with the specified account.</p>
    fn delete_portfolio_share(
        &self,
        input: DeletePortfolioShareInput,
    ) -> RusotoFuture<DeletePortfolioShareOutput, DeletePortfolioShareError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DeletePortfolioShare",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeletePortfolioShareOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeletePortfolioShareError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the specified product.</p> <p>You cannot delete a product if it was shared with you or is associated with a portfolio.</p>
    fn delete_product(
        &self,
        input: DeleteProductInput,
    ) -> RusotoFuture<DeleteProductOutput, DeleteProductError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWS242ServiceCatalogService.DeleteProduct");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteProductOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteProductError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the specified plan.</p>
    fn delete_provisioned_product_plan(
        &self,
        input: DeleteProvisionedProductPlanInput,
    ) -> RusotoFuture<DeleteProvisionedProductPlanOutput, DeleteProvisionedProductPlanError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DeleteProvisionedProductPlan",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteProvisionedProductPlanOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteProvisionedProductPlanError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the specified provisioning artifact (also known as a version) for the specified product.</p> <p>You cannot delete a provisioning artifact associated with a product that was shared with you. You cannot delete the last provisioning artifact for a product, because a product must have at least one provisioning artifact.</p>
    fn delete_provisioning_artifact(
        &self,
        input: DeleteProvisioningArtifactInput,
    ) -> RusotoFuture<DeleteProvisioningArtifactOutput, DeleteProvisioningArtifactError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DeleteProvisioningArtifact",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteProvisioningArtifactOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteProvisioningArtifactError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the specified TagOption.</p> <p>You cannot delete a TagOption if it is associated with a product or portfolio.</p>
    fn delete_tag_option(
        &self,
        input: DeleteTagOptionInput,
    ) -> RusotoFuture<DeleteTagOptionOutput, DeleteTagOptionError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DeleteTagOption",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteTagOptionOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteTagOptionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets information about the specified constraint.</p>
    fn describe_constraint(
        &self,
        input: DescribeConstraintInput,
    ) -> RusotoFuture<DescribeConstraintOutput, DescribeConstraintError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DescribeConstraint",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeConstraintOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeConstraintError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets the status of the specified copy product operation.</p>
    fn describe_copy_product_status(
        &self,
        input: DescribeCopyProductStatusInput,
    ) -> RusotoFuture<DescribeCopyProductStatusOutput, DescribeCopyProductStatusError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DescribeCopyProductStatus",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeCopyProductStatusOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeCopyProductStatusError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets information about the specified portfolio.</p>
    fn describe_portfolio(
        &self,
        input: DescribePortfolioInput,
    ) -> RusotoFuture<DescribePortfolioOutput, DescribePortfolioError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DescribePortfolio",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribePortfolioOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribePortfolioError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets information about the specified product.</p>
    fn describe_product(
        &self,
        input: DescribeProductInput,
    ) -> RusotoFuture<DescribeProductOutput, DescribeProductError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DescribeProduct",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeProductOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeProductError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets information about the specified product. This operation is run with administrator access.</p>
    fn describe_product_as_admin(
        &self,
        input: DescribeProductAsAdminInput,
    ) -> RusotoFuture<DescribeProductAsAdminOutput, DescribeProductAsAdminError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DescribeProductAsAdmin",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeProductAsAdminOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeProductAsAdminError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets information about the specified product.</p>
    fn describe_product_view(
        &self,
        input: DescribeProductViewInput,
    ) -> RusotoFuture<DescribeProductViewOutput, DescribeProductViewError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DescribeProductView",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeProductViewOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeProductViewError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets information about the specified provisioned product.</p>
    fn describe_provisioned_product(
        &self,
        input: DescribeProvisionedProductInput,
    ) -> RusotoFuture<DescribeProvisionedProductOutput, DescribeProvisionedProductError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DescribeProvisionedProduct",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeProvisionedProductOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeProvisionedProductError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets information about the resource changes for the specified plan.</p>
    fn describe_provisioned_product_plan(
        &self,
        input: DescribeProvisionedProductPlanInput,
    ) -> RusotoFuture<DescribeProvisionedProductPlanOutput, DescribeProvisionedProductPlanError>
    {
        let mut request = SignedRequest::new("POST", "servicecatalog", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DescribeProvisionedProductPlan",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeProvisionedProductPlanOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeProvisionedProductPlanError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets information about the specified provisioning artifact (also known as a version) for the specified product.</p>
    fn describe_provisioning_artifact(
        &self,
        input: DescribeProvisioningArtifactInput,
    ) -> RusotoFuture<DescribeProvisioningArtifactOutput, DescribeProvisioningArtifactError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DescribeProvisioningArtifact",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeProvisioningArtifactOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeProvisioningArtifactError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets information about the configuration required to provision the specified product using the specified provisioning artifact.</p> <p>If the output contains a TagOption key with an empty list of values, there is a TagOption conflict for that key. The end user cannot take action to fix the conflict, and launch is not blocked. In subsequent calls to <a>ProvisionProduct</a>, do not include conflicted TagOption keys as tags, or this causes the error "Parameter validation failed: Missing required parameter in Tags[<i>N</i>]:<i>Value</i>". Tag the provisioned product with the value <code>sc-tagoption-conflict-portfolioId-productId</code>.</p>
    fn describe_provisioning_parameters(
        &self,
        input: DescribeProvisioningParametersInput,
    ) -> RusotoFuture<DescribeProvisioningParametersOutput, DescribeProvisioningParametersError>
    {
        let mut request = SignedRequest::new("POST", "servicecatalog", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DescribeProvisioningParameters",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeProvisioningParametersOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeProvisioningParametersError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets information about the specified request operation.</p> <p>Use this operation after calling a request operation (for example, <a>ProvisionProduct</a>, <a>TerminateProvisionedProduct</a>, or <a>UpdateProvisionedProduct</a>). </p>
    fn describe_record(
        &self,
        input: DescribeRecordInput,
    ) -> RusotoFuture<DescribeRecordOutput, DescribeRecordError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWS242ServiceCatalogService.DescribeRecord");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeRecordOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeRecordError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets information about the specified TagOption.</p>
    fn describe_tag_option(
        &self,
        input: DescribeTagOptionInput,
    ) -> RusotoFuture<DescribeTagOptionOutput, DescribeTagOptionError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DescribeTagOption",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeTagOptionOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeTagOptionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Disassociates a previously associated principal ARN from a specified portfolio.</p>
    fn disassociate_principal_from_portfolio(
        &self,
        input: DisassociatePrincipalFromPortfolioInput,
    ) -> RusotoFuture<
        DisassociatePrincipalFromPortfolioOutput,
        DisassociatePrincipalFromPortfolioError,
    > {
        let mut request = SignedRequest::new("POST", "servicecatalog", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DisassociatePrincipalFromPortfolio",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DisassociatePrincipalFromPortfolioOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DisassociatePrincipalFromPortfolioError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Disassociates the specified product from the specified portfolio. </p>
    fn disassociate_product_from_portfolio(
        &self,
        input: DisassociateProductFromPortfolioInput,
    ) -> RusotoFuture<DisassociateProductFromPortfolioOutput, DisassociateProductFromPortfolioError>
    {
        let mut request = SignedRequest::new("POST", "servicecatalog", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DisassociateProductFromPortfolio",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DisassociateProductFromPortfolioOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DisassociateProductFromPortfolioError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Disassociates the specified TagOption from the specified resource.</p>
    fn disassociate_tag_option_from_resource(
        &self,
        input: DisassociateTagOptionFromResourceInput,
    ) -> RusotoFuture<DisassociateTagOptionFromResourceOutput, DisassociateTagOptionFromResourceError>
    {
        let mut request = SignedRequest::new("POST", "servicecatalog", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DisassociateTagOptionFromResource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DisassociateTagOptionFromResourceOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DisassociateTagOptionFromResourceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Provisions or modifies a product based on the resource changes for the specified plan.</p>
    fn execute_provisioned_product_plan(
        &self,
        input: ExecuteProvisionedProductPlanInput,
    ) -> RusotoFuture<ExecuteProvisionedProductPlanOutput, ExecuteProvisionedProductPlanError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.ExecuteProvisionedProductPlan",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ExecuteProvisionedProductPlanOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ExecuteProvisionedProductPlanError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists all portfolios for which sharing was accepted by this account.</p>
    fn list_accepted_portfolio_shares(
        &self,
        input: ListAcceptedPortfolioSharesInput,
    ) -> RusotoFuture<ListAcceptedPortfolioSharesOutput, ListAcceptedPortfolioSharesError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.ListAcceptedPortfolioShares",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListAcceptedPortfolioSharesOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListAcceptedPortfolioSharesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists the constraints for the specified portfolio and product.</p>
    fn list_constraints_for_portfolio(
        &self,
        input: ListConstraintsForPortfolioInput,
    ) -> RusotoFuture<ListConstraintsForPortfolioOutput, ListConstraintsForPortfolioError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.ListConstraintsForPortfolio",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListConstraintsForPortfolioOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListConstraintsForPortfolioError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists the paths to the specified product. A path is how the user has access to a specified product, and is necessary when provisioning a product. A path also determines the constraints put on the product.</p>
    fn list_launch_paths(
        &self,
        input: ListLaunchPathsInput,
    ) -> RusotoFuture<ListLaunchPathsOutput, ListLaunchPathsError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.ListLaunchPaths",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListLaunchPathsOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListLaunchPathsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists the account IDs that have access to the specified portfolio.</p>
    fn list_portfolio_access(
        &self,
        input: ListPortfolioAccessInput,
    ) -> RusotoFuture<ListPortfolioAccessOutput, ListPortfolioAccessError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.ListPortfolioAccess",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListPortfolioAccessOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListPortfolioAccessError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists all portfolios in the catalog.</p>
    fn list_portfolios(
        &self,
        input: ListPortfoliosInput,
    ) -> RusotoFuture<ListPortfoliosOutput, ListPortfoliosError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWS242ServiceCatalogService.ListPortfolios");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListPortfoliosOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListPortfoliosError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists all portfolios that the specified product is associated with.</p>
    fn list_portfolios_for_product(
        &self,
        input: ListPortfoliosForProductInput,
    ) -> RusotoFuture<ListPortfoliosForProductOutput, ListPortfoliosForProductError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.ListPortfoliosForProduct",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListPortfoliosForProductOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListPortfoliosForProductError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists all principal ARNs associated with the specified portfolio.</p>
    fn list_principals_for_portfolio(
        &self,
        input: ListPrincipalsForPortfolioInput,
    ) -> RusotoFuture<ListPrincipalsForPortfolioOutput, ListPrincipalsForPortfolioError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.ListPrincipalsForPortfolio",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListPrincipalsForPortfolioOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListPrincipalsForPortfolioError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists the plans for the specified provisioned product or all plans to which the user has access.</p>
    fn list_provisioned_product_plans(
        &self,
        input: ListProvisionedProductPlansInput,
    ) -> RusotoFuture<ListProvisionedProductPlansOutput, ListProvisionedProductPlansError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.ListProvisionedProductPlans",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListProvisionedProductPlansOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListProvisionedProductPlansError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists all provisioning artifacts (also known as versions) for the specified product.</p>
    fn list_provisioning_artifacts(
        &self,
        input: ListProvisioningArtifactsInput,
    ) -> RusotoFuture<ListProvisioningArtifactsOutput, ListProvisioningArtifactsError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.ListProvisioningArtifacts",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListProvisioningArtifactsOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListProvisioningArtifactsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists the specified requests or all performed requests.</p>
    fn list_record_history(
        &self,
        input: ListRecordHistoryInput,
    ) -> RusotoFuture<ListRecordHistoryOutput, ListRecordHistoryError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.ListRecordHistory",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListRecordHistoryOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListRecordHistoryError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists the resources associated with the specified TagOption.</p>
    fn list_resources_for_tag_option(
        &self,
        input: ListResourcesForTagOptionInput,
    ) -> RusotoFuture<ListResourcesForTagOptionOutput, ListResourcesForTagOptionError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.ListResourcesForTagOption",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListResourcesForTagOptionOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListResourcesForTagOptionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists the specified TagOptions or all TagOptions.</p>
    fn list_tag_options(
        &self,
        input: ListTagOptionsInput,
    ) -> RusotoFuture<ListTagOptionsOutput, ListTagOptionsError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWS242ServiceCatalogService.ListTagOptions");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListTagOptionsOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListTagOptionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Provisions the specified product.</p> <p>A provisioned product is a resourced instance of a product. For example, provisioning a product based on a CloudFormation template launches a CloudFormation stack and its underlying resources. You can check the status of this request using <a>DescribeRecord</a>.</p> <p>If the request contains a tag key with an empty list of values, there is a tag conflict for that key. Do not include conflicted keys as tags, or this causes the error "Parameter validation failed: Missing required parameter in Tags[<i>N</i>]:<i>Value</i>".</p>
    fn provision_product(
        &self,
        input: ProvisionProductInput,
    ) -> RusotoFuture<ProvisionProductOutput, ProvisionProductError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.ProvisionProduct",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ProvisionProductOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ProvisionProductError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Rejects an offer to share the specified portfolio.</p>
    fn reject_portfolio_share(
        &self,
        input: RejectPortfolioShareInput,
    ) -> RusotoFuture<RejectPortfolioShareOutput, RejectPortfolioShareError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.RejectPortfolioShare",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<RejectPortfolioShareOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(RejectPortfolioShareError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists the provisioned products that are available (not terminated).</p> <p>To use additional filtering, see <a>SearchProvisionedProducts</a>.</p>
    fn scan_provisioned_products(
        &self,
        input: ScanProvisionedProductsInput,
    ) -> RusotoFuture<ScanProvisionedProductsOutput, ScanProvisionedProductsError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.ScanProvisionedProducts",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ScanProvisionedProductsOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ScanProvisionedProductsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets information about the products to which the caller has access.</p>
    fn search_products(
        &self,
        input: SearchProductsInput,
    ) -> RusotoFuture<SearchProductsOutput, SearchProductsError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWS242ServiceCatalogService.SearchProducts");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<SearchProductsOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(SearchProductsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets information about the products for the specified portfolio or all products.</p>
    fn search_products_as_admin(
        &self,
        input: SearchProductsAsAdminInput,
    ) -> RusotoFuture<SearchProductsAsAdminOutput, SearchProductsAsAdminError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.SearchProductsAsAdmin",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<SearchProductsAsAdminOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(SearchProductsAsAdminError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets information about the provisioned products that meet the specified criteria.</p>
    fn search_provisioned_products(
        &self,
        input: SearchProvisionedProductsInput,
    ) -> RusotoFuture<SearchProvisionedProductsOutput, SearchProvisionedProductsError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.SearchProvisionedProducts",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<SearchProvisionedProductsOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(SearchProvisionedProductsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Terminates the specified provisioned product.</p> <p>This operation does not delete any records associated with the provisioned product.</p> <p>You can check the status of this request using <a>DescribeRecord</a>.</p>
    fn terminate_provisioned_product(
        &self,
        input: TerminateProvisionedProductInput,
    ) -> RusotoFuture<TerminateProvisionedProductOutput, TerminateProvisionedProductError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.TerminateProvisionedProduct",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<TerminateProvisionedProductOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(TerminateProvisionedProductError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Updates the specified constraint.</p>
    fn update_constraint(
        &self,
        input: UpdateConstraintInput,
    ) -> RusotoFuture<UpdateConstraintOutput, UpdateConstraintError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.UpdateConstraint",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateConstraintOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateConstraintError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Updates the specified portfolio.</p> <p>You cannot update a product that was shared with you.</p>
    fn update_portfolio(
        &self,
        input: UpdatePortfolioInput,
    ) -> RusotoFuture<UpdatePortfolioOutput, UpdatePortfolioError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.UpdatePortfolio",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdatePortfolioOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdatePortfolioError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Updates the specified product.</p>
    fn update_product(
        &self,
        input: UpdateProductInput,
    ) -> RusotoFuture<UpdateProductOutput, UpdateProductError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWS242ServiceCatalogService.UpdateProduct");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateProductOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateProductError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Requests updates to the configuration of the specified provisioned product.</p> <p>If there are tags associated with the object, they cannot be updated or added. Depending on the specific updates requested, this operation can update with no interruption, with some interruption, or replace the provisioned product entirely.</p> <p>You can check the status of this request using <a>DescribeRecord</a>.</p>
    fn update_provisioned_product(
        &self,
        input: UpdateProvisionedProductInput,
    ) -> RusotoFuture<UpdateProvisionedProductOutput, UpdateProvisionedProductError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.UpdateProvisionedProduct",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateProvisionedProductOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateProvisionedProductError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Updates the specified provisioning artifact (also known as a version) for the specified product.</p> <p>You cannot update a provisioning artifact for a product that was shared with you.</p>
    fn update_provisioning_artifact(
        &self,
        input: UpdateProvisioningArtifactInput,
    ) -> RusotoFuture<UpdateProvisioningArtifactOutput, UpdateProvisioningArtifactError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.UpdateProvisioningArtifact",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateProvisioningArtifactOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateProvisioningArtifactError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Updates the specified TagOption.</p>
    fn update_tag_option(
        &self,
        input: UpdateTagOptionInput,
    ) -> RusotoFuture<UpdateTagOptionOutput, UpdateTagOptionError> {
        let mut request = SignedRequest::new("POST", "servicecatalog", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.UpdateTagOption",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateTagOptionOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateTagOptionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }
}

#[cfg(test)]
mod protocol_tests {}
