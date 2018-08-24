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
use rusoto_core::request::DispatchSignedRequest;
use rusoto_core::{Client, RusotoFuture};

use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};
use rusoto_core::request::HttpDispatchError;

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_str;
use serde_json::Value as SerdeJsonValue;
/// <p>Details about the application.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Application {
    /// <p>The application Amazon Resource Name (ARN).</p>
    pub application_id: String,
    /// <p>The name of the author publishing the app.</p><p>Minimum length=1. Maximum length=127.</p><p>Pattern "^[a-z0-9](([a-z0-9]|-(?!-))*[a-z0-9])?$";</p>
    pub author: String,
    /// <p>The date and time this resource was created.</p>
    pub creation_time: Option<String>,
    /// <p>The description of the application.</p><p>Minimum length=1. Maximum length=256</p>
    pub description: String,
    /// <p>A URL with more information about the application, for example
    /// the location of your GitHub repository for the application.</p>
    pub home_page_url: Option<String>,
    /// <p>Labels to improve discovery of apps in search results.</p><p>Minimum length=1. Maximum length=127. Maximum number of labels: 10</p><p>Pattern: "^[a-zA-Z0-9+\\-_:\\/@]+$";</p>
    pub labels: Option<Vec<String>>,
    /// <p>A link to a license file of the app that matches the spdxLicenseID value of your application.</p><p>Maximum size 5 MB</p>
    pub license_url: Option<String>,
    /// <p>The name of the application.</p><p>Minimum length=1. Maximum length=140</p><p>Pattern: "[a-zA-Z0-9\\-]+";</p>
    pub name: String,
    /// <p>A link to the readme file in Markdown language that contains a more detailed description of the application and how it works.</p><p>Maximum size 5 MB</p>
    pub readme_url: Option<String>,
    /// <p>A valid identifier from https://spdx.org/licenses/.</p>
    pub spdx_license_id: Option<String>,
    /// <p>Version information about the application.</p>
    pub version: Option<Version>,
}

/// <p>A list of application details.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ApplicationPage {
    /// <p>An array of application summaries.</p>
    pub applications: Vec<ApplicationSummary>,
    /// <p>The token to request the next page of results.</p>
    pub next_token: Option<String>,
}

/// <p>Policy statements applied to the application.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ApplicationPolicy {
    /// <p>An array of policy statements applied to the application.</p>
    pub statements: Vec<ApplicationPolicyStatement>,
}

/// <p>Policy statement applied to the application.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplicationPolicyStatement {
    /// <p>See <a href="https://docs.aws.amazon.com/serverlessrepo/latest/devguide/access-control-resource-based.html#application-permissions">Application Permissions</a> for the list of supported actions.</p>
    #[serde(rename = "Actions")]
    pub actions: Vec<String>,
    /// <p>An AWS account ID, or * to make the application public.</p>
    #[serde(rename = "Principals")]
    pub principals: Vec<String>,
    /// <p>A unique ID for the statement.</p>
    #[serde(rename = "StatementId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_id: Option<String>,
}

/// <p>Summary of details about the application.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ApplicationSummary {
    /// <p>The application Amazon Resource Name (ARN).</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The name of the author publishing the app.</p><p>Minimum length=1. Maximum length=127.</p><p>Pattern "^[a-z0-9](([a-z0-9]|-(?!-))*[a-z0-9])?$";</p>
    #[serde(rename = "Author")]
    pub author: String,
    /// <p>The date and time this resource was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    /// <p>The description of the application.</p><p>Minimum length=1. Maximum length=256</p>
    #[serde(rename = "Description")]
    pub description: String,
    /// <p>A URL with more information about the application, for example
    /// the location of your GitHub repository for the application.</p>
    #[serde(rename = "HomePageUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_page_url: Option<String>,
    /// <p>Labels to improve discovery of apps in search results.</p><p>Minimum length=1. Maximum length=127. Maximum number of labels: 10</p><p>Pattern: "^[a-zA-Z0-9+\\-_:\\/@]+$";</p>
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    /// <p>The name of the application.</p><p>Minimum length=1. Maximum length=140</p><p>Pattern: "[a-zA-Z0-9\\-]+";</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>A valid identifier from <a href="https://spdx.org/licenses/">https://spdx.org/licenses/</a>.</p>
    #[serde(rename = "SpdxLicenseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spdx_license_id: Option<String>,
}

/// <p>A list of version summaries for the application.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ApplicationVersionPage {
    /// <p>The token to request the next page of results.</p>
    pub next_token: Option<String>,
    /// <p>An array of version summaries for the application.</p>
    pub versions: Vec<VersionSummary>,
}

/// <p>Details of the change set.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ChangeSetDetails {
    /// <p>The application Amazon Resource Name (ARN).</p>
    pub application_id: String,
    /// <p>The Amazon Resource Name (ARN) of the change set.</p><p>Length constraints: Minimum length of 1.</p><p>Pattern: ARN:[-a-zA-Z0-9:/]*</p>
    pub change_set_id: String,
    /// <p>The semantic version of the application:</p><p>
    /// <a href="https://semver.org/">https://semver.org/</a>
    /// </p>
    pub semantic_version: String,
    /// <p>The unique ID of the stack.</p>
    pub stack_id: String,
}

