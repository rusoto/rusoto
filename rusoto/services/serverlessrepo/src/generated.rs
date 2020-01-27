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
/// <p>A nested application summary.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ApplicationDependencySummary {
    /// <p>The Amazon Resource Name (ARN) of the nested application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The semantic version of the nested application.</p>
    #[serde(rename = "SemanticVersion")]
    pub semantic_version: String,
}

/// <p>Policy statement applied to the application.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplicationPolicyStatement {
    /// <p>For the list of actions supported for this operation, see <a href="https://docs.aws.amazon.com/serverlessrepo/latest/devguide/access-control-resource-based.html#application-permissions">Application
    /// Permissions</a>.</p>
    #[serde(rename = "Actions")]
    pub actions: Vec<String>,
    /// <p>An array of AWS account IDs, or * to make the application public.</p>
    #[serde(rename = "Principals")]
    pub principals: Vec<String>,
    /// <p>A unique ID for the statement.</p>
    #[serde(rename = "StatementId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_id: Option<String>,
}

/// <p>Summary of details about the application.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>A URL with more information about the application, for example the location of your GitHub repository for the application.</p>
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateApplicationRequest {
    /// <p>The name of the author publishing the app.</p><p>Minimum length=1. Maximum length=127.</p><p>Pattern "^[a-z0-9](([a-z0-9]|-(?!-))*[a-z0-9])?$";</p>
    #[serde(rename = "Author")]
    pub author: String,
    /// <p>The description of the application.</p><p>Minimum length=1. Maximum length=256</p>
    #[serde(rename = "Description")]
    pub description: String,
    /// <p>A URL with more information about the application, for example the location of your GitHub repository for the application.</p>
    #[serde(rename = "HomePageUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_page_url: Option<String>,
    /// <p>Labels to improve discovery of apps in search results.</p><p>Minimum length=1. Maximum length=127. Maximum number of labels: 10</p><p>Pattern: "^[a-zA-Z0-9+\\-_:\\/@]+$";</p>
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    /// <p>A local text file that contains the license of the app that matches the spdxLicenseID value of your application.
    /// The file has the format file://&lt;path>/&lt;filename>.</p><p>Maximum size 5 MB</p><p>You can specify only one of licenseBody and licenseUrl; otherwise, an error results.</p>
    #[serde(rename = "LicenseBody")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_body: Option<String>,
    /// <p>A link to the S3 object that contains the license of the app that matches the spdxLicenseID value of your application.</p><p>Maximum size 5 MB</p><p>You can specify only one of licenseBody and licenseUrl; otherwise, an error results.</p>
    #[serde(rename = "LicenseUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_url: Option<String>,
    /// <p>The name of the application that you want to publish.</p><p>Minimum length=1. Maximum length=140</p><p>Pattern: "[a-zA-Z0-9\\-]+";</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>A local text readme file in Markdown language that contains a more detailed description of the application and how it works.
    /// The file has the format file://&lt;path>/&lt;filename>.</p><p>Maximum size 5 MB</p><p>You can specify only one of readmeBody and readmeUrl; otherwise, an error results.</p>
    #[serde(rename = "ReadmeBody")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readme_body: Option<String>,
    /// <p>A link to the S3 object in Markdown language that contains a more detailed description of the application and how it works.</p><p>Maximum size 5 MB</p><p>You can specify only one of readmeBody and readmeUrl; otherwise, an error results.</p>
    #[serde(rename = "ReadmeUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readme_url: Option<String>,
    /// <p>The semantic version of the application:</p><p>
    /// <a href="https://semver.org/">https://semver.org/</a>
    /// </p>
    #[serde(rename = "SemanticVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_version: Option<String>,
    /// <p>A link to the S3 object that contains the ZIP archive of the source code for this version of your application.</p><p>Maximum size 50 MB</p>
    #[serde(rename = "SourceCodeArchiveUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_code_archive_url: Option<String>,
    /// <p>A link to a public repository for the source code of your application, for example the URL of a specific GitHub commit.</p>
    #[serde(rename = "SourceCodeUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_code_url: Option<String>,
    /// <p>A valid identifier from <a href="https://spdx.org/licenses/">https://spdx.org/licenses/</a>.</p>
    #[serde(rename = "SpdxLicenseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spdx_license_id: Option<String>,
    /// <p>The local raw packaged AWS SAM template file of your application.
    /// The file has the format file://&lt;path>/&lt;filename>.</p><p>You can specify only one of templateBody and templateUrl; otherwise an error results.</p>
    #[serde(rename = "TemplateBody")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_body: Option<String>,
    /// <p>A link to the S3 object containing the packaged AWS SAM template of your application.</p><p>You can specify only one of templateBody and templateUrl; otherwise an error results.</p>
    #[serde(rename = "TemplateUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>A URL with more information about the application, for example the location of your GitHub repository for the application.</p>
    #[serde(rename = "HomePageUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_page_url: Option<String>,
    /// <p>Whether the author of this application has been verified. This means means that AWS has made a good faith review, as a reasonable and prudent service provider, of the information provided by the requester and has confirmed that the requester's identity is as claimed.</p>
    #[serde(rename = "IsVerifiedAuthor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_verified_author: Option<bool>,
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
    /// <p>The URL to the public profile of a verified author. This URL is submitted by the author.</p>
    #[serde(rename = "VerifiedAuthorUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_author_url: Option<String>,
    /// <p>Version information about the application.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<Version>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateApplicationVersionRequest {
    /// <p>The Amazon Resource Name (ARN) of the application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The semantic version of the new version.</p>
    #[serde(rename = "SemanticVersion")]
    pub semantic_version: String,
    /// <p>A link to the S3 object that contains the ZIP archive of the source code for this version of your application.</p><p>Maximum size 50 MB</p>
    #[serde(rename = "SourceCodeArchiveUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_code_archive_url: Option<String>,
    /// <p>A link to a public repository for the source code of your application, for example the URL of a specific GitHub commit.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>A list of values that you must specify before you can deploy certain applications.
    /// Some applications might include resources that can affect permissions in your AWS
    /// account, for example, by creating new AWS Identity and Access Management (IAM) users.
    /// For those applications, you must explicitly acknowledge their capabilities by
    /// specifying this parameter.</p><p>The only valid values are CAPABILITY_IAM, CAPABILITY_NAMED_IAM,
    /// CAPABILITY_RESOURCE_POLICY, and CAPABILITY_AUTO_EXPAND.</p><p>The following resources require you to specify CAPABILITY_IAM or
    /// CAPABILITY_NAMED_IAM:
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-group.html">AWS::IAM::Group</a>,
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-instanceprofile.html">AWS::IAM::InstanceProfile</a>,
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-policy.html">AWS::IAM::Policy</a>, and
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-role.html">AWS::IAM::Role</a>.
    /// If the application contains IAM resources, you can specify either CAPABILITY_IAM
    /// or CAPABILITY_NAMED_IAM. If the application contains IAM resources
    /// with custom names, you must specify CAPABILITY_NAMED_IAM.</p><p>The following resources require you to specify CAPABILITY_RESOURCE_POLICY:
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-permission.html">AWS::Lambda::Permission</a>,
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-policy.html">AWS::IAM:Policy</a>,
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-applicationautoscaling-scalingpolicy.html">AWS::ApplicationAutoScaling::ScalingPolicy</a>,
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-policy.html">AWS::S3::BucketPolicy</a>,
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sqs-policy.html">AWS::SQS::QueuePolicy</a>, and
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sns-policy.html">AWS::SNS::TopicPolicy</a>.</p><p>Applications that contain one or more nested applications require you to specify
    /// CAPABILITY_AUTO_EXPAND.</p><p>If your application template contains any of the above resources, we recommend that you review
    /// all permissions associated with the application before deploying. If you don't specify
    /// this parameter for an application that requires capabilities, the call will fail.</p>
    #[serde(rename = "RequiredCapabilities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_capabilities: Option<Vec<String>>,
    /// <p>Whether all of the AWS resources contained in this application are supported in the region
    /// in which it is being retrieved.</p>
    #[serde(rename = "ResourcesSupported")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources_supported: Option<bool>,
    /// <p>The semantic version of the application:</p><p>
    /// <a href="https://semver.org/">https://semver.org/</a>
    /// </p>
    #[serde(rename = "SemanticVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_version: Option<String>,
    /// <p>A link to the S3 object that contains the ZIP archive of the source code for this version of your application.</p><p>Maximum size 50 MB</p>
    #[serde(rename = "SourceCodeArchiveUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_code_archive_url: Option<String>,
    /// <p>A link to a public repository for the source code of your application, for example the URL of a specific GitHub commit.</p>
    #[serde(rename = "SourceCodeUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_code_url: Option<String>,
    /// <p>A link to the packaged AWS SAM template of your application.</p>
    #[serde(rename = "TemplateUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateCloudFormationChangeSetRequest {
    /// <p>The Amazon Resource Name (ARN) of the application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>A list of values that you must specify before you can deploy certain applications.
    /// Some applications might include resources that can affect permissions in your AWS
    /// account, for example, by creating new AWS Identity and Access Management (IAM) users.
    /// For those applications, you must explicitly acknowledge their capabilities by
    /// specifying this parameter.</p><p>The only valid values are CAPABILITY_IAM, CAPABILITY_NAMED_IAM,
    /// CAPABILITY_RESOURCE_POLICY, and CAPABILITY_AUTO_EXPAND.</p><p>The following resources require you to specify CAPABILITY_IAM or
    /// CAPABILITY_NAMED_IAM:
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-group.html">AWS::IAM::Group</a>,
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-instanceprofile.html">AWS::IAM::InstanceProfile</a>,
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-policy.html">AWS::IAM::Policy</a>, and
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-role.html">AWS::IAM::Role</a>.
    /// If the application contains IAM resources, you can specify either CAPABILITY_IAM
    /// or CAPABILITY_NAMED_IAM. If the application contains IAM resources
    /// with custom names, you must specify CAPABILITY_NAMED_IAM.</p><p>The following resources require you to specify CAPABILITY_RESOURCE_POLICY:
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-permission.html">AWS::Lambda::Permission</a>,
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-policy.html">AWS::IAM:Policy</a>,
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-applicationautoscaling-scalingpolicy.html">AWS::ApplicationAutoScaling::ScalingPolicy</a>,
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-policy.html">AWS::S3::BucketPolicy</a>,
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sqs-policy.html">AWS::SQS::QueuePolicy</a>, and
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sns-policy.html">AWS::SNS:TopicPolicy</a>.</p><p>Applications that contain one or more nested applications require you to specify
    /// CAPABILITY_AUTO_EXPAND.</p><p>If your application template contains any of the above resources, we recommend that you review
    /// all permissions associated with the application before deploying. If you don't specify
    /// this parameter for an application that requires capabilities, the call will fail.</p>
    #[serde(rename = "Capabilities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Vec<String>>,
    /// <p>This property corresponds to the parameter of the same name for the <i>AWS CloudFormation <a href="https://docs.aws.amazon.com/goto/WebAPI/cloudformation-2010-05-15/CreateChangeSet">CreateChangeSet</a>
    /// </i> API.</p>
    #[serde(rename = "ChangeSetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_set_name: Option<String>,
    /// <p>This property corresponds to the parameter of the same name for the <i>AWS CloudFormation <a href="https://docs.aws.amazon.com/goto/WebAPI/cloudformation-2010-05-15/CreateChangeSet">CreateChangeSet</a>
    /// </i> API.</p>
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>This property corresponds to the parameter of the same name for the <i>AWS CloudFormation <a href="https://docs.aws.amazon.com/goto/WebAPI/cloudformation-2010-05-15/CreateChangeSet">CreateChangeSet</a>
    /// </i> API.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>This property corresponds to the parameter of the same name for the <i>AWS CloudFormation <a href="https://docs.aws.amazon.com/goto/WebAPI/cloudformation-2010-05-15/CreateChangeSet">CreateChangeSet</a>
    /// </i> API.</p>
    #[serde(rename = "NotificationArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_arns: Option<Vec<String>>,
    /// <p>A list of parameter values for the parameters of the application.</p>
    #[serde(rename = "ParameterOverrides")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_overrides: Option<Vec<ParameterValue>>,
    /// <p>This property corresponds to the parameter of the same name for the <i>AWS CloudFormation <a href="https://docs.aws.amazon.com/goto/WebAPI/cloudformation-2010-05-15/CreateChangeSet">CreateChangeSet</a>
    /// </i> API.</p>
    #[serde(rename = "ResourceTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_types: Option<Vec<String>>,
    /// <p>This property corresponds to the parameter of the same name for the <i>AWS CloudFormation <a href="https://docs.aws.amazon.com/goto/WebAPI/cloudformation-2010-05-15/CreateChangeSet">CreateChangeSet</a>
    /// </i> API.</p>
    #[serde(rename = "RollbackConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollback_configuration: Option<RollbackConfiguration>,
    /// <p>The semantic version of the application:</p><p>
    /// <a href="https://semver.org/">https://semver.org/</a>
    /// </p>
    #[serde(rename = "SemanticVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_version: Option<String>,
    /// <p>This property corresponds to the parameter of the same name for the <i>AWS CloudFormation <a href="https://docs.aws.amazon.com/goto/WebAPI/cloudformation-2010-05-15/CreateChangeSet">CreateChangeSet</a>
    /// </i> API.</p>
    #[serde(rename = "StackName")]
    pub stack_name: String,
    /// <p>This property corresponds to the parameter of the same name for the <i>AWS CloudFormation <a href="https://docs.aws.amazon.com/goto/WebAPI/cloudformation-2010-05-15/CreateChangeSet">CreateChangeSet</a>
    /// </i> API.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The UUID returned by CreateCloudFormationTemplate.</p><p>Pattern: [0-9a-fA-F]{8}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{12}</p>
    #[serde(rename = "TemplateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateCloudFormationTemplateRequest {
    /// <p>The Amazon Resource Name (ARN) of the application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The semantic version of the application:</p><p>
    /// <a href="https://semver.org/">https://semver.org/</a>
    /// </p>
    #[serde(rename = "SemanticVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateCloudFormationTemplateResponse {
    /// <p>The application Amazon Resource Name (ARN).</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>The date and time this resource was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    /// <p>The date and time this template expires. Templates
    /// expire 1 hour after creation.</p>
    #[serde(rename = "ExpirationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    /// <p>The semantic version of the application:</p><p>
    /// <a href="https://semver.org/">https://semver.org/</a>
    /// </p>
    #[serde(rename = "SemanticVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_version: Option<String>,
    /// <p>Status of the template creation workflow.</p><p>Possible values: PREPARING | ACTIVE | EXPIRED
    /// </p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The UUID returned by CreateCloudFormationTemplate.</p><p>Pattern: [0-9a-fA-F]{8}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{12}</p>
    #[serde(rename = "TemplateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    /// <p>A link to the template that can be used to deploy the application using
    /// AWS CloudFormation.</p>
    #[serde(rename = "TemplateUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteApplicationRequest {
    /// <p>The Amazon Resource Name (ARN) of the application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetApplicationPolicyRequest {
    /// <p>The Amazon Resource Name (ARN) of the application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetApplicationPolicyResponse {
    /// <p>An array of policy statements applied to the application.</p>
    #[serde(rename = "Statements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statements: Option<Vec<ApplicationPolicyStatement>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>A URL with more information about the application, for example the location of your GitHub repository for the application.</p>
    #[serde(rename = "HomePageUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_page_url: Option<String>,
    /// <p>Whether the author of this application has been verified. This means means that AWS has made a good faith review, as a reasonable and prudent service provider, of the information provided by the requester and has confirmed that the requester's identity is as claimed.</p>
    #[serde(rename = "IsVerifiedAuthor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_verified_author: Option<bool>,
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
    /// <p>The URL to the public profile of a verified author. This URL is submitted by the author.</p>
    #[serde(rename = "VerifiedAuthorUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_author_url: Option<String>,
    /// <p>Version information about the application.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<Version>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetCloudFormationTemplateRequest {
    /// <p>The Amazon Resource Name (ARN) of the application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The UUID returned by CreateCloudFormationTemplate.</p><p>Pattern: [0-9a-fA-F]{8}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{12}</p>
    #[serde(rename = "TemplateId")]
    pub template_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetCloudFormationTemplateResponse {
    /// <p>The application Amazon Resource Name (ARN).</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>The date and time this resource was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    /// <p>The date and time this template expires. Templates
    /// expire 1 hour after creation.</p>
    #[serde(rename = "ExpirationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    /// <p>The semantic version of the application:</p><p>
    /// <a href="https://semver.org/">https://semver.org/</a>
    /// </p>
    #[serde(rename = "SemanticVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_version: Option<String>,
    /// <p>Status of the template creation workflow.</p><p>Possible values: PREPARING | ACTIVE | EXPIRED
    /// </p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The UUID returned by CreateCloudFormationTemplate.</p><p>Pattern: [0-9a-fA-F]{8}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{12}</p>
    #[serde(rename = "TemplateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    /// <p>A link to the template that can be used to deploy the application using
    /// AWS CloudFormation.</p>
    #[serde(rename = "TemplateUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListApplicationDependenciesRequest {
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
    /// <p>The semantic version of the application to get.</p>
    #[serde(rename = "SemanticVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListApplicationDependenciesResponse {
    /// <p>An array of application summaries nested in the application.</p>
    #[serde(rename = "Dependencies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<ApplicationDependencySummary>>,
    /// <p>The token to request the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutApplicationPolicyRequest {
    /// <p>The Amazon Resource Name (ARN) of the application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>An array of policy statements applied to the application.</p>
    #[serde(rename = "Statements")]
    pub statements: Vec<ApplicationPolicyStatement>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutApplicationPolicyResponse {
    /// <p>An array of policy statements applied to the application.</p>
    #[serde(rename = "Statements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statements: Option<Vec<ApplicationPolicyStatement>>,
}

/// <p>This property corresponds to the <i>AWS CloudFormation <a href="https://docs.aws.amazon.com/goto/WebAPI/cloudformation-2010-05-15/RollbackConfiguration">RollbackConfiguration</a>
/// </i> Data Type.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RollbackConfiguration {
    /// <p>This property corresponds to the content of the same name for the <i>AWS CloudFormation <a href="https://docs.aws.amazon.com/goto/WebAPI/cloudformation-2010-05-15/RollbackConfiguration">RollbackConfiguration</a>
    /// </i> Data Type.</p>
    #[serde(rename = "MonitoringTimeInMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_time_in_minutes: Option<i64>,
    /// <p>This property corresponds to the content of the same name for the <i>AWS CloudFormation <a href="https://docs.aws.amazon.com/goto/WebAPI/cloudformation-2010-05-15/RollbackConfiguration">RollbackConfiguration</a>
    /// </i> Data Type.</p>
    #[serde(rename = "RollbackTriggers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollback_triggers: Option<Vec<RollbackTrigger>>,
}

/// <p>This property corresponds to the <i>AWS CloudFormation <a href="https://docs.aws.amazon.com/goto/WebAPI/cloudformation-2010-05-15/RollbackTrigger">RollbackTrigger</a>
/// </i> Data Type.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RollbackTrigger {
    /// <p>This property corresponds to the content of the same name for the <i>AWS CloudFormation <a href="https://docs.aws.amazon.com/goto/WebAPI/cloudformation-2010-05-15/RollbackTrigger">RollbackTrigger</a>
    /// </i> Data Type.</p>
    #[serde(rename = "Arn")]
    pub arn: String,
    /// <p>This property corresponds to the content of the same name for the <i>AWS CloudFormation <a href="https://docs.aws.amazon.com/goto/WebAPI/cloudformation-2010-05-15/RollbackTrigger">RollbackTrigger</a>
    /// </i> Data Type.</p>
    #[serde(rename = "Type")]
    pub type_: String,
}

