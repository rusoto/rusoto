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
/// <p> Details of an EC2 AMI. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Ami {
    /// <p>The description of the EC2 AMI. </p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The AMI ID of the EC2 AMI. </p>
    #[serde(rename = "image")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// <p>The name of the EC2 AMI. </p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The AWS Region of the EC2 AMI. </p>
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<ImageState>,
}

/// <p> Define and configure the output AMIs of the pipeline. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AmiDistributionConfiguration {
    /// <p>The tags to apply to AMIs distributed to this Region. </p>
    #[serde(rename = "amiTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ami_tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The description of the distribution configuration. </p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p> The KMS key identifier used to encrypt the distributed image. </p>
    #[serde(rename = "kmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p> Launch permissions can be used to configure which AWS accounts can use the AMI to launch instances. </p>
    #[serde(rename = "launchPermission")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_permission: Option<LaunchPermissionConfiguration>,
    /// <p>The name of the distribution configuration. </p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CancelImageCreationRequest {
    /// <p>The idempotency token used to make this request idempotent.</p>
    #[serde(rename = "clientToken")]
    pub client_token: String,
    /// <p>The Amazon Resource Name (ARN) of the image whose creation you want to cancel.</p>
    #[serde(rename = "imageBuildVersionArn")]
    pub image_build_version_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CancelImageCreationResponse {
    /// <p>The idempotency token used to make this request idempotent.</p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the image whose creation has been cancelled.</p>
    #[serde(rename = "imageBuildVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_build_version_arn: Option<String>,
    /// <p>The request ID that uniquely identifies this request.</p>
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// <p>A detailed view of a component.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Component {
    /// <p>The Amazon Resource Name (ARN) of the component.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The change description of the component.</p>
    #[serde(rename = "changeDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_description: Option<String>,
    /// <p>The data of the component.</p>
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    /// <p>The date that the component was created.</p>
    #[serde(rename = "dateCreated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// <p>The description of the component.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The encryption status of the component.</p>
    #[serde(rename = "encrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    /// <p>The KMS key identifier used to encrypt the component.</p>
    #[serde(rename = "kmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>The name of the component.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The owner of the component.</p>
    #[serde(rename = "owner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// <p>The platform of the component.</p>
    #[serde(rename = "platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// <p>The operating system (OS) version supported by the component. If the OS information is available, a prefix match is performed against the parent image OS version during image recipe creation. </p>
    #[serde(rename = "supportedOsVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_os_versions: Option<Vec<String>>,
    /// <p>The tags associated with the component.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The type of the component denotes whether the component is used to build the image or only to test it.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The version of the component.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p> Configuration details of the component. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ComponentConfiguration {
    /// <p>The Amazon Resource Name (ARN) of the component. </p>
    #[serde(rename = "componentArn")]
    pub component_arn: String,
}