/// <p>Create an application request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateApplicationInput {
    /// <p>The name of the author publishing the app.</p><p>Minimum length=1. Maximum length=127.</p><p>Pattern "^[a-z0-9](([a-z0-9]|-(?!-))*[a-z0-9])?$";</p>
    pub author: String,
    /// <p>The description of the application.</p><p>Minimum length=1. Maximum length=256</p>
    pub description: String,
    /// <p>A URL with more information about the application, for example
    /// the location of your GitHub repository for the application.</p>
    pub home_page_url: Option<String>,
    /// <p>Labels to improve discovery of apps in search results.</p><p>Minimum length=1. Maximum length=127. Maximum number of labels: 10</p><p>Pattern: "^[a-zA-Z0-9+\\-_:\\/@]+$";</p>
    pub labels: Option<Vec<String>>,
    /// <p>A local text file that contains the license of the app that matches the spdxLicenseID value of your application.
    /// The file is of the format file://&lt;path>/&lt;filename>.</p><p>Maximum size 5 MB</p><p>Note: Only one of licenseBody and licenseUrl can be specified, otherwise an error will result.</p>
    pub license_body: Option<String>,
    /// <p>A link to the S3 object that contains the license of the app that matches the spdxLicenseID value of your application.</p><p>Maximum size 5 MB</p><p>Note: Only one of licenseBody and licenseUrl can be specified, otherwise an error will result.</p>
    pub license_url: Option<String>,
    /// <p>The name of the application that you want to publish.</p><p>Minimum length=1. Maximum length=140</p><p>Pattern: "[a-zA-Z0-9\\-]+";</p>
    pub name: String,
    /// <p>A local text readme file in Markdown language that contains a more detailed description of the application and how it works.
    /// The file is of the format file://&lt;path>/&lt;filename>.</p><p>Maximum size 5 MB</p><p>Note: Only one of readmeBody and readmeUrl can be specified, otherwise an error will result.</p>
    pub readme_body: Option<String>,
    /// <p>A link to the S3 object in Markdown language that contains a more detailed description of the application and how it works.</p><p>Maximum size 5 MB</p><p>Note: Only one of readmeBody and readmeUrl can be specified, otherwise an error will result.</p>
    pub readme_url: Option<String>,
    /// <p>The semantic version of the application:</p><p>
    /// <a href="https://semver.org/">https://semver.org/</a>
    /// </p>
    pub semantic_version: Option<String>,
    /// <p>A link to a public repository for the source code of your application.</p>
    pub source_code_url: Option<String>,
    /// <p>A valid identifier from <a href="https://spdx.org/licenses/">https://spdx.org/licenses/</a>.</p>
    pub spdx_license_id: Option<String>,
    /// <p>The local raw packaged AWS SAM template file of your application.
    /// The file is of the format file://&lt;path>/&lt;filename>.</p><p>Note: Only one of templateBody and templateUrl can be specified, otherwise an error will result.</p>
    pub template_body: Option<String>,
    /// <p>A link to the S3 object cotaining the packaged AWS SAM template of your application.</p><p>Note: Only one of templateBody and templateUrl can be specified, otherwise an error will result.</p>
    pub template_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateApplicationRequest {
    /// <p>The name of the author publishing the app.</p><p>Minimum length=1. Maximum length=127.</p><p>Pattern "^[a-z0-9](([a-z0-9]|-(?!-))*[a-z0-9])?$";</p>
    #[serde(rename = "Author")]
    pub author: String,
    /// <p>The description of the application.</p><p>Minimum length=1. Maximum length=256</p>
    #[serde(rename = "Description")]
    pub description: String,
    /// <p>A URL with more information about the application, for example
    /// the location of your GitHub repository for the application.</p>
    #[serde(rename = "HomePageUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_page_url: Option<String>,
    /// <p>Labels to improve discovery of apps in search results.</p><p>Minimum length=1. Maximum length=127. Maximum number of labels: 10</p><p>Pattern: "^[a-zA-Z0-9+\\-_:\\/@]+$";</p>
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    /// <p>A local text file that contains the license of the app that matches the spdxLicenseID value of your application.
    /// The file is of the format file://&lt;path>/&lt;filename>.</p><p>Maximum size 5 MB</p><p>Note: Only one of licenseBody and licenseUrl can be specified, otherwise an error will result.</p>
    #[serde(rename = "LicenseBody")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_body: Option<String>,
    /// <p>A link to the S3 object that contains the license of the app that matches the spdxLicenseID value of your application.</p><p>Maximum size 5 MB</p><p>Note: Only one of licenseBody and licenseUrl can be specified, otherwise an error will result.</p>
    #[serde(rename = "LicenseUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_url: Option<String>,
    /// <p>The name of the application that you want to publish.</p><p>Minimum length=1. Maximum length=140</p><p>Pattern: "[a-zA-Z0-9\\-]+";</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>A local text readme file in Markdown language that contains a more detailed description of the application and how it works.
    /// The file is of the format file://&lt;path>/&lt;filename>.</p><p>Maximum size 5 MB</p><p>Note: Only one of readmeBody and readmeUrl can be specified, otherwise an error will result.</p>
    #[serde(rename = "ReadmeBody")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readme_body: Option<String>,
    /// <p>A link to the S3 object in Markdown language that contains a more detailed description of the application and how it works.</p><p>Maximum size 5 MB</p><p>Note: Only one of readmeBody and readmeUrl can be specified, otherwise an error will result.</p>
    #[serde(rename = "ReadmeUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readme_url: Option<String>,
    /// <p>The semantic version of the application:</p><p>
    /// <a href="https://semver.org/">https://semver.org/</a>
    /// </p>
    #[serde(rename = "SemanticVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_version: Option<String>,
    /// <p>A link to a public repository for the source code of your application.</p>
    #[serde(rename = "SourceCodeUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_code_url: Option<String>,
    /// <p>A valid identifier from <a href="https://spdx.org/licenses/">https://spdx.org/licenses/</a>.</p>
    #[serde(rename = "SpdxLicenseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spdx_license_id: Option<String>,
    /// <p>The local raw packaged AWS SAM template file of your application.
    /// The file is of the format file://&lt;path>/&lt;filename>.</p><p>Note: Only one of templateBody and templateUrl can be specified, otherwise an error will result.</p>
    #[serde(rename = "TemplateBody")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_body: Option<String>,
    /// <p>A link to the S3 object cotaining the packaged AWS SAM template of your application.</p><p>Note: Only one of templateBody and templateUrl can be specified, otherwise an error will result.</p>
    #[serde(rename = "TemplateUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateApplicationResponse {
    /// <p>The application Amazon Resource Name (ARN).</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>The name of the author publishing the app.</p><p>Minimum length=1. Maximum length=127.</p><p>Pattern "^[a-z0-9](([a-z0-9]|-(?!-))*[a-z0-9])?$";</p>
    #[serde(rename = "Author")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    /// <p>The date and time this resource was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    /// <p>The description of the application.</p><p>Minimum length=1. Maximum length=256</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A URL with more information about the application, for example
    /// the location of your GitHub repository for the application.</p>
    #[serde(rename = "HomePageUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_page_url: Option<String>,
    /// <p>Labels to improve discovery of apps in search results.</p><p>Minimum length=1. Maximum length=127. Maximum number of labels: 10</p><p>Pattern: "^[a-zA-Z0-9+\\-_:\\/@]+$";</p>
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    /// <p>A link to a license file of the app that matches the spdxLicenseID value of your application.</p><p>Maximum size 5 MB</p>
    #[serde(rename = "LicenseUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_url: Option<String>,
    /// <p>The name of the application.</p><p>Minimum length=1. Maximum length=140</p><p>Pattern: "[a-zA-Z0-9\\-]+";</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A link to the readme file in Markdown language that contains a more detailed description of the application and how it works.</p><p>Maximum size 5 MB</p>
    #[serde(rename = "ReadmeUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readme_url: Option<String>,
    /// <p>A valid identifier from https://spdx.org/licenses/.</p>
    #[serde(rename = "SpdxLicenseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spdx_license_id: Option<String>,
    /// <p>Version information about the application.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<Version>,
}

/// <p>Create a version request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateApplicationVersionInput {
    /// <p>A link to a public repository for the source code of your application.</p>
    pub source_code_url: Option<String>,
    /// <p>The raw packaged AWS SAM template of your application.</p>
    pub template_body: Option<String>,
    /// <p>A link to the packaged AWS SAM template of your application.</p>
    pub template_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateApplicationVersionRequest {
    /// <p>The Amazon Resource Name (ARN) of the application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The semantic version of the new version.</p>
    #[serde(rename = "SemanticVersion")]
    pub semantic_version: String,
    /// <p>A link to a public repository for the source code of your application.</p>
    #[serde(rename = "SourceCodeUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_code_url: Option<String>,
    /// <p>The raw packaged AWS SAM template of your application.</p>
    #[serde(rename = "TemplateBody")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_body: Option<String>,
    /// <p>A link to the packaged AWS SAM template of your application.</p>
    #[serde(rename = "TemplateUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateApplicationVersionResponse {
    /// <p>The application Amazon Resource Name (ARN).</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>The date and time this resource was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    /// <p>An array of parameter types supported by the application.</p>
    #[serde(rename = "ParameterDefinitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_definitions: Option<Vec<ParameterDefinition>>,
    /// <p>The semantic version of the application:</p><p>
    /// <a href="https://semver.org/">https://semver.org/</a>
    /// </p>
    #[serde(rename = "SemanticVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_version: Option<String>,
    /// <p>A link to a public repository for the source code of your application.</p>
    #[serde(rename = "SourceCodeUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_code_url: Option<String>,
    /// <p>A link to the packaged AWS SAM template of your application.</p>
    #[serde(rename = "TemplateUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_url: Option<String>,
}