/// <p>This property corresponds to the <i>AWS CloudFormation <a href="https://docs.aws.amazon.com/goto/WebAPI/cloudformation-2010-05-15/Tag">Tag</a>
/// </i> Data Type.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Tag {
    /// <p>This property corresponds to the content of the same name for the <i>AWS CloudFormation <a href="https://docs.aws.amazon.com/goto/WebAPI/cloudformation-2010-05-15/Tag">Tag</a>
    /// </i> Data Type.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>This property corresponds to the content of the same name for the <i>AWS CloudFormation <a href="https://docs.aws.amazon.com/goto/WebAPI/cloudformation-2010-05-15/Tag">
    /// Tag</a>
    /// </i>
    /// Data Type.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    /// <p>A URL with more information about the application, for example the location of your GitHub repository for the application.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>A URL with more information about the application, for example the location of your GitHub repository for the application.</p>
    #[serde(rename = "HomePageUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_page_url: Option<String>,
    /// <p>Whether the author of this application has been verified. This means means that AWS has made a good faith review, as a reasonable and prudent service provider, of the information provided by the requester and has confirmed that the requester's identity is as claimed.</p>
    #[serde(rename = "IsVerifiedAuthor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_verified_author: Option<bool>,
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
    /// <p>The URL to the public profile of a verified author. This URL is submitted by the author.</p>
    #[serde(rename = "VerifiedAuthorUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_author_url: Option<String>,
    /// <p>Version information about the application.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<Version>,
}

