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

use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
use serde::{Deserialize, Serialize};
use serde_json;
/// <p>Details about license consumption.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ConsumedLicenseSummary {
    /// <p>Number of licenses consumed by a resource.</p>
    #[serde(rename = "ConsumedLicenses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_licenses: Option<i64>,
    /// <p>Resource type of the resource consuming a license (instance, host, or AMI).</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateLicenseConfigurationRequest {
    /// <p>Human-friendly description of the license configuration.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Number of licenses managed by the license configuration.</p>
    #[serde(rename = "LicenseCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_count: Option<i64>,
    /// <p>Flag indicating whether hard or soft license enforcement is used. Exceeding a hard limit results in the blocked deployment of new instances.</p>
    #[serde(rename = "LicenseCountHardLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_count_hard_limit: Option<bool>,
    /// <p>Dimension to use to track the license inventory.</p>
    #[serde(rename = "LicenseCountingType")]
    pub license_counting_type: String,
    /// <p>Array of configured License Manager rules.</p>
    #[serde(rename = "LicenseRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_rules: Option<Vec<String>>,
    /// <p>Name of the license configuration.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p><p>The tags to apply to the resources during launch. You can only tag instances and volumes on launch. The specified tags are applied to all instances or volumes that are created during launch. To tag a resource after it has been created, see CreateTags .</p> <p/></p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateLicenseConfigurationResponse {
    /// <p>ARN of the license configuration object after its creation.</p>
    #[serde(rename = "LicenseConfigurationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_configuration_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteLicenseConfigurationRequest {
    /// <p>Unique ID of the configuration object to delete.</p>
    #[serde(rename = "LicenseConfigurationArn")]
    pub license_configuration_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteLicenseConfigurationResponse {}

/// <p>A filter name and value pair that is used to return a more specific list of results from a describe operation. Filters can be used to match a set of resources by specific criteria, such as tags, attributes, or IDs. The filters supported by a <code>Describe</code> operation are documented with the <code>Describe</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct Filter {
    /// <p>Name of the filter. Filter names are case-sensitive.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>One or more filter values. Filter values are case-sensitive.</p>
    #[serde(rename = "Values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetLicenseConfigurationRequest {
    /// <p>ARN of the license configuration being requested.</p>
    #[serde(rename = "LicenseConfigurationArn")]
    pub license_configuration_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetLicenseConfigurationResponse {
    /// <p>List of summaries for consumed licenses used by various resources.</p>
    #[serde(rename = "ConsumedLicenseSummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_license_summary_list: Option<Vec<ConsumedLicenseSummary>>,
    /// <p>Number of licenses assigned to resources.</p>
    #[serde(rename = "ConsumedLicenses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_licenses: Option<i64>,
    /// <p>Description of the license configuration.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>ARN of the license configuration requested.</p>
    #[serde(rename = "LicenseConfigurationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_configuration_arn: Option<String>,
    /// <p>Unique ID for the license configuration.</p>
    #[serde(rename = "LicenseConfigurationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_configuration_id: Option<String>,
    /// <p>Number of available licenses.</p>
    #[serde(rename = "LicenseCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_count: Option<i64>,
    /// <p>Sets the number of available licenses as a hard limit.</p>
    #[serde(rename = "LicenseCountHardLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_count_hard_limit: Option<bool>,
    /// <p>Dimension on which the licenses are counted (for example, instances, cores, sockets, or VCPUs).</p>
    #[serde(rename = "LicenseCountingType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_counting_type: Option<String>,
    /// <p>List of flexible text strings designating license rules.</p>
    #[serde(rename = "LicenseRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_rules: Option<Vec<String>>,
    /// <p>List of summaries of managed resources.</p>
    #[serde(rename = "ManagedResourceSummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_resource_summary_list: Option<Vec<ManagedResourceSummary>>,
    /// <p>Name of the license configuration.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Owner account ID for the license configuration.</p>
    #[serde(rename = "OwnerAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_account_id: Option<String>,
    /// <p>License configuration status (active, etc.).</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>List of tags attached to the license configuration.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetServiceSettingsRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetServiceSettingsResponse {
    /// <p>Indicates whether cross-account discovery has been enabled.</p>
    #[serde(rename = "EnableCrossAccountsDiscovery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_cross_accounts_discovery: Option<bool>,
    /// <p>Indicates whether AWS Organizations has been integrated with License Manager for cross-account discovery.</p>
    #[serde(rename = "OrganizationConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_configuration: Option<OrganizationConfiguration>,
    /// <p>Regional S3 bucket path for storing reports, license trail event data, discovery data, etc.</p>
    #[serde(rename = "S3BucketArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_arn: Option<String>,
    /// <p>SNS topic configured to receive notifications from License Manager.</p>
    #[serde(rename = "SnsTopicArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arn: Option<String>,
}

/// <p>An inventory filter object.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct InventoryFilter {
    /// <p>The condition of the filter.</p>
    #[serde(rename = "Condition")]
    pub condition: String,
    /// <p>The name of the filter.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>Value of the filter.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>A license configuration is an abstraction of a customer license agreement that can be consumed and enforced by License Manager. Components include specifications for the license type (licensing by instance, socket, CPU, or VCPU), tenancy (shared tenancy, Amazon EC2 Dedicated Instance, Amazon EC2 Dedicated Host, or any of these), host affinity (how long a VM must be associated with a host), the number of licenses purchased and used.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct LicenseConfiguration {
    /// <p>List of summaries for licenses consumed by various resources.</p>
    #[serde(rename = "ConsumedLicenseSummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_license_summary_list: Option<Vec<ConsumedLicenseSummary>>,
    /// <p>Number of licenses consumed. </p>
    #[serde(rename = "ConsumedLicenses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_licenses: Option<i64>,
    /// <p>Description of the license configuration.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>ARN of the <code>LicenseConfiguration</code> object.</p>
    #[serde(rename = "LicenseConfigurationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_configuration_arn: Option<String>,
    /// <p>Unique ID of the <code>LicenseConfiguration</code> object.</p>
    #[serde(rename = "LicenseConfigurationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_configuration_id: Option<String>,
    /// <p>Number of licenses managed by the license configuration.</p>
    #[serde(rename = "LicenseCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_count: Option<i64>,
    /// <p>Sets the number of available licenses as a hard limit.</p>
    #[serde(rename = "LicenseCountHardLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_count_hard_limit: Option<bool>,
    /// <p>Dimension to use to track license inventory.</p>
    #[serde(rename = "LicenseCountingType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_counting_type: Option<String>,
    /// <p>Array of configured License Manager rules.</p>
    #[serde(rename = "LicenseRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_rules: Option<Vec<String>>,
    /// <p>List of summaries for managed resources.</p>
    #[serde(rename = "ManagedResourceSummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_resource_summary_list: Option<Vec<ManagedResourceSummary>>,
    /// <p>Name of the license configuration.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Account ID of the license configuration's owner.</p>
    #[serde(rename = "OwnerAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_account_id: Option<String>,
    /// <p>Status of the license configuration.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Describes a server resource that is associated with a license configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct LicenseConfigurationAssociation {
    /// <p>Time when the license configuration was associated with the resource.</p>
    #[serde(rename = "AssociationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_time: Option<f64>,
    /// <p>ARN of the resource associated with the license configuration.</p>
    #[serde(rename = "ResourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    /// <p>ID of the AWS account that owns the resource consuming licenses.</p>
    #[serde(rename = "ResourceOwnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_owner_id: Option<String>,
    /// <p>Type of server resource.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