/// <p>A high-level summary of a component.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ComponentSummary {
    /// <p>The Amazon Resource Name (ARN) of the component.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The change description of the component.</p>
    #[serde(rename = "changeDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_description: Option<String>,
    /// <p>The date that the component was created.</p>
    #[serde(rename = "dateCreated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// <p>The description of the component.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the component.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The owner of the component.</p>
    #[serde(rename = "owner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// <p>The platform of the component.</p>
    #[serde(rename = "platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// <p>The operating system (OS) version supported by the component. If the OS information is available, a prefix match is performed against the parent image OS version during image recipe creation. </p>
    #[serde(rename = "supportedOsVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_os_versions: Option<Vec<String>>,
    /// <p>The tags associated with the component.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The type of the component denotes whether the component is used to build the image or only to test it.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The version of the component.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>A high-level overview of a component semantic version.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ComponentVersion {
    /// <p>The Amazon Resource Name (ARN) of the component.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The date that the component was created.</p>
    #[serde(rename = "dateCreated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// <p>The description of the component.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the component.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The owner of the component.</p>
    #[serde(rename = "owner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// <p>The platform of the component.</p>
    #[serde(rename = "platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// <p> The operating system (OS) version supported by the component. If the OS information is available, a prefix match is performed against the parent image OS version during image recipe creation. </p>
    #[serde(rename = "supportedOsVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_os_versions: Option<Vec<String>>,
    /// <p>The type of the component denotes whether the component is used to build the image or only to test it.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The semantic version of the component.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateComponentRequest {
    /// <p>The change description of the component. Describes what change has been made in this version, or what makes this version different from other versions of this component.</p>
    #[serde(rename = "changeDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_description: Option<String>,
    /// <p>The idempotency token of the component.</p>
    #[serde(rename = "clientToken")]
    pub client_token: String,
    /// <p>The data of the component. Used to specify the data inline. Either <code>data</code> or <code>uri</code> can be used to specify the data within the component.</p>
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    /// <p>The description of the component. Describes the contents of the component.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ID of the KMS key that should be used to encrypt this component.</p>
    #[serde(rename = "kmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>The name of the component.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The platform of the component.</p>
    #[serde(rename = "platform")]
    pub platform: String,
    /// <p>The semantic version of the component. This version follows the semantic version syntax. For example, major.minor.patch. This could be versioned like software (2.0.1) or like a date (2019.12.01).</p>
    #[serde(rename = "semanticVersion")]
    pub semantic_version: String,
    /// <p> The operating system (OS) version supported by the component. If the OS information is available, a prefix match is performed against the parent image OS version during image recipe creation. </p>
    #[serde(rename = "supportedOsVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_os_versions: Option<Vec<String>>,
    /// <p>The tags of the component.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The uri of the component. Must be an S3 URL and the requester must have permission to access the S3 bucket. If you use S3, you can specify component content up to your service quota. Either <code>data</code> or <code>uri</code> can be used to specify the data within the component.</p>
    #[serde(rename = "uri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateComponentResponse {
    /// <p>The idempotency token used to make this request idempotent.</p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the component that was created by this request.</p>
    #[serde(rename = "componentBuildVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_build_version_arn: Option<String>,
    /// <p>The request ID that uniquely identifies this request.</p>
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDistributionConfigurationRequest {
    /// <p> The idempotency token of the distribution configuration. </p>
    #[serde(rename = "clientToken")]
    pub client_token: String,
    /// <p> The description of the distribution configuration. </p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p> The distributions of the distribution configuration. </p>
    #[serde(rename = "distributions")]
    pub distributions: Vec<Distribution>,
    /// <p> The name of the distribution configuration. </p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p> The tags of the distribution configuration. </p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateDistributionConfigurationResponse {
    /// <p> The idempotency token used to make this request idempotent. </p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p> The Amazon Resource Name (ARN) of the distribution configuration that was created by this request. </p>
    #[serde(rename = "distributionConfigurationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution_configuration_arn: Option<String>,
    /// <p> The request ID that uniquely identifies this request. </p>
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateImagePipelineRequest {
    /// <p> The idempotency token used to make this request idempotent. </p>
    #[serde(rename = "clientToken")]
    pub client_token: String,
    /// <p> The description of the image pipeline. </p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p> The Amazon Resource Name (ARN) of the distribution configuration that will be used to configure and distribute images created by this image pipeline. </p>
    #[serde(rename = "distributionConfigurationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution_configuration_arn: Option<String>,
    /// <p> Collects additional information about the image being created, including the operating system (OS) version and package list. This information is used to enhance the overall experience of using EC2 Image Builder. Enabled by default. </p>
    #[serde(rename = "enhancedImageMetadataEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enhanced_image_metadata_enabled: Option<bool>,
    /// <p> The Amazon Resource Name (ARN) of the image recipe that will be used to configure images created by this image pipeline. </p>
    #[serde(rename = "imageRecipeArn")]
    pub image_recipe_arn: String,
    /// <p> The image test configuration of the image pipeline. </p>
    #[serde(rename = "imageTestsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_tests_configuration: Option<ImageTestsConfiguration>,
    /// <p> The Amazon Resource Name (ARN) of the infrastructure configuration that will be used to build images created by this image pipeline. </p>
    #[serde(rename = "infrastructureConfigurationArn")]
    pub infrastructure_configuration_arn: String,
    /// <p> The name of the image pipeline. </p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p> The schedule of the image pipeline. </p>
    #[serde(rename = "schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Schedule>,
    /// <p> The status of the image pipeline. </p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p> The tags of the image pipeline. </p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateImagePipelineResponse {
    /// <p> The idempotency token used to make this request idempotent. </p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p> The Amazon Resource Name (ARN) of the image pipeline that was created by this request. </p>
    #[serde(rename = "imagePipelineArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_pipeline_arn: Option<String>,
    /// <p> The request ID that uniquely identifies this request. </p>
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateImageRecipeRequest {
    /// <p>The block device mappings of the image recipe. </p>
    #[serde(rename = "blockDeviceMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_device_mappings: Option<Vec<InstanceBlockDeviceMapping>>,
    /// <p>The idempotency token used to make this request idempotent. </p>
    #[serde(rename = "clientToken")]
    pub client_token: String,
    /// <p>The components of the image recipe. </p>
    #[serde(rename = "components")]
    pub components: Vec<ComponentConfiguration>,
    /// <p> The description of the image recipe. </p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p> The name of the image recipe. </p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The parent image of the image recipe. The value of the string can be the ARN of the parent image or an AMI ID. The format for the ARN follows this example: <code>arn:aws:imagebuilder:us-west-2:aws:image/windows-server-2016-english-full-base-x86/2019.x.x</code>. The ARN ends with <code>/20xx.x.x</code>, which communicates to EC2 Image Builder that you want to use the latest AMI created in 20xx (year). You can provide the specific version that you want to use, or you can use a wildcard in all of the fields. If you enter an AMI ID for the string value, you must have access to the AMI, and the AMI must be in the same Region in which you are using Image Builder. </p>
    #[serde(rename = "parentImage")]
    pub parent_image: String,
    /// <p>The semantic version of the image recipe. </p>
    #[serde(rename = "semanticVersion")]
    pub semantic_version: String,
    /// <p> The tags of the image recipe. </p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The working directory to be used during build and test workflows.</p>
    #[serde(rename = "workingDirectory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_directory: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateImageRecipeResponse {
    /// <p>The idempotency token used to make this request idempotent. </p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the image recipe that was created by this request. </p>
    #[serde(rename = "imageRecipeArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_recipe_arn: Option<String>,
    /// <p>The request ID that uniquely identifies this request. </p>
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateImageRequest {
    /// <p> The idempotency token used to make this request idempotent. </p>
    #[serde(rename = "clientToken")]
    pub client_token: String,
    /// <p> The Amazon Resource Name (ARN) of the distribution configuration that defines and configures the outputs of your pipeline. </p>
    #[serde(rename = "distributionConfigurationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution_configuration_arn: Option<String>,
    /// <p> Collects additional information about the image being created, including the operating system (OS) version and package list. This information is used to enhance the overall experience of using EC2 Image Builder. Enabled by default. </p>
    #[serde(rename = "enhancedImageMetadataEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enhanced_image_metadata_enabled: Option<bool>,
    /// <p> The Amazon Resource Name (ARN) of the image recipe that defines how images are configured, tested, and assessed. </p>
    #[serde(rename = "imageRecipeArn")]
    pub image_recipe_arn: String,
    /// <p> The image tests configuration of the image. </p>
    #[serde(rename = "imageTestsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_tests_configuration: Option<ImageTestsConfiguration>,
    /// <p> The Amazon Resource Name (ARN) of the infrastructure configuration that defines the environment in which your image will be built and tested. </p>
    #[serde(rename = "infrastructureConfigurationArn")]
    pub infrastructure_configuration_arn: String,
    /// <p> The tags of the image. </p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateImageResponse {
    /// <p> The idempotency token used to make this request idempotent. </p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p> The Amazon Resource Name (ARN) of the image that was created by this request. </p>
    #[serde(rename = "imageBuildVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_build_version_arn: Option<String>,
    /// <p> The request ID that uniquely identifies this request. </p>
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateInfrastructureConfigurationRequest {
    /// <p>The idempotency token used to make this request idempotent. </p>
    #[serde(rename = "clientToken")]
    pub client_token: String,
    /// <p>The description of the infrastructure configuration. </p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The instance profile to associate with the instance used to customize your EC2 AMI. </p>
    #[serde(rename = "instanceProfileName")]
    pub instance_profile_name: String,
    /// <p>The instance types of the infrastructure configuration. You can specify one or more instance types to use for this build. The service will pick one of these instance types based on availability. </p>
    #[serde(rename = "instanceTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_types: Option<Vec<String>>,
    /// <p>The key pair of the infrastructure configuration. This can be used to log on to and debug the instance used to create your image. </p>
    #[serde(rename = "keyPair")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_pair: Option<String>,
    /// <p>The logging configuration of the infrastructure configuration. </p>
    #[serde(rename = "logging")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<Logging>,
    /// <p>The name of the infrastructure configuration. </p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The tags attached to the resource created by Image Builder.</p>
    #[serde(rename = "resourceTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The security group IDs to associate with the instance used to customize your EC2 AMI. </p>
    #[serde(rename = "securityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    /// <p>The SNS topic on which to send image build events. </p>
    #[serde(rename = "snsTopicArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arn: Option<String>,
    /// <p>The subnet ID in which to place the instance used to customize your EC2 AMI. </p>
    #[serde(rename = "subnetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    /// <p>The tags of the infrastructure configuration. </p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The terminate instance on failure setting of the infrastructure configuration. Set to false if you want Image Builder to retain the instance used to configure your AMI if the build or test phase of your workflow fails. </p>
    #[serde(rename = "terminateInstanceOnFailure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminate_instance_on_failure: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateInfrastructureConfigurationResponse {
    /// <p>The idempotency token used to make this request idempotent. </p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the infrastructure configuration that was created by this request. </p>
    #[serde(rename = "infrastructureConfigurationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infrastructure_configuration_arn: Option<String>,
    /// <p>The request ID that uniquely identifies this request. </p>
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteComponentRequest {
    /// <p>The Amazon Resource Name (ARN) of the component build version to delete. </p>
    #[serde(rename = "componentBuildVersionArn")]
    pub component_build_version_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteComponentResponse {
    /// <p>The Amazon Resource Name (ARN) of the component build version that was deleted. </p>
    #[serde(rename = "componentBuildVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_build_version_arn: Option<String>,
    /// <p>The request ID that uniquely identifies this request. </p>
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteDistributionConfigurationRequest {
    /// <p>The Amazon Resource Name (ARN) of the distribution configuration to delete. </p>
    #[serde(rename = "distributionConfigurationArn")]
    pub distribution_configuration_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteDistributionConfigurationResponse {
    /// <p>The Amazon Resource Name (ARN) of the distribution configuration that was deleted. </p>
    #[serde(rename = "distributionConfigurationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution_configuration_arn: Option<String>,
    /// <p>The request ID that uniquely identifies this request. </p>
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteImagePipelineRequest {
    /// <p>The Amazon Resource Name (ARN) of the image pipeline to delete. </p>
    #[serde(rename = "imagePipelineArn")]
    pub image_pipeline_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteImagePipelineResponse {
    /// <p>The Amazon Resource Name (ARN) of the image pipeline that was deleted. </p>
    #[serde(rename = "imagePipelineArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_pipeline_arn: Option<String>,
    /// <p>The request ID that uniquely identifies this request. </p>
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteImageRecipeRequest {
    /// <p>The Amazon Resource Name (ARN) of the image recipe to delete. </p>
    #[serde(rename = "imageRecipeArn")]
    pub image_recipe_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteImageRecipeResponse {
    /// <p>The Amazon Resource Name (ARN) of the image recipe that was deleted. </p>
    #[serde(rename = "imageRecipeArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_recipe_arn: Option<String>,
    /// <p>The request ID that uniquely identifies this request. </p>
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteImageRequest {
    /// <p>The Amazon Resource Name (ARN) of the image to delete. </p>
    #[serde(rename = "imageBuildVersionArn")]
    pub image_build_version_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteImageResponse {
    /// <p>The Amazon Resource Name (ARN) of the image that was deleted. </p>
    #[serde(rename = "imageBuildVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_build_version_arn: Option<String>,
    /// <p>The request ID that uniquely identifies this request. </p>
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteInfrastructureConfigurationRequest {
    /// <p>The Amazon Resource Name (ARN) of the infrastructure configuration to delete. </p>
    #[serde(rename = "infrastructureConfigurationArn")]
    pub infrastructure_configuration_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteInfrastructureConfigurationResponse {
    /// <p>The Amazon Resource Name (ARN) of the infrastructure configuration that was deleted. </p>
    #[serde(rename = "infrastructureConfigurationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infrastructure_configuration_arn: Option<String>,
    /// <p>The request ID that uniquely identifies this request. </p>
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// <p> Defines the settings for a specific Region. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Distribution {
    /// <p>The specific AMI settings (for example, launch permissions, AMI tags). </p>
    #[serde(rename = "amiDistributionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ami_distribution_configuration: Option<AmiDistributionConfiguration>,
    /// <p>The License Manager Configuration to associate with the AMI in the specified Region.</p>
    #[serde(rename = "licenseConfigurationArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_configuration_arns: Option<Vec<String>>,
    /// <p>The target Region. </p>
    #[serde(rename = "region")]
    pub region: String,
}

/// <p>A distribution configuration.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DistributionConfiguration {
    /// <p>The Amazon Resource Name (ARN) of the distribution configuration.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The date on which this distribution configuration was created.</p>
    #[serde(rename = "dateCreated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// <p>The date on which this distribution configuration was last updated.</p>
    #[serde(rename = "dateUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// <p>The description of the distribution configuration.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The distributions of the distribution configuration.</p>
    #[serde(rename = "distributions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distributions: Option<Vec<Distribution>>,
    /// <p>The name of the distribution configuration.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The tags of the distribution configuration.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The maximum duration in minutes for this distribution configuration.</p>
    #[serde(rename = "timeoutMinutes")]
    pub timeout_minutes: i64,
}

/// <p>A high-level overview of a distribution configuration.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DistributionConfigurationSummary {
    /// <p>The Amazon Resource Name (ARN) of the distribution configuration.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The date on which the distribution configuration was created.</p>
    #[serde(rename = "dateCreated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// <p>The date on which the distribution configuration was updated.</p>
    #[serde(rename = "dateUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// <p>The description of the distribution configuration.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the distribution configuration.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The tags associated with the distribution configuration.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Amazon EBS-specific block device mapping specifications.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct EbsInstanceBlockDeviceSpecification {
    /// <p>Use to configure delete on termination of the associated device.</p>
    #[serde(rename = "deleteOnTermination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_on_termination: Option<bool>,
    /// <p>Use to configure device encryption.</p>
    #[serde(rename = "encrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    /// <p>Use to configure device IOPS.</p>
    #[serde(rename = "iops")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iops: Option<i64>,
    /// <p>Use to configure the KMS key to use when encrypting the device.</p>
    #[serde(rename = "kmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>The snapshot that defines the device contents.</p>
    #[serde(rename = "snapshotId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
    /// <p>Use to override the device's volume size.</p>
    #[serde(rename = "volumeSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_size: Option<i64>,
    /// <p>Use to override the device's volume type.</p>
    #[serde(rename = "volumeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_type: Option<String>,
}

/// <p>A filter name and value pair that is used to return a more specific list of results from a list operation. Filters can be used to match a set of resources by specific criteria, such as tags, attributes, or IDs. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Filter {
    /// <p>The name of the filter. Filter names are case-sensitive. </p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The filter values. Filter values are case-sensitive. </p>
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetComponentPolicyRequest {
    /// <p>The Amazon Resource Name (ARN) of the component whose policy you want to retrieve. </p>
    #[serde(rename = "componentArn")]
    pub component_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetComponentPolicyResponse {
    /// <p>The component policy. </p>
    #[serde(rename = "policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    /// <p>The request ID that uniquely identifies this request. </p>
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetComponentRequest {
    /// <p>The Amazon Resource Name (ARN) of the component that you want to retrieve. Regex requires "/\d+$" suffix.</p>
    #[serde(rename = "componentBuildVersionArn")]
    pub component_build_version_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetComponentResponse {
    /// <p>The component object associated with the specified ARN. </p>
    #[serde(rename = "component")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<Component>,
    /// <p>The request ID that uniquely identifies this request. </p>
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDistributionConfigurationRequest {
    /// <p>The Amazon Resource Name (ARN) of the distribution configuration that you want to retrieve. </p>
    #[serde(rename = "distributionConfigurationArn")]
    pub distribution_configuration_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDistributionConfigurationResponse {
    /// <p>The distribution configuration object. </p>
    #[serde(rename = "distributionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution_configuration: Option<DistributionConfiguration>,
    /// <p>The request ID that uniquely identifies this request. </p>
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetImagePipelineRequest {
    /// <p>The Amazon Resource Name (ARN) of the image pipeline that you want to retrieve. </p>
    #[serde(rename = "imagePipelineArn")]
    pub image_pipeline_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetImagePipelineResponse {
    /// <p>The image pipeline object. </p>
    #[serde(rename = "imagePipeline")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_pipeline: Option<ImagePipeline>,
    /// <p>The request ID that uniquely identifies this request. </p>
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetImagePolicyRequest {
    /// <p>The Amazon Resource Name (ARN) of the image whose policy you want to retrieve. </p>
    #[serde(rename = "imageArn")]
    pub image_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetImagePolicyResponse {
    /// <p>The image policy object. </p>
    #[serde(rename = "policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    /// <p>The request ID that uniquely identifies this request. </p>
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetImageRecipePolicyRequest {
    /// <p>The Amazon Resource Name (ARN) of the image recipe whose policy you want to retrieve. </p>
    #[serde(rename = "imageRecipeArn")]
    pub image_recipe_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetImageRecipePolicyResponse {
    /// <p>The image recipe policy object. </p>
    #[serde(rename = "policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    /// <p>The request ID that uniquely identifies this request. </p>
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetImageRecipeRequest {
    /// <p>The Amazon Resource Name (ARN) of the image recipe that you want to retrieve. </p>
    #[serde(rename = "imageRecipeArn")]
    pub image_recipe_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetImageRecipeResponse {
    /// <p>The image recipe object. </p>
    #[serde(rename = "imageRecipe")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_recipe: Option<ImageRecipe>,
    /// <p>The request ID that uniquely identifies this request. </p>
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetImageRequest {
    /// <p>The Amazon Resource Name (ARN) of the image that you want to retrieve. </p>
    #[serde(rename = "imageBuildVersionArn")]
    pub image_build_version_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetImageResponse {
    /// <p>The image object. </p>
    #[serde(rename = "image")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Image>,
    /// <p>The request ID that uniquely identifies this request. </p>
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// <p> GetInfrastructureConfiguration request object. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetInfrastructureConfigurationRequest {
    /// <p>The Amazon Resource Name (ARN) of the infrastructure configuration that you want to retrieve. </p>
    #[serde(rename = "infrastructureConfigurationArn")]
    pub infrastructure_configuration_arn: String,
}

/// <p>GetInfrastructureConfiguration response object. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetInfrastructureConfigurationResponse {
    /// <p>The infrastructure configuration object. </p>
    #[serde(rename = "infrastructureConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infrastructure_configuration: Option<InfrastructureConfiguration>,
    /// <p>The request ID that uniquely identifies this request. </p>
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// <p>An image build version.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Image {
    /// <p>The Amazon Resource Name (ARN) of the image.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The date on which this image was created.</p>
    #[serde(rename = "dateCreated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// <p>The distribution configuration used when creating this image.</p>
    #[serde(rename = "distributionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution_configuration: Option<DistributionConfiguration>,
    /// <p> Collects additional information about the image being created, including the operating system (OS) version and package list. This information is used to enhance the overall experience of using EC2 Image Builder. Enabled by default. </p>
    #[serde(rename = "enhancedImageMetadataEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enhanced_image_metadata_enabled: Option<bool>,
    /// <p>The image recipe used when creating the image.</p>
    #[serde(rename = "imageRecipe")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_recipe: Option<ImageRecipe>,
    /// <p>The image tests configuration used when creating this image.</p>
    #[serde(rename = "imageTestsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_tests_configuration: Option<ImageTestsConfiguration>,
    /// <p>The infrastructure used when creating this image.</p>
    #[serde(rename = "infrastructureConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infrastructure_configuration: Option<InfrastructureConfiguration>,
    /// <p>The name of the image.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The operating system version of the instance. For example, Amazon Linux 2, Ubuntu 18, or Microsoft Windows Server 2019. </p>
    #[serde(rename = "osVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    /// <p>The output resources produced when creating this image.</p>
    #[serde(rename = "outputResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_resources: Option<OutputResources>,
    /// <p>The platform of the image.</p>
    #[serde(rename = "platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the image pipeline that created this image.</p>
    #[serde(rename = "sourcePipelineArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_pipeline_arn: Option<String>,
    /// <p>The name of the image pipeline that created this image.</p>
    #[serde(rename = "sourcePipelineName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_pipeline_name: Option<String>,
    /// <p>The state of the image.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<ImageState>,
    /// <p>The tags of the image.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The semantic version of the image.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>Details of an image pipeline.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ImagePipeline {
    /// <p>The Amazon Resource Name (ARN) of the image pipeline.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The date on which this image pipeline was created.</p>
    #[serde(rename = "dateCreated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// <p>The date on which this image pipeline was last run.</p>
    #[serde(rename = "dateLastRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_last_run: Option<String>,
    /// <p>The date on which this image pipeline will next be run.</p>
    #[serde(rename = "dateNextRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_next_run: Option<String>,
    /// <p>The date on which this image pipeline was last updated.</p>
    #[serde(rename = "dateUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// <p>The description of the image pipeline.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the distribution configuration associated with this image pipeline.</p>
    #[serde(rename = "distributionConfigurationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution_configuration_arn: Option<String>,
    /// <p> Collects additional information about the image being created, including the operating system (OS) version and package list. This information is used to enhance the overall experience of using EC2 Image Builder. Enabled by default. </p>
    #[serde(rename = "enhancedImageMetadataEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enhanced_image_metadata_enabled: Option<bool>,
    /// <p>The Amazon Resource Name (ARN) of the image recipe associated with this image pipeline.</p>
    #[serde(rename = "imageRecipeArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_recipe_arn: Option<String>,
    /// <p>The image tests configuration of the image pipeline.</p>
    #[serde(rename = "imageTestsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_tests_configuration: Option<ImageTestsConfiguration>,
    /// <p>The Amazon Resource Name (ARN) of the infrastructure configuration associated with this image pipeline.</p>
    #[serde(rename = "infrastructureConfigurationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infrastructure_configuration_arn: Option<String>,
    /// <p>The name of the image pipeline.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The platform of the image pipeline.</p>
    #[serde(rename = "platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// <p>The schedule of the image pipeline.</p>
    #[serde(rename = "schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Schedule>,
    /// <p>The status of the image pipeline.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The tags of this image pipeline.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>An image recipe.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ImageRecipe {
    /// <p>The Amazon Resource Name (ARN) of the image recipe.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The block device mappings to apply when creating images from this recipe.</p>
    #[serde(rename = "blockDeviceMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_device_mappings: Option<Vec<InstanceBlockDeviceMapping>>,
    /// <p>The components of the image recipe.</p>
    #[serde(rename = "components")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<ComponentConfiguration>>,
    /// <p>The date on which this image recipe was created.</p>
    #[serde(rename = "dateCreated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// <p>The description of the image recipe.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the image recipe.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The owner of the image recipe.</p>
    #[serde(rename = "owner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// <p>The parent image of the image recipe.</p>
    #[serde(rename = "parentImage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_image: Option<String>,
    /// <p>The platform of the image recipe.</p>
    #[serde(rename = "platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// <p>The tags of the image recipe.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The version of the image recipe.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// <p>The working directory to be used during build and test workflows.</p>
    #[serde(rename = "workingDirectory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_directory: Option<String>,
}

/// <p>A summary of an image recipe.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ImageRecipeSummary {
    /// <p>The Amazon Resource Name (ARN) of the image recipe.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The date on which this image recipe was created.</p>
    #[serde(rename = "dateCreated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// <p>The name of the image recipe.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The owner of the image recipe.</p>
    #[serde(rename = "owner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// <p>The parent image of the image recipe.</p>
    #[serde(rename = "parentImage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_image: Option<String>,
    /// <p>The platform of the image recipe.</p>
    #[serde(rename = "platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// <p>The tags of the image recipe.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p> Image state shows the image status and the reason for that status. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ImageState {
    /// <p>The reason for the image's status. </p>
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// <p>The status of the image. </p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>An image summary.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ImageSummary {
    /// <p>The Amazon Resource Name (ARN) of the image.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The date on which this image was created.</p>
    #[serde(rename = "dateCreated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// <p>The name of the image.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The operating system version of the instance. For example, Amazon Linux 2, Ubuntu 18, or Microsoft Windows Server 2019. </p>
    #[serde(rename = "osVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    /// <p>The output resources produced when creating this image.</p>
    #[serde(rename = "outputResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_resources: Option<OutputResources>,
    /// <p>The owner of the image.</p>
    #[serde(rename = "owner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// <p>The platform of the image.</p>
    #[serde(rename = "platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// <p>The state of the image.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<ImageState>,
    /// <p>The tags of the image.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The version of the image.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>Image tests configuration.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ImageTestsConfiguration {
    /// <p>Defines if tests should be executed when building this image.</p>
    #[serde(rename = "imageTestsEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_tests_enabled: Option<bool>,
    /// <p>The maximum time in minutes that tests are permitted to run.</p>
    #[serde(rename = "timeoutMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_minutes: Option<i64>,
}

/// <p>An image semantic version.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ImageVersion {
    /// <p>The Amazon Resource Name (ARN) of the image semantic version.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The date at which this image semantic version was created.</p>
    #[serde(rename = "dateCreated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// <p>The name of the image semantic version.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p> The operating system version of the instance. For example, Amazon Linux 2, Ubuntu 18, or Microsoft Windows Server 2019. </p>
    #[serde(rename = "osVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    /// <p>The owner of the image semantic version.</p>
    #[serde(rename = "owner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// <p>The platform of the image semantic version.</p>
    #[serde(rename = "platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// <p>The semantic version of the image semantic version.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ImportComponentRequest {
    /// <p>The change description of the component. Describes what change has been made in this version, or what makes this version different from other versions of this component. </p>
    #[serde(rename = "changeDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_description: Option<String>,
    /// <p>The idempotency token of the component. </p>
    #[serde(rename = "clientToken")]
    pub client_token: String,
    /// <p>The data of the component. Used to specify the data inline. Either <code>data</code> or <code>uri</code> can be used to specify the data within the component.</p>
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    /// <p>The description of the component. Describes the contents of the component. </p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The format of the resource that you want to import as a component. </p>
    #[serde(rename = "format")]
    pub format: String,
    /// <p>The ID of the KMS key that should be used to encrypt this component. </p>
    #[serde(rename = "kmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p> The name of the component. </p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The platform of the component. </p>
    #[serde(rename = "platform")]
    pub platform: String,
    /// <p>The semantic version of the component. This version follows the semantic version syntax. For example, major.minor.patch. This could be versioned like software (2.0.1) or like a date (2019.12.01).</p>
    #[serde(rename = "semanticVersion")]
    pub semantic_version: String,
    /// <p>The tags of the component. </p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The type of the component denotes whether the component is used to build the image or only to test it. </p>
    #[serde(rename = "type")]
    pub type_: String,
    /// <p>The uri of the component. Must be an S3 URL and the requester must have permission to access the S3 bucket. If you use S3, you can specify component content up to your service quota. Either <code>data</code> or <code>uri</code> can be used to specify the data within the component. </p>
    #[serde(rename = "uri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ImportComponentResponse {
    /// <p>The idempotency token used to make this request idempotent. </p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the imported component. </p>
    #[serde(rename = "componentBuildVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_build_version_arn: Option<String>,
    /// <p>The request ID that uniquely identifies this request. </p>
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// <p>Details of the infrastructure configuration.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InfrastructureConfiguration {
    /// <p>The Amazon Resource Name (ARN) of the infrastructure configuration.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The date on which the infrastructure configuration was created.</p>
    #[serde(rename = "dateCreated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// <p>The date on which the infrastructure configuration was last updated.</p>
    #[serde(rename = "dateUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// <p>The description of the infrastructure configuration.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The instance profile of the infrastructure configuration.</p>
    #[serde(rename = "instanceProfileName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_profile_name: Option<String>,
    /// <p>The instance types of the infrastructure configuration.</p>
    #[serde(rename = "instanceTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_types: Option<Vec<String>>,
    /// <p>The EC2 key pair of the infrastructure configuration.</p>
    #[serde(rename = "keyPair")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_pair: Option<String>,
    /// <p>The logging configuration of the infrastructure configuration.</p>
    #[serde(rename = "logging")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<Logging>,
    /// <p>The name of the infrastructure configuration.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The tags attached to the resource created by Image Builder.</p>
    #[serde(rename = "resourceTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The security group IDs of the infrastructure configuration.</p>
    #[serde(rename = "securityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    /// <p>The SNS topic Amazon Resource Name (ARN) of the infrastructure configuration.</p>
    #[serde(rename = "snsTopicArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arn: Option<String>,
    /// <p>The subnet ID of the infrastructure configuration.</p>
    #[serde(rename = "subnetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    /// <p>The tags of the infrastructure configuration.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The terminate instance on failure configuration of the infrastructure configuration.</p>
    #[serde(rename = "terminateInstanceOnFailure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminate_instance_on_failure: Option<bool>,
}

/// <p>The infrastructure used when building EC2 AMIs.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InfrastructureConfigurationSummary {
    /// <p>The Amazon Resource Name (ARN) of the infrastructure configuration.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The date on which the infrastructure configuration was created.</p>
    #[serde(rename = "dateCreated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// <p>The date on which the infrastructure configuration was last updated.</p>
    #[serde(rename = "dateUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// <p>The description of the infrastructure configuration.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the infrastructure configuration.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The tags attached to the image created by Image Builder.</p>
    #[serde(rename = "resourceTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The tags of the infrastructure configuration.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Defines block device mappings for the instance used to configure your image.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct InstanceBlockDeviceMapping {
    /// <p>The device to which these mappings apply.</p>
    #[serde(rename = "deviceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    /// <p>Use to manage Amazon EBS-specific configuration for this mapping.</p>
    #[serde(rename = "ebs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs: Option<EbsInstanceBlockDeviceSpecification>,
    /// <p>Use to remove a mapping from the parent image.</p>
    #[serde(rename = "noDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_device: Option<String>,
    /// <p>Use to manage instance ephemeral devices.</p>
    #[serde(rename = "virtualName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_name: Option<String>,
}

/// <p>Describes the configuration for a launch permission. The launch permission modification request is sent to the <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_ModifyImageAttribute.html">EC2 ModifyImageAttribute</a> API on behalf of the user for each Region they have selected to distribute the AMI. To make an AMI public, set the launch permission authorized accounts to <code>all</code>. See the examples for making an AMI public at <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_ModifyImageAttribute.html">EC2 ModifyImageAttribute</a>. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct LaunchPermissionConfiguration {
    /// <p>The name of the group. </p>
    #[serde(rename = "userGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_groups: Option<Vec<String>>,
    /// <p>The AWS account ID. </p>
    #[serde(rename = "userIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListComponentBuildVersionsRequest {
    /// <p>The component version Amazon Resource Name (ARN) whose versions you want to list. </p>
    #[serde(rename = "componentVersionArn")]
    pub component_version_arn: String,
    /// <p>The maximum items to return in a request. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token to specify where to start paginating. This is the NextToken from a previously truncated response. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListComponentBuildVersionsResponse {
    /// <p>The list of component summaries for the specified semantic version. </p>
    #[serde(rename = "componentSummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_summary_list: Option<Vec<ComponentSummary>>,
    /// <p>The next token used for paginated responses. When this is not empty, there are additional elements that the service has not included in this request. Use this token with the next request to retrieve additional objects. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The request ID that uniquely identifies this request. </p>
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListComponentsRequest {
    /// <p>The filters. </p>
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>The maximum items to return in a request. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token to specify where to start paginating. This is the NextToken from a previously truncated response. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The owner defines which components you want to list. By default, this request will only show components owned by your account. You can use this field to specify if you want to view components owned by yourself, by Amazon, or those components that have been shared with you by other customers. </p>
    #[serde(rename = "owner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListComponentsResponse {
    /// <p>The list of component semantic versions. </p>
    #[serde(rename = "componentVersionList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_version_list: Option<Vec<ComponentVersion>>,
    /// <p>The next token used for paginated responses. When this is not empty, there are additional elements that the service has not included in this request. Use this token with the next request to retrieve additional objects. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The request ID that uniquely identifies this request. </p>
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListDistributionConfigurationsRequest {
    /// <p>The filters. </p>
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>The maximum items to return in a request. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token to specify where to start paginating. This is the NextToken from a previously truncated response. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListDistributionConfigurationsResponse {
    /// <p>The list of distributions. </p>
    #[serde(rename = "distributionConfigurationSummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution_configuration_summary_list: Option<Vec<DistributionConfigurationSummary>>,
    /// <p>The next token used for paginated responses. When this is not empty, there are additional elements that the service has not included in this request. Use this token with the next request to retrieve additional objects. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The request ID that uniquely identifies this request. </p>
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListImageBuildVersionsRequest {
    /// <p>The filters. </p>
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>The Amazon Resource Name (ARN) of the image whose build versions you want to retrieve. </p>
    #[serde(rename = "imageVersionArn")]
    pub image_version_arn: String,
    /// <p>The maximum items to return in a request. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token to specify where to start paginating. This is the NextToken from a previously truncated response. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListImageBuildVersionsResponse {
    /// <p>The list of image build versions. </p>
    #[serde(rename = "imageSummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_summary_list: Option<Vec<ImageSummary>>,
    /// <p>The next token used for paginated responses. When this is not empty, there are additional elements that the service has not included in this request. Use this token with the next request to retrieve additional objects. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The request ID that uniquely identifies this request. </p>
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListImagePipelineImagesRequest {
    /// <p>The filters. </p>
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>The Amazon Resource Name (ARN) of the image pipeline whose images you want to view. </p>
    #[serde(rename = "imagePipelineArn")]
    pub image_pipeline_arn: String,
    /// <p>The maximum items to return in a request. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token to specify where to start paginating. This is the NextToken from a previously truncated response. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListImagePipelineImagesResponse {
    /// <p>The list of images built by this pipeline. </p>
    #[serde(rename = "imageSummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_summary_list: Option<Vec<ImageSummary>>,
    /// <p>The next token used for paginated responses. When this is not empty, there are additional elements that the service has not included in this request. Use this token with the next request to retrieve additional objects. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The request ID that uniquely identifies this request. </p>
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListImagePipelinesRequest {
    /// <p>The filters. </p>
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>The maximum items to return in a request. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token to specify where to start paginating. This is the NextToken from a previously truncated response. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListImagePipelinesResponse {
    /// <p>The list of image pipelines. </p>
    #[serde(rename = "imagePipelineList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_pipeline_list: Option<Vec<ImagePipeline>>,
    /// <p>The next token used for paginated responses. When this is not empty, there are additional elements that the service has not included in this request. Use this token with the next request to retrieve additional objects. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The request ID that uniquely identifies this request. </p>
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListImageRecipesRequest {
    /// <p>The filters. </p>
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>The maximum items to return in a request. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token to specify where to start paginating. This is the NextToken from a previously truncated response. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The owner defines which image recipes you want to list. By default, this request will only show image recipes owned by your account. You can use this field to specify if you want to view image recipes owned by yourself, by Amazon, or those image recipes that have been shared with you by other customers. </p>
    #[serde(rename = "owner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListImageRecipesResponse {
    /// <p>The list of image pipelines. </p>
    #[serde(rename = "imageRecipeSummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_recipe_summary_list: Option<Vec<ImageRecipeSummary>>,
    /// <p>The next token used for paginated responses. When this is not empty, there are additional elements that the service has not included in this request. Use this token with the next request to retrieve additional objects. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The request ID that uniquely identifies this request. </p>
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListImagesRequest {
    /// <p>The filters. </p>
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>The maximum items to return in a request. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token to specify where to start paginating. This is the NextToken from a previously truncated response. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The owner defines which images you want to list. By default, this request will only show images owned by your account. You can use this field to specify if you want to view images owned by yourself, by Amazon, or those images that have been shared with you by other customers. </p>
    #[serde(rename = "owner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListImagesResponse {
    /// <p>The list of image semantic versions. </p>
    #[serde(rename = "imageVersionList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_version_list: Option<Vec<ImageVersion>>,
    /// <p>The next token used for paginated responses. When this is not empty, there are additional elements that the service has not included in this request. Use this token with the next request to retrieve additional objects. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The request ID that uniquely identifies this request. </p>
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListInfrastructureConfigurationsRequest {
    /// <p>The filters. </p>
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>The maximum items to return in a request. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token to specify where to start paginating. This is the NextToken from a previously truncated response. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListInfrastructureConfigurationsResponse {
    /// <p>The list of infrastructure configurations. </p>
    #[serde(rename = "infrastructureConfigurationSummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infrastructure_configuration_summary_list: Option<Vec<InfrastructureConfigurationSummary>>,
    /// <p>The next token used for paginated responses. When this is not empty, there are additional elements that the service has not included in this request. Use this token with the next request to retrieve additional objects.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The request ID that uniquely identifies this request. </p>
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource whose tags you want to retrieve. </p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>The tags for the specified resource. </p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Logging configuration defines where Image Builder uploads your logs.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Logging {
    /// <p>The Amazon S3 logging configuration.</p>
    #[serde(rename = "s3Logs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_3_logs: Option<S3Logs>,
}

/// <p>The resources produced by this image. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct OutputResources {
    /// <p>The EC2 AMIs created by this image. </p>
    #[serde(rename = "amis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amis: Option<Vec<Ami>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutComponentPolicyRequest {
    /// <p>The Amazon Resource Name (ARN) of the component that this policy should be applied to. </p>
    #[serde(rename = "componentArn")]
    pub component_arn: String,
    /// <p>The policy to apply. </p>
    #[serde(rename = "policy")]
    pub policy: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutComponentPolicyResponse {
    /// <p>The Amazon Resource Name (ARN) of the component that this policy was applied to. </p>
    #[serde(rename = "componentArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_arn: Option<String>,
    /// <p>The request ID that uniquely identifies this request. </p>
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutImagePolicyRequest {
    /// <p>The Amazon Resource Name (ARN) of the image that this policy should be applied to. </p>
    #[serde(rename = "imageArn")]
    pub image_arn: String,
    /// <p>The policy to apply. </p>
    #[serde(rename = "policy")]
    pub policy: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutImagePolicyResponse {
    /// <p>The Amazon Resource Name (ARN) of the image that this policy was applied to. </p>
    #[serde(rename = "imageArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_arn: Option<String>,
    /// <p>The request ID that uniquely identifies this request. </p>
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutImageRecipePolicyRequest {
    /// <p>The Amazon Resource Name (ARN) of the image recipe that this policy should be applied to. </p>
    #[serde(rename = "imageRecipeArn")]
    pub image_recipe_arn: String,
    /// <p>The policy to apply. </p>
    #[serde(rename = "policy")]
    pub policy: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutImageRecipePolicyResponse {
    /// <p>The Amazon Resource Name (ARN) of the image recipe that this policy was applied to. </p>
    #[serde(rename = "imageRecipeArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_recipe_arn: Option<String>,
    /// <p>The request ID that uniquely identifies this request. </p>
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// <p>Amazon S3 logging configuration.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct S3Logs {
    /// <p>The Amazon S3 bucket in which to store the logs.</p>
    #[serde(rename = "s3BucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_3_bucket_name: Option<String>,
    /// <p>The Amazon S3 path in which to store the logs.</p>
    #[serde(rename = "s3KeyPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_3_key_prefix: Option<String>,
}

/// <p>A schedule configures how often and when a pipeline will automatically create a new image. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Schedule {
    /// <p>The condition configures when the pipeline should trigger a new image build. When the <code>pipelineExecutionStartCondition</code> is set to <code>EXPRESSION_MATCH_AND_DEPENDENCY_UPDATES_AVAILABLE</code>, EC2 Image Builder will build a new image only when there are known changes pending. When it is set to <code>EXPRESSION_MATCH_ONLY</code>, it will build a new image every time the CRON expression matches the current time.</p>
    #[serde(rename = "pipelineExecutionStartCondition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_execution_start_condition: Option<String>,
    /// <p>The expression determines how often EC2 Image Builder evaluates your <code>pipelineExecutionStartCondition</code>.</p>
    #[serde(rename = "scheduleExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartImagePipelineExecutionRequest {
    /// <p>The idempotency token used to make this request idempotent. </p>
    #[serde(rename = "clientToken")]
    pub client_token: String,
    /// <p>The Amazon Resource Name (ARN) of the image pipeline that you want to manually invoke. </p>
    #[serde(rename = "imagePipelineArn")]
    pub image_pipeline_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartImagePipelineExecutionResponse {
    /// <p>The idempotency token used to make this request idempotent.</p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the image that was created by this request.</p>
    #[serde(rename = "imageBuildVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_build_version_arn: Option<String>,
    /// <p>The request ID that uniquely identifies this request. </p>
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource that you want to tag. </p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The tags to apply to the resource. </p>
    #[serde(rename = "tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource that you want to untag. </p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The tag keys to remove from the resource. </p>
    #[serde(rename = "tagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateDistributionConfigurationRequest {
    /// <p>The idempotency token of the distribution configuration. </p>
    #[serde(rename = "clientToken")]
    pub client_token: String,
    /// <p>The description of the distribution configuration. </p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the distribution configuration that you want to update. </p>
    #[serde(rename = "distributionConfigurationArn")]
    pub distribution_configuration_arn: String,
    /// <p>The distributions of the distribution configuration. </p>
    #[serde(rename = "distributions")]
    pub distributions: Vec<Distribution>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateDistributionConfigurationResponse {
    /// <p>The idempotency token used to make this request idempotent. </p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the distribution configuration that was updated by this request. </p>
    #[serde(rename = "distributionConfigurationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution_configuration_arn: Option<String>,
    /// <p>The request ID that uniquely identifies this request. </p>
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateImagePipelineRequest {
    /// <p>The idempotency token used to make this request idempotent. </p>
    #[serde(rename = "clientToken")]
    pub client_token: String,
    /// <p>The description of the image pipeline. </p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the distribution configuration that will be used to configure and distribute images updated by this image pipeline. </p>
    #[serde(rename = "distributionConfigurationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution_configuration_arn: Option<String>,
    /// <p> Collects additional information about the image being created, including the operating system (OS) version and package list. This information is used to enhance the overall experience of using EC2 Image Builder. Enabled by default. </p>
    #[serde(rename = "enhancedImageMetadataEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enhanced_image_metadata_enabled: Option<bool>,
    /// <p>The Amazon Resource Name (ARN) of the image pipeline that you want to update. </p>
    #[serde(rename = "imagePipelineArn")]
    pub image_pipeline_arn: String,
    /// <p>The Amazon Resource Name (ARN) of the image recipe that will be used to configure images updated by this image pipeline. </p>
    #[serde(rename = "imageRecipeArn")]
    pub image_recipe_arn: String,
    /// <p>The image test configuration of the image pipeline. </p>
    #[serde(rename = "imageTestsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_tests_configuration: Option<ImageTestsConfiguration>,
    /// <p>The Amazon Resource Name (ARN) of the infrastructure configuration that will be used to build images updated by this image pipeline. </p>
    #[serde(rename = "infrastructureConfigurationArn")]
    pub infrastructure_configuration_arn: String,
    /// <p>The schedule of the image pipeline. </p>
    #[serde(rename = "schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Schedule>,
    /// <p>The status of the image pipeline. </p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateImagePipelineResponse {
    /// <p>The idempotency token used to make this request idempotent. </p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the image pipeline that was updated by this request. </p>
    #[serde(rename = "imagePipelineArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_pipeline_arn: Option<String>,
    /// <p>The request ID that uniquely identifies this request. </p>
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateInfrastructureConfigurationRequest {
    /// <p>The idempotency token used to make this request idempotent. </p>
    #[serde(rename = "clientToken")]
    pub client_token: String,
    /// <p>The description of the infrastructure configuration. </p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the infrastructure configuration that you want to update. </p>
    #[serde(rename = "infrastructureConfigurationArn")]
    pub infrastructure_configuration_arn: String,
    /// <p>The instance profile to associate with the instance used to customize your EC2 AMI. </p>
    #[serde(rename = "instanceProfileName")]
    pub instance_profile_name: String,
    /// <p>The instance types of the infrastructure configuration. You can specify one or more instance types to use for this build. The service will pick one of these instance types based on availability. </p>
    #[serde(rename = "instanceTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_types: Option<Vec<String>>,
    /// <p>The key pair of the infrastructure configuration. This can be used to log on to and debug the instance used to create your image. </p>
    #[serde(rename = "keyPair")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_pair: Option<String>,
    /// <p>The logging configuration of the infrastructure configuration. </p>
    #[serde(rename = "logging")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<Logging>,
    /// <p>The tags attached to the resource created by Image Builder.</p>
    #[serde(rename = "resourceTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The security group IDs to associate with the instance used to customize your EC2 AMI. </p>
    #[serde(rename = "securityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    /// <p>The SNS topic on which to send image build events. </p>
    #[serde(rename = "snsTopicArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arn: Option<String>,
    /// <p>The subnet ID to place the instance used to customize your EC2 AMI in. </p>
    #[serde(rename = "subnetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    /// <p>The terminate instance on failure setting of the infrastructure configuration. Set to false if you want Image Builder to retain the instance used to configure your AMI if the build or test phase of your workflow fails. </p>
    #[serde(rename = "terminateInstanceOnFailure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminate_instance_on_failure: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateInfrastructureConfigurationResponse {
    /// <p>The idempotency token used to make this request idempotent. </p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the infrastructure configuration that was updated by this request. </p>
    #[serde(rename = "infrastructureConfigurationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infrastructure_configuration_arn: Option<String>,
    /// <p>The request ID that uniquely identifies this request. </p>
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// Errors returned by CancelImageCreation
#[derive(Debug, PartialEq)]
pub enum CancelImageCreationError {
    /// <p>You have exceeded the permitted request rate for the specific operation.</p>
    CallRateLimitExceeded(String),
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an invalid resource identifier.</p>
    Client(String),
    /// <p>You are not authorized to perform the requested operation.</p>
    Forbidden(String),
    /// <p>You have specified a client token for an operation using parameter values that differ from a previous request that used the same client token.</p>
    IdempotentParameterMismatch(String),
    /// <p>You have made a request for an action that is not supported by the service.</p>
    InvalidRequest(String),
    /// <p>The resource that you are trying to operate on is currently in use. Review the message details and retry later.</p>
    ResourceInUse(String),
    /// <p>This exception is thrown when the service encounters an unrecoverable exception.</p>
    Service(String),
    /// <p>The service is unable to process your request at this time.</p>
    ServiceUnavailable(String),
}

impl CancelImageCreationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CancelImageCreationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "CallRateLimitExceededException" => {
                    return RusotoError::Service(CancelImageCreationError::CallRateLimitExceeded(
                        err.msg,
                    ))
                }
                "ClientException" => {
                    return RusotoError::Service(CancelImageCreationError::Client(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CancelImageCreationError::Forbidden(err.msg))
                }
                "IdempotentParameterMismatchException" => {
                    return RusotoError::Service(
                        CancelImageCreationError::IdempotentParameterMismatch(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CancelImageCreationError::InvalidRequest(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(CancelImageCreationError::ResourceInUse(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(CancelImageCreationError::Service(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CancelImageCreationError::ServiceUnavailable(
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
impl fmt::Display for CancelImageCreationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CancelImageCreationError::CallRateLimitExceeded(ref cause) => write!(f, "{}", cause),
            CancelImageCreationError::Client(ref cause) => write!(f, "{}", cause),
            CancelImageCreationError::Forbidden(ref cause) => write!(f, "{}", cause),
            CancelImageCreationError::IdempotentParameterMismatch(ref cause) => {
                write!(f, "{}", cause)
            }
            CancelImageCreationError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            CancelImageCreationError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            CancelImageCreationError::Service(ref cause) => write!(f, "{}", cause),
            CancelImageCreationError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CancelImageCreationError {}
/// Errors returned by CreateComponent
#[derive(Debug, PartialEq)]
pub enum CreateComponentError {
    /// <p>You have exceeded the permitted request rate for the specific operation.</p>
    CallRateLimitExceeded(String),
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an invalid resource identifier.</p>
    Client(String),
    /// <p>You are not authorized to perform the requested operation.</p>
    Forbidden(String),
    /// <p>You have specified a client token for an operation using parameter values that differ from a previous request that used the same client token.</p>
    IdempotentParameterMismatch(String),
    /// <p>You have specified two or more mutually exclusive parameters. Review the error message for details.</p>
    InvalidParameterCombination(String),
    /// <p>You have made a request for an action that is not supported by the service.</p>
    InvalidRequest(String),
    /// <p>Your version number is out of bounds or does not follow the required syntax.</p>
    InvalidVersionNumber(String),
    /// <p>The resource that you are trying to operate on is currently in use. Review the message details and retry later.</p>
    ResourceInUse(String),
    /// <p>This exception is thrown when the service encounters an unrecoverable exception.</p>
    Service(String),
    /// <p>You have exceeded the number of permitted resources or operations for this service. For service quotas, see <a href="https://docs.aws.amazon.com/general/latest/gr/imagebuilder.html#limits_imagebuilder">EC2 Image Builder endpoints and quotas</a>.</p>
    ServiceQuotaExceeded(String),
    /// <p>The service is unable to process your request at this time.</p>
    ServiceUnavailable(String),
}

impl CreateComponentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateComponentError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "CallRateLimitExceededException" => {
                    return RusotoError::Service(CreateComponentError::CallRateLimitExceeded(
                        err.msg,
                    ))
                }
                "ClientException" => {
                    return RusotoError::Service(CreateComponentError::Client(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateComponentError::Forbidden(err.msg))
                }
                "IdempotentParameterMismatchException" => {
                    return RusotoError::Service(CreateComponentError::IdempotentParameterMismatch(
                        err.msg,
                    ))
                }
                "InvalidParameterCombinationException" => {
                    return RusotoError::Service(CreateComponentError::InvalidParameterCombination(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateComponentError::InvalidRequest(err.msg))
                }
                "InvalidVersionNumberException" => {
                    return RusotoError::Service(CreateComponentError::InvalidVersionNumber(
                        err.msg,
                    ))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(CreateComponentError::ResourceInUse(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(CreateComponentError::Service(err.msg))
                }
                "ServiceQuotaExceededException" => {
                    return RusotoError::Service(CreateComponentError::ServiceQuotaExceeded(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateComponentError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateComponentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateComponentError::CallRateLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateComponentError::Client(ref cause) => write!(f, "{}", cause),
            CreateComponentError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreateComponentError::IdempotentParameterMismatch(ref cause) => write!(f, "{}", cause),
            CreateComponentError::InvalidParameterCombination(ref cause) => write!(f, "{}", cause),
            CreateComponentError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            CreateComponentError::InvalidVersionNumber(ref cause) => write!(f, "{}", cause),
            CreateComponentError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            CreateComponentError::Service(ref cause) => write!(f, "{}", cause),
            CreateComponentError::ServiceQuotaExceeded(ref cause) => write!(f, "{}", cause),
            CreateComponentError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateComponentError {}
/// Errors returned by CreateDistributionConfiguration
#[derive(Debug, PartialEq)]
pub enum CreateDistributionConfigurationError {
    /// <p>You have exceeded the permitted request rate for the specific operation.</p>
    CallRateLimitExceeded(String),
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an invalid resource identifier.</p>
    Client(String),
    /// <p>You are not authorized to perform the requested operation.</p>
    Forbidden(String),
    /// <p>You have specified a client token for an operation using parameter values that differ from a previous request that used the same client token.</p>
    IdempotentParameterMismatch(String),
    /// <p>You have specified two or more mutually exclusive parameters. Review the error message for details.</p>
    InvalidParameterCombination(String),
    /// <p>You have made a request for an action that is not supported by the service.</p>
    InvalidRequest(String),
    /// <p>The resource that you are trying to create already exists.</p>
    ResourceAlreadyExists(String),
    /// <p>The resource that you are trying to operate on is currently in use. Review the message details and retry later.</p>
    ResourceInUse(String),
    /// <p>This exception is thrown when the service encounters an unrecoverable exception.</p>
    Service(String),
    /// <p>You have exceeded the number of permitted resources or operations for this service. For service quotas, see <a href="https://docs.aws.amazon.com/general/latest/gr/imagebuilder.html#limits_imagebuilder">EC2 Image Builder endpoints and quotas</a>.</p>
    ServiceQuotaExceeded(String),
    /// <p>The service is unable to process your request at this time.</p>
    ServiceUnavailable(String),
}

impl CreateDistributionConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateDistributionConfigurationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "CallRateLimitExceededException" => {
                    return RusotoError::Service(
                        CreateDistributionConfigurationError::CallRateLimitExceeded(err.msg),
                    )
                }
                "ClientException" => {
                    return RusotoError::Service(CreateDistributionConfigurationError::Client(
                        err.msg,
                    ))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateDistributionConfigurationError::Forbidden(
                        err.msg,
                    ))
                }
                "IdempotentParameterMismatchException" => {
                    return RusotoError::Service(
                        CreateDistributionConfigurationError::IdempotentParameterMismatch(err.msg),
                    )
                }
                "InvalidParameterCombinationException" => {
                    return RusotoError::Service(
                        CreateDistributionConfigurationError::InvalidParameterCombination(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        CreateDistributionConfigurationError::InvalidRequest(err.msg),
                    )
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(
                        CreateDistributionConfigurationError::ResourceAlreadyExists(err.msg),
                    )
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(
                        CreateDistributionConfigurationError::ResourceInUse(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(CreateDistributionConfigurationError::Service(
                        err.msg,
                    ))
                }
                "ServiceQuotaExceededException" => {
                    return RusotoError::Service(
                        CreateDistributionConfigurationError::ServiceQuotaExceeded(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        CreateDistributionConfigurationError::ServiceUnavailable(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateDistributionConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateDistributionConfigurationError::CallRateLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDistributionConfigurationError::Client(ref cause) => write!(f, "{}", cause),
            CreateDistributionConfigurationError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreateDistributionConfigurationError::IdempotentParameterMismatch(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDistributionConfigurationError::InvalidParameterCombination(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDistributionConfigurationError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDistributionConfigurationError::ResourceAlreadyExists(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDistributionConfigurationError::ResourceInUse(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDistributionConfigurationError::Service(ref cause) => write!(f, "{}", cause),
            CreateDistributionConfigurationError::ServiceQuotaExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDistributionConfigurationError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateDistributionConfigurationError {}
/// Errors returned by CreateImage
#[derive(Debug, PartialEq)]
pub enum CreateImageError {
    /// <p>You have exceeded the permitted request rate for the specific operation.</p>
    CallRateLimitExceeded(String),
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an invalid resource identifier.</p>
    Client(String),
    /// <p>You are not authorized to perform the requested operation.</p>
    Forbidden(String),
    /// <p>You have specified a client token for an operation using parameter values that differ from a previous request that used the same client token.</p>
    IdempotentParameterMismatch(String),
    /// <p>You have made a request for an action that is not supported by the service.</p>
    InvalidRequest(String),
    /// <p>The resource that you are trying to operate on is currently in use. Review the message details and retry later.</p>
    ResourceInUse(String),
    /// <p>This exception is thrown when the service encounters an unrecoverable exception.</p>
    Service(String),
    /// <p>You have exceeded the number of permitted resources or operations for this service. For service quotas, see <a href="https://docs.aws.amazon.com/general/latest/gr/imagebuilder.html#limits_imagebuilder">EC2 Image Builder endpoints and quotas</a>.</p>
    ServiceQuotaExceeded(String),
    /// <p>The service is unable to process your request at this time.</p>
    ServiceUnavailable(String),
}

impl CreateImageError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateImageError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "CallRateLimitExceededException" => {
                    return RusotoError::Service(CreateImageError::CallRateLimitExceeded(err.msg))
                }
                "ClientException" => {
                    return RusotoError::Service(CreateImageError::Client(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateImageError::Forbidden(err.msg))
                }
                "IdempotentParameterMismatchException" => {
                    return RusotoError::Service(CreateImageError::IdempotentParameterMismatch(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateImageError::InvalidRequest(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(CreateImageError::ResourceInUse(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(CreateImageError::Service(err.msg))
                }
                "ServiceQuotaExceededException" => {
                    return RusotoError::Service(CreateImageError::ServiceQuotaExceeded(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateImageError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateImageError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateImageError::CallRateLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateImageError::Client(ref cause) => write!(f, "{}", cause),
            CreateImageError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreateImageError::IdempotentParameterMismatch(ref cause) => write!(f, "{}", cause),
            CreateImageError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            CreateImageError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            CreateImageError::Service(ref cause) => write!(f, "{}", cause),
            CreateImageError::ServiceQuotaExceeded(ref cause) => write!(f, "{}", cause),
            CreateImageError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateImageError {}
/// Errors returned by CreateImagePipeline
#[derive(Debug, PartialEq)]
pub enum CreateImagePipelineError {
    /// <p>You have exceeded the permitted request rate for the specific operation.</p>
    CallRateLimitExceeded(String),
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an invalid resource identifier.</p>
    Client(String),
    /// <p>You are not authorized to perform the requested operation.</p>
    Forbidden(String),
    /// <p>You have specified a client token for an operation using parameter values that differ from a previous request that used the same client token.</p>
    IdempotentParameterMismatch(String),
    /// <p>You have made a request for an action that is not supported by the service.</p>
    InvalidRequest(String),
    /// <p>The resource that you are trying to create already exists.</p>
    ResourceAlreadyExists(String),
    /// <p>The resource that you are trying to operate on is currently in use. Review the message details and retry later.</p>
    ResourceInUse(String),
    /// <p>This exception is thrown when the service encounters an unrecoverable exception.</p>
    Service(String),
    /// <p>You have exceeded the number of permitted resources or operations for this service. For service quotas, see <a href="https://docs.aws.amazon.com/general/latest/gr/imagebuilder.html#limits_imagebuilder">EC2 Image Builder endpoints and quotas</a>.</p>
    ServiceQuotaExceeded(String),
    /// <p>The service is unable to process your request at this time.</p>
    ServiceUnavailable(String),
}

impl CreateImagePipelineError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateImagePipelineError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "CallRateLimitExceededException" => {
                    return RusotoError::Service(CreateImagePipelineError::CallRateLimitExceeded(
                        err.msg,
                    ))
                }
                "ClientException" => {
                    return RusotoError::Service(CreateImagePipelineError::Client(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateImagePipelineError::Forbidden(err.msg))
                }
                "IdempotentParameterMismatchException" => {
                    return RusotoError::Service(
                        CreateImagePipelineError::IdempotentParameterMismatch(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateImagePipelineError::InvalidRequest(err.msg))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(CreateImagePipelineError::ResourceAlreadyExists(
                        err.msg,
                    ))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(CreateImagePipelineError::ResourceInUse(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(CreateImagePipelineError::Service(err.msg))
                }
                "ServiceQuotaExceededException" => {
                    return RusotoError::Service(CreateImagePipelineError::ServiceQuotaExceeded(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateImagePipelineError::ServiceUnavailable(
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
impl fmt::Display for CreateImagePipelineError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateImagePipelineError::CallRateLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateImagePipelineError::Client(ref cause) => write!(f, "{}", cause),
            CreateImagePipelineError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreateImagePipelineError::IdempotentParameterMismatch(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateImagePipelineError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            CreateImagePipelineError::ResourceAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateImagePipelineError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            CreateImagePipelineError::Service(ref cause) => write!(f, "{}", cause),
            CreateImagePipelineError::ServiceQuotaExceeded(ref cause) => write!(f, "{}", cause),
            CreateImagePipelineError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateImagePipelineError {}
/// Errors returned by CreateImageRecipe
#[derive(Debug, PartialEq)]
pub enum CreateImageRecipeError {
    /// <p>You have exceeded the permitted request rate for the specific operation.</p>
    CallRateLimitExceeded(String),
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an invalid resource identifier.</p>
    Client(String),
    /// <p>You are not authorized to perform the requested operation.</p>
    Forbidden(String),
    /// <p>You have specified a client token for an operation using parameter values that differ from a previous request that used the same client token.</p>
    IdempotentParameterMismatch(String),
    /// <p>You have made a request for an action that is not supported by the service.</p>
    InvalidRequest(String),
    /// <p>Your version number is out of bounds or does not follow the required syntax.</p>
    InvalidVersionNumber(String),
    /// <p>The resource that you are trying to create already exists.</p>
    ResourceAlreadyExists(String),
    /// <p>The resource that you are trying to operate on is currently in use. Review the message details and retry later.</p>
    ResourceInUse(String),
    /// <p>This exception is thrown when the service encounters an unrecoverable exception.</p>
    Service(String),
    /// <p>You have exceeded the number of permitted resources or operations for this service. For service quotas, see <a href="https://docs.aws.amazon.com/general/latest/gr/imagebuilder.html#limits_imagebuilder">EC2 Image Builder endpoints and quotas</a>.</p>
    ServiceQuotaExceeded(String),
    /// <p>The service is unable to process your request at this time.</p>
    ServiceUnavailable(String),
}

impl CreateImageRecipeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateImageRecipeError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "CallRateLimitExceededException" => {
                    return RusotoError::Service(CreateImageRecipeError::CallRateLimitExceeded(
                        err.msg,
                    ))
                }
                "ClientException" => {
                    return RusotoError::Service(CreateImageRecipeError::Client(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateImageRecipeError::Forbidden(err.msg))
                }
                "IdempotentParameterMismatchException" => {
                    return RusotoError::Service(
                        CreateImageRecipeError::IdempotentParameterMismatch(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateImageRecipeError::InvalidRequest(err.msg))
                }
                "InvalidVersionNumberException" => {
                    return RusotoError::Service(CreateImageRecipeError::InvalidVersionNumber(
                        err.msg,
                    ))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(CreateImageRecipeError::ResourceAlreadyExists(
                        err.msg,
                    ))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(CreateImageRecipeError::ResourceInUse(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(CreateImageRecipeError::Service(err.msg))
                }
                "ServiceQuotaExceededException" => {
                    return RusotoError::Service(CreateImageRecipeError::ServiceQuotaExceeded(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateImageRecipeError::ServiceUnavailable(
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
impl fmt::Display for CreateImageRecipeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateImageRecipeError::CallRateLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateImageRecipeError::Client(ref cause) => write!(f, "{}", cause),
            CreateImageRecipeError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreateImageRecipeError::IdempotentParameterMismatch(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateImageRecipeError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            CreateImageRecipeError::InvalidVersionNumber(ref cause) => write!(f, "{}", cause),
            CreateImageRecipeError::ResourceAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateImageRecipeError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            CreateImageRecipeError::Service(ref cause) => write!(f, "{}", cause),
            CreateImageRecipeError::ServiceQuotaExceeded(ref cause) => write!(f, "{}", cause),
            CreateImageRecipeError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateImageRecipeError {}
/// Errors returned by CreateInfrastructureConfiguration
#[derive(Debug, PartialEq)]
pub enum CreateInfrastructureConfigurationError {
    /// <p>You have exceeded the permitted request rate for the specific operation.</p>
    CallRateLimitExceeded(String),
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an invalid resource identifier.</p>
    Client(String),
    /// <p>You are not authorized to perform the requested operation.</p>
    Forbidden(String),
    /// <p>You have specified a client token for an operation using parameter values that differ from a previous request that used the same client token.</p>
    IdempotentParameterMismatch(String),
    /// <p>You have made a request for an action that is not supported by the service.</p>
    InvalidRequest(String),
    /// <p>The resource that you are trying to create already exists.</p>
    ResourceAlreadyExists(String),
    /// <p>The resource that you are trying to operate on is currently in use. Review the message details and retry later.</p>
    ResourceInUse(String),
    /// <p>This exception is thrown when the service encounters an unrecoverable exception.</p>
    Service(String),
    /// <p>You have exceeded the number of permitted resources or operations for this service. For service quotas, see <a href="https://docs.aws.amazon.com/general/latest/gr/imagebuilder.html#limits_imagebuilder">EC2 Image Builder endpoints and quotas</a>.</p>
    ServiceQuotaExceeded(String),
    /// <p>The service is unable to process your request at this time.</p>
    ServiceUnavailable(String),
}

impl CreateInfrastructureConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateInfrastructureConfigurationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "CallRateLimitExceededException" => {
                    return RusotoError::Service(
                        CreateInfrastructureConfigurationError::CallRateLimitExceeded(err.msg),
                    )
                }
                "ClientException" => {
                    return RusotoError::Service(CreateInfrastructureConfigurationError::Client(
                        err.msg,
                    ))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateInfrastructureConfigurationError::Forbidden(
                        err.msg,
                    ))
                }
                "IdempotentParameterMismatchException" => {
                    return RusotoError::Service(
                        CreateInfrastructureConfigurationError::IdempotentParameterMismatch(
                            err.msg,
                        ),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        CreateInfrastructureConfigurationError::InvalidRequest(err.msg),
                    )
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(
                        CreateInfrastructureConfigurationError::ResourceAlreadyExists(err.msg),
                    )
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(
                        CreateInfrastructureConfigurationError::ResourceInUse(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(CreateInfrastructureConfigurationError::Service(
                        err.msg,
                    ))
                }
                "ServiceQuotaExceededException" => {
                    return RusotoError::Service(
                        CreateInfrastructureConfigurationError::ServiceQuotaExceeded(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        CreateInfrastructureConfigurationError::ServiceUnavailable(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateInfrastructureConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateInfrastructureConfigurationError::CallRateLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateInfrastructureConfigurationError::Client(ref cause) => write!(f, "{}", cause),
            CreateInfrastructureConfigurationError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreateInfrastructureConfigurationError::IdempotentParameterMismatch(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateInfrastructureConfigurationError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateInfrastructureConfigurationError::ResourceAlreadyExists(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateInfrastructureConfigurationError::ResourceInUse(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateInfrastructureConfigurationError::Service(ref cause) => write!(f, "{}", cause),
            CreateInfrastructureConfigurationError::ServiceQuotaExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateInfrastructureConfigurationError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateInfrastructureConfigurationError {}
/// Errors returned by DeleteComponent
#[derive(Debug, PartialEq)]
pub enum DeleteComponentError {
    /// <p>You have exceeded the permitted request rate for the specific operation.</p>
    CallRateLimitExceeded(String),
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an invalid resource identifier.</p>
    Client(String),
    /// <p>You are not authorized to perform the requested operation.</p>
    Forbidden(String),
    /// <p>You have made a request for an action that is not supported by the service.</p>
    InvalidRequest(String),
    /// <p>You have attempted to mutate or delete a resource with a dependency that prohibits this action. See the error message for more details.</p>
    ResourceDependency(String),
    /// <p>This exception is thrown when the service encounters an unrecoverable exception.</p>
    Service(String),
    /// <p>The service is unable to process your request at this time.</p>
    ServiceUnavailable(String),
}

impl DeleteComponentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteComponentError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "CallRateLimitExceededException" => {
                    return RusotoError::Service(DeleteComponentError::CallRateLimitExceeded(
                        err.msg,
                    ))
                }
                "ClientException" => {
                    return RusotoError::Service(DeleteComponentError::Client(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteComponentError::Forbidden(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteComponentError::InvalidRequest(err.msg))
                }
                "ResourceDependencyException" => {
                    return RusotoError::Service(DeleteComponentError::ResourceDependency(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(DeleteComponentError::Service(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteComponentError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteComponentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteComponentError::CallRateLimitExceeded(ref cause) => write!(f, "{}", cause),
            DeleteComponentError::Client(ref cause) => write!(f, "{}", cause),
            DeleteComponentError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteComponentError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DeleteComponentError::ResourceDependency(ref cause) => write!(f, "{}", cause),
            DeleteComponentError::Service(ref cause) => write!(f, "{}", cause),
            DeleteComponentError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteComponentError {}
/// Errors returned by DeleteDistributionConfiguration
#[derive(Debug, PartialEq)]
pub enum DeleteDistributionConfigurationError {
    /// <p>You have exceeded the permitted request rate for the specific operation.</p>
    CallRateLimitExceeded(String),
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an invalid resource identifier.</p>
    Client(String),
    /// <p>You are not authorized to perform the requested operation.</p>
    Forbidden(String),
    /// <p>You have made a request for an action that is not supported by the service.</p>
    InvalidRequest(String),
    /// <p>You have attempted to mutate or delete a resource with a dependency that prohibits this action. See the error message for more details.</p>
    ResourceDependency(String),
    /// <p>This exception is thrown when the service encounters an unrecoverable exception.</p>
    Service(String),
    /// <p>The service is unable to process your request at this time.</p>
    ServiceUnavailable(String),
}

impl DeleteDistributionConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteDistributionConfigurationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "CallRateLimitExceededException" => {
                    return RusotoError::Service(
                        DeleteDistributionConfigurationError::CallRateLimitExceeded(err.msg),
                    )
                }
                "ClientException" => {
                    return RusotoError::Service(DeleteDistributionConfigurationError::Client(
                        err.msg,
                    ))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteDistributionConfigurationError::Forbidden(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        DeleteDistributionConfigurationError::InvalidRequest(err.msg),
                    )
                }
                "ResourceDependencyException" => {
                    return RusotoError::Service(
                        DeleteDistributionConfigurationError::ResourceDependency(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(DeleteDistributionConfigurationError::Service(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        DeleteDistributionConfigurationError::ServiceUnavailable(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteDistributionConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteDistributionConfigurationError::CallRateLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteDistributionConfigurationError::Client(ref cause) => write!(f, "{}", cause),
            DeleteDistributionConfigurationError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteDistributionConfigurationError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteDistributionConfigurationError::ResourceDependency(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteDistributionConfigurationError::Service(ref cause) => write!(f, "{}", cause),
            DeleteDistributionConfigurationError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteDistributionConfigurationError {}
/// Errors returned by DeleteImage
#[derive(Debug, PartialEq)]
pub enum DeleteImageError {
    /// <p>You have exceeded the permitted request rate for the specific operation.</p>
    CallRateLimitExceeded(String),
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an invalid resource identifier.</p>
    Client(String),
    /// <p>You are not authorized to perform the requested operation.</p>
    Forbidden(String),
    /// <p>You have made a request for an action that is not supported by the service.</p>
    InvalidRequest(String),
    /// <p>You have attempted to mutate or delete a resource with a dependency that prohibits this action. See the error message for more details.</p>
    ResourceDependency(String),
    /// <p>This exception is thrown when the service encounters an unrecoverable exception.</p>
    Service(String),
    /// <p>The service is unable to process your request at this time.</p>
    ServiceUnavailable(String),
}

impl DeleteImageError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteImageError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "CallRateLimitExceededException" => {
                    return RusotoError::Service(DeleteImageError::CallRateLimitExceeded(err.msg))
                }
                "ClientException" => {
                    return RusotoError::Service(DeleteImageError::Client(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteImageError::Forbidden(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteImageError::InvalidRequest(err.msg))
                }
                "ResourceDependencyException" => {
                    return RusotoError::Service(DeleteImageError::ResourceDependency(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(DeleteImageError::Service(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteImageError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteImageError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteImageError::CallRateLimitExceeded(ref cause) => write!(f, "{}", cause),
            DeleteImageError::Client(ref cause) => write!(f, "{}", cause),
            DeleteImageError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteImageError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DeleteImageError::ResourceDependency(ref cause) => write!(f, "{}", cause),
            DeleteImageError::Service(ref cause) => write!(f, "{}", cause),
            DeleteImageError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteImageError {}
/// Errors returned by DeleteImagePipeline
#[derive(Debug, PartialEq)]
pub enum DeleteImagePipelineError {
    /// <p>You have exceeded the permitted request rate for the specific operation.</p>
    CallRateLimitExceeded(String),
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an invalid resource identifier.</p>
    Client(String),
    /// <p>You are not authorized to perform the requested operation.</p>
    Forbidden(String),
    /// <p>You have made a request for an action that is not supported by the service.</p>
    InvalidRequest(String),
    /// <p>You have attempted to mutate or delete a resource with a dependency that prohibits this action. See the error message for more details.</p>
    ResourceDependency(String),
    /// <p>This exception is thrown when the service encounters an unrecoverable exception.</p>
    Service(String),
    /// <p>The service is unable to process your request at this time.</p>
    ServiceUnavailable(String),
}

impl DeleteImagePipelineError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteImagePipelineError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "CallRateLimitExceededException" => {
                    return RusotoError::Service(DeleteImagePipelineError::CallRateLimitExceeded(
                        err.msg,
                    ))
                }
                "ClientException" => {
                    return RusotoError::Service(DeleteImagePipelineError::Client(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteImagePipelineError::Forbidden(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteImagePipelineError::InvalidRequest(err.msg))
                }
                "ResourceDependencyException" => {
                    return RusotoError::Service(DeleteImagePipelineError::ResourceDependency(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(DeleteImagePipelineError::Service(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteImagePipelineError::ServiceUnavailable(
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
impl fmt::Display for DeleteImagePipelineError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteImagePipelineError::CallRateLimitExceeded(ref cause) => write!(f, "{}", cause),
            DeleteImagePipelineError::Client(ref cause) => write!(f, "{}", cause),
            DeleteImagePipelineError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteImagePipelineError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DeleteImagePipelineError::ResourceDependency(ref cause) => write!(f, "{}", cause),
            DeleteImagePipelineError::Service(ref cause) => write!(f, "{}", cause),
            DeleteImagePipelineError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteImagePipelineError {}
/// Errors returned by DeleteImageRecipe
#[derive(Debug, PartialEq)]
pub enum DeleteImageRecipeError {
    /// <p>You have exceeded the permitted request rate for the specific operation.</p>
    CallRateLimitExceeded(String),
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an invalid resource identifier.</p>
    Client(String),
    /// <p>You are not authorized to perform the requested operation.</p>
    Forbidden(String),
    /// <p>You have made a request for an action that is not supported by the service.</p>
    InvalidRequest(String),
    /// <p>You have attempted to mutate or delete a resource with a dependency that prohibits this action. See the error message for more details.</p>
    ResourceDependency(String),
    /// <p>This exception is thrown when the service encounters an unrecoverable exception.</p>
    Service(String),
    /// <p>The service is unable to process your request at this time.</p>
    ServiceUnavailable(String),
}

impl DeleteImageRecipeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteImageRecipeError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "CallRateLimitExceededException" => {
                    return RusotoError::Service(DeleteImageRecipeError::CallRateLimitExceeded(
                        err.msg,
                    ))
                }
                "ClientException" => {
                    return RusotoError::Service(DeleteImageRecipeError::Client(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteImageRecipeError::Forbidden(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteImageRecipeError::InvalidRequest(err.msg))
                }
                "ResourceDependencyException" => {
                    return RusotoError::Service(DeleteImageRecipeError::ResourceDependency(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(DeleteImageRecipeError::Service(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteImageRecipeError::ServiceUnavailable(
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
impl fmt::Display for DeleteImageRecipeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteImageRecipeError::CallRateLimitExceeded(ref cause) => write!(f, "{}", cause),
            DeleteImageRecipeError::Client(ref cause) => write!(f, "{}", cause),
            DeleteImageRecipeError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteImageRecipeError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DeleteImageRecipeError::ResourceDependency(ref cause) => write!(f, "{}", cause),
            DeleteImageRecipeError::Service(ref cause) => write!(f, "{}", cause),
            DeleteImageRecipeError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteImageRecipeError {}
/// Errors returned by DeleteInfrastructureConfiguration
#[derive(Debug, PartialEq)]
pub enum DeleteInfrastructureConfigurationError {
    /// <p>You have exceeded the permitted request rate for the specific operation.</p>
    CallRateLimitExceeded(String),
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an invalid resource identifier.</p>
    Client(String),
    /// <p>You are not authorized to perform the requested operation.</p>
    Forbidden(String),
    /// <p>You have made a request for an action that is not supported by the service.</p>
    InvalidRequest(String),
    /// <p>You have attempted to mutate or delete a resource with a dependency that prohibits this action. See the error message for more details.</p>
    ResourceDependency(String),
    /// <p>This exception is thrown when the service encounters an unrecoverable exception.</p>
    Service(String),
    /// <p>The service is unable to process your request at this time.</p>
    ServiceUnavailable(String),
}

impl DeleteInfrastructureConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteInfrastructureConfigurationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "CallRateLimitExceededException" => {
                    return RusotoError::Service(
                        DeleteInfrastructureConfigurationError::CallRateLimitExceeded(err.msg),
                    )
                }
                "ClientException" => {
                    return RusotoError::Service(DeleteInfrastructureConfigurationError::Client(
                        err.msg,
                    ))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteInfrastructureConfigurationError::Forbidden(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        DeleteInfrastructureConfigurationError::InvalidRequest(err.msg),
                    )
                }
                "ResourceDependencyException" => {
                    return RusotoError::Service(
                        DeleteInfrastructureConfigurationError::ResourceDependency(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(DeleteInfrastructureConfigurationError::Service(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        DeleteInfrastructureConfigurationError::ServiceUnavailable(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteInfrastructureConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteInfrastructureConfigurationError::CallRateLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteInfrastructureConfigurationError::Client(ref cause) => write!(f, "{}", cause),
            DeleteInfrastructureConfigurationError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteInfrastructureConfigurationError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteInfrastructureConfigurationError::ResourceDependency(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteInfrastructureConfigurationError::Service(ref cause) => write!(f, "{}", cause),
            DeleteInfrastructureConfigurationError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteInfrastructureConfigurationError {}
/// Errors returned by GetComponent
#[derive(Debug, PartialEq)]
pub enum GetComponentError {
    /// <p>You have exceeded the permitted request rate for the specific operation.</p>
    CallRateLimitExceeded(String),
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an invalid resource identifier.</p>
    Client(String),
    /// <p>You are not authorized to perform the requested operation.</p>
    Forbidden(String),
    /// <p>You have made a request for an action that is not supported by the service.</p>
    InvalidRequest(String),
    /// <p>This exception is thrown when the service encounters an unrecoverable exception.</p>
    Service(String),
    /// <p>The service is unable to process your request at this time.</p>
    ServiceUnavailable(String),
}

impl GetComponentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetComponentError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "CallRateLimitExceededException" => {
                    return RusotoError::Service(GetComponentError::CallRateLimitExceeded(err.msg))
                }
                "ClientException" => {
                    return RusotoError::Service(GetComponentError::Client(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetComponentError::Forbidden(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(GetComponentError::InvalidRequest(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetComponentError::Service(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(GetComponentError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetComponentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetComponentError::CallRateLimitExceeded(ref cause) => write!(f, "{}", cause),
            GetComponentError::Client(ref cause) => write!(f, "{}", cause),
            GetComponentError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetComponentError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            GetComponentError::Service(ref cause) => write!(f, "{}", cause),
            GetComponentError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetComponentError {}
/// Errors returned by GetComponentPolicy
#[derive(Debug, PartialEq)]
pub enum GetComponentPolicyError {
    /// <p>You have exceeded the permitted request rate for the specific operation.</p>
    CallRateLimitExceeded(String),
    /// <p>You are not authorized to perform the requested operation.</p>
    Forbidden(String),
    /// <p>You have made a request for an action that is not supported by the service.</p>
    InvalidRequest(String),
    /// <p>At least one of the resources referenced by your request does not exist.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the service encounters an unrecoverable exception.</p>
    Service(String),
    /// <p>The service is unable to process your request at this time.</p>
    ServiceUnavailable(String),
}

impl GetComponentPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetComponentPolicyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "CallRateLimitExceededException" => {
                    return RusotoError::Service(GetComponentPolicyError::CallRateLimitExceeded(
                        err.msg,
                    ))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetComponentPolicyError::Forbidden(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(GetComponentPolicyError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetComponentPolicyError::ResourceNotFound(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetComponentPolicyError::Service(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(GetComponentPolicyError::ServiceUnavailable(
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
impl fmt::Display for GetComponentPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetComponentPolicyError::CallRateLimitExceeded(ref cause) => write!(f, "{}", cause),
            GetComponentPolicyError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetComponentPolicyError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            GetComponentPolicyError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetComponentPolicyError::Service(ref cause) => write!(f, "{}", cause),
            GetComponentPolicyError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetComponentPolicyError {}
/// Errors returned by GetDistributionConfiguration
#[derive(Debug, PartialEq)]
pub enum GetDistributionConfigurationError {
    /// <p>You have exceeded the permitted request rate for the specific operation.</p>
    CallRateLimitExceeded(String),
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an invalid resource identifier.</p>
    Client(String),
    /// <p>You are not authorized to perform the requested operation.</p>
    Forbidden(String),
    /// <p>You have made a request for an action that is not supported by the service.</p>
    InvalidRequest(String),
    /// <p>This exception is thrown when the service encounters an unrecoverable exception.</p>
    Service(String),
    /// <p>The service is unable to process your request at this time.</p>
    ServiceUnavailable(String),
}

impl GetDistributionConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetDistributionConfigurationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "CallRateLimitExceededException" => {
                    return RusotoError::Service(
                        GetDistributionConfigurationError::CallRateLimitExceeded(err.msg),
                    )
                }
                "ClientException" => {
                    return RusotoError::Service(GetDistributionConfigurationError::Client(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetDistributionConfigurationError::Forbidden(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(GetDistributionConfigurationError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetDistributionConfigurationError::Service(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        GetDistributionConfigurationError::ServiceUnavailable(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetDistributionConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDistributionConfigurationError::CallRateLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            GetDistributionConfigurationError::Client(ref cause) => write!(f, "{}", cause),
            GetDistributionConfigurationError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetDistributionConfigurationError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            GetDistributionConfigurationError::Service(ref cause) => write!(f, "{}", cause),
            GetDistributionConfigurationError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetDistributionConfigurationError {}
/// Errors returned by GetImage
#[derive(Debug, PartialEq)]
pub enum GetImageError {
    /// <p>You have exceeded the permitted request rate for the specific operation.</p>
    CallRateLimitExceeded(String),
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an invalid resource identifier.</p>
    Client(String),
    /// <p>You are not authorized to perform the requested operation.</p>
    Forbidden(String),
    /// <p>You have made a request for an action that is not supported by the service.</p>
    InvalidRequest(String),
    /// <p>This exception is thrown when the service encounters an unrecoverable exception.</p>
    Service(String),
    /// <p>The service is unable to process your request at this time.</p>
    ServiceUnavailable(String),
}

impl GetImageError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetImageError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "CallRateLimitExceededException" => {
                    return RusotoError::Service(GetImageError::CallRateLimitExceeded(err.msg))
                }
                "ClientException" => return RusotoError::Service(GetImageError::Client(err.msg)),
                "ForbiddenException" => {
                    return RusotoError::Service(GetImageError::Forbidden(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(GetImageError::InvalidRequest(err.msg))
                }
                "ServiceException" => return RusotoError::Service(GetImageError::Service(err.msg)),
                "ServiceUnavailableException" => {
                    return RusotoError::Service(GetImageError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetImageError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetImageError::CallRateLimitExceeded(ref cause) => write!(f, "{}", cause),
            GetImageError::Client(ref cause) => write!(f, "{}", cause),
            GetImageError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetImageError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            GetImageError::Service(ref cause) => write!(f, "{}", cause),
            GetImageError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetImageError {}
/// Errors returned by GetImagePipeline
#[derive(Debug, PartialEq)]
pub enum GetImagePipelineError {
    /// <p>You have exceeded the permitted request rate for the specific operation.</p>
    CallRateLimitExceeded(String),
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an invalid resource identifier.</p>
    Client(String),
    /// <p>You are not authorized to perform the requested operation.</p>
    Forbidden(String),
    /// <p>You have made a request for an action that is not supported by the service.</p>
    InvalidRequest(String),
    /// <p>This exception is thrown when the service encounters an unrecoverable exception.</p>
    Service(String),
    /// <p>The service is unable to process your request at this time.</p>
    ServiceUnavailable(String),
}

impl GetImagePipelineError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetImagePipelineError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "CallRateLimitExceededException" => {
                    return RusotoError::Service(GetImagePipelineError::CallRateLimitExceeded(
                        err.msg,
                    ))
                }
                "ClientException" => {
                    return RusotoError::Service(GetImagePipelineError::Client(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetImagePipelineError::Forbidden(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(GetImagePipelineError::InvalidRequest(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetImagePipelineError::Service(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(GetImagePipelineError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetImagePipelineError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetImagePipelineError::CallRateLimitExceeded(ref cause) => write!(f, "{}", cause),
            GetImagePipelineError::Client(ref cause) => write!(f, "{}", cause),
            GetImagePipelineError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetImagePipelineError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            GetImagePipelineError::Service(ref cause) => write!(f, "{}", cause),
            GetImagePipelineError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetImagePipelineError {}
/// Errors returned by GetImagePolicy
#[derive(Debug, PartialEq)]
pub enum GetImagePolicyError {
    /// <p>You have exceeded the permitted request rate for the specific operation.</p>
    CallRateLimitExceeded(String),
    /// <p>You are not authorized to perform the requested operation.</p>
    Forbidden(String),
    /// <p>You have made a request for an action that is not supported by the service.</p>
    InvalidRequest(String),
    /// <p>At least one of the resources referenced by your request does not exist.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the service encounters an unrecoverable exception.</p>
    Service(String),
    /// <p>The service is unable to process your request at this time.</p>
    ServiceUnavailable(String),
}

impl GetImagePolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetImagePolicyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "CallRateLimitExceededException" => {
                    return RusotoError::Service(GetImagePolicyError::CallRateLimitExceeded(
                        err.msg,
                    ))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetImagePolicyError::Forbidden(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(GetImagePolicyError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetImagePolicyError::ResourceNotFound(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetImagePolicyError::Service(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(GetImagePolicyError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetImagePolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetImagePolicyError::CallRateLimitExceeded(ref cause) => write!(f, "{}", cause),
            GetImagePolicyError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetImagePolicyError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            GetImagePolicyError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetImagePolicyError::Service(ref cause) => write!(f, "{}", cause),
            GetImagePolicyError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetImagePolicyError {}
/// Errors returned by GetImageRecipe
#[derive(Debug, PartialEq)]
pub enum GetImageRecipeError {
    /// <p>You have exceeded the permitted request rate for the specific operation.</p>
    CallRateLimitExceeded(String),
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an invalid resource identifier.</p>
    Client(String),
    /// <p>You are not authorized to perform the requested operation.</p>
    Forbidden(String),
    /// <p>You have made a request for an action that is not supported by the service.</p>
    InvalidRequest(String),
    /// <p>This exception is thrown when the service encounters an unrecoverable exception.</p>
    Service(String),
    /// <p>The service is unable to process your request at this time.</p>
    ServiceUnavailable(String),
}

impl GetImageRecipeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetImageRecipeError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "CallRateLimitExceededException" => {
                    return RusotoError::Service(GetImageRecipeError::CallRateLimitExceeded(
                        err.msg,
                    ))
                }
                "ClientException" => {
                    return RusotoError::Service(GetImageRecipeError::Client(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetImageRecipeError::Forbidden(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(GetImageRecipeError::InvalidRequest(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetImageRecipeError::Service(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(GetImageRecipeError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetImageRecipeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetImageRecipeError::CallRateLimitExceeded(ref cause) => write!(f, "{}", cause),
            GetImageRecipeError::Client(ref cause) => write!(f, "{}", cause),
            GetImageRecipeError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetImageRecipeError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            GetImageRecipeError::Service(ref cause) => write!(f, "{}", cause),
            GetImageRecipeError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetImageRecipeError {}
/// Errors returned by GetImageRecipePolicy
#[derive(Debug, PartialEq)]
pub enum GetImageRecipePolicyError {
    /// <p>You have exceeded the permitted request rate for the specific operation.</p>
    CallRateLimitExceeded(String),
    /// <p>You are not authorized to perform the requested operation.</p>
    Forbidden(String),
    /// <p>You have made a request for an action that is not supported by the service.</p>
    InvalidRequest(String),
    /// <p>At least one of the resources referenced by your request does not exist.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the service encounters an unrecoverable exception.</p>
    Service(String),
    /// <p>The service is unable to process your request at this time.</p>
    ServiceUnavailable(String),
}

impl GetImageRecipePolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetImageRecipePolicyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "CallRateLimitExceededException" => {
                    return RusotoError::Service(GetImageRecipePolicyError::CallRateLimitExceeded(
                        err.msg,
                    ))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetImageRecipePolicyError::Forbidden(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(GetImageRecipePolicyError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetImageRecipePolicyError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetImageRecipePolicyError::Service(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(GetImageRecipePolicyError::ServiceUnavailable(
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
impl fmt::Display for GetImageRecipePolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetImageRecipePolicyError::CallRateLimitExceeded(ref cause) => write!(f, "{}", cause),
            GetImageRecipePolicyError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetImageRecipePolicyError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            GetImageRecipePolicyError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetImageRecipePolicyError::Service(ref cause) => write!(f, "{}", cause),
            GetImageRecipePolicyError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetImageRecipePolicyError {}
/// Errors returned by GetInfrastructureConfiguration
#[derive(Debug, PartialEq)]
pub enum GetInfrastructureConfigurationError {
    /// <p>You have exceeded the permitted request rate for the specific operation.</p>
    CallRateLimitExceeded(String),
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an invalid resource identifier.</p>
    Client(String),
    /// <p>You are not authorized to perform the requested operation.</p>
    Forbidden(String),
    /// <p>You have made a request for an action that is not supported by the service.</p>
    InvalidRequest(String),
    /// <p>This exception is thrown when the service encounters an unrecoverable exception.</p>
    Service(String),
    /// <p>The service is unable to process your request at this time.</p>
    ServiceUnavailable(String),
}

impl GetInfrastructureConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetInfrastructureConfigurationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "CallRateLimitExceededException" => {
                    return RusotoError::Service(
                        GetInfrastructureConfigurationError::CallRateLimitExceeded(err.msg),
                    )
                }
                "ClientException" => {
                    return RusotoError::Service(GetInfrastructureConfigurationError::Client(
                        err.msg,
                    ))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetInfrastructureConfigurationError::Forbidden(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        GetInfrastructureConfigurationError::InvalidRequest(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(GetInfrastructureConfigurationError::Service(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        GetInfrastructureConfigurationError::ServiceUnavailable(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetInfrastructureConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetInfrastructureConfigurationError::CallRateLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            GetInfrastructureConfigurationError::Client(ref cause) => write!(f, "{}", cause),
            GetInfrastructureConfigurationError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetInfrastructureConfigurationError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            GetInfrastructureConfigurationError::Service(ref cause) => write!(f, "{}", cause),
            GetInfrastructureConfigurationError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetInfrastructureConfigurationError {}
/// Errors returned by ImportComponent
#[derive(Debug, PartialEq)]
pub enum ImportComponentError {
    /// <p>You have exceeded the permitted request rate for the specific operation.</p>
    CallRateLimitExceeded(String),
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an invalid resource identifier.</p>
    Client(String),
    /// <p>You are not authorized to perform the requested operation.</p>
    Forbidden(String),
    /// <p>You have specified a client token for an operation using parameter values that differ from a previous request that used the same client token.</p>
    IdempotentParameterMismatch(String),
    /// <p>You have specified two or more mutually exclusive parameters. Review the error message for details.</p>
    InvalidParameterCombination(String),
    /// <p>You have made a request for an action that is not supported by the service.</p>
    InvalidRequest(String),
    /// <p>Your version number is out of bounds or does not follow the required syntax.</p>
    InvalidVersionNumber(String),
    /// <p>The resource that you are trying to operate on is currently in use. Review the message details and retry later.</p>
    ResourceInUse(String),
    /// <p>This exception is thrown when the service encounters an unrecoverable exception.</p>
    Service(String),
    /// <p>The service is unable to process your request at this time.</p>
    ServiceUnavailable(String),
}

impl ImportComponentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ImportComponentError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "CallRateLimitExceededException" => {
                    return RusotoError::Service(ImportComponentError::CallRateLimitExceeded(
                        err.msg,
                    ))
                }
                "ClientException" => {
                    return RusotoError::Service(ImportComponentError::Client(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ImportComponentError::Forbidden(err.msg))
                }
                "IdempotentParameterMismatchException" => {
                    return RusotoError::Service(ImportComponentError::IdempotentParameterMismatch(
                        err.msg,
                    ))
                }
                "InvalidParameterCombinationException" => {
                    return RusotoError::Service(ImportComponentError::InvalidParameterCombination(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ImportComponentError::InvalidRequest(err.msg))
                }
                "InvalidVersionNumberException" => {
                    return RusotoError::Service(ImportComponentError::InvalidVersionNumber(
                        err.msg,
                    ))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(ImportComponentError::ResourceInUse(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(ImportComponentError::Service(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ImportComponentError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ImportComponentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ImportComponentError::CallRateLimitExceeded(ref cause) => write!(f, "{}", cause),
            ImportComponentError::Client(ref cause) => write!(f, "{}", cause),
            ImportComponentError::Forbidden(ref cause) => write!(f, "{}", cause),
            ImportComponentError::IdempotentParameterMismatch(ref cause) => write!(f, "{}", cause),
            ImportComponentError::InvalidParameterCombination(ref cause) => write!(f, "{}", cause),
            ImportComponentError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ImportComponentError::InvalidVersionNumber(ref cause) => write!(f, "{}", cause),
            ImportComponentError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            ImportComponentError::Service(ref cause) => write!(f, "{}", cause),
            ImportComponentError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ImportComponentError {}
/// Errors returned by ListComponentBuildVersions
#[derive(Debug, PartialEq)]
pub enum ListComponentBuildVersionsError {
    /// <p>You have exceeded the permitted request rate for the specific operation.</p>
    CallRateLimitExceeded(String),
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an invalid resource identifier.</p>
    Client(String),
    /// <p>You are not authorized to perform the requested operation.</p>
    Forbidden(String),
    /// <p>You have provided an invalid pagination token in your request.</p>
    InvalidPaginationToken(String),
    /// <p>You have made a request for an action that is not supported by the service.</p>
    InvalidRequest(String),
    /// <p>This exception is thrown when the service encounters an unrecoverable exception.</p>
    Service(String),
    /// <p>The service is unable to process your request at this time.</p>
    ServiceUnavailable(String),
}

impl ListComponentBuildVersionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListComponentBuildVersionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "CallRateLimitExceededException" => {
                    return RusotoError::Service(
                        ListComponentBuildVersionsError::CallRateLimitExceeded(err.msg),
                    )
                }
                "ClientException" => {
                    return RusotoError::Service(ListComponentBuildVersionsError::Client(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListComponentBuildVersionsError::Forbidden(
                        err.msg,
                    ))
                }
                "InvalidPaginationTokenException" => {
                    return RusotoError::Service(
                        ListComponentBuildVersionsError::InvalidPaginationToken(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListComponentBuildVersionsError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(ListComponentBuildVersionsError::Service(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        ListComponentBuildVersionsError::ServiceUnavailable(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListComponentBuildVersionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListComponentBuildVersionsError::CallRateLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            ListComponentBuildVersionsError::Client(ref cause) => write!(f, "{}", cause),
            ListComponentBuildVersionsError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListComponentBuildVersionsError::InvalidPaginationToken(ref cause) => {
                write!(f, "{}", cause)
            }
            ListComponentBuildVersionsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListComponentBuildVersionsError::Service(ref cause) => write!(f, "{}", cause),
            ListComponentBuildVersionsError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListComponentBuildVersionsError {}
/// Errors returned by ListComponents
#[derive(Debug, PartialEq)]
pub enum ListComponentsError {
    /// <p>You have exceeded the permitted request rate for the specific operation.</p>
    CallRateLimitExceeded(String),
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an invalid resource identifier.</p>
    Client(String),
    /// <p>You are not authorized to perform the requested operation.</p>
    Forbidden(String),
    /// <p>You have provided an invalid pagination token in your request.</p>
    InvalidPaginationToken(String),
    /// <p>You have made a request for an action that is not supported by the service.</p>
    InvalidRequest(String),
    /// <p>This exception is thrown when the service encounters an unrecoverable exception.</p>
    Service(String),
    /// <p>The service is unable to process your request at this time.</p>
    ServiceUnavailable(String),
}

impl ListComponentsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListComponentsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "CallRateLimitExceededException" => {
                    return RusotoError::Service(ListComponentsError::CallRateLimitExceeded(
                        err.msg,
                    ))
                }
                "ClientException" => {
                    return RusotoError::Service(ListComponentsError::Client(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListComponentsError::Forbidden(err.msg))
                }
                "InvalidPaginationTokenException" => {
                    return RusotoError::Service(ListComponentsError::InvalidPaginationToken(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListComponentsError::InvalidRequest(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(ListComponentsError::Service(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListComponentsError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListComponentsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListComponentsError::CallRateLimitExceeded(ref cause) => write!(f, "{}", cause),
            ListComponentsError::Client(ref cause) => write!(f, "{}", cause),
            ListComponentsError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListComponentsError::InvalidPaginationToken(ref cause) => write!(f, "{}", cause),
            ListComponentsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListComponentsError::Service(ref cause) => write!(f, "{}", cause),
            ListComponentsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListComponentsError {}
/// Errors returned by ListDistributionConfigurations
#[derive(Debug, PartialEq)]
pub enum ListDistributionConfigurationsError {
    /// <p>You have exceeded the permitted request rate for the specific operation.</p>
    CallRateLimitExceeded(String),
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an invalid resource identifier.</p>
    Client(String),
    /// <p>You are not authorized to perform the requested operation.</p>
    Forbidden(String),
    /// <p>You have provided an invalid pagination token in your request.</p>
    InvalidPaginationToken(String),
    /// <p>You have made a request for an action that is not supported by the service.</p>
    InvalidRequest(String),
    /// <p>This exception is thrown when the service encounters an unrecoverable exception.</p>
    Service(String),
    /// <p>The service is unable to process your request at this time.</p>
    ServiceUnavailable(String),
}

impl ListDistributionConfigurationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListDistributionConfigurationsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "CallRateLimitExceededException" => {
                    return RusotoError::Service(
                        ListDistributionConfigurationsError::CallRateLimitExceeded(err.msg),
                    )
                }
                "ClientException" => {
                    return RusotoError::Service(ListDistributionConfigurationsError::Client(
                        err.msg,
                    ))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListDistributionConfigurationsError::Forbidden(
                        err.msg,
                    ))
                }
                "InvalidPaginationTokenException" => {
                    return RusotoError::Service(
                        ListDistributionConfigurationsError::InvalidPaginationToken(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        ListDistributionConfigurationsError::InvalidRequest(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(ListDistributionConfigurationsError::Service(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        ListDistributionConfigurationsError::ServiceUnavailable(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListDistributionConfigurationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDistributionConfigurationsError::CallRateLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            ListDistributionConfigurationsError::Client(ref cause) => write!(f, "{}", cause),
            ListDistributionConfigurationsError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListDistributionConfigurationsError::InvalidPaginationToken(ref cause) => {
                write!(f, "{}", cause)
            }
            ListDistributionConfigurationsError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            ListDistributionConfigurationsError::Service(ref cause) => write!(f, "{}", cause),
            ListDistributionConfigurationsError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListDistributionConfigurationsError {}
/// Errors returned by ListImageBuildVersions
#[derive(Debug, PartialEq)]
pub enum ListImageBuildVersionsError {
    /// <p>You have exceeded the permitted request rate for the specific operation.</p>
    CallRateLimitExceeded(String),
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an invalid resource identifier.</p>
    Client(String),
    /// <p>You are not authorized to perform the requested operation.</p>
    Forbidden(String),
    /// <p>You have provided an invalid pagination token in your request.</p>
    InvalidPaginationToken(String),
    /// <p>You have made a request for an action that is not supported by the service.</p>
    InvalidRequest(String),
    /// <p>This exception is thrown when the service encounters an unrecoverable exception.</p>
    Service(String),
    /// <p>The service is unable to process your request at this time.</p>
    ServiceUnavailable(String),
}

impl ListImageBuildVersionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListImageBuildVersionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "CallRateLimitExceededException" => {
                    return RusotoError::Service(
                        ListImageBuildVersionsError::CallRateLimitExceeded(err.msg),
                    )
                }
                "ClientException" => {
                    return RusotoError::Service(ListImageBuildVersionsError::Client(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListImageBuildVersionsError::Forbidden(err.msg))
                }
                "InvalidPaginationTokenException" => {
                    return RusotoError::Service(
                        ListImageBuildVersionsError::InvalidPaginationToken(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListImageBuildVersionsError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(ListImageBuildVersionsError::Service(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListImageBuildVersionsError::ServiceUnavailable(
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
impl fmt::Display for ListImageBuildVersionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListImageBuildVersionsError::CallRateLimitExceeded(ref cause) => write!(f, "{}", cause),
            ListImageBuildVersionsError::Client(ref cause) => write!(f, "{}", cause),
            ListImageBuildVersionsError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListImageBuildVersionsError::InvalidPaginationToken(ref cause) => {
                write!(f, "{}", cause)
            }
            ListImageBuildVersionsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListImageBuildVersionsError::Service(ref cause) => write!(f, "{}", cause),
            ListImageBuildVersionsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListImageBuildVersionsError {}
/// Errors returned by ListImagePipelineImages
#[derive(Debug, PartialEq)]
pub enum ListImagePipelineImagesError {
    /// <p>You have exceeded the permitted request rate for the specific operation.</p>
    CallRateLimitExceeded(String),
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an invalid resource identifier.</p>
    Client(String),
    /// <p>You are not authorized to perform the requested operation.</p>
    Forbidden(String),
    /// <p>You have provided an invalid pagination token in your request.</p>
    InvalidPaginationToken(String),
    /// <p>You have made a request for an action that is not supported by the service.</p>
    InvalidRequest(String),
    /// <p>At least one of the resources referenced by your request does not exist.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the service encounters an unrecoverable exception.</p>
    Service(String),
    /// <p>The service is unable to process your request at this time.</p>
    ServiceUnavailable(String),
}

impl ListImagePipelineImagesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListImagePipelineImagesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "CallRateLimitExceededException" => {
                    return RusotoError::Service(
                        ListImagePipelineImagesError::CallRateLimitExceeded(err.msg),
                    )
                }
                "ClientException" => {
                    return RusotoError::Service(ListImagePipelineImagesError::Client(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListImagePipelineImagesError::Forbidden(err.msg))
                }
                "InvalidPaginationTokenException" => {
                    return RusotoError::Service(
                        ListImagePipelineImagesError::InvalidPaginationToken(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListImagePipelineImagesError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListImagePipelineImagesError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(ListImagePipelineImagesError::Service(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListImagePipelineImagesError::ServiceUnavailable(
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
impl fmt::Display for ListImagePipelineImagesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListImagePipelineImagesError::CallRateLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            ListImagePipelineImagesError::Client(ref cause) => write!(f, "{}", cause),
            ListImagePipelineImagesError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListImagePipelineImagesError::InvalidPaginationToken(ref cause) => {
                write!(f, "{}", cause)
            }
            ListImagePipelineImagesError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListImagePipelineImagesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListImagePipelineImagesError::Service(ref cause) => write!(f, "{}", cause),
            ListImagePipelineImagesError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListImagePipelineImagesError {}
/// Errors returned by ListImagePipelines
#[derive(Debug, PartialEq)]
pub enum ListImagePipelinesError {
    /// <p>You have exceeded the permitted request rate for the specific operation.</p>
    CallRateLimitExceeded(String),
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an invalid resource identifier.</p>
    Client(String),
    /// <p>You are not authorized to perform the requested operation.</p>
    Forbidden(String),
    /// <p>You have provided an invalid pagination token in your request.</p>
    InvalidPaginationToken(String),
    /// <p>You have made a request for an action that is not supported by the service.</p>
    InvalidRequest(String),
    /// <p>This exception is thrown when the service encounters an unrecoverable exception.</p>
    Service(String),
    /// <p>The service is unable to process your request at this time.</p>
    ServiceUnavailable(String),
}

impl ListImagePipelinesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListImagePipelinesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "CallRateLimitExceededException" => {
                    return RusotoError::Service(ListImagePipelinesError::CallRateLimitExceeded(
                        err.msg,
                    ))
                }
                "ClientException" => {
                    return RusotoError::Service(ListImagePipelinesError::Client(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListImagePipelinesError::Forbidden(err.msg))
                }
                "InvalidPaginationTokenException" => {
                    return RusotoError::Service(ListImagePipelinesError::InvalidPaginationToken(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListImagePipelinesError::InvalidRequest(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(ListImagePipelinesError::Service(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListImagePipelinesError::ServiceUnavailable(
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
impl fmt::Display for ListImagePipelinesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListImagePipelinesError::CallRateLimitExceeded(ref cause) => write!(f, "{}", cause),
            ListImagePipelinesError::Client(ref cause) => write!(f, "{}", cause),
            ListImagePipelinesError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListImagePipelinesError::InvalidPaginationToken(ref cause) => write!(f, "{}", cause),
            ListImagePipelinesError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListImagePipelinesError::Service(ref cause) => write!(f, "{}", cause),
            ListImagePipelinesError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListImagePipelinesError {}
/// Errors returned by ListImageRecipes
#[derive(Debug, PartialEq)]
pub enum ListImageRecipesError {
    /// <p>You have exceeded the permitted request rate for the specific operation.</p>
    CallRateLimitExceeded(String),
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an invalid resource identifier.</p>
    Client(String),
    /// <p>You are not authorized to perform the requested operation.</p>
    Forbidden(String),
    /// <p>You have provided an invalid pagination token in your request.</p>
    InvalidPaginationToken(String),
    /// <p>You have made a request for an action that is not supported by the service.</p>
    InvalidRequest(String),
    /// <p>This exception is thrown when the service encounters an unrecoverable exception.</p>
    Service(String),
    /// <p>The service is unable to process your request at this time.</p>
    ServiceUnavailable(String),
}

impl ListImageRecipesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListImageRecipesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "CallRateLimitExceededException" => {
                    return RusotoError::Service(ListImageRecipesError::CallRateLimitExceeded(
                        err.msg,
                    ))
                }
                "ClientException" => {
                    return RusotoError::Service(ListImageRecipesError::Client(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListImageRecipesError::Forbidden(err.msg))
                }
                "InvalidPaginationTokenException" => {
                    return RusotoError::Service(ListImageRecipesError::InvalidPaginationToken(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListImageRecipesError::InvalidRequest(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(ListImageRecipesError::Service(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListImageRecipesError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListImageRecipesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListImageRecipesError::CallRateLimitExceeded(ref cause) => write!(f, "{}", cause),
            ListImageRecipesError::Client(ref cause) => write!(f, "{}", cause),
            ListImageRecipesError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListImageRecipesError::InvalidPaginationToken(ref cause) => write!(f, "{}", cause),
            ListImageRecipesError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListImageRecipesError::Service(ref cause) => write!(f, "{}", cause),
            ListImageRecipesError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListImageRecipesError {}
/// Errors returned by ListImages
#[derive(Debug, PartialEq)]
pub enum ListImagesError {
    /// <p>You have exceeded the permitted request rate for the specific operation.</p>
    CallRateLimitExceeded(String),
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an invalid resource identifier.</p>
    Client(String),
    /// <p>You are not authorized to perform the requested operation.</p>
    Forbidden(String),
    /// <p>You have provided an invalid pagination token in your request.</p>
    InvalidPaginationToken(String),
    /// <p>You have made a request for an action that is not supported by the service.</p>
    InvalidRequest(String),
    /// <p>This exception is thrown when the service encounters an unrecoverable exception.</p>
    Service(String),
    /// <p>The service is unable to process your request at this time.</p>
    ServiceUnavailable(String),
}

impl ListImagesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListImagesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "CallRateLimitExceededException" => {
                    return RusotoError::Service(ListImagesError::CallRateLimitExceeded(err.msg))
                }
                "ClientException" => return RusotoError::Service(ListImagesError::Client(err.msg)),
                "ForbiddenException" => {
                    return RusotoError::Service(ListImagesError::Forbidden(err.msg))
                }
                "InvalidPaginationTokenException" => {
                    return RusotoError::Service(ListImagesError::InvalidPaginationToken(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListImagesError::InvalidRequest(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(ListImagesError::Service(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListImagesError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListImagesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListImagesError::CallRateLimitExceeded(ref cause) => write!(f, "{}", cause),
            ListImagesError::Client(ref cause) => write!(f, "{}", cause),
            ListImagesError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListImagesError::InvalidPaginationToken(ref cause) => write!(f, "{}", cause),
            ListImagesError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListImagesError::Service(ref cause) => write!(f, "{}", cause),
            ListImagesError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListImagesError {}
/// Errors returned by ListInfrastructureConfigurations
#[derive(Debug, PartialEq)]
pub enum ListInfrastructureConfigurationsError {
    /// <p>You have exceeded the permitted request rate for the specific operation.</p>
    CallRateLimitExceeded(String),
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an invalid resource identifier.</p>
    Client(String),
    /// <p>You are not authorized to perform the requested operation.</p>
    Forbidden(String),
    /// <p>You have provided an invalid pagination token in your request.</p>
    InvalidPaginationToken(String),
    /// <p>You have made a request for an action that is not supported by the service.</p>
    InvalidRequest(String),
    /// <p>This exception is thrown when the service encounters an unrecoverable exception.</p>
    Service(String),
    /// <p>The service is unable to process your request at this time.</p>
    ServiceUnavailable(String),
}

impl ListInfrastructureConfigurationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListInfrastructureConfigurationsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "CallRateLimitExceededException" => {
                    return RusotoError::Service(
                        ListInfrastructureConfigurationsError::CallRateLimitExceeded(err.msg),
                    )
                }
                "ClientException" => {
                    return RusotoError::Service(ListInfrastructureConfigurationsError::Client(
                        err.msg,
                    ))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListInfrastructureConfigurationsError::Forbidden(
                        err.msg,
                    ))
                }
                "InvalidPaginationTokenException" => {
                    return RusotoError::Service(
                        ListInfrastructureConfigurationsError::InvalidPaginationToken(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        ListInfrastructureConfigurationsError::InvalidRequest(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(ListInfrastructureConfigurationsError::Service(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        ListInfrastructureConfigurationsError::ServiceUnavailable(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListInfrastructureConfigurationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListInfrastructureConfigurationsError::CallRateLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            ListInfrastructureConfigurationsError::Client(ref cause) => write!(f, "{}", cause),
            ListInfrastructureConfigurationsError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListInfrastructureConfigurationsError::InvalidPaginationToken(ref cause) => {
                write!(f, "{}", cause)
            }
            ListInfrastructureConfigurationsError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            ListInfrastructureConfigurationsError::Service(ref cause) => write!(f, "{}", cause),
            ListInfrastructureConfigurationsError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListInfrastructureConfigurationsError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>At least one of the resources referenced by your request does not exist.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the service encounters an unrecoverable exception.</p>
    Service(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(ListTagsForResourceError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListTagsForResourceError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(ListTagsForResourceError::Service(err.msg))
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
            ListTagsForResourceError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::Service(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by PutComponentPolicy
#[derive(Debug, PartialEq)]
pub enum PutComponentPolicyError {
    /// <p>You have exceeded the permitted request rate for the specific operation.</p>
    CallRateLimitExceeded(String),
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an invalid resource identifier.</p>
    Client(String),
    /// <p>You are not authorized to perform the requested operation.</p>
    Forbidden(String),
    /// <p>The value that you provided for the specified parameter is invalid.</p>
    InvalidParameterValue(String),
    /// <p>You have made a request for an action that is not supported by the service.</p>
    InvalidRequest(String),
    /// <p>At least one of the resources referenced by your request does not exist.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the service encounters an unrecoverable exception.</p>
    Service(String),
    /// <p>The service is unable to process your request at this time.</p>
    ServiceUnavailable(String),
}

impl PutComponentPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutComponentPolicyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "CallRateLimitExceededException" => {
                    return RusotoError::Service(PutComponentPolicyError::CallRateLimitExceeded(
                        err.msg,
                    ))
                }
                "ClientException" => {
                    return RusotoError::Service(PutComponentPolicyError::Client(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(PutComponentPolicyError::Forbidden(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(PutComponentPolicyError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(PutComponentPolicyError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(PutComponentPolicyError::ResourceNotFound(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(PutComponentPolicyError::Service(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(PutComponentPolicyError::ServiceUnavailable(
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
impl fmt::Display for PutComponentPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutComponentPolicyError::CallRateLimitExceeded(ref cause) => write!(f, "{}", cause),
            PutComponentPolicyError::Client(ref cause) => write!(f, "{}", cause),
            PutComponentPolicyError::Forbidden(ref cause) => write!(f, "{}", cause),
            PutComponentPolicyError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            PutComponentPolicyError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            PutComponentPolicyError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            PutComponentPolicyError::Service(ref cause) => write!(f, "{}", cause),
            PutComponentPolicyError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutComponentPolicyError {}
/// Errors returned by PutImagePolicy
#[derive(Debug, PartialEq)]
pub enum PutImagePolicyError {
    /// <p>You have exceeded the permitted request rate for the specific operation.</p>
    CallRateLimitExceeded(String),
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an invalid resource identifier.</p>
    Client(String),
    /// <p>You are not authorized to perform the requested operation.</p>
    Forbidden(String),
    /// <p>The value that you provided for the specified parameter is invalid.</p>
    InvalidParameterValue(String),
    /// <p>You have made a request for an action that is not supported by the service.</p>
    InvalidRequest(String),
    /// <p>At least one of the resources referenced by your request does not exist.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the service encounters an unrecoverable exception.</p>
    Service(String),
    /// <p>The service is unable to process your request at this time.</p>
    ServiceUnavailable(String),
}

impl PutImagePolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutImagePolicyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "CallRateLimitExceededException" => {
                    return RusotoError::Service(PutImagePolicyError::CallRateLimitExceeded(
                        err.msg,
                    ))
                }
                "ClientException" => {
                    return RusotoError::Service(PutImagePolicyError::Client(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(PutImagePolicyError::Forbidden(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(PutImagePolicyError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(PutImagePolicyError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(PutImagePolicyError::ResourceNotFound(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(PutImagePolicyError::Service(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(PutImagePolicyError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutImagePolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutImagePolicyError::CallRateLimitExceeded(ref cause) => write!(f, "{}", cause),
            PutImagePolicyError::Client(ref cause) => write!(f, "{}", cause),
            PutImagePolicyError::Forbidden(ref cause) => write!(f, "{}", cause),
            PutImagePolicyError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            PutImagePolicyError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            PutImagePolicyError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            PutImagePolicyError::Service(ref cause) => write!(f, "{}", cause),
            PutImagePolicyError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutImagePolicyError {}
/// Errors returned by PutImageRecipePolicy
#[derive(Debug, PartialEq)]
pub enum PutImageRecipePolicyError {
    /// <p>You have exceeded the permitted request rate for the specific operation.</p>
    CallRateLimitExceeded(String),
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an invalid resource identifier.</p>
    Client(String),
    /// <p>You are not authorized to perform the requested operation.</p>
    Forbidden(String),
    /// <p>The value that you provided for the specified parameter is invalid.</p>
    InvalidParameterValue(String),
    /// <p>You have made a request for an action that is not supported by the service.</p>
    InvalidRequest(String),
    /// <p>At least one of the resources referenced by your request does not exist.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the service encounters an unrecoverable exception.</p>
    Service(String),
    /// <p>The service is unable to process your request at this time.</p>
    ServiceUnavailable(String),
}

impl PutImageRecipePolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutImageRecipePolicyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "CallRateLimitExceededException" => {
                    return RusotoError::Service(PutImageRecipePolicyError::CallRateLimitExceeded(
                        err.msg,
                    ))
                }
                "ClientException" => {
                    return RusotoError::Service(PutImageRecipePolicyError::Client(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(PutImageRecipePolicyError::Forbidden(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(PutImageRecipePolicyError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(PutImageRecipePolicyError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(PutImageRecipePolicyError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(PutImageRecipePolicyError::Service(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(PutImageRecipePolicyError::ServiceUnavailable(
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
impl fmt::Display for PutImageRecipePolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutImageRecipePolicyError::CallRateLimitExceeded(ref cause) => write!(f, "{}", cause),
            PutImageRecipePolicyError::Client(ref cause) => write!(f, "{}", cause),
            PutImageRecipePolicyError::Forbidden(ref cause) => write!(f, "{}", cause),
            PutImageRecipePolicyError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            PutImageRecipePolicyError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            PutImageRecipePolicyError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            PutImageRecipePolicyError::Service(ref cause) => write!(f, "{}", cause),
            PutImageRecipePolicyError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutImageRecipePolicyError {}
/// Errors returned by StartImagePipelineExecution
#[derive(Debug, PartialEq)]
pub enum StartImagePipelineExecutionError {
    /// <p>You have exceeded the permitted request rate for the specific operation.</p>
    CallRateLimitExceeded(String),
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an invalid resource identifier.</p>
    Client(String),
    /// <p>You are not authorized to perform the requested operation.</p>
    Forbidden(String),
    /// <p>You have specified a client token for an operation using parameter values that differ from a previous request that used the same client token.</p>
    IdempotentParameterMismatch(String),
    /// <p>You have made a request for an action that is not supported by the service.</p>
    InvalidRequest(String),
    /// <p>The resource that you are trying to operate on is currently in use. Review the message details and retry later.</p>
    ResourceInUse(String),
    /// <p>At least one of the resources referenced by your request does not exist.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the service encounters an unrecoverable exception.</p>
    Service(String),
    /// <p>The service is unable to process your request at this time.</p>
    ServiceUnavailable(String),
}

impl StartImagePipelineExecutionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<StartImagePipelineExecutionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "CallRateLimitExceededException" => {
                    return RusotoError::Service(
                        StartImagePipelineExecutionError::CallRateLimitExceeded(err.msg),
                    )
                }
                "ClientException" => {
                    return RusotoError::Service(StartImagePipelineExecutionError::Client(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(StartImagePipelineExecutionError::Forbidden(
                        err.msg,
                    ))
                }
                "IdempotentParameterMismatchException" => {
                    return RusotoError::Service(
                        StartImagePipelineExecutionError::IdempotentParameterMismatch(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(StartImagePipelineExecutionError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(StartImagePipelineExecutionError::ResourceInUse(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        StartImagePipelineExecutionError::ResourceNotFound(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(StartImagePipelineExecutionError::Service(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        StartImagePipelineExecutionError::ServiceUnavailable(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartImagePipelineExecutionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartImagePipelineExecutionError::CallRateLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            StartImagePipelineExecutionError::Client(ref cause) => write!(f, "{}", cause),
            StartImagePipelineExecutionError::Forbidden(ref cause) => write!(f, "{}", cause),
            StartImagePipelineExecutionError::IdempotentParameterMismatch(ref cause) => {
                write!(f, "{}", cause)
            }
            StartImagePipelineExecutionError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            StartImagePipelineExecutionError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            StartImagePipelineExecutionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            StartImagePipelineExecutionError::Service(ref cause) => write!(f, "{}", cause),
            StartImagePipelineExecutionError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for StartImagePipelineExecutionError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>At least one of the resources referenced by your request does not exist.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the service encounters an unrecoverable exception.</p>
    Service(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(TagResourceError::InvalidParameter(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(TagResourceError::ResourceNotFound(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(TagResourceError::Service(err.msg))
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
            TagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            TagResourceError::Service(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>At least one of the resources referenced by your request does not exist.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the service encounters an unrecoverable exception.</p>
    Service(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(UntagResourceError::InvalidParameter(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UntagResourceError::ResourceNotFound(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(UntagResourceError::Service(err.msg))
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
            UntagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UntagResourceError::Service(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateDistributionConfiguration
#[derive(Debug, PartialEq)]
pub enum UpdateDistributionConfigurationError {
    /// <p>You have exceeded the permitted request rate for the specific operation.</p>
    CallRateLimitExceeded(String),
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an invalid resource identifier.</p>
    Client(String),
    /// <p>You are not authorized to perform the requested operation.</p>
    Forbidden(String),
    /// <p>You have specified a client token for an operation using parameter values that differ from a previous request that used the same client token.</p>
    IdempotentParameterMismatch(String),
    /// <p>You have specified two or more mutually exclusive parameters. Review the error message for details.</p>
    InvalidParameterCombination(String),
    /// <p>You have made a request for an action that is not supported by the service.</p>
    InvalidRequest(String),
    /// <p>The resource that you are trying to operate on is currently in use. Review the message details and retry later.</p>
    ResourceInUse(String),
    /// <p>This exception is thrown when the service encounters an unrecoverable exception.</p>
    Service(String),
    /// <p>The service is unable to process your request at this time.</p>
    ServiceUnavailable(String),
}

impl UpdateDistributionConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateDistributionConfigurationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "CallRateLimitExceededException" => {
                    return RusotoError::Service(
                        UpdateDistributionConfigurationError::CallRateLimitExceeded(err.msg),
                    )
                }
                "ClientException" => {
                    return RusotoError::Service(UpdateDistributionConfigurationError::Client(
                        err.msg,
                    ))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateDistributionConfigurationError::Forbidden(
                        err.msg,
                    ))
                }
                "IdempotentParameterMismatchException" => {
                    return RusotoError::Service(
                        UpdateDistributionConfigurationError::IdempotentParameterMismatch(err.msg),
                    )
                }
                "InvalidParameterCombinationException" => {
                    return RusotoError::Service(
                        UpdateDistributionConfigurationError::InvalidParameterCombination(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        UpdateDistributionConfigurationError::InvalidRequest(err.msg),
                    )
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(
                        UpdateDistributionConfigurationError::ResourceInUse(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(UpdateDistributionConfigurationError::Service(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        UpdateDistributionConfigurationError::ServiceUnavailable(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateDistributionConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateDistributionConfigurationError::CallRateLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateDistributionConfigurationError::Client(ref cause) => write!(f, "{}", cause),
            UpdateDistributionConfigurationError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateDistributionConfigurationError::IdempotentParameterMismatch(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateDistributionConfigurationError::InvalidParameterCombination(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateDistributionConfigurationError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateDistributionConfigurationError::ResourceInUse(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateDistributionConfigurationError::Service(ref cause) => write!(f, "{}", cause),
            UpdateDistributionConfigurationError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateDistributionConfigurationError {}
/// Errors returned by UpdateImagePipeline
#[derive(Debug, PartialEq)]
pub enum UpdateImagePipelineError {
    /// <p>You have exceeded the permitted request rate for the specific operation.</p>
    CallRateLimitExceeded(String),
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an invalid resource identifier.</p>
    Client(String),
    /// <p>You are not authorized to perform the requested operation.</p>
    Forbidden(String),
    /// <p>You have specified a client token for an operation using parameter values that differ from a previous request that used the same client token.</p>
    IdempotentParameterMismatch(String),
    /// <p>You have made a request for an action that is not supported by the service.</p>
    InvalidRequest(String),
    /// <p>The resource that you are trying to operate on is currently in use. Review the message details and retry later.</p>
    ResourceInUse(String),
    /// <p>This exception is thrown when the service encounters an unrecoverable exception.</p>
    Service(String),
    /// <p>The service is unable to process your request at this time.</p>
    ServiceUnavailable(String),
}

impl UpdateImagePipelineError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateImagePipelineError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "CallRateLimitExceededException" => {
                    return RusotoError::Service(UpdateImagePipelineError::CallRateLimitExceeded(
                        err.msg,
                    ))
                }
                "ClientException" => {
                    return RusotoError::Service(UpdateImagePipelineError::Client(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateImagePipelineError::Forbidden(err.msg))
                }
                "IdempotentParameterMismatchException" => {
                    return RusotoError::Service(
                        UpdateImagePipelineError::IdempotentParameterMismatch(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateImagePipelineError::InvalidRequest(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(UpdateImagePipelineError::ResourceInUse(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(UpdateImagePipelineError::Service(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdateImagePipelineError::ServiceUnavailable(
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
impl fmt::Display for UpdateImagePipelineError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateImagePipelineError::CallRateLimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateImagePipelineError::Client(ref cause) => write!(f, "{}", cause),
            UpdateImagePipelineError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateImagePipelineError::IdempotentParameterMismatch(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateImagePipelineError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UpdateImagePipelineError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            UpdateImagePipelineError::Service(ref cause) => write!(f, "{}", cause),
            UpdateImagePipelineError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateImagePipelineError {}
/// Errors returned by UpdateInfrastructureConfiguration
#[derive(Debug, PartialEq)]
pub enum UpdateInfrastructureConfigurationError {
    /// <p>You have exceeded the permitted request rate for the specific operation.</p>
    CallRateLimitExceeded(String),
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an invalid resource identifier.</p>
    Client(String),
    /// <p>You are not authorized to perform the requested operation.</p>
    Forbidden(String),
    /// <p>You have specified a client token for an operation using parameter values that differ from a previous request that used the same client token.</p>
    IdempotentParameterMismatch(String),
    /// <p>You have made a request for an action that is not supported by the service.</p>
    InvalidRequest(String),
    /// <p>The resource that you are trying to operate on is currently in use. Review the message details and retry later.</p>
    ResourceInUse(String),
    /// <p>This exception is thrown when the service encounters an unrecoverable exception.</p>
    Service(String),
    /// <p>The service is unable to process your request at this time.</p>
    ServiceUnavailable(String),
}

impl UpdateInfrastructureConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateInfrastructureConfigurationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "CallRateLimitExceededException" => {
                    return RusotoError::Service(
                        UpdateInfrastructureConfigurationError::CallRateLimitExceeded(err.msg),
                    )
                }
                "ClientException" => {
                    return RusotoError::Service(UpdateInfrastructureConfigurationError::Client(
                        err.msg,
                    ))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateInfrastructureConfigurationError::Forbidden(
                        err.msg,
                    ))
                }
                "IdempotentParameterMismatchException" => {
                    return RusotoError::Service(
                        UpdateInfrastructureConfigurationError::IdempotentParameterMismatch(
                            err.msg,
                        ),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        UpdateInfrastructureConfigurationError::InvalidRequest(err.msg),
                    )
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(
                        UpdateInfrastructureConfigurationError::ResourceInUse(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(UpdateInfrastructureConfigurationError::Service(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        UpdateInfrastructureConfigurationError::ServiceUnavailable(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateInfrastructureConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateInfrastructureConfigurationError::CallRateLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateInfrastructureConfigurationError::Client(ref cause) => write!(f, "{}", cause),
            UpdateInfrastructureConfigurationError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateInfrastructureConfigurationError::IdempotentParameterMismatch(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateInfrastructureConfigurationError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateInfrastructureConfigurationError::ResourceInUse(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateInfrastructureConfigurationError::Service(ref cause) => write!(f, "{}", cause),
            UpdateInfrastructureConfigurationError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateInfrastructureConfigurationError {}
/// Trait representing the capabilities of the imagebuilder API. imagebuilder clients implement this trait.
#[async_trait]
pub trait ImageBuilder {
    /// <p>CancelImageCreation cancels the creation of Image. This operation can only be used on images in a non-terminal state.</p>
    async fn cancel_image_creation(
        &self,
        input: CancelImageCreationRequest,
    ) -> Result<CancelImageCreationResponse, RusotoError<CancelImageCreationError>>;

    /// <p>Creates a new component that can be used to build, validate, test, and assess your image.</p>
    async fn create_component(
        &self,
        input: CreateComponentRequest,
    ) -> Result<CreateComponentResponse, RusotoError<CreateComponentError>>;

    /// <p>Creates a new distribution configuration. Distribution configurations define and configure the outputs of your pipeline. </p>
    async fn create_distribution_configuration(
        &self,
        input: CreateDistributionConfigurationRequest,
    ) -> Result<
        CreateDistributionConfigurationResponse,
        RusotoError<CreateDistributionConfigurationError>,
    >;

    /// <p> Creates a new image. This request will create a new image along with all of the configured output resources defined in the distribution configuration. </p>
    async fn create_image(
        &self,
        input: CreateImageRequest,
    ) -> Result<CreateImageResponse, RusotoError<CreateImageError>>;

    /// <p> Creates a new image pipeline. Image pipelines enable you to automate the creation and distribution of images. </p>
    async fn create_image_pipeline(
        &self,
        input: CreateImagePipelineRequest,
    ) -> Result<CreateImagePipelineResponse, RusotoError<CreateImagePipelineError>>;

    /// <p> Creates a new image recipe. Image recipes define how images are configured, tested, and assessed. </p>
    async fn create_image_recipe(
        &self,
        input: CreateImageRecipeRequest,
    ) -> Result<CreateImageRecipeResponse, RusotoError<CreateImageRecipeError>>;

    /// <p> Creates a new infrastructure configuration. An infrastructure configuration defines the environment in which your image will be built and tested. </p>
    async fn create_infrastructure_configuration(
        &self,
        input: CreateInfrastructureConfigurationRequest,
    ) -> Result<
        CreateInfrastructureConfigurationResponse,
        RusotoError<CreateInfrastructureConfigurationError>,
    >;

    /// <p> Deletes a component build version. </p>
    async fn delete_component(
        &self,
        input: DeleteComponentRequest,
    ) -> Result<DeleteComponentResponse, RusotoError<DeleteComponentError>>;

    /// <p> Deletes a distribution configuration. </p>
    async fn delete_distribution_configuration(
        &self,
        input: DeleteDistributionConfigurationRequest,
    ) -> Result<
        DeleteDistributionConfigurationResponse,
        RusotoError<DeleteDistributionConfigurationError>,
    >;

    /// <p> Deletes an image. </p>
    async fn delete_image(
        &self,
        input: DeleteImageRequest,
    ) -> Result<DeleteImageResponse, RusotoError<DeleteImageError>>;

    /// <p> Deletes an image pipeline. </p>
    async fn delete_image_pipeline(
        &self,
        input: DeleteImagePipelineRequest,
    ) -> Result<DeleteImagePipelineResponse, RusotoError<DeleteImagePipelineError>>;

    /// <p> Deletes an image recipe. </p>
    async fn delete_image_recipe(
        &self,
        input: DeleteImageRecipeRequest,
    ) -> Result<DeleteImageRecipeResponse, RusotoError<DeleteImageRecipeError>>;

    /// <p> Deletes an infrastructure configuration. </p>
    async fn delete_infrastructure_configuration(
        &self,
        input: DeleteInfrastructureConfigurationRequest,
    ) -> Result<
        DeleteInfrastructureConfigurationResponse,
        RusotoError<DeleteInfrastructureConfigurationError>,
    >;

    /// <p> Gets a component object. </p>
    async fn get_component(
        &self,
        input: GetComponentRequest,
    ) -> Result<GetComponentResponse, RusotoError<GetComponentError>>;

    /// <p> Gets a component policy. </p>
    async fn get_component_policy(
        &self,
        input: GetComponentPolicyRequest,
    ) -> Result<GetComponentPolicyResponse, RusotoError<GetComponentPolicyError>>;

    /// <p> Gets a distribution configuration. </p>
    async fn get_distribution_configuration(
        &self,
        input: GetDistributionConfigurationRequest,
    ) -> Result<GetDistributionConfigurationResponse, RusotoError<GetDistributionConfigurationError>>;

    /// <p> Gets an image. </p>
    async fn get_image(
        &self,
        input: GetImageRequest,
    ) -> Result<GetImageResponse, RusotoError<GetImageError>>;

    /// <p> Gets an image pipeline. </p>
    async fn get_image_pipeline(
        &self,
        input: GetImagePipelineRequest,
    ) -> Result<GetImagePipelineResponse, RusotoError<GetImagePipelineError>>;

    /// <p> Gets an image policy. </p>
    async fn get_image_policy(
        &self,
        input: GetImagePolicyRequest,
    ) -> Result<GetImagePolicyResponse, RusotoError<GetImagePolicyError>>;

    /// <p> Gets an image recipe. </p>
    async fn get_image_recipe(
        &self,
        input: GetImageRecipeRequest,
    ) -> Result<GetImageRecipeResponse, RusotoError<GetImageRecipeError>>;

    /// <p> Gets an image recipe policy. </p>
    async fn get_image_recipe_policy(
        &self,
        input: GetImageRecipePolicyRequest,
    ) -> Result<GetImageRecipePolicyResponse, RusotoError<GetImageRecipePolicyError>>;

    /// <p> Gets an infrastructure configuration. </p>
    async fn get_infrastructure_configuration(
        &self,
        input: GetInfrastructureConfigurationRequest,
    ) -> Result<
        GetInfrastructureConfigurationResponse,
        RusotoError<GetInfrastructureConfigurationError>,
    >;

    /// <p>Imports a component and transforms its data into a component document. </p>
    async fn import_component(
        &self,
        input: ImportComponentRequest,
    ) -> Result<ImportComponentResponse, RusotoError<ImportComponentError>>;

    /// <p> Returns the list of component build versions for the specified semantic version. </p>
    async fn list_component_build_versions(
        &self,
        input: ListComponentBuildVersionsRequest,
    ) -> Result<ListComponentBuildVersionsResponse, RusotoError<ListComponentBuildVersionsError>>;

    /// <p>Returns the list of component build versions for the specified semantic version. </p>
    async fn list_components(
        &self,
        input: ListComponentsRequest,
    ) -> Result<ListComponentsResponse, RusotoError<ListComponentsError>>;

    /// <p> Returns a list of distribution configurations. </p>
    async fn list_distribution_configurations(
        &self,
        input: ListDistributionConfigurationsRequest,
    ) -> Result<
        ListDistributionConfigurationsResponse,
        RusotoError<ListDistributionConfigurationsError>,
    >;

    /// <p> Returns a list of image build versions. </p>
    async fn list_image_build_versions(
        &self,
        input: ListImageBuildVersionsRequest,
    ) -> Result<ListImageBuildVersionsResponse, RusotoError<ListImageBuildVersionsError>>;

    /// <p> Returns a list of images created by the specified pipeline. </p>
    async fn list_image_pipeline_images(
        &self,
        input: ListImagePipelineImagesRequest,
    ) -> Result<ListImagePipelineImagesResponse, RusotoError<ListImagePipelineImagesError>>;

    /// <p>Returns a list of image pipelines. </p>
    async fn list_image_pipelines(
        &self,
        input: ListImagePipelinesRequest,
    ) -> Result<ListImagePipelinesResponse, RusotoError<ListImagePipelinesError>>;

    /// <p> Returns a list of image recipes. </p>
    async fn list_image_recipes(
        &self,
        input: ListImageRecipesRequest,
    ) -> Result<ListImageRecipesResponse, RusotoError<ListImageRecipesError>>;

    /// <p> Returns the list of images that you have access to. </p>
    async fn list_images(
        &self,
        input: ListImagesRequest,
    ) -> Result<ListImagesResponse, RusotoError<ListImagesError>>;

    /// <p> Returns a list of infrastructure configurations. </p>
    async fn list_infrastructure_configurations(
        &self,
        input: ListInfrastructureConfigurationsRequest,
    ) -> Result<
        ListInfrastructureConfigurationsResponse,
        RusotoError<ListInfrastructureConfigurationsError>,
    >;

    /// <p> Returns the list of tags for the specified resource. </p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p> Applies a policy to a component. We recommend that you call the RAM API <a href="https://docs.aws.amazon.com/ram/latest/APIReference/API_CreateResourceShare.html">CreateResourceShare</a> to share resources. If you call the Image Builder API <code>PutComponentPolicy</code>, you must also call the RAM API <a href="https://docs.aws.amazon.com/ram/latest/APIReference/API_PromoteResourceShareCreatedFromPolicy.html">PromoteResourceShareCreatedFromPolicy</a> in order for the resource to be visible to all principals with whom the resource is shared. </p>
    async fn put_component_policy(
        &self,
        input: PutComponentPolicyRequest,
    ) -> Result<PutComponentPolicyResponse, RusotoError<PutComponentPolicyError>>;

    /// <p>Applies a policy to an image. We recommend that you call the RAM API <a href="https://docs.aws.amazon.com/ram/latest/APIReference/API_CreateResourceShare.html">CreateResourceShare</a> to share resources. If you call the Image Builder API <code>PutImagePolicy</code>, you must also call the RAM API <a href="https://docs.aws.amazon.com/ram/latest/APIReference/API_PromoteResourceShareCreatedFromPolicy.html">PromoteResourceShareCreatedFromPolicy</a> in order for the resource to be visible to all principals with whom the resource is shared. </p>
    async fn put_image_policy(
        &self,
        input: PutImagePolicyRequest,
    ) -> Result<PutImagePolicyResponse, RusotoError<PutImagePolicyError>>;

    /// <p> Applies a policy to an image recipe. We recommend that you call the RAM API <a href="https://docs.aws.amazon.com/ram/latest/APIReference/API_CreateResourceShare.html">CreateResourceShare</a> to share resources. If you call the Image Builder API <code>PutImageRecipePolicy</code>, you must also call the RAM API <a href="https://docs.aws.amazon.com/ram/latest/APIReference/API_PromoteResourceShareCreatedFromPolicy.html">PromoteResourceShareCreatedFromPolicy</a> in order for the resource to be visible to all principals with whom the resource is shared. </p>
    async fn put_image_recipe_policy(
        &self,
        input: PutImageRecipePolicyRequest,
    ) -> Result<PutImageRecipePolicyResponse, RusotoError<PutImageRecipePolicyError>>;

    /// <p> Manually triggers a pipeline to create an image. </p>
    async fn start_image_pipeline_execution(
        &self,
        input: StartImagePipelineExecutionRequest,
    ) -> Result<StartImagePipelineExecutionResponse, RusotoError<StartImagePipelineExecutionError>>;

    /// <p> Adds a tag to a resource. </p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>>;

    /// <p> Removes a tag from a resource. </p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>>;

    /// <p> Updates a new distribution configuration. Distribution configurations define and configure the outputs of your pipeline. </p>
    async fn update_distribution_configuration(
        &self,
        input: UpdateDistributionConfigurationRequest,
    ) -> Result<
        UpdateDistributionConfigurationResponse,
        RusotoError<UpdateDistributionConfigurationError>,
    >;

    /// <p> Updates a new image pipeline. Image pipelines enable you to automate the creation and distribution of images. </p>
    async fn update_image_pipeline(
        &self,
        input: UpdateImagePipelineRequest,
    ) -> Result<UpdateImagePipelineResponse, RusotoError<UpdateImagePipelineError>>;

    /// <p> Updates a new infrastructure configuration. An infrastructure configuration defines the environment in which your image will be built and tested. </p>
    async fn update_infrastructure_configuration(
        &self,
        input: UpdateInfrastructureConfigurationRequest,
    ) -> Result<
        UpdateInfrastructureConfigurationResponse,
        RusotoError<UpdateInfrastructureConfigurationError>,
    >;
}
/// A client for the imagebuilder API.
#[derive(Clone)]
pub struct ImageBuilderClient {
    client: Client,
    region: region::Region,
}

impl ImageBuilderClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> ImageBuilderClient {
        ImageBuilderClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> ImageBuilderClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        ImageBuilderClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> ImageBuilderClient {
        ImageBuilderClient { client, region }
    }
}

#[async_trait]
impl ImageBuilder for ImageBuilderClient {
    /// <p>CancelImageCreation cancels the creation of Image. This operation can only be used on images in a non-terminal state.</p>
    #[allow(unused_mut)]
    async fn cancel_image_creation(
        &self,
        input: CancelImageCreationRequest,
    ) -> Result<CancelImageCreationResponse, RusotoError<CancelImageCreationError>> {
        let request_uri = "/CancelImageCreation";

        let mut request = SignedRequest::new("PUT", "imagebuilder", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CancelImageCreationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CancelImageCreationError::from_response(response))
        }
    }

    /// <p>Creates a new component that can be used to build, validate, test, and assess your image.</p>
    #[allow(unused_mut)]
    async fn create_component(
        &self,
        input: CreateComponentRequest,
    ) -> Result<CreateComponentResponse, RusotoError<CreateComponentError>> {
        let request_uri = "/CreateComponent";

        let mut request = SignedRequest::new("PUT", "imagebuilder", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateComponentResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateComponentError::from_response(response))
        }
    }

    /// <p>Creates a new distribution configuration. Distribution configurations define and configure the outputs of your pipeline. </p>
    #[allow(unused_mut)]
    async fn create_distribution_configuration(
        &self,
        input: CreateDistributionConfigurationRequest,
    ) -> Result<
        CreateDistributionConfigurationResponse,
        RusotoError<CreateDistributionConfigurationError>,
    > {
        let request_uri = "/CreateDistributionConfiguration";

        let mut request = SignedRequest::new("PUT", "imagebuilder", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateDistributionConfigurationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateDistributionConfigurationError::from_response(
                response,
            ))
        }
    }

    /// <p> Creates a new image. This request will create a new image along with all of the configured output resources defined in the distribution configuration. </p>
    #[allow(unused_mut)]
    async fn create_image(
        &self,
        input: CreateImageRequest,
    ) -> Result<CreateImageResponse, RusotoError<CreateImageError>> {
        let request_uri = "/CreateImage";

        let mut request = SignedRequest::new("PUT", "imagebuilder", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateImageResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateImageError::from_response(response))
        }
    }

    /// <p> Creates a new image pipeline. Image pipelines enable you to automate the creation and distribution of images. </p>
    #[allow(unused_mut)]
    async fn create_image_pipeline(
        &self,
        input: CreateImagePipelineRequest,
    ) -> Result<CreateImagePipelineResponse, RusotoError<CreateImagePipelineError>> {
        let request_uri = "/CreateImagePipeline";

        let mut request = SignedRequest::new("PUT", "imagebuilder", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateImagePipelineResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateImagePipelineError::from_response(response))
        }
    }

    /// <p> Creates a new image recipe. Image recipes define how images are configured, tested, and assessed. </p>
    #[allow(unused_mut)]
    async fn create_image_recipe(
        &self,
        input: CreateImageRecipeRequest,
    ) -> Result<CreateImageRecipeResponse, RusotoError<CreateImageRecipeError>> {
        let request_uri = "/CreateImageRecipe";

        let mut request = SignedRequest::new("PUT", "imagebuilder", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateImageRecipeResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateImageRecipeError::from_response(response))
        }
    }

    /// <p> Creates a new infrastructure configuration. An infrastructure configuration defines the environment in which your image will be built and tested. </p>
    #[allow(unused_mut)]
    async fn create_infrastructure_configuration(
        &self,
        input: CreateInfrastructureConfigurationRequest,
    ) -> Result<
        CreateInfrastructureConfigurationResponse,
        RusotoError<CreateInfrastructureConfigurationError>,
    > {
        let request_uri = "/CreateInfrastructureConfiguration";

        let mut request = SignedRequest::new("PUT", "imagebuilder", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateInfrastructureConfigurationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateInfrastructureConfigurationError::from_response(
                response,
            ))
        }
    }

    /// <p> Deletes a component build version. </p>
    #[allow(unused_mut)]
    async fn delete_component(
        &self,
        input: DeleteComponentRequest,
    ) -> Result<DeleteComponentResponse, RusotoError<DeleteComponentError>> {
        let request_uri = "/DeleteComponent";

        let mut request = SignedRequest::new("DELETE", "imagebuilder", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        params.put(
            "componentBuildVersionArn",
            &input.component_build_version_arn,
        );
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteComponentResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteComponentError::from_response(response))
        }
    }

    /// <p> Deletes a distribution configuration. </p>
    #[allow(unused_mut)]
    async fn delete_distribution_configuration(
        &self,
        input: DeleteDistributionConfigurationRequest,
    ) -> Result<
        DeleteDistributionConfigurationResponse,
        RusotoError<DeleteDistributionConfigurationError>,
    > {
        let request_uri = "/DeleteDistributionConfiguration";

        let mut request = SignedRequest::new("DELETE", "imagebuilder", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        params.put(
            "distributionConfigurationArn",
            &input.distribution_configuration_arn,
        );
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteDistributionConfigurationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteDistributionConfigurationError::from_response(
                response,
            ))
        }
    }

    /// <p> Deletes an image. </p>
    #[allow(unused_mut)]
    async fn delete_image(
        &self,
        input: DeleteImageRequest,
    ) -> Result<DeleteImageResponse, RusotoError<DeleteImageError>> {
        let request_uri = "/DeleteImage";

        let mut request = SignedRequest::new("DELETE", "imagebuilder", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        params.put("imageBuildVersionArn", &input.image_build_version_arn);
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteImageResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteImageError::from_response(response))
        }
    }

    /// <p> Deletes an image pipeline. </p>
    #[allow(unused_mut)]
    async fn delete_image_pipeline(
        &self,
        input: DeleteImagePipelineRequest,
    ) -> Result<DeleteImagePipelineResponse, RusotoError<DeleteImagePipelineError>> {
        let request_uri = "/DeleteImagePipeline";

        let mut request = SignedRequest::new("DELETE", "imagebuilder", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        params.put("imagePipelineArn", &input.image_pipeline_arn);
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteImagePipelineResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteImagePipelineError::from_response(response))
        }
    }

    /// <p> Deletes an image recipe. </p>
    #[allow(unused_mut)]
    async fn delete_image_recipe(
        &self,
        input: DeleteImageRecipeRequest,
    ) -> Result<DeleteImageRecipeResponse, RusotoError<DeleteImageRecipeError>> {
        let request_uri = "/DeleteImageRecipe";

        let mut request = SignedRequest::new("DELETE", "imagebuilder", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        params.put("imageRecipeArn", &input.image_recipe_arn);
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteImageRecipeResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteImageRecipeError::from_response(response))
        }
    }

    /// <p> Deletes an infrastructure configuration. </p>
    #[allow(unused_mut)]
    async fn delete_infrastructure_configuration(
        &self,
        input: DeleteInfrastructureConfigurationRequest,
    ) -> Result<
        DeleteInfrastructureConfigurationResponse,
        RusotoError<DeleteInfrastructureConfigurationError>,
    > {
        let request_uri = "/DeleteInfrastructureConfiguration";

        let mut request = SignedRequest::new("DELETE", "imagebuilder", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        params.put(
            "infrastructureConfigurationArn",
            &input.infrastructure_configuration_arn,
        );
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteInfrastructureConfigurationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteInfrastructureConfigurationError::from_response(
                response,
            ))
        }
    }

    /// <p> Gets a component object. </p>
    #[allow(unused_mut)]
    async fn get_component(
        &self,
        input: GetComponentRequest,
    ) -> Result<GetComponentResponse, RusotoError<GetComponentError>> {
        let request_uri = "/GetComponent";

        let mut request = SignedRequest::new("GET", "imagebuilder", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        params.put(
            "componentBuildVersionArn",
            &input.component_build_version_arn,
        );
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetComponentResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetComponentError::from_response(response))
        }
    }

    /// <p> Gets a component policy. </p>
    #[allow(unused_mut)]
    async fn get_component_policy(
        &self,
        input: GetComponentPolicyRequest,
    ) -> Result<GetComponentPolicyResponse, RusotoError<GetComponentPolicyError>> {
        let request_uri = "/GetComponentPolicy";

        let mut request = SignedRequest::new("GET", "imagebuilder", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        params.put("componentArn", &input.component_arn);
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetComponentPolicyResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetComponentPolicyError::from_response(response))
        }
    }

    /// <p> Gets a distribution configuration. </p>
    #[allow(unused_mut)]
    async fn get_distribution_configuration(
        &self,
        input: GetDistributionConfigurationRequest,
    ) -> Result<GetDistributionConfigurationResponse, RusotoError<GetDistributionConfigurationError>>
    {
        let request_uri = "/GetDistributionConfiguration";

        let mut request = SignedRequest::new("GET", "imagebuilder", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        params.put(
            "distributionConfigurationArn",
            &input.distribution_configuration_arn,
        );
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetDistributionConfigurationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetDistributionConfigurationError::from_response(response))
        }
    }

    /// <p> Gets an image. </p>
    #[allow(unused_mut)]
    async fn get_image(
        &self,
        input: GetImageRequest,
    ) -> Result<GetImageResponse, RusotoError<GetImageError>> {
        let request_uri = "/GetImage";

        let mut request = SignedRequest::new("GET", "imagebuilder", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        params.put("imageBuildVersionArn", &input.image_build_version_arn);
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetImageResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetImageError::from_response(response))
        }
    }

    /// <p> Gets an image pipeline. </p>
    #[allow(unused_mut)]
    async fn get_image_pipeline(
        &self,
        input: GetImagePipelineRequest,
    ) -> Result<GetImagePipelineResponse, RusotoError<GetImagePipelineError>> {
        let request_uri = "/GetImagePipeline";

        let mut request = SignedRequest::new("GET", "imagebuilder", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        params.put("imagePipelineArn", &input.image_pipeline_arn);
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetImagePipelineResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetImagePipelineError::from_response(response))
        }
    }

    /// <p> Gets an image policy. </p>
    #[allow(unused_mut)]
    async fn get_image_policy(
        &self,
        input: GetImagePolicyRequest,
    ) -> Result<GetImagePolicyResponse, RusotoError<GetImagePolicyError>> {
        let request_uri = "/GetImagePolicy";

        let mut request = SignedRequest::new("GET", "imagebuilder", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        params.put("imageArn", &input.image_arn);
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetImagePolicyResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetImagePolicyError::from_response(response))
        }
    }

    /// <p> Gets an image recipe. </p>
    #[allow(unused_mut)]
    async fn get_image_recipe(
        &self,
        input: GetImageRecipeRequest,
    ) -> Result<GetImageRecipeResponse, RusotoError<GetImageRecipeError>> {
        let request_uri = "/GetImageRecipe";

        let mut request = SignedRequest::new("GET", "imagebuilder", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        params.put("imageRecipeArn", &input.image_recipe_arn);
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetImageRecipeResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetImageRecipeError::from_response(response))
        }
    }

    /// <p> Gets an image recipe policy. </p>
    #[allow(unused_mut)]
    async fn get_image_recipe_policy(
        &self,
        input: GetImageRecipePolicyRequest,
    ) -> Result<GetImageRecipePolicyResponse, RusotoError<GetImageRecipePolicyError>> {
        let request_uri = "/GetImageRecipePolicy";

        let mut request = SignedRequest::new("GET", "imagebuilder", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        params.put("imageRecipeArn", &input.image_recipe_arn);
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetImageRecipePolicyResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetImageRecipePolicyError::from_response(response))
        }
    }

    /// <p> Gets an infrastructure configuration. </p>
    #[allow(unused_mut)]
    async fn get_infrastructure_configuration(
        &self,
        input: GetInfrastructureConfigurationRequest,
    ) -> Result<
        GetInfrastructureConfigurationResponse,
        RusotoError<GetInfrastructureConfigurationError>,
    > {
        let request_uri = "/GetInfrastructureConfiguration";

        let mut request = SignedRequest::new("GET", "imagebuilder", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        params.put(
            "infrastructureConfigurationArn",
            &input.infrastructure_configuration_arn,
        );
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetInfrastructureConfigurationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetInfrastructureConfigurationError::from_response(response))
        }
    }

    /// <p>Imports a component and transforms its data into a component document. </p>
    #[allow(unused_mut)]
    async fn import_component(
        &self,
        input: ImportComponentRequest,
    ) -> Result<ImportComponentResponse, RusotoError<ImportComponentError>> {
        let request_uri = "/ImportComponent";

        let mut request = SignedRequest::new("PUT", "imagebuilder", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ImportComponentResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ImportComponentError::from_response(response))
        }
    }

    /// <p> Returns the list of component build versions for the specified semantic version. </p>
    #[allow(unused_mut)]
    async fn list_component_build_versions(
        &self,
        input: ListComponentBuildVersionsRequest,
    ) -> Result<ListComponentBuildVersionsResponse, RusotoError<ListComponentBuildVersionsError>>
    {
        let request_uri = "/ListComponentBuildVersions";

        let mut request = SignedRequest::new("POST", "imagebuilder", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListComponentBuildVersionsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListComponentBuildVersionsError::from_response(response))
        }
    }

    /// <p>Returns the list of component build versions for the specified semantic version. </p>
    #[allow(unused_mut)]
    async fn list_components(
        &self,
        input: ListComponentsRequest,
    ) -> Result<ListComponentsResponse, RusotoError<ListComponentsError>> {
        let request_uri = "/ListComponents";

        let mut request = SignedRequest::new("POST", "imagebuilder", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListComponentsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListComponentsError::from_response(response))
        }
    }

    /// <p> Returns a list of distribution configurations. </p>
    #[allow(unused_mut)]
    async fn list_distribution_configurations(
        &self,
        input: ListDistributionConfigurationsRequest,
    ) -> Result<
        ListDistributionConfigurationsResponse,
        RusotoError<ListDistributionConfigurationsError>,
    > {
        let request_uri = "/ListDistributionConfigurations";

        let mut request = SignedRequest::new("POST", "imagebuilder", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListDistributionConfigurationsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListDistributionConfigurationsError::from_response(response))
        }
    }

    /// <p> Returns a list of image build versions. </p>
    #[allow(unused_mut)]
    async fn list_image_build_versions(
        &self,
        input: ListImageBuildVersionsRequest,
    ) -> Result<ListImageBuildVersionsResponse, RusotoError<ListImageBuildVersionsError>> {
        let request_uri = "/ListImageBuildVersions";

        let mut request = SignedRequest::new("POST", "imagebuilder", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListImageBuildVersionsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListImageBuildVersionsError::from_response(response))
        }
    }

    /// <p> Returns a list of images created by the specified pipeline. </p>
    #[allow(unused_mut)]
    async fn list_image_pipeline_images(
        &self,
        input: ListImagePipelineImagesRequest,
    ) -> Result<ListImagePipelineImagesResponse, RusotoError<ListImagePipelineImagesError>> {
        let request_uri = "/ListImagePipelineImages";

        let mut request = SignedRequest::new("POST", "imagebuilder", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListImagePipelineImagesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListImagePipelineImagesError::from_response(response))
        }
    }

    /// <p>Returns a list of image pipelines. </p>
    #[allow(unused_mut)]
    async fn list_image_pipelines(
        &self,
        input: ListImagePipelinesRequest,
    ) -> Result<ListImagePipelinesResponse, RusotoError<ListImagePipelinesError>> {
        let request_uri = "/ListImagePipelines";

        let mut request = SignedRequest::new("POST", "imagebuilder", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListImagePipelinesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListImagePipelinesError::from_response(response))
        }
    }

    /// <p> Returns a list of image recipes. </p>
    #[allow(unused_mut)]
    async fn list_image_recipes(
        &self,
        input: ListImageRecipesRequest,
    ) -> Result<ListImageRecipesResponse, RusotoError<ListImageRecipesError>> {
        let request_uri = "/ListImageRecipes";

        let mut request = SignedRequest::new("POST", "imagebuilder", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListImageRecipesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListImageRecipesError::from_response(response))
        }
    }

    /// <p> Returns the list of images that you have access to. </p>
    #[allow(unused_mut)]
    async fn list_images(
        &self,
        input: ListImagesRequest,
    ) -> Result<ListImagesResponse, RusotoError<ListImagesError>> {
        let request_uri = "/ListImages";

        let mut request = SignedRequest::new("POST", "imagebuilder", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListImagesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListImagesError::from_response(response))
        }
    }

    /// <p> Returns a list of infrastructure configurations. </p>
    #[allow(unused_mut)]
    async fn list_infrastructure_configurations(
        &self,
        input: ListInfrastructureConfigurationsRequest,
    ) -> Result<
        ListInfrastructureConfigurationsResponse,
        RusotoError<ListInfrastructureConfigurationsError>,
    > {
        let request_uri = "/ListInfrastructureConfigurations";

        let mut request = SignedRequest::new("POST", "imagebuilder", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListInfrastructureConfigurationsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListInfrastructureConfigurationsError::from_response(
                response,
            ))
        }
    }

    /// <p> Returns the list of tags for the specified resource. </p>
    #[allow(unused_mut)]
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("GET", "imagebuilder", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListTagsForResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListTagsForResourceError::from_response(response))
        }
    }

    /// <p> Applies a policy to a component. We recommend that you call the RAM API <a href="https://docs.aws.amazon.com/ram/latest/APIReference/API_CreateResourceShare.html">CreateResourceShare</a> to share resources. If you call the Image Builder API <code>PutComponentPolicy</code>, you must also call the RAM API <a href="https://docs.aws.amazon.com/ram/latest/APIReference/API_PromoteResourceShareCreatedFromPolicy.html">PromoteResourceShareCreatedFromPolicy</a> in order for the resource to be visible to all principals with whom the resource is shared. </p>
    #[allow(unused_mut)]
    async fn put_component_policy(
        &self,
        input: PutComponentPolicyRequest,
    ) -> Result<PutComponentPolicyResponse, RusotoError<PutComponentPolicyError>> {
        let request_uri = "/PutComponentPolicy";

        let mut request = SignedRequest::new("PUT", "imagebuilder", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PutComponentPolicyResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutComponentPolicyError::from_response(response))
        }
    }

    /// <p>Applies a policy to an image. We recommend that you call the RAM API <a href="https://docs.aws.amazon.com/ram/latest/APIReference/API_CreateResourceShare.html">CreateResourceShare</a> to share resources. If you call the Image Builder API <code>PutImagePolicy</code>, you must also call the RAM API <a href="https://docs.aws.amazon.com/ram/latest/APIReference/API_PromoteResourceShareCreatedFromPolicy.html">PromoteResourceShareCreatedFromPolicy</a> in order for the resource to be visible to all principals with whom the resource is shared. </p>
    #[allow(unused_mut)]
    async fn put_image_policy(
        &self,
        input: PutImagePolicyRequest,
    ) -> Result<PutImagePolicyResponse, RusotoError<PutImagePolicyError>> {
        let request_uri = "/PutImagePolicy";

        let mut request = SignedRequest::new("PUT", "imagebuilder", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PutImagePolicyResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutImagePolicyError::from_response(response))
        }
    }

    /// <p> Applies a policy to an image recipe. We recommend that you call the RAM API <a href="https://docs.aws.amazon.com/ram/latest/APIReference/API_CreateResourceShare.html">CreateResourceShare</a> to share resources. If you call the Image Builder API <code>PutImageRecipePolicy</code>, you must also call the RAM API <a href="https://docs.aws.amazon.com/ram/latest/APIReference/API_PromoteResourceShareCreatedFromPolicy.html">PromoteResourceShareCreatedFromPolicy</a> in order for the resource to be visible to all principals with whom the resource is shared. </p>
    #[allow(unused_mut)]
    async fn put_image_recipe_policy(
        &self,
        input: PutImageRecipePolicyRequest,
    ) -> Result<PutImageRecipePolicyResponse, RusotoError<PutImageRecipePolicyError>> {
        let request_uri = "/PutImageRecipePolicy";

        let mut request = SignedRequest::new("PUT", "imagebuilder", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PutImageRecipePolicyResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutImageRecipePolicyError::from_response(response))
        }
    }

    /// <p> Manually triggers a pipeline to create an image. </p>
    #[allow(unused_mut)]
    async fn start_image_pipeline_execution(
        &self,
        input: StartImagePipelineExecutionRequest,
    ) -> Result<StartImagePipelineExecutionResponse, RusotoError<StartImagePipelineExecutionError>>
    {
        let request_uri = "/StartImagePipelineExecution";

        let mut request = SignedRequest::new("PUT", "imagebuilder", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<StartImagePipelineExecutionResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(StartImagePipelineExecutionError::from_response(response))
        }
    }

    /// <p> Adds a tag to a resource. </p>
    #[allow(unused_mut)]
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("POST", "imagebuilder", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<TagResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(TagResourceError::from_response(response))
        }
    }

    /// <p> Removes a tag from a resource. </p>
    #[allow(unused_mut)]
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("DELETE", "imagebuilder", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        for item in input.tag_keys.iter() {
            params.put("tagKeys", item);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UntagResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UntagResourceError::from_response(response))
        }
    }

    /// <p> Updates a new distribution configuration. Distribution configurations define and configure the outputs of your pipeline. </p>
    #[allow(unused_mut)]
    async fn update_distribution_configuration(
        &self,
        input: UpdateDistributionConfigurationRequest,
    ) -> Result<
        UpdateDistributionConfigurationResponse,
        RusotoError<UpdateDistributionConfigurationError>,
    > {
        let request_uri = "/UpdateDistributionConfiguration";

        let mut request = SignedRequest::new("PUT", "imagebuilder", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateDistributionConfigurationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateDistributionConfigurationError::from_response(
                response,
            ))
        }
    }

    /// <p> Updates a new image pipeline. Image pipelines enable you to automate the creation and distribution of images. </p>
    #[allow(unused_mut)]
    async fn update_image_pipeline(
        &self,
        input: UpdateImagePipelineRequest,
    ) -> Result<UpdateImagePipelineResponse, RusotoError<UpdateImagePipelineError>> {
        let request_uri = "/UpdateImagePipeline";

        let mut request = SignedRequest::new("PUT", "imagebuilder", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateImagePipelineResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateImagePipelineError::from_response(response))
        }
    }

    /// <p> Updates a new infrastructure configuration. An infrastructure configuration defines the environment in which your image will be built and tested. </p>
    #[allow(unused_mut)]
    async fn update_infrastructure_configuration(
        &self,
        input: UpdateInfrastructureConfigurationRequest,
    ) -> Result<
        UpdateInfrastructureConfigurationResponse,
        RusotoError<UpdateInfrastructureConfigurationError>,
    > {
        let request_uri = "/UpdateInfrastructureConfiguration";

        let mut request = SignedRequest::new("PUT", "imagebuilder", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateInfrastructureConfigurationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateInfrastructureConfigurationError::from_response(
                response,
            ))
        }
    }
}