/// <p>Application version details.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>A list of values that you must specify before you can deploy certain applications.
    /// Some applications might include resources that can affect permissions in your AWS
    /// account, for example, by creating new AWS Identity and Access Management (IAM) users.
    /// For those applications, you must explicitly acknowledge their capabilities by
    /// specifying this parameter.</p><p>The only valid values are CAPABILITY_IAM, CAPABILITY_NAMED_IAM,
    /// CAPABILITY_RESOURCE_POLICY, and CAPABILITY_AUTO_EXPAND.</p><p>The following resources require you to specify CAPABILITY_IAM or
    /// CAPABILITY_NAMED_IAM:
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-group.html">AWS::IAM::Group</a>,
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-instanceprofile.html">AWS::IAM::InstanceProfile</a>,
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-policy.html">AWS::IAM::Policy</a>, and
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-role.html">AWS::IAM::Role</a>.
    /// If the application contains IAM resources, you can specify either CAPABILITY_IAM
    /// or CAPABILITY_NAMED_IAM. If the application contains IAM resources
    /// with custom names, you must specify CAPABILITY_NAMED_IAM.</p><p>The following resources require you to specify CAPABILITY_RESOURCE_POLICY:
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-permission.html">AWS::Lambda::Permission</a>,
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-policy.html">AWS::IAM:Policy</a>,
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-applicationautoscaling-scalingpolicy.html">AWS::ApplicationAutoScaling::ScalingPolicy</a>,
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-policy.html">AWS::S3::BucketPolicy</a>,
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sqs-policy.html">AWS::SQS::QueuePolicy</a>, and
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sns-policy.html">AWS::SNS::TopicPolicy</a>.</p><p>Applications that contain one or more nested applications require you to specify
    /// CAPABILITY_AUTO_EXPAND.</p><p>If your application template contains any of the above resources, we recommend that you review
    /// all permissions associated with the application before deploying. If you don't specify
    /// this parameter for an application that requires capabilities, the call will fail.</p>
    #[serde(rename = "RequiredCapabilities")]
    pub required_capabilities: Vec<String>,
    /// <p>Whether all of the AWS resources contained in this application are supported in the region
    /// in which it is being retrieved.</p>
    #[serde(rename = "ResourcesSupported")]
    pub resources_supported: bool,
    /// <p>The semantic version of the application:</p><p>
    /// <a href="https://semver.org/">https://semver.org/</a>
    /// </p>
    #[serde(rename = "SemanticVersion")]
    pub semantic_version: String,
    /// <p>A link to the S3 object that contains the ZIP archive of the source code for this version of your application.</p><p>Maximum size 50 MB</p>
    #[serde(rename = "SourceCodeArchiveUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_code_archive_url: Option<String>,
    /// <p>A link to a public repository for the source code of your application, for example the URL of a specific GitHub commit.</p>
    #[serde(rename = "SourceCodeUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_code_url: Option<String>,
    /// <p>A link to the packaged AWS SAM template of your application.</p>
    #[serde(rename = "TemplateUrl")]
    pub template_url: String,
}