/// <p>Contains details of the usage of each resource from the license pool.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct LicenseConfigurationUsage {
    /// <p>Time when the license configuration was initially associated with a resource.</p>
    #[serde(rename = "AssociationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_time: Option<f64>,
    /// <p>Number of licenses consumed out of the total provisioned in the license configuration.</p>
    #[serde(rename = "ConsumedLicenses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_licenses: Option<i64>,
    /// <p>ARN of the resource associated with a license configuration.</p>
    #[serde(rename = "ResourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    /// <p>ID of the account that owns a resource that is associated with the license configuration.</p>
    #[serde(rename = "ResourceOwnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_owner_id: Option<String>,
    /// <p>Status of a resource associated with the license configuration.</p>
    #[serde(rename = "ResourceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_status: Option<String>,
    /// <p>Type of resource associated with athe license configuration.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

/// <p>Object used for associating a license configuration with a resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LicenseSpecification {
    /// <p>ARN of the <code>LicenseConfiguration</code> object.</p>
    #[serde(rename = "LicenseConfigurationArn")]
    pub license_configuration_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListAssociationsForLicenseConfigurationRequest {
    /// <p>ARN of a <code>LicenseConfiguration</code> object.</p>
    #[serde(rename = "LicenseConfigurationArn")]
    pub license_configuration_arn: String,
    /// <p>Maximum number of results to return in a single call. To retrieve the remaining results, make another call with the returned <code>NextToken</code> value.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListAssociationsForLicenseConfigurationResponse {
    /// <p>Lists association objects for the license configuration, each containing the association time, number of consumed licenses, resource ARN, resource ID, account ID that owns the resource, resource size, and resource type.</p>
    #[serde(rename = "LicenseConfigurationAssociations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_configuration_associations: Option<Vec<LicenseConfigurationAssociation>>,
    /// <p>Token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListLicenseConfigurationsRequest {
    /// <p>One or more filters.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>An array of ARNs for the calling account’s license configurations.</p>
    #[serde(rename = "LicenseConfigurationArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_configuration_arns: Option<Vec<String>>,
    /// <p>Maximum number of results to return in a single call. To retrieve the remaining results, make another call with the returned <code>NextToken</code> value.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListLicenseConfigurationsResponse {
    /// <p>Array of license configuration objects.</p>
    #[serde(rename = "LicenseConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_configurations: Option<Vec<LicenseConfiguration>>,
    /// <p>Token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListLicenseSpecificationsForResourceRequest {
    /// <p>Maximum number of results to return in a single call. To retrieve the remaining results, make another call with the returned <code>NextToken</code> value.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>ARN of an AMI or Amazon EC2 instance that has an associated license configuration.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListLicenseSpecificationsForResourceResponse {
    /// <p>License configurations associated with a resource.</p>
    #[serde(rename = "LicenseSpecifications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_specifications: Option<Vec<LicenseSpecification>>,
    /// <p>Token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListResourceInventoryRequest {
    /// <p>One or more filters.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<InventoryFilter>>,
    /// <p>Maximum number of results to return in a single call. To retrieve the remaining results, make another call with the returned <code>NextToken</code> value.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListResourceInventoryResponse {
    /// <p>Token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The detailed list of resources.</p>
    #[serde(rename = "ResourceInventoryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_inventory_list: Option<Vec<ResourceInventory>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListTagsForResourceRequest {
    /// <p>ARN for the resource.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>List of tags attached to the resource.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListUsageForLicenseConfigurationRequest {
    /// <p>List of filters to apply.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>ARN of the targeted <code>LicenseConfiguration</code> object.</p>
    #[serde(rename = "LicenseConfigurationArn")]
    pub license_configuration_arn: String,
    /// <p>Maximum number of results to return in a single call. To retrieve the remaining results, make another call with the returned <code>NextToken</code> value.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListUsageForLicenseConfigurationResponse {
    /// <p>An array of <code>LicenseConfigurationUsage</code> objects.</p>
    #[serde(rename = "LicenseConfigurationUsageList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_configuration_usage_list: Option<Vec<LicenseConfigurationUsage>>,
    /// <p>Token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Summary for a resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ManagedResourceSummary {
    /// <p>Number of resources associated with licenses.</p>
    #[serde(rename = "AssociationCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_count: Option<i64>,
    /// <p>Type of resource associated with a license (instance, host, or AMI).</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

/// <p>Object containing configuration information for AWS Organizations.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrganizationConfiguration {
    /// <p>Flag to activate AWS Organization integration.</p>
    #[serde(rename = "EnableIntegration")]
    pub enable_integration: bool,
}

/// <p>A set of attributes that describe a resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ResourceInventory {
    /// <p>The platform of the resource.</p>
    #[serde(rename = "Platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// <p>Platform version of the resource in the inventory.</p>
    #[serde(rename = "PlatformVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<String>,
    /// <p>The ARN of the resource.</p>
    #[serde(rename = "ResourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    /// <p>Unique ID of the resource.</p>
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// <p>Unique ID of the account that owns the resource.</p>
    #[serde(rename = "ResourceOwningAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_owning_account_id: Option<String>,
    /// <p>The type of resource.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