/// <p>Create an application change set request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateCloudFormationChangeSetInput {
    /// <p>A list of parameter values for the parameters of the application.</p>
    pub parameter_overrides: Option<Vec<ParameterValue>>,
    /// <p>The semantic version of the application:</p><p>
    /// <a href="https://semver.org/">https://semver.org/</a>
    /// </p>
    pub semantic_version: Option<String>,
    /// <p>The name or the unique ID of the stack for which you are creating a change set. AWS CloudFormation generates
    /// the change set by comparing this stack's information with the information that you submit, such as a modified
    /// template or different parameter input values. </p><p>Constraints: Minimum length of 1.</p><p>Pattern: ([a-zA-Z][-a-zA-Z0-9]*)|(arn:\b(aws|aws-us-gov|aws-cn)\b:[-a-zA-Z0-9:/._+]*)</p>
    pub stack_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateCloudFormationChangeSetRequest {
    /// <p>The Amazon Resource Name (ARN) of the application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>A list of parameter values for the parameters of the application.</p>
    #[serde(rename = "ParameterOverrides")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_overrides: Option<Vec<ParameterValue>>,
    /// <p>The semantic version of the application:</p><p>
    /// <a href="https://semver.org/">https://semver.org/</a>
    /// </p>
    #[serde(rename = "SemanticVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_version: Option<String>,
    /// <p>The name or the unique ID of the stack for which you are creating a change set. AWS CloudFormation generates
    /// the change set by comparing this stack's information with the information that you submit, such as a modified
    /// template or different parameter input values. </p><p>Constraints: Minimum length of 1.</p><p>Pattern: ([a-zA-Z][-a-zA-Z0-9]*)|(arn:\b(aws|aws-us-gov|aws-cn)\b:[-a-zA-Z0-9:/._+]*)</p>
    #[serde(rename = "StackName")]
    pub stack_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateCloudFormationChangeSetResponse {
    /// <p>The application Amazon Resource Name (ARN).</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the change set.</p><p>Length constraints: Minimum length of 1.</p><p>Pattern: ARN:[-a-zA-Z0-9:/]*</p>
    #[serde(rename = "ChangeSetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_set_id: Option<String>,
    /// <p>The semantic version of the application:</p><p>
    /// <a href="https://semver.org/">https://semver.org/</a>
    /// </p>
    #[serde(rename = "SemanticVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_version: Option<String>,
    /// <p>The unique ID of the stack.</p>
    #[serde(rename = "StackId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteApplicationRequest {
    /// <p>The Amazon Resource Name (ARN) of the application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetApplicationPolicyRequest {
    /// <p>The Amazon Resource Name (ARN) of the application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetApplicationPolicyResponse {
    /// <p>An array of policy statements applied to the application.</p>
    #[serde(rename = "Statements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statements: Option<Vec<ApplicationPolicyStatement>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetApplicationRequest {
    /// <p>The Amazon Resource Name (ARN) of the application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The semantic version of the application to get.</p>
    #[serde(rename = "SemanticVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetApplicationResponse {
    /// <p>The application Amazon Resource Name (ARN).</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>The name of the author publishing the app.</p><p>Minimum length=1. Maximum length=127.</p><p>Pattern "^[a-z0-9](([a-z0-9]|-(?!-))*[a-z0-9])?$";</p>
    #[serde(rename = "Author")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    /// <p>The date and time this resource was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    /// <p>The description of the application.</p><p>Minimum length=1. Maximum length=256</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A URL with more information about the application, for example
    /// the location of your GitHub repository for the application.</p>
    #[serde(rename = "HomePageUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_page_url: Option<String>,
    /// <p>Labels to improve discovery of apps in search results.</p><p>Minimum length=1. Maximum length=127. Maximum number of labels: 10</p><p>Pattern: "^[a-zA-Z0-9+\\-_:\\/@]+$";</p>
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    /// <p>A link to a license file of the app that matches the spdxLicenseID value of your application.</p><p>Maximum size 5 MB</p>
    #[serde(rename = "LicenseUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_url: Option<String>,
    /// <p>The name of the application.</p><p>Minimum length=1. Maximum length=140</p><p>Pattern: "[a-zA-Z0-9\\-]+";</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A link to the readme file in Markdown language that contains a more detailed description of the application and how it works.</p><p>Maximum size 5 MB</p>
    #[serde(rename = "ReadmeUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readme_url: Option<String>,
    /// <p>A valid identifier from https://spdx.org/licenses/.</p>
    #[serde(rename = "SpdxLicenseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spdx_license_id: Option<String>,
    /// <p>Version information about the application.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<Version>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListApplicationVersionsRequest {
    /// <p>The Amazon Resource Name (ARN) of the application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The total number of items to return.</p>
    #[serde(rename = "MaxItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i64>,
    /// <p>A token to specify where to start paginating.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListApplicationVersionsResponse {
    /// <p>The token to request the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of version summaries for the application.</p>
    #[serde(rename = "Versions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<VersionSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListApplicationsRequest {
    /// <p>The total number of items to return.</p>
    #[serde(rename = "MaxItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i64>,
    /// <p>A token to specify where to start paginating.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListApplicationsResponse {
    /// <p>An array of application summaries.</p>
    #[serde(rename = "Applications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applications: Option<Vec<ApplicationSummary>>,
    /// <p>The token to request the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Parameters supported by the application.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ParameterDefinition {
    /// <p>A regular expression that represents the patterns to allow for String types.</p>
    #[serde(rename = "AllowedPattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_pattern: Option<String>,
    /// <p>An array containing the list of values allowed for the parameter.</p>
    #[serde(rename = "AllowedValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_values: Option<Vec<String>>,
    /// <p>A string that explains a constraint when the constraint is violated. For example, without a constraint description,
    /// a parameter that has an allowed pattern of [A-Za-z0-9]+ displays the following error message when the user
    /// specifies an invalid value:</p><p>
    /// Malformed input-Parameter MyParameter must match pattern [A-Za-z0-9]+
    /// </p><p>By adding a constraint description, such as "must contain only uppercase and lowercase letters and numbers," you can display
    /// the following customized error message:</p><p>
    /// Malformed input-Parameter MyParameter must contain only uppercase and lowercase letters and numbers.
    /// </p>
    #[serde(rename = "ConstraintDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraint_description: Option<String>,
    /// <p>A value of the appropriate type for the template to use if no value is specified when a stack is created.
    /// If you define constraints for the parameter, you must specify a value that adheres to those constraints.</p>
    #[serde(rename = "DefaultValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    /// <p>A string of up to 4,000 characters that describes the parameter.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>An integer value that determines the largest number of characters that you want to allow for String types.</p>
    #[serde(rename = "MaxLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<i64>,
    /// <p>A numeric value that determines the largest numeric value that you want to allow for Number types.</p>
    #[serde(rename = "MaxValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_value: Option<i64>,
    /// <p>An integer value that determines the smallest number of characters that you want to allow for String types.</p>
    #[serde(rename = "MinLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<i64>,
    /// <p>A numeric value that determines the smallest numeric value that you want to allow for Number types.</p>
    #[serde(rename = "MinValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_value: Option<i64>,
    /// <p>The name of the parameter.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>Whether to mask the parameter value whenever anyone makes a call that describes the stack. If you set the
    /// value to true, the parameter value is masked with asterisks (*****).</p>
    #[serde(rename = "NoEcho")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_echo: Option<bool>,
    /// <p>A list of AWS SAM resources that use this parameter.</p>
    #[serde(rename = "ReferencedByResources")]
    pub referenced_by_resources: Vec<String>,
    /// <p>The type of the parameter.</p><p>Valid values: String | Number | List&lt;Number> | CommaDelimitedList
    /// </p><p>
    /// String: A literal string.</p><p>For example, users can specify "MyUserName".</p><p>
    /// Number: An integer or float. AWS CloudFormation validates the parameter value as a number. However, when you use the
    /// parameter elsewhere in your template (for example, by using the Ref intrinsic function), the parameter value becomes a string.</p><p>For example, users might specify "8888".</p><p>
    /// List&lt;Number>: An array of integers or floats that are separated by commas. AWS CloudFormation validates the parameter value as numbers. However, when
    /// you use the parameter elsewhere in your template (for example, by using the Ref intrinsic function), the parameter value becomes a list of strings.</p><p>For example, users might specify "80,20", and then Ref results in ["80","20"].</p><p>
    /// CommaDelimitedList: An array of literal strings that are separated by commas. The total number of strings should be one more than the total number of commas.
    /// Also, each member string is space-trimmed.</p><p>For example, users might specify "test,dev,prod", and then Ref results in ["test","dev","prod"].</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Parameter value of the application.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ParameterValue {
    /// <p>The key associated with the parameter. If you don't specify a key and value for a particular parameter, AWS CloudFormation
    /// uses the default value that is specified in your template.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The input value associated with the parameter.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutApplicationPolicyRequest {
    /// <p>The Amazon Resource Name (ARN) of the application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>An array of policy statements applied to the application.</p>
    #[serde(rename = "Statements")]
    pub statements: Vec<ApplicationPolicyStatement>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PutApplicationPolicyResponse {
    /// <p>An array of policy statements applied to the application.</p>
    #[serde(rename = "Statements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statements: Option<Vec<ApplicationPolicyStatement>>,
}

/// <p>Update the application request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UpdateApplicationInput {
    /// <p>The name of the author publishing the app.</p><p>Minimum length=1. Maximum length=127.</p><p>Pattern "^[a-z0-9](([a-z0-9]|-(?!-))*[a-z0-9])?$";</p>
    pub author: Option<String>,
    /// <p>The description of the application.</p><p>Minimum length=1. Maximum length=256</p>
    pub description: Option<String>,
    /// <p>A URL with more information about the application, for example
    /// the location of your GitHub repository for the application.</p>
    pub home_page_url: Option<String>,
    /// <p>Labels to improve discovery of apps in search results.</p><p>Minimum length=1. Maximum length=127. Maximum number of labels: 10</p><p>Pattern: "^[a-zA-Z0-9+\\-_:\\/@]+$";</p>
    pub labels: Option<Vec<String>>,
    /// <p>A text readme file in Markdown language that contains a more detailed description of the application and how it works.</p><p>Maximum size 5 MB</p>
    pub readme_body: Option<String>,
    /// <p>A link to the readme file in Markdown language that contains a more detailed description of the application and how it works.</p><p>Maximum size 5 MB</p>
    pub readme_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateApplicationRequest {
    /// <p>The Amazon Resource Name (ARN) of the application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The name of the author publishing the app.</p><p>Minimum length=1. Maximum length=127.</p><p>Pattern "^[a-z0-9](([a-z0-9]|-(?!-))*[a-z0-9])?$";</p>
    #[serde(rename = "Author")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    /// <p>The description of the application.</p><p>Minimum length=1. Maximum length=256</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A URL with more information about the application, for example
    /// the location of your GitHub repository for the application.</p>
    #[serde(rename = "HomePageUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_page_url: Option<String>,
    /// <p>Labels to improve discovery of apps in search results.</p><p>Minimum length=1. Maximum length=127. Maximum number of labels: 10</p><p>Pattern: "^[a-zA-Z0-9+\\-_:\\/@]+$";</p>
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    /// <p>A text readme file in Markdown language that contains a more detailed description of the application and how it works.</p><p>Maximum size 5 MB</p>
    #[serde(rename = "ReadmeBody")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readme_body: Option<String>,
    /// <p>A link to the readme file in Markdown language that contains a more detailed description of the application and how it works.</p><p>Maximum size 5 MB</p>
    #[serde(rename = "ReadmeUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readme_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateApplicationResponse {
    /// <p>The application Amazon Resource Name (ARN).</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>The name of the author publishing the app.</p><p>Minimum length=1. Maximum length=127.</p><p>Pattern "^[a-z0-9](([a-z0-9]|-(?!-))*[a-z0-9])?$";</p>
    #[serde(rename = "Author")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    /// <p>The date and time this resource was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    /// <p>The description of the application.</p><p>Minimum length=1. Maximum length=256</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A URL with more information about the application, for example
    /// the location of your GitHub repository for the application.</p>
    #[serde(rename = "HomePageUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_page_url: Option<String>,
    /// <p>Labels to improve discovery of apps in search results.</p><p>Minimum length=1. Maximum length=127. Maximum number of labels: 10</p><p>Pattern: "^[a-zA-Z0-9+\\-_:\\/@]+$";</p>
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    /// <p>A link to a license file of the app that matches the spdxLicenseID value of your application.</p><p>Maximum size 5 MB</p>
    #[serde(rename = "LicenseUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_url: Option<String>,
    /// <p>The name of the application.</p><p>Minimum length=1. Maximum length=140</p><p>Pattern: "[a-zA-Z0-9\\-]+";</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A link to the readme file in Markdown language that contains a more detailed description of the application and how it works.</p><p>Maximum size 5 MB</p>
    #[serde(rename = "ReadmeUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readme_url: Option<String>,
    /// <p>A valid identifier from https://spdx.org/licenses/.</p>
    #[serde(rename = "SpdxLicenseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spdx_license_id: Option<String>,
    /// <p>Version information about the application.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<Version>,
}

/// <p>Application version details.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Version {
    /// <p>The application Amazon Resource Name (ARN).</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The date and time this resource was created.</p>
    #[serde(rename = "CreationTime")]
    pub creation_time: String,
    /// <p>An array of parameter types supported by the application.</p>
    #[serde(rename = "ParameterDefinitions")]
    pub parameter_definitions: Vec<ParameterDefinition>,
    /// <p>The semantic version of the application:</p><p>
    /// <a href="https://semver.org/">https://semver.org/</a>
    /// </p>
    #[serde(rename = "SemanticVersion")]
    pub semantic_version: String,
    /// <p>A link to a public repository for the source code of your application.</p>
    #[serde(rename = "SourceCodeUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_code_url: Option<String>,
    /// <p>A link to the packaged AWS SAM template of your application.</p>
    #[serde(rename = "TemplateUrl")]
    pub template_url: String,
}

/// <p>An application version summary.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct VersionSummary {
    /// <p>The application Amazon Resource Name (ARN).</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The date and time this resource was created.</p>
    #[serde(rename = "CreationTime")]
    pub creation_time: String,
    /// <p>The semantic version of the application:</p><p>
    /// <a href="https://semver.org/">https://semver.org/</a>
    /// </p>
    #[serde(rename = "SemanticVersion")]
    pub semantic_version: String,
    /// <p>A link to a public repository for the source code of your application.</p>
    #[serde(rename = "SourceCodeUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_code_url: Option<String>,
}