/// <p>An application version summary.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>A link to a public repository for the source code of your application, for example the URL of a specific GitHub commit.</p>
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
}

impl CreateApplicationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateApplicationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateApplicationError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateApplicationError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateApplicationError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateApplicationError::InternalServerError(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateApplicationError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateApplicationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateApplicationError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateApplicationError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateApplicationError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreateApplicationError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreateApplicationError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateApplicationError {}
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
}

impl CreateApplicationVersionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateApplicationVersionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateApplicationVersionError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateApplicationVersionError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateApplicationVersionError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        CreateApplicationVersionError::InternalServerError(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateApplicationVersionError::TooManyRequests(
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
impl fmt::Display for CreateApplicationVersionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateApplicationVersionError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateApplicationVersionError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateApplicationVersionError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreateApplicationVersionError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreateApplicationVersionError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateApplicationVersionError {}
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
}

impl CreateCloudFormationChangeSetError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateCloudFormationChangeSetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateCloudFormationChangeSetError::BadRequest(
                        err.msg,
                    ))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateCloudFormationChangeSetError::Forbidden(
                        err.msg,
                    ))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        CreateCloudFormationChangeSetError::InternalServerError(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        CreateCloudFormationChangeSetError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateCloudFormationChangeSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateCloudFormationChangeSetError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateCloudFormationChangeSetError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreateCloudFormationChangeSetError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateCloudFormationChangeSetError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateCloudFormationChangeSetError {}
/// Errors returned by CreateCloudFormationTemplate
#[derive(Debug, PartialEq)]
pub enum CreateCloudFormationTemplateError {
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
}

impl CreateCloudFormationTemplateError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateCloudFormationTemplateError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateCloudFormationTemplateError::BadRequest(
                        err.msg,
                    ))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateCloudFormationTemplateError::Forbidden(
                        err.msg,
                    ))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        CreateCloudFormationTemplateError::InternalServerError(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateCloudFormationTemplateError::NotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        CreateCloudFormationTemplateError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateCloudFormationTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateCloudFormationTemplateError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateCloudFormationTemplateError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreateCloudFormationTemplateError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateCloudFormationTemplateError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateCloudFormationTemplateError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateCloudFormationTemplateError {}
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
}