/// <p>Tag for a resource in a key-value format.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>Key for the resource tag.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>Value for the resource tag.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TagResourceRequest {
    /// <p>Resource of the ARN to be tagged.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>Names of the tags to attach to the resource.</p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct TagResourceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UntagResourceRequest {
    /// <p>ARN of the resource.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>List keys identifying tags to remove.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UntagResourceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateLicenseConfigurationRequest {
    /// <p>New human-friendly description of the license configuration.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>ARN for a license configuration.</p>
    #[serde(rename = "LicenseConfigurationArn")]
    pub license_configuration_arn: String,
    /// <p>New status of the license configuration (<code>ACTIVE</code> or <code>INACTIVE</code>).</p>
    #[serde(rename = "LicenseConfigurationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_configuration_status: Option<String>,
    /// <p>New number of licenses managed by the license configuration.</p>
    #[serde(rename = "LicenseCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_count: Option<i64>,
    /// <p>Sets the number of available licenses as a hard limit.</p>
    #[serde(rename = "LicenseCountHardLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_count_hard_limit: Option<bool>,
    /// <p>List of flexible text strings designating license rules.</p>
    #[serde(rename = "LicenseRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_rules: Option<Vec<String>>,
    /// <p>New name of the license configuration.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateLicenseConfigurationResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateLicenseSpecificationsForResourceRequest {
    /// <p>License configuration ARNs to be added to a resource.</p>
    #[serde(rename = "AddLicenseSpecifications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_license_specifications: Option<Vec<LicenseSpecification>>,
    /// <p>License configuration ARNs to be removed from a resource.</p>
    #[serde(rename = "RemoveLicenseSpecifications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_license_specifications: Option<Vec<LicenseSpecification>>,
    /// <p>ARN for an AWS server resource.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateLicenseSpecificationsForResourceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateServiceSettingsRequest {
    /// <p>Activates cross-account discovery.</p>
    #[serde(rename = "EnableCrossAccountsDiscovery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_cross_accounts_discovery: Option<bool>,
    /// <p>Integrates AWS Organizations with License Manager for cross-account discovery.</p>
    #[serde(rename = "OrganizationConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_configuration: Option<OrganizationConfiguration>,
    /// <p>ARN of the Amazon S3 bucket where License Manager information is stored.</p>
    #[serde(rename = "S3BucketArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_arn: Option<String>,
    /// <p>ARN of the Amazon SNS topic used for License Manager alerts.</p>
    #[serde(rename = "SnsTopicArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateServiceSettingsResponse {}