/// Errors returned by CreateApplication
#[derive(Debug, PartialEq)]
pub enum CreateApplicationError {
    /// <p>One of the parameters in the request is invalid.</p>
    BadRequest(String),
    /// <p>The resource already exists.</p>
    Conflict(String),
    /// <p>The client is not authenticated.</p>
    Forbidden(String),
    /// <p>The AWS Serverless Application Repository service encountered an internal error.</p>
    InternalServerError(String),
    /// <p>The client is sending more than the allowed number of requests per unit of time.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateApplicationError {
    pub fn from_body(body: &str) -> CreateApplicationError {
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
                    "BadRequestException" => {
                        CreateApplicationError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => {
                        CreateApplicationError::Conflict(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        CreateApplicationError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        CreateApplicationError::InternalServerError(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        CreateApplicationError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateApplicationError::Validation(error_message.to_string())
                    }
                    _ => CreateApplicationError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateApplicationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateApplicationError {
    fn from(err: serde_json::error::Error) -> CreateApplicationError {
        CreateApplicationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateApplicationError {
    fn from(err: CredentialsError) -> CreateApplicationError {
        CreateApplicationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateApplicationError {
    fn from(err: HttpDispatchError) -> CreateApplicationError {
        CreateApplicationError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateApplicationError {
    fn from(err: io::Error) -> CreateApplicationError {
        CreateApplicationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateApplicationError {
    fn description(&self) -> &str {
        match *self {
            CreateApplicationError::BadRequest(ref cause) => cause,
            CreateApplicationError::Conflict(ref cause) => cause,
            CreateApplicationError::Forbidden(ref cause) => cause,
            CreateApplicationError::InternalServerError(ref cause) => cause,
            CreateApplicationError::TooManyRequests(ref cause) => cause,
            CreateApplicationError::Validation(ref cause) => cause,
            CreateApplicationError::Credentials(ref err) => err.description(),
            CreateApplicationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateApplicationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateApplicationVersion
#[derive(Debug, PartialEq)]
pub enum CreateApplicationVersionError {
    /// <p>One of the parameters in the request is invalid.</p>
    BadRequest(String),
    /// <p>The resource already exists.</p>
    Conflict(String),
    /// <p>The client is not authenticated.</p>
    Forbidden(String),
    /// <p>The AWS Serverless Application Repository service encountered an internal error.</p>
    InternalServerError(String),
    /// <p>The client is sending more than the allowed number of requests per unit of time.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateApplicationVersionError {
    pub fn from_body(body: &str) -> CreateApplicationVersionError {
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
                    "BadRequestException" => {
                        CreateApplicationVersionError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => {
                        CreateApplicationVersionError::Conflict(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        CreateApplicationVersionError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        CreateApplicationVersionError::InternalServerError(String::from(
                            error_message,
                        ))
                    }
                    "TooManyRequestsException" => {
                        CreateApplicationVersionError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateApplicationVersionError::Validation(error_message.to_string())
                    }
                    _ => CreateApplicationVersionError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateApplicationVersionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateApplicationVersionError {
    fn from(err: serde_json::error::Error) -> CreateApplicationVersionError {
        CreateApplicationVersionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateApplicationVersionError {
    fn from(err: CredentialsError) -> CreateApplicationVersionError {
        CreateApplicationVersionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateApplicationVersionError {
    fn from(err: HttpDispatchError) -> CreateApplicationVersionError {
        CreateApplicationVersionError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateApplicationVersionError {
    fn from(err: io::Error) -> CreateApplicationVersionError {
        CreateApplicationVersionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateApplicationVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateApplicationVersionError {
    fn description(&self) -> &str {
        match *self {
            CreateApplicationVersionError::BadRequest(ref cause) => cause,
            CreateApplicationVersionError::Conflict(ref cause) => cause,
            CreateApplicationVersionError::Forbidden(ref cause) => cause,
            CreateApplicationVersionError::InternalServerError(ref cause) => cause,
            CreateApplicationVersionError::TooManyRequests(ref cause) => cause,
            CreateApplicationVersionError::Validation(ref cause) => cause,
            CreateApplicationVersionError::Credentials(ref err) => err.description(),
            CreateApplicationVersionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateApplicationVersionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateCloudFormationChangeSet
#[derive(Debug, PartialEq)]
pub enum CreateCloudFormationChangeSetError {
    /// <p>One of the parameters in the request is invalid.</p>
    BadRequest(String),
    /// <p>The client is not authenticated.</p>
    Forbidden(String),
    /// <p>The AWS Serverless Application Repository service encountered an internal error.</p>
    InternalServerError(String),
    /// <p>The client is sending more than the allowed number of requests per unit of time.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateCloudFormationChangeSetError {
    pub fn from_body(body: &str) -> CreateCloudFormationChangeSetError {
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
                    "BadRequestException" => {
                        CreateCloudFormationChangeSetError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        CreateCloudFormationChangeSetError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        CreateCloudFormationChangeSetError::InternalServerError(String::from(
                            error_message,
                        ))
                    }
                    "TooManyRequestsException" => {
                        CreateCloudFormationChangeSetError::TooManyRequests(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        CreateCloudFormationChangeSetError::Validation(error_message.to_string())
                    }
                    _ => CreateCloudFormationChangeSetError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateCloudFormationChangeSetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateCloudFormationChangeSetError {
    fn from(err: serde_json::error::Error) -> CreateCloudFormationChangeSetError {
        CreateCloudFormationChangeSetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateCloudFormationChangeSetError {
    fn from(err: CredentialsError) -> CreateCloudFormationChangeSetError {
        CreateCloudFormationChangeSetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateCloudFormationChangeSetError {
    fn from(err: HttpDispatchError) -> CreateCloudFormationChangeSetError {
        CreateCloudFormationChangeSetError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateCloudFormationChangeSetError {
    fn from(err: io::Error) -> CreateCloudFormationChangeSetError {
        CreateCloudFormationChangeSetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateCloudFormationChangeSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateCloudFormationChangeSetError {
    fn description(&self) -> &str {
        match *self {
            CreateCloudFormationChangeSetError::BadRequest(ref cause) => cause,
            CreateCloudFormationChangeSetError::Forbidden(ref cause) => cause,
            CreateCloudFormationChangeSetError::InternalServerError(ref cause) => cause,
            CreateCloudFormationChangeSetError::TooManyRequests(ref cause) => cause,
            CreateCloudFormationChangeSetError::Validation(ref cause) => cause,
            CreateCloudFormationChangeSetError::Credentials(ref err) => err.description(),
            CreateCloudFormationChangeSetError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateCloudFormationChangeSetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteApplication
#[derive(Debug, PartialEq)]
pub enum DeleteApplicationError {
    /// <p>One of the parameters in the request is invalid.</p>
    BadRequest(String),
    /// <p>The resource already exists.</p>
    Conflict(String),
    /// <p>The client is not authenticated.</p>
    Forbidden(String),
    /// <p>The AWS Serverless Application Repository service encountered an internal error.</p>
    InternalServerError(String),
    /// <p>The resource (for example, an access policy statement) specified in the request doesn't exist.</p>
    NotFound(String),
    /// <p>The client is sending more than the allowed number of requests per unit of time.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteApplicationError {
    pub fn from_body(body: &str) -> DeleteApplicationError {
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
                    "BadRequestException" => {
                        DeleteApplicationError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => {
                        DeleteApplicationError::Conflict(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        DeleteApplicationError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        DeleteApplicationError::InternalServerError(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DeleteApplicationError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        DeleteApplicationError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteApplicationError::Validation(error_message.to_string())
                    }
                    _ => DeleteApplicationError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteApplicationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteApplicationError {
    fn from(err: serde_json::error::Error) -> DeleteApplicationError {
        DeleteApplicationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteApplicationError {
    fn from(err: CredentialsError) -> DeleteApplicationError {
        DeleteApplicationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteApplicationError {
    fn from(err: HttpDispatchError) -> DeleteApplicationError {
        DeleteApplicationError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteApplicationError {
    fn from(err: io::Error) -> DeleteApplicationError {
        DeleteApplicationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteApplicationError {
    fn description(&self) -> &str {
        match *self {
            DeleteApplicationError::BadRequest(ref cause) => cause,
            DeleteApplicationError::Conflict(ref cause) => cause,
            DeleteApplicationError::Forbidden(ref cause) => cause,
            DeleteApplicationError::InternalServerError(ref cause) => cause,
            DeleteApplicationError::NotFound(ref cause) => cause,
            DeleteApplicationError::TooManyRequests(ref cause) => cause,
            DeleteApplicationError::Validation(ref cause) => cause,
            DeleteApplicationError::Credentials(ref err) => err.description(),
            DeleteApplicationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteApplicationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetApplication
#[derive(Debug, PartialEq)]
pub enum GetApplicationError {
    /// <p>One of the parameters in the request is invalid.</p>
    BadRequest(String),
    /// <p>The client is not authenticated.</p>
    Forbidden(String),
    /// <p>The AWS Serverless Application Repository service encountered an internal error.</p>
    InternalServerError(String),
    /// <p>The resource (for example, an access policy statement) specified in the request doesn't exist.</p>
    NotFound(String),
    /// <p>The client is sending more than the allowed number of requests per unit of time.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetApplicationError {
    pub fn from_body(body: &str) -> GetApplicationError {
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
                    "BadRequestException" => {
                        GetApplicationError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        GetApplicationError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        GetApplicationError::InternalServerError(String::from(error_message))
                    }
                    "NotFoundException" => {
                        GetApplicationError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        GetApplicationError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetApplicationError::Validation(error_message.to_string())
                    }
                    _ => GetApplicationError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetApplicationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetApplicationError {
    fn from(err: serde_json::error::Error) -> GetApplicationError {
        GetApplicationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetApplicationError {
    fn from(err: CredentialsError) -> GetApplicationError {
        GetApplicationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetApplicationError {
    fn from(err: HttpDispatchError) -> GetApplicationError {
        GetApplicationError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetApplicationError {
    fn from(err: io::Error) -> GetApplicationError {
        GetApplicationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetApplicationError {
    fn description(&self) -> &str {
        match *self {
            GetApplicationError::BadRequest(ref cause) => cause,
            GetApplicationError::Forbidden(ref cause) => cause,
            GetApplicationError::InternalServerError(ref cause) => cause,
            GetApplicationError::NotFound(ref cause) => cause,
            GetApplicationError::TooManyRequests(ref cause) => cause,
            GetApplicationError::Validation(ref cause) => cause,
            GetApplicationError::Credentials(ref err) => err.description(),
            GetApplicationError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetApplicationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetApplicationPolicy
#[derive(Debug, PartialEq)]
pub enum GetApplicationPolicyError {
    /// <p>One of the parameters in the request is invalid.</p>
    BadRequest(String),
    /// <p>The client is not authenticated.</p>
    Forbidden(String),
    /// <p>The AWS Serverless Application Repository service encountered an internal error.</p>
    InternalServerError(String),
    /// <p>The resource (for example, an access policy statement) specified in the request doesn't exist.</p>
    NotFound(String),
    /// <p>The client is sending more than the allowed number of requests per unit of time.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetApplicationPolicyError {
    pub fn from_body(body: &str) -> GetApplicationPolicyError {
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
                    "BadRequestException" => {
                        GetApplicationPolicyError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        GetApplicationPolicyError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        GetApplicationPolicyError::InternalServerError(String::from(error_message))
                    }
                    "NotFoundException" => {
                        GetApplicationPolicyError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        GetApplicationPolicyError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetApplicationPolicyError::Validation(error_message.to_string())
                    }
                    _ => GetApplicationPolicyError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetApplicationPolicyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetApplicationPolicyError {
    fn from(err: serde_json::error::Error) -> GetApplicationPolicyError {
        GetApplicationPolicyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetApplicationPolicyError {
    fn from(err: CredentialsError) -> GetApplicationPolicyError {
        GetApplicationPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetApplicationPolicyError {
    fn from(err: HttpDispatchError) -> GetApplicationPolicyError {
        GetApplicationPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetApplicationPolicyError {
    fn from(err: io::Error) -> GetApplicationPolicyError {
        GetApplicationPolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetApplicationPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetApplicationPolicyError {
    fn description(&self) -> &str {
        match *self {
            GetApplicationPolicyError::BadRequest(ref cause) => cause,
            GetApplicationPolicyError::Forbidden(ref cause) => cause,
            GetApplicationPolicyError::InternalServerError(ref cause) => cause,
            GetApplicationPolicyError::NotFound(ref cause) => cause,
            GetApplicationPolicyError::TooManyRequests(ref cause) => cause,
            GetApplicationPolicyError::Validation(ref cause) => cause,
            GetApplicationPolicyError::Credentials(ref err) => err.description(),
            GetApplicationPolicyError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetApplicationPolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListApplicationVersions
#[derive(Debug, PartialEq)]
pub enum ListApplicationVersionsError {
    /// <p>One of the parameters in the request is invalid.</p>
    BadRequest(String),
    /// <p>The client is not authenticated.</p>
    Forbidden(String),
    /// <p>The AWS Serverless Application Repository service encountered an internal error.</p>
    InternalServerError(String),
    /// <p>The resource (for example, an access policy statement) specified in the request doesn't exist.</p>
    NotFound(String),
    /// <p>The client is sending more than the allowed number of requests per unit of time.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListApplicationVersionsError {
    pub fn from_body(body: &str) -> ListApplicationVersionsError {
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
                    "BadRequestException" => {
                        ListApplicationVersionsError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        ListApplicationVersionsError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        ListApplicationVersionsError::InternalServerError(String::from(
                            error_message,
                        ))
                    }
                    "NotFoundException" => {
                        ListApplicationVersionsError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        ListApplicationVersionsError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListApplicationVersionsError::Validation(error_message.to_string())
                    }
                    _ => ListApplicationVersionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListApplicationVersionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListApplicationVersionsError {
    fn from(err: serde_json::error::Error) -> ListApplicationVersionsError {
        ListApplicationVersionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListApplicationVersionsError {
    fn from(err: CredentialsError) -> ListApplicationVersionsError {
        ListApplicationVersionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListApplicationVersionsError {
    fn from(err: HttpDispatchError) -> ListApplicationVersionsError {
        ListApplicationVersionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListApplicationVersionsError {
    fn from(err: io::Error) -> ListApplicationVersionsError {
        ListApplicationVersionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListApplicationVersionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListApplicationVersionsError {
    fn description(&self) -> &str {
        match *self {
            ListApplicationVersionsError::BadRequest(ref cause) => cause,
            ListApplicationVersionsError::Forbidden(ref cause) => cause,
            ListApplicationVersionsError::InternalServerError(ref cause) => cause,
            ListApplicationVersionsError::NotFound(ref cause) => cause,
            ListApplicationVersionsError::TooManyRequests(ref cause) => cause,
            ListApplicationVersionsError::Validation(ref cause) => cause,
            ListApplicationVersionsError::Credentials(ref err) => err.description(),
            ListApplicationVersionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListApplicationVersionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListApplications
#[derive(Debug, PartialEq)]
pub enum ListApplicationsError {
    /// <p>One of the parameters in the request is invalid.</p>
    BadRequest(String),
    /// <p>The client is not authenticated.</p>
    Forbidden(String),
    /// <p>The AWS Serverless Application Repository service encountered an internal error.</p>
    InternalServerError(String),
    /// <p>The resource (for example, an access policy statement) specified in the request doesn't exist.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListApplicationsError {
    pub fn from_body(body: &str) -> ListApplicationsError {
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
                    "BadRequestException" => {
                        ListApplicationsError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        ListApplicationsError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        ListApplicationsError::InternalServerError(String::from(error_message))
                    }
                    "NotFoundException" => {
                        ListApplicationsError::NotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListApplicationsError::Validation(error_message.to_string())
                    }
                    _ => ListApplicationsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListApplicationsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListApplicationsError {
    fn from(err: serde_json::error::Error) -> ListApplicationsError {
        ListApplicationsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListApplicationsError {
    fn from(err: CredentialsError) -> ListApplicationsError {
        ListApplicationsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListApplicationsError {
    fn from(err: HttpDispatchError) -> ListApplicationsError {
        ListApplicationsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListApplicationsError {
    fn from(err: io::Error) -> ListApplicationsError {
        ListApplicationsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListApplicationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListApplicationsError {
    fn description(&self) -> &str {
        match *self {
            ListApplicationsError::BadRequest(ref cause) => cause,
            ListApplicationsError::Forbidden(ref cause) => cause,
            ListApplicationsError::InternalServerError(ref cause) => cause,
            ListApplicationsError::NotFound(ref cause) => cause,
            ListApplicationsError::Validation(ref cause) => cause,
            ListApplicationsError::Credentials(ref err) => err.description(),
            ListApplicationsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListApplicationsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutApplicationPolicy
#[derive(Debug, PartialEq)]
pub enum PutApplicationPolicyError {
    /// <p>One of the parameters in the request is invalid.</p>
    BadRequest(String),
    /// <p>The client is not authenticated.</p>
    Forbidden(String),
    /// <p>The AWS Serverless Application Repository service encountered an internal error.</p>
    InternalServerError(String),
    /// <p>The resource (for example, an access policy statement) specified in the request doesn't exist.</p>
    NotFound(String),
    /// <p>The client is sending more than the allowed number of requests per unit of time.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutApplicationPolicyError {
    pub fn from_body(body: &str) -> PutApplicationPolicyError {
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
                    "BadRequestException" => {
                        PutApplicationPolicyError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        PutApplicationPolicyError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        PutApplicationPolicyError::InternalServerError(String::from(error_message))
                    }
                    "NotFoundException" => {
                        PutApplicationPolicyError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        PutApplicationPolicyError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        PutApplicationPolicyError::Validation(error_message.to_string())
                    }
                    _ => PutApplicationPolicyError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutApplicationPolicyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PutApplicationPolicyError {
    fn from(err: serde_json::error::Error) -> PutApplicationPolicyError {
        PutApplicationPolicyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PutApplicationPolicyError {
    fn from(err: CredentialsError) -> PutApplicationPolicyError {
        PutApplicationPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutApplicationPolicyError {
    fn from(err: HttpDispatchError) -> PutApplicationPolicyError {
        PutApplicationPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutApplicationPolicyError {
    fn from(err: io::Error) -> PutApplicationPolicyError {
        PutApplicationPolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutApplicationPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutApplicationPolicyError {
    fn description(&self) -> &str {
        match *self {
            PutApplicationPolicyError::BadRequest(ref cause) => cause,
            PutApplicationPolicyError::Forbidden(ref cause) => cause,
            PutApplicationPolicyError::InternalServerError(ref cause) => cause,
            PutApplicationPolicyError::NotFound(ref cause) => cause,
            PutApplicationPolicyError::TooManyRequests(ref cause) => cause,
            PutApplicationPolicyError::Validation(ref cause) => cause,
            PutApplicationPolicyError::Credentials(ref err) => err.description(),
            PutApplicationPolicyError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutApplicationPolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateApplication
#[derive(Debug, PartialEq)]
pub enum UpdateApplicationError {
    /// <p>One of the parameters in the request is invalid.</p>
    BadRequest(String),
    /// <p>The resource already exists.</p>
    Conflict(String),
    /// <p>The client is not authenticated.</p>
    Forbidden(String),
    /// <p>The AWS Serverless Application Repository service encountered an internal error.</p>
    InternalServerError(String),
    /// <p>The resource (for example, an access policy statement) specified in the request doesn't exist.</p>
    NotFound(String),
    /// <p>The client is sending more than the allowed number of requests per unit of time.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateApplicationError {
    pub fn from_body(body: &str) -> UpdateApplicationError {
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
                    "BadRequestException" => {
                        UpdateApplicationError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => {
                        UpdateApplicationError::Conflict(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        UpdateApplicationError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        UpdateApplicationError::InternalServerError(String::from(error_message))
                    }
                    "NotFoundException" => {
                        UpdateApplicationError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        UpdateApplicationError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateApplicationError::Validation(error_message.to_string())
                    }
                    _ => UpdateApplicationError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateApplicationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateApplicationError {
    fn from(err: serde_json::error::Error) -> UpdateApplicationError {
        UpdateApplicationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateApplicationError {
    fn from(err: CredentialsError) -> UpdateApplicationError {
        UpdateApplicationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateApplicationError {
    fn from(err: HttpDispatchError) -> UpdateApplicationError {
        UpdateApplicationError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateApplicationError {
    fn from(err: io::Error) -> UpdateApplicationError {
        UpdateApplicationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateApplicationError {
    fn description(&self) -> &str {
        match *self {
            UpdateApplicationError::BadRequest(ref cause) => cause,
            UpdateApplicationError::Conflict(ref cause) => cause,
            UpdateApplicationError::Forbidden(ref cause) => cause,
            UpdateApplicationError::InternalServerError(ref cause) => cause,
            UpdateApplicationError::NotFound(ref cause) => cause,
            UpdateApplicationError::TooManyRequests(ref cause) => cause,
            UpdateApplicationError::Validation(ref cause) => cause,
            UpdateApplicationError::Credentials(ref err) => err.description(),
            UpdateApplicationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateApplicationError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the AWSServerlessApplicationRepository API. AWSServerlessApplicationRepository clients implement this trait.
pub trait ServerlessRepo {
    /// <p>Creates an application, optionally including an AWS SAM file to create the first application version in the same call.</p>
    fn create_application(
        &self,
        input: CreateApplicationRequest,
    ) -> RusotoFuture<CreateApplicationResponse, CreateApplicationError>;

    /// <p>Creates an application version.</p>
    fn create_application_version(
        &self,
        input: CreateApplicationVersionRequest,
    ) -> RusotoFuture<CreateApplicationVersionResponse, CreateApplicationVersionError>;

    /// <p>Creates an AWS CloudFormation change set for the given application.</p>
    fn create_cloud_formation_change_set(
        &self,
        input: CreateCloudFormationChangeSetRequest,
    ) -> RusotoFuture<CreateCloudFormationChangeSetResponse, CreateCloudFormationChangeSetError>;

    /// <p>Deletes the specified application.</p>
    fn delete_application(
        &self,
        input: DeleteApplicationRequest,
    ) -> RusotoFuture<(), DeleteApplicationError>;

    /// <p>Gets the specified application.</p>
    fn get_application(
        &self,
        input: GetApplicationRequest,
    ) -> RusotoFuture<GetApplicationResponse, GetApplicationError>;

    /// <p>Retrieves the policy for the application.</p>
    fn get_application_policy(
        &self,
        input: GetApplicationPolicyRequest,
    ) -> RusotoFuture<GetApplicationPolicyResponse, GetApplicationPolicyError>;

    /// <p>Lists versions for the specified application.</p>
    fn list_application_versions(
        &self,
        input: ListApplicationVersionsRequest,
    ) -> RusotoFuture<ListApplicationVersionsResponse, ListApplicationVersionsError>;

    /// <p>Lists applications owned by the requester.</p>
    fn list_applications(
        &self,
        input: ListApplicationsRequest,
    ) -> RusotoFuture<ListApplicationsResponse, ListApplicationsError>;

    /// <p>Sets the permission policy for an application. See
    /// <a href="https://docs.aws.amazon.com/serverlessrepo/latest/devguide/access-control-resource-based.html#application-permissions">Application Permissions</a>
    /// for the list of supported actions that can be used with this operation.</p>
    fn put_application_policy(
        &self,
        input: PutApplicationPolicyRequest,
    ) -> RusotoFuture<PutApplicationPolicyResponse, PutApplicationPolicyError>;

    /// <p>Updates the specified application.</p>
    fn update_application(
        &self,
        input: UpdateApplicationRequest,
    ) -> RusotoFuture<UpdateApplicationResponse, UpdateApplicationError>;
}
/// A client for the AWSServerlessApplicationRepository API.
pub struct ServerlessRepoClient {
    client: Client,
    region: region::Region,
}

impl ServerlessRepoClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> ServerlessRepoClient {
        ServerlessRepoClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> ServerlessRepoClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        ServerlessRepoClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl ServerlessRepo for ServerlessRepoClient {
    /// <p>Creates an application, optionally including an AWS SAM file to create the first application version in the same call.</p>
    fn create_application(
        &self,
        input: CreateApplicationRequest,
    ) -> RusotoFuture<CreateApplicationResponse, CreateApplicationError> {
        let request_uri = "/applications";

        let mut request = SignedRequest::new("POST", "serverlessrepo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<CreateApplicationResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateApplicationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Creates an application version.</p>
    fn create_application_version(
        &self,
        input: CreateApplicationVersionRequest,
    ) -> RusotoFuture<CreateApplicationVersionResponse, CreateApplicationVersionError> {
        let request_uri = format!(
            "/applications/{application_id}/versions/{semantic_version}",
            application_id = input.application_id,
            semantic_version = input.semantic_version
        );

        let mut request = SignedRequest::new("PUT", "serverlessrepo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<CreateApplicationVersionResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateApplicationVersionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Creates an AWS CloudFormation change set for the given application.</p>
    fn create_cloud_formation_change_set(
        &self,
        input: CreateCloudFormationChangeSetRequest,
    ) -> RusotoFuture<CreateCloudFormationChangeSetResponse, CreateCloudFormationChangeSetError>
    {
        let request_uri = format!(
            "/applications/{application_id}/changesets",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("POST", "serverlessrepo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<CreateCloudFormationChangeSetResponse>(&body)
                            .unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateCloudFormationChangeSetError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Deletes the specified application.</p>
    fn delete_application(
        &self,
        input: DeleteApplicationRequest,
    ) -> RusotoFuture<(), DeleteApplicationError> {
        let request_uri = format!(
            "/applications/{application_id}",
            application_id = input.application_id
        );

        let mut request =
            SignedRequest::new("DELETE", "serverlessrepo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteApplicationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Gets the specified application.</p>
    fn get_application(
        &self,
        input: GetApplicationRequest,
    ) -> RusotoFuture<GetApplicationResponse, GetApplicationError> {
        let request_uri = format!(
            "/applications/{application_id}",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("GET", "serverlessrepo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.semantic_version {
            params.put("semanticVersion", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetApplicationResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetApplicationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Retrieves the policy for the application.</p>
    fn get_application_policy(
        &self,
        input: GetApplicationPolicyRequest,
    ) -> RusotoFuture<GetApplicationPolicyResponse, GetApplicationPolicyError> {
        let request_uri = format!(
            "/applications/{application_id}/policy",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("GET", "serverlessrepo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<GetApplicationPolicyResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetApplicationPolicyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Lists versions for the specified application.</p>
    fn list_application_versions(
        &self,
        input: ListApplicationVersionsRequest,
    ) -> RusotoFuture<ListApplicationVersionsResponse, ListApplicationVersionsError> {
        let request_uri = format!(
            "/applications/{application_id}/versions",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("GET", "serverlessrepo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_items {
            params.put("maxItems", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<ListApplicationVersionsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListApplicationVersionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Lists applications owned by the requester.</p>
    fn list_applications(
        &self,
        input: ListApplicationsRequest,
    ) -> RusotoFuture<ListApplicationsResponse, ListApplicationsError> {
        let request_uri = "/applications";

        let mut request = SignedRequest::new("GET", "serverlessrepo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_items {
            params.put("maxItems", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ListApplicationsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListApplicationsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Sets the permission policy for an application. See
    /// <a href="https://docs.aws.amazon.com/serverlessrepo/latest/devguide/access-control-resource-based.html#application-permissions">Application Permissions</a>
    /// for the list of supported actions that can be used with this operation.</p>
    fn put_application_policy(
        &self,
        input: PutApplicationPolicyRequest,
    ) -> RusotoFuture<PutApplicationPolicyResponse, PutApplicationPolicyError> {
        let request_uri = format!(
            "/applications/{application_id}/policy",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("PUT", "serverlessrepo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<PutApplicationPolicyResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutApplicationPolicyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Updates the specified application.</p>
    fn update_application(
        &self,
        input: UpdateApplicationRequest,
    ) -> RusotoFuture<UpdateApplicationResponse, UpdateApplicationError> {
        let request_uri = format!(
            "/applications/{application_id}",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("PATCH", "serverlessrepo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<UpdateApplicationResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateApplicationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