impl DeleteApplicationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteApplicationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteApplicationError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DeleteApplicationError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteApplicationError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteApplicationError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteApplicationError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteApplicationError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteApplicationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteApplicationError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteApplicationError::Conflict(ref cause) => write!(f, "{}", cause),
            DeleteApplicationError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteApplicationError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteApplicationError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteApplicationError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteApplicationError {}
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
}

impl GetApplicationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetApplicationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetApplicationError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetApplicationError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetApplicationError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetApplicationError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetApplicationError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetApplicationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetApplicationError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetApplicationError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetApplicationError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetApplicationError::NotFound(ref cause) => write!(f, "{}", cause),
            GetApplicationError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetApplicationError {}
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
}

impl GetApplicationPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetApplicationPolicyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetApplicationPolicyError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetApplicationPolicyError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetApplicationPolicyError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetApplicationPolicyError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetApplicationPolicyError::TooManyRequests(
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
impl fmt::Display for GetApplicationPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetApplicationPolicyError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetApplicationPolicyError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetApplicationPolicyError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetApplicationPolicyError::NotFound(ref cause) => write!(f, "{}", cause),
            GetApplicationPolicyError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetApplicationPolicyError {}
/// Errors returned by GetCloudFormationTemplate
#[derive(Debug, PartialEq)]
pub enum GetCloudFormationTemplateError {
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
}

impl GetCloudFormationTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetCloudFormationTemplateError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetCloudFormationTemplateError::BadRequest(
                        err.msg,
                    ))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetCloudFormationTemplateError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        GetCloudFormationTemplateError::InternalServerError(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetCloudFormationTemplateError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetCloudFormationTemplateError::TooManyRequests(
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
impl fmt::Display for GetCloudFormationTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetCloudFormationTemplateError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetCloudFormationTemplateError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetCloudFormationTemplateError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            GetCloudFormationTemplateError::NotFound(ref cause) => write!(f, "{}", cause),
            GetCloudFormationTemplateError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetCloudFormationTemplateError {}
/// Errors returned by ListApplicationDependencies
#[derive(Debug, PartialEq)]
pub enum ListApplicationDependenciesError {
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
}

impl ListApplicationDependenciesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListApplicationDependenciesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListApplicationDependenciesError::BadRequest(
                        err.msg,
                    ))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListApplicationDependenciesError::Forbidden(
                        err.msg,
                    ))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        ListApplicationDependenciesError::InternalServerError(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListApplicationDependenciesError::NotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListApplicationDependenciesError::TooManyRequests(
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
impl fmt::Display for ListApplicationDependenciesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListApplicationDependenciesError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListApplicationDependenciesError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListApplicationDependenciesError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            ListApplicationDependenciesError::NotFound(ref cause) => write!(f, "{}", cause),
            ListApplicationDependenciesError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListApplicationDependenciesError {}
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
}

impl ListApplicationVersionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListApplicationVersionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListApplicationVersionsError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListApplicationVersionsError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListApplicationVersionsError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListApplicationVersionsError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListApplicationVersionsError::TooManyRequests(
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
impl fmt::Display for ListApplicationVersionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListApplicationVersionsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListApplicationVersionsError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListApplicationVersionsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListApplicationVersionsError::NotFound(ref cause) => write!(f, "{}", cause),
            ListApplicationVersionsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListApplicationVersionsError {}
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
}

impl ListApplicationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListApplicationsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListApplicationsError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListApplicationsError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListApplicationsError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListApplicationsError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListApplicationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListApplicationsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListApplicationsError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListApplicationsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListApplicationsError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListApplicationsError {}
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
}