/// Errors returned by CreateLicenseConfiguration
#[derive(Debug, PartialEq)]
pub enum CreateLicenseConfigurationError {
    /// <p>Access to resource denied.</p>
    AccessDenied(String),
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    Authorization(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValue(String),
    /// <p>Too many requests have been submitted. Try again after a brief wait.</p>
    RateLimitExceeded(String),
    /// <p>Your resource limits have been exceeded.</p>
    ResourceLimitExceeded(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternal(String),
}

impl CreateLicenseConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateLicenseConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateLicenseConfigurationError::AccessDenied(
                        err.msg,
                    ))
                }
                "AuthorizationException" => {
                    return RusotoError::Service(CreateLicenseConfigurationError::Authorization(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        CreateLicenseConfigurationError::InvalidParameterValue(err.msg),
                    )
                }
                "RateLimitExceededException" => {
                    return RusotoError::Service(
                        CreateLicenseConfigurationError::RateLimitExceeded(err.msg),
                    )
                }
                "ResourceLimitExceededException" => {
                    return RusotoError::Service(
                        CreateLicenseConfigurationError::ResourceLimitExceeded(err.msg),
                    )
                }
                "ServerInternalException" => {
                    return RusotoError::Service(CreateLicenseConfigurationError::ServerInternal(
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
impl fmt::Display for CreateLicenseConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateLicenseConfigurationError {
    fn description(&self) -> &str {
        match *self {
            CreateLicenseConfigurationError::AccessDenied(ref cause) => cause,
            CreateLicenseConfigurationError::Authorization(ref cause) => cause,
            CreateLicenseConfigurationError::InvalidParameterValue(ref cause) => cause,
            CreateLicenseConfigurationError::RateLimitExceeded(ref cause) => cause,
            CreateLicenseConfigurationError::ResourceLimitExceeded(ref cause) => cause,
            CreateLicenseConfigurationError::ServerInternal(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteLicenseConfiguration
#[derive(Debug, PartialEq)]
pub enum DeleteLicenseConfigurationError {
    /// <p>Access to resource denied.</p>
    AccessDenied(String),
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    Authorization(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValue(String),
    /// <p>Too many requests have been submitted. Try again after a brief wait.</p>
    RateLimitExceeded(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternal(String),
}

impl DeleteLicenseConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteLicenseConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteLicenseConfigurationError::AccessDenied(
                        err.msg,
                    ))
                }
                "AuthorizationException" => {
                    return RusotoError::Service(DeleteLicenseConfigurationError::Authorization(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        DeleteLicenseConfigurationError::InvalidParameterValue(err.msg),
                    )
                }
                "RateLimitExceededException" => {
                    return RusotoError::Service(
                        DeleteLicenseConfigurationError::RateLimitExceeded(err.msg),
                    )
                }
                "ServerInternalException" => {
                    return RusotoError::Service(DeleteLicenseConfigurationError::ServerInternal(
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
impl fmt::Display for DeleteLicenseConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteLicenseConfigurationError {
    fn description(&self) -> &str {
        match *self {
            DeleteLicenseConfigurationError::AccessDenied(ref cause) => cause,
            DeleteLicenseConfigurationError::Authorization(ref cause) => cause,
            DeleteLicenseConfigurationError::InvalidParameterValue(ref cause) => cause,
            DeleteLicenseConfigurationError::RateLimitExceeded(ref cause) => cause,
            DeleteLicenseConfigurationError::ServerInternal(ref cause) => cause,
        }
    }
}
/// Errors returned by GetLicenseConfiguration
#[derive(Debug, PartialEq)]
pub enum GetLicenseConfigurationError {
    /// <p>Access to resource denied.</p>
    AccessDenied(String),
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    Authorization(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValue(String),
    /// <p>Too many requests have been submitted. Try again after a brief wait.</p>
    RateLimitExceeded(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternal(String),
}

impl GetLicenseConfigurationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetLicenseConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetLicenseConfigurationError::AccessDenied(
                        err.msg,
                    ))
                }
                "AuthorizationException" => {
                    return RusotoError::Service(GetLicenseConfigurationError::Authorization(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        GetLicenseConfigurationError::InvalidParameterValue(err.msg),
                    )
                }
                "RateLimitExceededException" => {
                    return RusotoError::Service(GetLicenseConfigurationError::RateLimitExceeded(
                        err.msg,
                    ))
                }
                "ServerInternalException" => {
                    return RusotoError::Service(GetLicenseConfigurationError::ServerInternal(
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
impl fmt::Display for GetLicenseConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetLicenseConfigurationError {
    fn description(&self) -> &str {
        match *self {
            GetLicenseConfigurationError::AccessDenied(ref cause) => cause,
            GetLicenseConfigurationError::Authorization(ref cause) => cause,
            GetLicenseConfigurationError::InvalidParameterValue(ref cause) => cause,
            GetLicenseConfigurationError::RateLimitExceeded(ref cause) => cause,
            GetLicenseConfigurationError::ServerInternal(ref cause) => cause,
        }
    }
}
/// Errors returned by GetServiceSettings
#[derive(Debug, PartialEq)]
pub enum GetServiceSettingsError {
    /// <p>Access to resource denied.</p>
    AccessDenied(String),
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    Authorization(String),
    /// <p>Too many requests have been submitted. Try again after a brief wait.</p>
    RateLimitExceeded(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternal(String),
}

impl GetServiceSettingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetServiceSettingsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetServiceSettingsError::AccessDenied(err.msg))
                }
                "AuthorizationException" => {
                    return RusotoError::Service(GetServiceSettingsError::Authorization(err.msg))
                }
                "RateLimitExceededException" => {
                    return RusotoError::Service(GetServiceSettingsError::RateLimitExceeded(
                        err.msg,
                    ))
                }
                "ServerInternalException" => {
                    return RusotoError::Service(GetServiceSettingsError::ServerInternal(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetServiceSettingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetServiceSettingsError {
    fn description(&self) -> &str {
        match *self {
            GetServiceSettingsError::AccessDenied(ref cause) => cause,
            GetServiceSettingsError::Authorization(ref cause) => cause,
            GetServiceSettingsError::RateLimitExceeded(ref cause) => cause,
            GetServiceSettingsError::ServerInternal(ref cause) => cause,
        }
    }
}
/// Errors returned by ListAssociationsForLicenseConfiguration
#[derive(Debug, PartialEq)]
pub enum ListAssociationsForLicenseConfigurationError {
    /// <p>Access to resource denied.</p>
    AccessDenied(String),
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    Authorization(String),
    /// <p>The request uses too many filters or too many filter values.</p>
    FilterLimitExceeded(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValue(String),
    /// <p>Too many requests have been submitted. Try again after a brief wait.</p>
    RateLimitExceeded(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternal(String),
}

impl ListAssociationsForLicenseConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListAssociationsForLicenseConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        ListAssociationsForLicenseConfigurationError::AccessDenied(err.msg),
                    )
                }
                "AuthorizationException" => {
                    return RusotoError::Service(
                        ListAssociationsForLicenseConfigurationError::Authorization(err.msg),
                    )
                }
                "FilterLimitExceededException" => {
                    return RusotoError::Service(
                        ListAssociationsForLicenseConfigurationError::FilterLimitExceeded(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        ListAssociationsForLicenseConfigurationError::InvalidParameterValue(
                            err.msg,
                        ),
                    )
                }
                "RateLimitExceededException" => {
                    return RusotoError::Service(
                        ListAssociationsForLicenseConfigurationError::RateLimitExceeded(err.msg),
                    )
                }
                "ServerInternalException" => {
                    return RusotoError::Service(
                        ListAssociationsForLicenseConfigurationError::ServerInternal(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListAssociationsForLicenseConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListAssociationsForLicenseConfigurationError {
    fn description(&self) -> &str {
        match *self {
            ListAssociationsForLicenseConfigurationError::AccessDenied(ref cause) => cause,
            ListAssociationsForLicenseConfigurationError::Authorization(ref cause) => cause,
            ListAssociationsForLicenseConfigurationError::FilterLimitExceeded(ref cause) => cause,
            ListAssociationsForLicenseConfigurationError::InvalidParameterValue(ref cause) => cause,
            ListAssociationsForLicenseConfigurationError::RateLimitExceeded(ref cause) => cause,
            ListAssociationsForLicenseConfigurationError::ServerInternal(ref cause) => cause,
        }
    }
}
/// Errors returned by ListLicenseConfigurations
#[derive(Debug, PartialEq)]
pub enum ListLicenseConfigurationsError {
    /// <p>Access to resource denied.</p>
    AccessDenied(String),
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    Authorization(String),
    /// <p>The request uses too many filters or too many filter values.</p>
    FilterLimitExceeded(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValue(String),
    /// <p>Too many requests have been submitted. Try again after a brief wait.</p>
    RateLimitExceeded(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternal(String),
}

impl ListLicenseConfigurationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListLicenseConfigurationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListLicenseConfigurationsError::AccessDenied(
                        err.msg,
                    ))
                }
                "AuthorizationException" => {
                    return RusotoError::Service(ListLicenseConfigurationsError::Authorization(
                        err.msg,
                    ))
                }
                "FilterLimitExceededException" => {
                    return RusotoError::Service(
                        ListLicenseConfigurationsError::FilterLimitExceeded(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        ListLicenseConfigurationsError::InvalidParameterValue(err.msg),
                    )
                }
                "RateLimitExceededException" => {
                    return RusotoError::Service(ListLicenseConfigurationsError::RateLimitExceeded(
                        err.msg,
                    ))
                }
                "ServerInternalException" => {
                    return RusotoError::Service(ListLicenseConfigurationsError::ServerInternal(
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
impl fmt::Display for ListLicenseConfigurationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListLicenseConfigurationsError {
    fn description(&self) -> &str {
        match *self {
            ListLicenseConfigurationsError::AccessDenied(ref cause) => cause,
            ListLicenseConfigurationsError::Authorization(ref cause) => cause,
            ListLicenseConfigurationsError::FilterLimitExceeded(ref cause) => cause,
            ListLicenseConfigurationsError::InvalidParameterValue(ref cause) => cause,
            ListLicenseConfigurationsError::RateLimitExceeded(ref cause) => cause,
            ListLicenseConfigurationsError::ServerInternal(ref cause) => cause,
        }
    }
}
/// Errors returned by ListLicenseSpecificationsForResource
#[derive(Debug, PartialEq)]
pub enum ListLicenseSpecificationsForResourceError {
    /// <p>Access to resource denied.</p>
    AccessDenied(String),
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    Authorization(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValue(String),
    /// <p>Too many requests have been submitted. Try again after a brief wait.</p>
    RateLimitExceeded(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternal(String),
}

impl ListLicenseSpecificationsForResourceError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListLicenseSpecificationsForResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        ListLicenseSpecificationsForResourceError::AccessDenied(err.msg),
                    )
                }
                "AuthorizationException" => {
                    return RusotoError::Service(
                        ListLicenseSpecificationsForResourceError::Authorization(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        ListLicenseSpecificationsForResourceError::InvalidParameterValue(err.msg),
                    )
                }
                "RateLimitExceededException" => {
                    return RusotoError::Service(
                        ListLicenseSpecificationsForResourceError::RateLimitExceeded(err.msg),
                    )
                }
                "ServerInternalException" => {
                    return RusotoError::Service(
                        ListLicenseSpecificationsForResourceError::ServerInternal(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListLicenseSpecificationsForResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListLicenseSpecificationsForResourceError {
    fn description(&self) -> &str {
        match *self {
            ListLicenseSpecificationsForResourceError::AccessDenied(ref cause) => cause,
            ListLicenseSpecificationsForResourceError::Authorization(ref cause) => cause,
            ListLicenseSpecificationsForResourceError::InvalidParameterValue(ref cause) => cause,
            ListLicenseSpecificationsForResourceError::RateLimitExceeded(ref cause) => cause,
            ListLicenseSpecificationsForResourceError::ServerInternal(ref cause) => cause,
        }
    }
}
/// Errors returned by ListResourceInventory
#[derive(Debug, PartialEq)]
pub enum ListResourceInventoryError {
    /// <p>Access to resource denied.</p>
    AccessDenied(String),
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    Authorization(String),
    /// <p>A dependency required to run the API is missing.</p>
    FailedDependency(String),
    /// <p>The request uses too many filters or too many filter values.</p>
    FilterLimitExceeded(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValue(String),
    /// <p>Too many requests have been submitted. Try again after a brief wait.</p>
    RateLimitExceeded(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternal(String),
}

impl ListResourceInventoryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListResourceInventoryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListResourceInventoryError::AccessDenied(err.msg))
                }
                "AuthorizationException" => {
                    return RusotoError::Service(ListResourceInventoryError::Authorization(err.msg))
                }
                "FailedDependencyException" => {
                    return RusotoError::Service(ListResourceInventoryError::FailedDependency(
                        err.msg,
                    ))
                }
                "FilterLimitExceededException" => {
                    return RusotoError::Service(ListResourceInventoryError::FilterLimitExceeded(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(ListResourceInventoryError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "RateLimitExceededException" => {
                    return RusotoError::Service(ListResourceInventoryError::RateLimitExceeded(
                        err.msg,
                    ))
                }
                "ServerInternalException" => {
                    return RusotoError::Service(ListResourceInventoryError::ServerInternal(
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
impl fmt::Display for ListResourceInventoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListResourceInventoryError {
    fn description(&self) -> &str {
        match *self {
            ListResourceInventoryError::AccessDenied(ref cause) => cause,
            ListResourceInventoryError::Authorization(ref cause) => cause,
            ListResourceInventoryError::FailedDependency(ref cause) => cause,
            ListResourceInventoryError::FilterLimitExceeded(ref cause) => cause,
            ListResourceInventoryError::InvalidParameterValue(ref cause) => cause,
            ListResourceInventoryError::RateLimitExceeded(ref cause) => cause,
            ListResourceInventoryError::ServerInternal(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>Access to resource denied.</p>
    AccessDenied(String),
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    Authorization(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValue(String),
    /// <p>Too many requests have been submitted. Try again after a brief wait.</p>
    RateLimitExceeded(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternal(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListTagsForResourceError::AccessDenied(err.msg))
                }
                "AuthorizationException" => {
                    return RusotoError::Service(ListTagsForResourceError::Authorization(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(ListTagsForResourceError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "RateLimitExceededException" => {
                    return RusotoError::Service(ListTagsForResourceError::RateLimitExceeded(
                        err.msg,
                    ))
                }
                "ServerInternalException" => {
                    return RusotoError::Service(ListTagsForResourceError::ServerInternal(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListTagsForResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTagsForResourceError {
    fn description(&self) -> &str {
        match *self {
            ListTagsForResourceError::AccessDenied(ref cause) => cause,
            ListTagsForResourceError::Authorization(ref cause) => cause,
            ListTagsForResourceError::InvalidParameterValue(ref cause) => cause,
            ListTagsForResourceError::RateLimitExceeded(ref cause) => cause,
            ListTagsForResourceError::ServerInternal(ref cause) => cause,
        }
    }
}
/// Errors returned by ListUsageForLicenseConfiguration
#[derive(Debug, PartialEq)]
pub enum ListUsageForLicenseConfigurationError {
    /// <p>Access to resource denied.</p>
    AccessDenied(String),
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    Authorization(String),
    /// <p>The request uses too many filters or too many filter values.</p>
    FilterLimitExceeded(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValue(String),
    /// <p>Too many requests have been submitted. Try again after a brief wait.</p>
    RateLimitExceeded(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternal(String),
}

impl ListUsageForLicenseConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListUsageForLicenseConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        ListUsageForLicenseConfigurationError::AccessDenied(err.msg),
                    )
                }
                "AuthorizationException" => {
                    return RusotoError::Service(
                        ListUsageForLicenseConfigurationError::Authorization(err.msg),
                    )
                }
                "FilterLimitExceededException" => {
                    return RusotoError::Service(
                        ListUsageForLicenseConfigurationError::FilterLimitExceeded(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        ListUsageForLicenseConfigurationError::InvalidParameterValue(err.msg),
                    )
                }
                "RateLimitExceededException" => {
                    return RusotoError::Service(
                        ListUsageForLicenseConfigurationError::RateLimitExceeded(err.msg),
                    )
                }
                "ServerInternalException" => {
                    return RusotoError::Service(
                        ListUsageForLicenseConfigurationError::ServerInternal(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListUsageForLicenseConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListUsageForLicenseConfigurationError {
    fn description(&self) -> &str {
        match *self {
            ListUsageForLicenseConfigurationError::AccessDenied(ref cause) => cause,
            ListUsageForLicenseConfigurationError::Authorization(ref cause) => cause,
            ListUsageForLicenseConfigurationError::FilterLimitExceeded(ref cause) => cause,
            ListUsageForLicenseConfigurationError::InvalidParameterValue(ref cause) => cause,
            ListUsageForLicenseConfigurationError::RateLimitExceeded(ref cause) => cause,
            ListUsageForLicenseConfigurationError::ServerInternal(ref cause) => cause,
        }
    }
}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>Access to resource denied.</p>
    AccessDenied(String),
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    Authorization(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValue(String),
    /// <p>Too many requests have been submitted. Try again after a brief wait.</p>
    RateLimitExceeded(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternal(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(TagResourceError::AccessDenied(err.msg))
                }
                "AuthorizationException" => {
                    return RusotoError::Service(TagResourceError::Authorization(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(TagResourceError::InvalidParameterValue(err.msg))
                }
                "RateLimitExceededException" => {
                    return RusotoError::Service(TagResourceError::RateLimitExceeded(err.msg))
                }
                "ServerInternalException" => {
                    return RusotoError::Service(TagResourceError::ServerInternal(err.msg))
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
            TagResourceError::AccessDenied(ref cause) => cause,
            TagResourceError::Authorization(ref cause) => cause,
            TagResourceError::InvalidParameterValue(ref cause) => cause,
            TagResourceError::RateLimitExceeded(ref cause) => cause,
            TagResourceError::ServerInternal(ref cause) => cause,
        }
    }
}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>Access to resource denied.</p>
    AccessDenied(String),
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    Authorization(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValue(String),
    /// <p>Too many requests have been submitted. Try again after a brief wait.</p>
    RateLimitExceeded(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternal(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UntagResourceError::AccessDenied(err.msg))
                }
                "AuthorizationException" => {
                    return RusotoError::Service(UntagResourceError::Authorization(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(UntagResourceError::InvalidParameterValue(err.msg))
                }
                "RateLimitExceededException" => {
                    return RusotoError::Service(UntagResourceError::RateLimitExceeded(err.msg))
                }
                "ServerInternalException" => {
                    return RusotoError::Service(UntagResourceError::ServerInternal(err.msg))
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
            UntagResourceError::AccessDenied(ref cause) => cause,
            UntagResourceError::Authorization(ref cause) => cause,
            UntagResourceError::InvalidParameterValue(ref cause) => cause,
            UntagResourceError::RateLimitExceeded(ref cause) => cause,
            UntagResourceError::ServerInternal(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateLicenseConfiguration
#[derive(Debug, PartialEq)]
pub enum UpdateLicenseConfigurationError {
    /// <p>Access to resource denied.</p>
    AccessDenied(String),
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    Authorization(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValue(String),
    /// <p>Too many requests have been submitted. Try again after a brief wait.</p>
    RateLimitExceeded(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternal(String),
}

impl UpdateLicenseConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateLicenseConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateLicenseConfigurationError::AccessDenied(
                        err.msg,
                    ))
                }
                "AuthorizationException" => {
                    return RusotoError::Service(UpdateLicenseConfigurationError::Authorization(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        UpdateLicenseConfigurationError::InvalidParameterValue(err.msg),
                    )
                }
                "RateLimitExceededException" => {
                    return RusotoError::Service(
                        UpdateLicenseConfigurationError::RateLimitExceeded(err.msg),
                    )
                }
                "ServerInternalException" => {
                    return RusotoError::Service(UpdateLicenseConfigurationError::ServerInternal(
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
impl fmt::Display for UpdateLicenseConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateLicenseConfigurationError {
    fn description(&self) -> &str {
        match *self {
            UpdateLicenseConfigurationError::AccessDenied(ref cause) => cause,
            UpdateLicenseConfigurationError::Authorization(ref cause) => cause,
            UpdateLicenseConfigurationError::InvalidParameterValue(ref cause) => cause,
            UpdateLicenseConfigurationError::RateLimitExceeded(ref cause) => cause,
            UpdateLicenseConfigurationError::ServerInternal(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateLicenseSpecificationsForResource
#[derive(Debug, PartialEq)]
pub enum UpdateLicenseSpecificationsForResourceError {
    /// <p>Access to resource denied.</p>
    AccessDenied(String),
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    Authorization(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValue(String),
    /// <p>License Manager cannot allocate a license to a resource because of its state. </p> <p>For example, you cannot allocate a license to an instance in the process of shutting down.</p>
    InvalidResourceState(String),
    /// <p>You do not have enough licenses available to support a new resource launch.</p>
    LicenseUsage(String),
    /// <p>Too many requests have been submitted. Try again after a brief wait.</p>
    RateLimitExceeded(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternal(String),
}

impl UpdateLicenseSpecificationsForResourceError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateLicenseSpecificationsForResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        UpdateLicenseSpecificationsForResourceError::AccessDenied(err.msg),
                    )
                }
                "AuthorizationException" => {
                    return RusotoError::Service(
                        UpdateLicenseSpecificationsForResourceError::Authorization(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        UpdateLicenseSpecificationsForResourceError::InvalidParameterValue(err.msg),
                    )
                }
                "InvalidResourceStateException" => {
                    return RusotoError::Service(
                        UpdateLicenseSpecificationsForResourceError::InvalidResourceState(err.msg),
                    )
                }
                "LicenseUsageException" => {
                    return RusotoError::Service(
                        UpdateLicenseSpecificationsForResourceError::LicenseUsage(err.msg),
                    )
                }
                "RateLimitExceededException" => {
                    return RusotoError::Service(
                        UpdateLicenseSpecificationsForResourceError::RateLimitExceeded(err.msg),
                    )
                }
                "ServerInternalException" => {
                    return RusotoError::Service(
                        UpdateLicenseSpecificationsForResourceError::ServerInternal(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateLicenseSpecificationsForResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateLicenseSpecificationsForResourceError {
    fn description(&self) -> &str {
        match *self {
            UpdateLicenseSpecificationsForResourceError::AccessDenied(ref cause) => cause,
            UpdateLicenseSpecificationsForResourceError::Authorization(ref cause) => cause,
            UpdateLicenseSpecificationsForResourceError::InvalidParameterValue(ref cause) => cause,
            UpdateLicenseSpecificationsForResourceError::InvalidResourceState(ref cause) => cause,
            UpdateLicenseSpecificationsForResourceError::LicenseUsage(ref cause) => cause,
            UpdateLicenseSpecificationsForResourceError::RateLimitExceeded(ref cause) => cause,
            UpdateLicenseSpecificationsForResourceError::ServerInternal(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateServiceSettings
#[derive(Debug, PartialEq)]
pub enum UpdateServiceSettingsError {
    /// <p>Access to resource denied.</p>
    AccessDenied(String),
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    Authorization(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValue(String),
    /// <p>Too many requests have been submitted. Try again after a brief wait.</p>
    RateLimitExceeded(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternal(String),
}

impl UpdateServiceSettingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateServiceSettingsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateServiceSettingsError::AccessDenied(err.msg))
                }
                "AuthorizationException" => {
                    return RusotoError::Service(UpdateServiceSettingsError::Authorization(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(UpdateServiceSettingsError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "RateLimitExceededException" => {
                    return RusotoError::Service(UpdateServiceSettingsError::RateLimitExceeded(
                        err.msg,
                    ))
                }
                "ServerInternalException" => {
                    return RusotoError::Service(UpdateServiceSettingsError::ServerInternal(
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
impl fmt::Display for UpdateServiceSettingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateServiceSettingsError {
    fn description(&self) -> &str {
        match *self {
            UpdateServiceSettingsError::AccessDenied(ref cause) => cause,
            UpdateServiceSettingsError::Authorization(ref cause) => cause,
            UpdateServiceSettingsError::InvalidParameterValue(ref cause) => cause,
            UpdateServiceSettingsError::RateLimitExceeded(ref cause) => cause,
            UpdateServiceSettingsError::ServerInternal(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the AWS License Manager API. AWS License Manager clients implement this trait.
#[async_trait]
pub trait LicenseManager {
    /// <p>Creates a new license configuration object. A license configuration is an abstraction of a customer license agreement that can be consumed and enforced by License Manager. Components include specifications for the license type (licensing by instance, socket, CPU, or VCPU), tenancy (shared tenancy, Amazon EC2 Dedicated Instance, Amazon EC2 Dedicated Host, or any of these), host affinity (how long a VM must be associated with a host), the number of licenses purchased and used.</p>
    async fn create_license_configuration(
        &self,
        input: CreateLicenseConfigurationRequest,
    ) -> Result<CreateLicenseConfigurationResponse, RusotoError<CreateLicenseConfigurationError>>;

    /// <p>Deletes an existing license configuration. This action fails if the configuration is in use.</p>
    async fn delete_license_configuration(
        &self,
        input: DeleteLicenseConfigurationRequest,
    ) -> Result<DeleteLicenseConfigurationResponse, RusotoError<DeleteLicenseConfigurationError>>;

    /// <p>Returns a detailed description of a license configuration.</p>
    async fn get_license_configuration(
        &self,
        input: GetLicenseConfigurationRequest,
    ) -> Result<GetLicenseConfigurationResponse, RusotoError<GetLicenseConfigurationError>>;

    /// <p>Gets License Manager settings for a region. Exposes the configured S3 bucket, SNS topic, etc., for inspection. </p>
    async fn get_service_settings(
        &self,
    ) -> Result<GetServiceSettingsResponse, RusotoError<GetServiceSettingsError>>;

    /// <p>Lists the resource associations for a license configuration. Resource associations need not consume licenses from a license configuration. For example, an AMI or a stopped instance may not consume a license (depending on the license rules). Use this operation to find all resources associated with a license configuration.</p>
    async fn list_associations_for_license_configuration(
        &self,
        input: ListAssociationsForLicenseConfigurationRequest,
    ) -> Result<
        ListAssociationsForLicenseConfigurationResponse,
        RusotoError<ListAssociationsForLicenseConfigurationError>,
    >;

    /// <p>Lists license configuration objects for an account, each containing the name, description, license type, and other license terms modeled from a license agreement.</p>
    async fn list_license_configurations(
        &self,
        input: ListLicenseConfigurationsRequest,
    ) -> Result<ListLicenseConfigurationsResponse, RusotoError<ListLicenseConfigurationsError>>;

    /// <p>Returns the license configuration for a resource.</p>
    async fn list_license_specifications_for_resource(
        &self,
        input: ListLicenseSpecificationsForResourceRequest,
    ) -> Result<
        ListLicenseSpecificationsForResourceResponse,
        RusotoError<ListLicenseSpecificationsForResourceError>,
    >;

    /// <p>Returns a detailed list of resources.</p>
    async fn list_resource_inventory(
        &self,
        input: ListResourceInventoryRequest,
    ) -> Result<ListResourceInventoryResponse, RusotoError<ListResourceInventoryError>>;

    /// <p>Lists tags attached to a resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>Lists all license usage records for a license configuration, displaying license consumption details by resource at a selected point in time. Use this action to audit the current license consumption for any license inventory and configuration.</p>
    async fn list_usage_for_license_configuration(
        &self,
        input: ListUsageForLicenseConfigurationRequest,
    ) -> Result<
        ListUsageForLicenseConfigurationResponse,
        RusotoError<ListUsageForLicenseConfigurationError>,
    >;

    /// <p>Attach one of more tags to any resource.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>>;

    /// <p>Remove tags from a resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>>;

    /// <p>Modifies the attributes of an existing license configuration object. A license configuration is an abstraction of a customer license agreement that can be consumed and enforced by License Manager. Components include specifications for the license type (Instances, cores, sockets, VCPUs), tenancy (shared or Dedicated Host), host affinity (how long a VM is associated with a host), the number of licenses purchased and used.</p>
    async fn update_license_configuration(
        &self,
        input: UpdateLicenseConfigurationRequest,
    ) -> Result<UpdateLicenseConfigurationResponse, RusotoError<UpdateLicenseConfigurationError>>;

    /// <p>Adds or removes license configurations for a specified AWS resource. This operation currently supports updating the license specifications of AMIs, instances, and hosts. Launch templates and AWS CloudFormation templates are not managed from this operation as those resources send the license configurations directly to a resource creation operation, such as <code>RunInstances</code>.</p>
    async fn update_license_specifications_for_resource(
        &self,
        input: UpdateLicenseSpecificationsForResourceRequest,
    ) -> Result<
        UpdateLicenseSpecificationsForResourceResponse,
        RusotoError<UpdateLicenseSpecificationsForResourceError>,
    >;

    /// <p>Updates License Manager service settings.</p>
    async fn update_service_settings(
        &self,
        input: UpdateServiceSettingsRequest,
    ) -> Result<UpdateServiceSettingsResponse, RusotoError<UpdateServiceSettingsError>>;
}
/// A client for the AWS License Manager API.
#[derive(Clone)]
pub struct LicenseManagerClient {
    client: Client,
    region: region::Region,
}

impl LicenseManagerClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> LicenseManagerClient {
        LicenseManagerClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> LicenseManagerClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        LicenseManagerClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }
}

#[async_trait]
impl LicenseManager for LicenseManagerClient {
    /// <p>Creates a new license configuration object. A license configuration is an abstraction of a customer license agreement that can be consumed and enforced by License Manager. Components include specifications for the license type (licensing by instance, socket, CPU, or VCPU), tenancy (shared tenancy, Amazon EC2 Dedicated Instance, Amazon EC2 Dedicated Host, or any of these), host affinity (how long a VM must be associated with a host), the number of licenses purchased and used.</p>
    async fn create_license_configuration(
        &self,
        input: CreateLicenseConfigurationRequest,
    ) -> Result<CreateLicenseConfigurationResponse, RusotoError<CreateLicenseConfigurationError>>
    {
        let mut request = SignedRequest::new("POST", "license-manager", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSLicenseManager.CreateLicenseConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateLicenseConfigurationResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateLicenseConfigurationError::from_response(response))
        }
    }

    /// <p>Deletes an existing license configuration. This action fails if the configuration is in use.</p>
    async fn delete_license_configuration(
        &self,
        input: DeleteLicenseConfigurationRequest,
    ) -> Result<DeleteLicenseConfigurationResponse, RusotoError<DeleteLicenseConfigurationError>>
    {
        let mut request = SignedRequest::new("POST", "license-manager", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSLicenseManager.DeleteLicenseConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteLicenseConfigurationResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteLicenseConfigurationError::from_response(response))
        }
    }

    /// <p>Returns a detailed description of a license configuration.</p>
    async fn get_license_configuration(
        &self,
        input: GetLicenseConfigurationRequest,
    ) -> Result<GetLicenseConfigurationResponse, RusotoError<GetLicenseConfigurationError>> {
        let mut request = SignedRequest::new("POST", "license-manager", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSLicenseManager.GetLicenseConfiguration");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<GetLicenseConfigurationResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetLicenseConfigurationError::from_response(response))
        }
    }

    /// <p>Gets License Manager settings for a region. Exposes the configured S3 bucket, SNS topic, etc., for inspection. </p>
    async fn get_service_settings(
        &self,
    ) -> Result<GetServiceSettingsResponse, RusotoError<GetServiceSettingsError>> {
        let mut request = SignedRequest::new("POST", "license-manager", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSLicenseManager.GetServiceSettings");
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<GetServiceSettingsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetServiceSettingsError::from_response(response))
        }
    }

    /// <p>Lists the resource associations for a license configuration. Resource associations need not consume licenses from a license configuration. For example, an AMI or a stopped instance may not consume a license (depending on the license rules). Use this operation to find all resources associated with a license configuration.</p>
    async fn list_associations_for_license_configuration(
        &self,
        input: ListAssociationsForLicenseConfigurationRequest,
    ) -> Result<
        ListAssociationsForLicenseConfigurationResponse,
        RusotoError<ListAssociationsForLicenseConfigurationError>,
    > {
        let mut request = SignedRequest::new("POST", "license-manager", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSLicenseManager.ListAssociationsForLicenseConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<ListAssociationsForLicenseConfigurationResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListAssociationsForLicenseConfigurationError::from_response(
                response,
            ))
        }
    }

    /// <p>Lists license configuration objects for an account, each containing the name, description, license type, and other license terms modeled from a license agreement.</p>
    async fn list_license_configurations(
        &self,
        input: ListLicenseConfigurationsRequest,
    ) -> Result<ListLicenseConfigurationsResponse, RusotoError<ListLicenseConfigurationsError>>
    {
        let mut request = SignedRequest::new("POST", "license-manager", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSLicenseManager.ListLicenseConfigurations",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<ListLicenseConfigurationsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListLicenseConfigurationsError::from_response(response))
        }
    }

    /// <p>Returns the license configuration for a resource.</p>
    async fn list_license_specifications_for_resource(
        &self,
        input: ListLicenseSpecificationsForResourceRequest,
    ) -> Result<
        ListLicenseSpecificationsForResourceResponse,
        RusotoError<ListLicenseSpecificationsForResourceError>,
    > {
        let mut request = SignedRequest::new("POST", "license-manager", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSLicenseManager.ListLicenseSpecificationsForResource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<ListLicenseSpecificationsForResourceResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListLicenseSpecificationsForResourceError::from_response(
                response,
            ))
        }
    }

    /// <p>Returns a detailed list of resources.</p>
    async fn list_resource_inventory(
        &self,
        input: ListResourceInventoryRequest,
    ) -> Result<ListResourceInventoryResponse, RusotoError<ListResourceInventoryError>> {
        let mut request = SignedRequest::new("POST", "license-manager", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSLicenseManager.ListResourceInventory");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<ListResourceInventoryResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListResourceInventoryError::from_response(response))
        }
    }

    /// <p>Lists tags attached to a resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let mut request = SignedRequest::new("POST", "license-manager", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSLicenseManager.ListTagsForResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<ListTagsForResourceResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListTagsForResourceError::from_response(response))
        }
    }

    /// <p>Lists all license usage records for a license configuration, displaying license consumption details by resource at a selected point in time. Use this action to audit the current license consumption for any license inventory and configuration.</p>
    async fn list_usage_for_license_configuration(
        &self,
        input: ListUsageForLicenseConfigurationRequest,
    ) -> Result<
        ListUsageForLicenseConfigurationResponse,
        RusotoError<ListUsageForLicenseConfigurationError>,
    > {
        let mut request = SignedRequest::new("POST", "license-manager", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSLicenseManager.ListUsageForLicenseConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<ListUsageForLicenseConfigurationResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListUsageForLicenseConfigurationError::from_response(
                response,
            ))
        }
    }

    /// <p>Attach one of more tags to any resource.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>> {
        let mut request = SignedRequest::new("POST", "license-manager", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSLicenseManager.TagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<TagResourceResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(TagResourceError::from_response(response))
        }
    }

    /// <p>Remove tags from a resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>> {
        let mut request = SignedRequest::new("POST", "license-manager", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSLicenseManager.UntagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<UntagResourceResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UntagResourceError::from_response(response))
        }
    }

    /// <p>Modifies the attributes of an existing license configuration object. A license configuration is an abstraction of a customer license agreement that can be consumed and enforced by License Manager. Components include specifications for the license type (Instances, cores, sockets, VCPUs), tenancy (shared or Dedicated Host), host affinity (how long a VM is associated with a host), the number of licenses purchased and used.</p>
    async fn update_license_configuration(
        &self,
        input: UpdateLicenseConfigurationRequest,
    ) -> Result<UpdateLicenseConfigurationResponse, RusotoError<UpdateLicenseConfigurationError>>
    {
        let mut request = SignedRequest::new("POST", "license-manager", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSLicenseManager.UpdateLicenseConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateLicenseConfigurationResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateLicenseConfigurationError::from_response(response))
        }
    }

    /// <p>Adds or removes license configurations for a specified AWS resource. This operation currently supports updating the license specifications of AMIs, instances, and hosts. Launch templates and AWS CloudFormation templates are not managed from this operation as those resources send the license configurations directly to a resource creation operation, such as <code>RunInstances</code>.</p>
    async fn update_license_specifications_for_resource(
        &self,
        input: UpdateLicenseSpecificationsForResourceRequest,
    ) -> Result<
        UpdateLicenseSpecificationsForResourceResponse,
        RusotoError<UpdateLicenseSpecificationsForResourceError>,
    > {
        let mut request = SignedRequest::new("POST", "license-manager", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSLicenseManager.UpdateLicenseSpecificationsForResource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateLicenseSpecificationsForResourceResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateLicenseSpecificationsForResourceError::from_response(
                response,
            ))
        }
    }

    /// <p>Updates License Manager service settings.</p>
    async fn update_service_settings(
        &self,
        input: UpdateServiceSettingsRequest,
    ) -> Result<UpdateServiceSettingsResponse, RusotoError<UpdateServiceSettingsError>> {
        let mut request = SignedRequest::new("POST", "license-manager", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSLicenseManager.UpdateServiceSettings");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateServiceSettingsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateServiceSettingsError::from_response(response))
        }
    }
}