impl PutApplicationPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutApplicationPolicyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(PutApplicationPolicyError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(PutApplicationPolicyError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(PutApplicationPolicyError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(PutApplicationPolicyError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(PutApplicationPolicyError::TooManyRequests(
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
impl fmt::Display for PutApplicationPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutApplicationPolicyError::BadRequest(ref cause) => write!(f, "{}", cause),
            PutApplicationPolicyError::Forbidden(ref cause) => write!(f, "{}", cause),
            PutApplicationPolicyError::InternalServerError(ref cause) => write!(f, "{}", cause),
            PutApplicationPolicyError::NotFound(ref cause) => write!(f, "{}", cause),
            PutApplicationPolicyError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutApplicationPolicyError {}
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
}

impl UpdateApplicationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateApplicationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateApplicationError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateApplicationError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateApplicationError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateApplicationError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateApplicationError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateApplicationError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateApplicationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateApplicationError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateApplicationError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateApplicationError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateApplicationError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateApplicationError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateApplicationError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateApplicationError {}
/// Trait representing the capabilities of the AWSServerlessApplicationRepository API. AWSServerlessApplicationRepository clients implement this trait.
#[async_trait]
pub trait ServerlessRepo {
    /// <p>Creates an application, optionally including an AWS SAM file to create the first application version in the same call.</p>
    async fn create_application(
        &self,
        input: CreateApplicationRequest,
    ) -> Result<CreateApplicationResponse, RusotoError<CreateApplicationError>>;

    /// <p>Creates an application version.</p>
    async fn create_application_version(
        &self,
        input: CreateApplicationVersionRequest,
    ) -> Result<CreateApplicationVersionResponse, RusotoError<CreateApplicationVersionError>>;

    /// <p>Creates an AWS CloudFormation change set for the given application.</p>
    async fn create_cloud_formation_change_set(
        &self,
        input: CreateCloudFormationChangeSetRequest,
    ) -> Result<
        CreateCloudFormationChangeSetResponse,
        RusotoError<CreateCloudFormationChangeSetError>,
    >;

    /// <p>Creates an AWS CloudFormation template.</p>
    async fn create_cloud_formation_template(
        &self,
        input: CreateCloudFormationTemplateRequest,
    ) -> Result<CreateCloudFormationTemplateResponse, RusotoError<CreateCloudFormationTemplateError>>;

    /// <p>Deletes the specified application.</p>
    async fn delete_application(
        &self,
        input: DeleteApplicationRequest,
    ) -> Result<(), RusotoError<DeleteApplicationError>>;

    /// <p>Gets the specified application.</p>
    async fn get_application(
        &self,
        input: GetApplicationRequest,
    ) -> Result<GetApplicationResponse, RusotoError<GetApplicationError>>;

    /// <p>Retrieves the policy for the application.</p>
    async fn get_application_policy(
        &self,
        input: GetApplicationPolicyRequest,
    ) -> Result<GetApplicationPolicyResponse, RusotoError<GetApplicationPolicyError>>;

    /// <p>Gets the specified AWS CloudFormation template.</p>
    async fn get_cloud_formation_template(
        &self,
        input: GetCloudFormationTemplateRequest,
    ) -> Result<GetCloudFormationTemplateResponse, RusotoError<GetCloudFormationTemplateError>>;

    /// <p>Retrieves the list of applications nested in the containing application.</p>
    async fn list_application_dependencies(
        &self,
        input: ListApplicationDependenciesRequest,
    ) -> Result<ListApplicationDependenciesResponse, RusotoError<ListApplicationDependenciesError>>;

    /// <p>Lists versions for the specified application.</p>
    async fn list_application_versions(
        &self,
        input: ListApplicationVersionsRequest,
    ) -> Result<ListApplicationVersionsResponse, RusotoError<ListApplicationVersionsError>>;

    /// <p>Lists applications owned by the requester.</p>
    async fn list_applications(
        &self,
        input: ListApplicationsRequest,
    ) -> Result<ListApplicationsResponse, RusotoError<ListApplicationsError>>;

    /// <p>Sets the permission policy for an application. For the list of actions supported for this operation, see
    /// <a href="https://docs.aws.amazon.com/serverlessrepo/latest/devguide/access-control-resource-based.html#application-permissions">Application
    /// Permissions</a>
    /// .</p>
    async fn put_application_policy(
        &self,
        input: PutApplicationPolicyRequest,
    ) -> Result<PutApplicationPolicyResponse, RusotoError<PutApplicationPolicyError>>;

    /// <p>Updates the specified application.</p>
    async fn update_application(
        &self,
        input: UpdateApplicationRequest,
    ) -> Result<UpdateApplicationResponse, RusotoError<UpdateApplicationError>>;
}
/// A client for the AWSServerlessApplicationRepository API.
#[derive(Clone)]
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
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> ServerlessRepoClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        ServerlessRepoClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> ServerlessRepoClient {
        ServerlessRepoClient { client, region }
    }
}

#[async_trait]
impl ServerlessRepo for ServerlessRepoClient {
    /// <p>Creates an application, optionally including an AWS SAM file to create the first application version in the same call.</p>
    async fn create_application(
        &self,
        input: CreateApplicationRequest,
    ) -> Result<CreateApplicationResponse, RusotoError<CreateApplicationError>> {
        let request_uri = "/applications";

        let mut request = SignedRequest::new("POST", "serverlessrepo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateApplicationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateApplicationError::from_response(response))
        }
    }

    /// <p>Creates an application version.</p>
    async fn create_application_version(
        &self,
        input: CreateApplicationVersionRequest,
    ) -> Result<CreateApplicationVersionResponse, RusotoError<CreateApplicationVersionError>> {
        let request_uri = format!(
            "/applications/{application_id}/versions/{semantic_version}",
            application_id = input.application_id,
            semantic_version = input.semantic_version
        );

        let mut request = SignedRequest::new("PUT", "serverlessrepo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateApplicationVersionResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateApplicationVersionError::from_response(response))
        }
    }

    /// <p>Creates an AWS CloudFormation change set for the given application.</p>
    async fn create_cloud_formation_change_set(
        &self,
        input: CreateCloudFormationChangeSetRequest,
    ) -> Result<
        CreateCloudFormationChangeSetResponse,
        RusotoError<CreateCloudFormationChangeSetError>,
    > {
        let request_uri = format!(
            "/applications/{application_id}/changesets",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("POST", "serverlessrepo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateCloudFormationChangeSetResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateCloudFormationChangeSetError::from_response(response))
        }
    }

    /// <p>Creates an AWS CloudFormation template.</p>
    async fn create_cloud_formation_template(
        &self,
        input: CreateCloudFormationTemplateRequest,
    ) -> Result<CreateCloudFormationTemplateResponse, RusotoError<CreateCloudFormationTemplateError>>
    {
        let request_uri = format!(
            "/applications/{application_id}/templates",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("POST", "serverlessrepo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateCloudFormationTemplateResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateCloudFormationTemplateError::from_response(response))
        }
    }

    /// <p>Deletes the specified application.</p>
    async fn delete_application(
        &self,
        input: DeleteApplicationRequest,
    ) -> Result<(), RusotoError<DeleteApplicationError>> {
        let request_uri = format!(
            "/applications/{application_id}",
            application_id = input.application_id
        );

        let mut request =
            SignedRequest::new("DELETE", "serverlessrepo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteApplicationError::from_response(response))
        }
    }

    /// <p>Gets the specified application.</p>
    async fn get_application(
        &self,
        input: GetApplicationRequest,
    ) -> Result<GetApplicationResponse, RusotoError<GetApplicationError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetApplicationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetApplicationError::from_response(response))
        }
    }

    /// <p>Retrieves the policy for the application.</p>
    async fn get_application_policy(
        &self,
        input: GetApplicationPolicyRequest,
    ) -> Result<GetApplicationPolicyResponse, RusotoError<GetApplicationPolicyError>> {
        let request_uri = format!(
            "/applications/{application_id}/policy",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("GET", "serverlessrepo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetApplicationPolicyResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetApplicationPolicyError::from_response(response))
        }
    }

    /// <p>Gets the specified AWS CloudFormation template.</p>
    async fn get_cloud_formation_template(
        &self,
        input: GetCloudFormationTemplateRequest,
    ) -> Result<GetCloudFormationTemplateResponse, RusotoError<GetCloudFormationTemplateError>>
    {
        let request_uri = format!(
            "/applications/{application_id}/templates/{template_id}",
            application_id = input.application_id,
            template_id = input.template_id
        );

        let mut request = SignedRequest::new("GET", "serverlessrepo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetCloudFormationTemplateResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetCloudFormationTemplateError::from_response(response))
        }
    }

    /// <p>Retrieves the list of applications nested in the containing application.</p>
    async fn list_application_dependencies(
        &self,
        input: ListApplicationDependenciesRequest,
    ) -> Result<ListApplicationDependenciesResponse, RusotoError<ListApplicationDependenciesError>>
    {
        let request_uri = format!(
            "/applications/{application_id}/dependencies",
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
        if let Some(ref x) = input.semantic_version {
            params.put("semanticVersion", x);
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
                .deserialize::<ListApplicationDependenciesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListApplicationDependenciesError::from_response(response))
        }
    }

    /// <p>Lists versions for the specified application.</p>
    async fn list_application_versions(
        &self,
        input: ListApplicationVersionsRequest,
    ) -> Result<ListApplicationVersionsResponse, RusotoError<ListApplicationVersionsError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListApplicationVersionsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListApplicationVersionsError::from_response(response))
        }
    }

    /// <p>Lists applications owned by the requester.</p>
    async fn list_applications(
        &self,
        input: ListApplicationsRequest,
    ) -> Result<ListApplicationsResponse, RusotoError<ListApplicationsError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListApplicationsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListApplicationsError::from_response(response))
        }
    }

    /// <p>Sets the permission policy for an application. For the list of actions supported for this operation, see
    /// <a href="https://docs.aws.amazon.com/serverlessrepo/latest/devguide/access-control-resource-based.html#application-permissions">Application
    /// Permissions</a>
    /// .</p>
    async fn put_application_policy(
        &self,
        input: PutApplicationPolicyRequest,
    ) -> Result<PutApplicationPolicyResponse, RusotoError<PutApplicationPolicyError>> {
        let request_uri = format!(
            "/applications/{application_id}/policy",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("PUT", "serverlessrepo", &self.region, &request_uri);
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
                .deserialize::<PutApplicationPolicyResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutApplicationPolicyError::from_response(response))
        }
    }

    /// <p>Updates the specified application.</p>
    async fn update_application(
        &self,
        input: UpdateApplicationRequest,
    ) -> Result<UpdateApplicationResponse, RusotoError<UpdateApplicationError>> {
        let request_uri = format!(
            "/applications/{application_id}",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("PATCH", "serverlessrepo", &self.region, &request_uri);
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
                .deserialize::<UpdateApplicationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateApplicationError::from_response(response))
        }
    }
}
