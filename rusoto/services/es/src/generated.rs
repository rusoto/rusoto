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

use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError};

use futures::prelude::*;
use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
use std::pin::Pin;
/// <p>The configured access rules for the domain's document and search endpoints, and the current status of those rules.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AccessPoliciesStatus {
    /// <p>The access policy configured for the Elasticsearch domain. Access policies may be resource-based, IP-based, or IAM-based. See <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-createupdatedomains.html#es-createdomain-configure-access-policies" target="_blank"> Configuring Access Policies</a>for more information.</p>
    #[serde(rename = "Options")]
    pub options: String,
    /// <p>The status of the access policy for the Elasticsearch domain. See <code>OptionStatus</code> for the status information that's included. </p>
    #[serde(rename = "Status")]
    pub status: OptionStatus,
}

/// <p>Container for the parameters to the <code><a>AddTags</a></code> operation. Specify the tags that you want to attach to the Elasticsearch domain.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AddTagsRequest {
    /// <p> Specify the <code>ARN</code> for which you want to add the tags.</p>
    #[serde(rename = "ARN")]
    pub arn: String,
    /// <p> List of <code>Tag</code> that need to be added for the Elasticsearch domain. </p>
    #[serde(rename = "TagList")]
    pub tag_list: Vec<Tag>,
}

/// <p> List of limits that are specific to a given InstanceType and for each of it's <code> <a>InstanceRole</a> </code> . </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AdditionalLimit {
    /// <p> Name of Additional Limit is specific to a given InstanceType and for each of it's <code> <a>InstanceRole</a> </code> etc. <br/> Attributes and their details: <br/> <ul> <li>MaximumNumberOfDataNodesSupported</li> This attribute will be present in Master node only to specify how much data nodes upto which given <code> <a>ESPartitionInstanceType</a> </code> can support as master node. <li>MaximumNumberOfDataNodesWithoutMasterNode</li> This attribute will be present in Data node only to specify how much data nodes of given <code> <a>ESPartitionInstanceType</a> </code> upto which you don't need any master nodes to govern them. </ul> </p>
    #[serde(rename = "LimitName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_name: Option<String>,
    /// <p> Value for given <code> <a>AdditionalLimit$LimitName</a> </code> . </p>
    #[serde(rename = "LimitValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_values: Option<Vec<String>>,
}

/// <p> Status of the advanced options for the specified Elasticsearch domain. Currently, the following advanced options are available:</p> <ul> <li>Option to allow references to indices in an HTTP request body. Must be <code>false</code> when configuring access to individual sub-resources. By default, the value is <code>true</code>. See <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-createupdatedomains.html#es-createdomain-configure-advanced-options" target="_blank">Configuration Advanced Options</a> for more information.</li> <li>Option to specify the percentage of heap space that is allocated to field data. By default, this setting is unbounded.</li> </ul> <p>For more information, see <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-createupdatedomains.html#es-createdomain-configure-advanced-options">Configuring Advanced Options</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AdvancedOptionsStatus {
    /// <p> Specifies the status of advanced options for the specified Elasticsearch domain.</p>
    #[serde(rename = "Options")]
    pub options: ::std::collections::HashMap<String, String>,
    /// <p> Specifies the status of <code>OptionStatus</code> for advanced options for the specified Elasticsearch domain.</p>
    #[serde(rename = "Status")]
    pub status: OptionStatus,
}

/// <p>Container for the parameters to the <code><a>CancelElasticsearchServiceSoftwareUpdate</a></code> operation. Specifies the name of the Elasticsearch domain that you wish to cancel a service software update on.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CancelElasticsearchServiceSoftwareUpdateRequest {
    /// <p>The name of the domain that you want to stop the latest service software update on.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
}

/// <p>The result of a <code>CancelElasticsearchServiceSoftwareUpdate</code> operation. Contains the status of the update.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CancelElasticsearchServiceSoftwareUpdateResponse {
    /// <p>The current status of the Elasticsearch service software update.</p>
    #[serde(rename = "ServiceSoftwareOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_software_options: Option<ServiceSoftwareOptions>,
}

/// <p>Options to specify the Cognito user and identity pools for Kibana authentication. For more information, see <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-cognito-auth.html" target="_blank">Amazon Cognito Authentication for Kibana</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CognitoOptions {
    /// <p>Specifies the option to enable Cognito for Kibana authentication.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>Specifies the Cognito identity pool ID for Kibana authentication.</p>
    #[serde(rename = "IdentityPoolId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_pool_id: Option<String>,
    /// <p>Specifies the role ARN that provides Elasticsearch permissions for accessing Cognito resources.</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>Specifies the Cognito user pool ID for Kibana authentication.</p>
    #[serde(rename = "UserPoolId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_id: Option<String>,
}

/// <p>Status of the Cognito options for the specified Elasticsearch domain.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CognitoOptionsStatus {
    /// <p>Specifies the Cognito options for the specified Elasticsearch domain.</p>
    #[serde(rename = "Options")]
    pub options: CognitoOptions,
    /// <p>Specifies the status of the Cognito options for the specified Elasticsearch domain.</p>
    #[serde(rename = "Status")]
    pub status: OptionStatus,
}

/// <p> A map from an <code> <a>ElasticsearchVersion</a> </code> to a list of compatible <code> <a>ElasticsearchVersion</a> </code> s to which the domain can be upgraded. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CompatibleVersionsMap {
    /// <p>The current version of Elasticsearch on which a domain is.</p>
    #[serde(rename = "SourceVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_version: Option<String>,
    #[serde(rename = "TargetVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_versions: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateElasticsearchDomainRequest {
    /// <p> IAM access policy as a JSON-formatted string.</p>
    #[serde(rename = "AccessPolicies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_policies: Option<String>,
    /// <p> Option to allow references to indices in an HTTP request body. Must be <code>false</code> when configuring access to individual sub-resources. By default, the value is <code>true</code>. See <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-createupdatedomains.html#es-createdomain-configure-advanced-options" target="_blank">Configuration Advanced Options</a> for more information.</p>
    #[serde(rename = "AdvancedOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_options: Option<::std::collections::HashMap<String, String>>,
    /// <p>Options to specify the Cognito user and identity pools for Kibana authentication. For more information, see <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-cognito-auth.html" target="_blank">Amazon Cognito Authentication for Kibana</a>.</p>
    #[serde(rename = "CognitoOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cognito_options: Option<CognitoOptions>,
    /// <p>Options to specify configuration that will be applied to the domain endpoint.</p>
    #[serde(rename = "DomainEndpointOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_endpoint_options: Option<DomainEndpointOptions>,
    /// <p>The name of the Elasticsearch domain that you are creating. Domain names are unique across the domains owned by an account within an AWS region. Domain names must start with a lowercase letter and can contain the following characters: a-z (lowercase), 0-9, and - (hyphen).</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// <p>Options to enable, disable and specify the type and size of EBS storage volumes. </p>
    #[serde(rename = "EBSOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_options: Option<EBSOptions>,
    /// <p>Configuration options for an Elasticsearch domain. Specifies the instance type and number of instances in the domain cluster. </p>
    #[serde(rename = "ElasticsearchClusterConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elasticsearch_cluster_config: Option<ElasticsearchClusterConfig>,
    /// <p>String of format X.Y to specify version for the Elasticsearch domain eg. "1.5" or "2.3". For more information, see <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-createupdatedomains.html#es-createdomains" target="_blank">Creating Elasticsearch Domains</a> in the <i>Amazon Elasticsearch Service Developer Guide</i>.</p>
    #[serde(rename = "ElasticsearchVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elasticsearch_version: Option<String>,
    /// <p>Specifies the Encryption At Rest Options.</p>
    #[serde(rename = "EncryptionAtRestOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_at_rest_options: Option<EncryptionAtRestOptions>,
    /// <p>Map of <code>LogType</code> and <code>LogPublishingOption</code>, each containing options to publish a given type of Elasticsearch log.</p>
    #[serde(rename = "LogPublishingOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_publishing_options: Option<::std::collections::HashMap<String, LogPublishingOption>>,
    /// <p>Specifies the NodeToNodeEncryptionOptions.</p>
    #[serde(rename = "NodeToNodeEncryptionOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_to_node_encryption_options: Option<NodeToNodeEncryptionOptions>,
    /// <p>Option to set time, in UTC format, of the daily automated snapshot. Default value is 0 hours. </p>
    #[serde(rename = "SnapshotOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_options: Option<SnapshotOptions>,
    /// <p>Options to specify the subnets and security groups for VPC endpoint. For more information, see <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-vpc.html#es-creating-vpc" target="_blank">Creating a VPC</a> in <i>VPC Endpoints for Amazon Elasticsearch Service Domains</i></p>
    #[serde(rename = "VPCOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_options: Option<VPCOptions>,
}

/// <p>The result of a <code>CreateElasticsearchDomain</code> operation. Contains the status of the newly created Elasticsearch domain.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateElasticsearchDomainResponse {
    /// <p>The status of the newly created Elasticsearch domain. </p>
    #[serde(rename = "DomainStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_status: Option<ElasticsearchDomainStatus>,
}

/// <p>Container for the parameters to the <code><a>DeleteElasticsearchDomain</a></code> operation. Specifies the name of the Elasticsearch domain that you want to delete.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteElasticsearchDomainRequest {
    /// <p>The name of the Elasticsearch domain that you want to permanently delete.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
}

/// <p>The result of a <code>DeleteElasticsearchDomain</code> request. Contains the status of the pending deletion, or no status if the domain and all of its resources have been deleted.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteElasticsearchDomainResponse {
    /// <p>The status of the Elasticsearch domain being deleted.</p>
    #[serde(rename = "DomainStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_status: Option<ElasticsearchDomainStatus>,
}

/// <p> Container for the parameters to the <code>DescribeElasticsearchDomainConfig</code> operation. Specifies the domain name for which you want configuration information.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeElasticsearchDomainConfigRequest {
    /// <p>The Elasticsearch domain that you want to get information about.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
}

/// <p>The result of a <code>DescribeElasticsearchDomainConfig</code> request. Contains the configuration information of the requested domain.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeElasticsearchDomainConfigResponse {
    /// <p>The configuration information of the domain requested in the <code>DescribeElasticsearchDomainConfig</code> request.</p>
    #[serde(rename = "DomainConfig")]
    pub domain_config: ElasticsearchDomainConfig,
}

/// <p>Container for the parameters to the <code><a>DescribeElasticsearchDomain</a></code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeElasticsearchDomainRequest {
    /// <p>The name of the Elasticsearch domain for which you want information.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
}

/// <p>The result of a <code>DescribeElasticsearchDomain</code> request. Contains the status of the domain specified in the request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeElasticsearchDomainResponse {
    /// <p>The current status of the Elasticsearch domain.</p>
    #[serde(rename = "DomainStatus")]
    pub domain_status: ElasticsearchDomainStatus,
}

/// <p>Container for the parameters to the <code><a>DescribeElasticsearchDomains</a></code> operation. By default, the API returns the status of all Elasticsearch domains.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeElasticsearchDomainsRequest {
    /// <p>The Elasticsearch domains for which you want information.</p>
    #[serde(rename = "DomainNames")]
    pub domain_names: Vec<String>,
}

/// <p>The result of a <code>DescribeElasticsearchDomains</code> request. Contains the status of the specified domains or all domains owned by the account.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeElasticsearchDomainsResponse {
    /// <p>The status of the domains requested in the <code>DescribeElasticsearchDomains</code> request.</p>
    #[serde(rename = "DomainStatusList")]
    pub domain_status_list: Vec<ElasticsearchDomainStatus>,
}

/// <p> Container for the parameters to <code> <a>DescribeElasticsearchInstanceTypeLimits</a> </code> operation. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeElasticsearchInstanceTypeLimitsRequest {
    /// <p> DomainName represents the name of the Domain that we are trying to modify. This should be present only if we are querying for Elasticsearch <code> <a>Limits</a> </code> for existing domain. </p>
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// <p> Version of Elasticsearch for which <code> <a>Limits</a> </code> are needed. </p>
    #[serde(rename = "ElasticsearchVersion")]
    pub elasticsearch_version: String,
    /// <p> The instance type for an Elasticsearch cluster for which Elasticsearch <code> <a>Limits</a> </code> are needed. </p>
    #[serde(rename = "InstanceType")]
    pub instance_type: String,
}

/// <p> Container for the parameters received from <code> <a>DescribeElasticsearchInstanceTypeLimits</a> </code> operation. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeElasticsearchInstanceTypeLimitsResponse {
    #[serde(rename = "LimitsByRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits_by_role: Option<::std::collections::HashMap<String, Limits>>,
}

/// <p>Container for parameters to <code>DescribeReservedElasticsearchInstanceOfferings</code></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeReservedElasticsearchInstanceOfferingsRequest {
    /// <p>Set this value to limit the number of results returned. If not specified, defaults to 100.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>NextToken should be sent in case if earlier API call produced result containing NextToken. It is used for pagination.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The offering identifier filter value. Use this parameter to show only the available offering that matches the specified reservation identifier.</p>
    #[serde(rename = "ReservedElasticsearchInstanceOfferingId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_elasticsearch_instance_offering_id: Option<String>,
}

/// <p>Container for results from <code>DescribeReservedElasticsearchInstanceOfferings</code></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeReservedElasticsearchInstanceOfferingsResponse {
    /// <p>Provides an identifier to allow retrieval of paginated results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>List of reserved Elasticsearch instance offerings</p>
    #[serde(rename = "ReservedElasticsearchInstanceOfferings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_elasticsearch_instance_offerings:
        Option<Vec<ReservedElasticsearchInstanceOffering>>,
}

/// <p>Container for parameters to <code>DescribeReservedElasticsearchInstances</code></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeReservedElasticsearchInstancesRequest {
    /// <p>Set this value to limit the number of results returned. If not specified, defaults to 100.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>NextToken should be sent in case if earlier API call produced result containing NextToken. It is used for pagination.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The reserved instance identifier filter value. Use this parameter to show only the reservation that matches the specified reserved Elasticsearch instance ID.</p>
    #[serde(rename = "ReservedElasticsearchInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_elasticsearch_instance_id: Option<String>,
}

/// <p>Container for results from <code>DescribeReservedElasticsearchInstances</code></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeReservedElasticsearchInstancesResponse {
    /// <p>Provides an identifier to allow retrieval of paginated results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>List of reserved Elasticsearch instances.</p>
    #[serde(rename = "ReservedElasticsearchInstances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_elasticsearch_instances: Option<Vec<ReservedElasticsearchInstance>>,
}

/// <p>Options to configure endpoint for the Elasticsearch domain.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DomainEndpointOptions {
    /// <p>Specify if only HTTPS endpoint should be enabled for the Elasticsearch domain.</p>
    #[serde(rename = "EnforceHTTPS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforce_https: Option<bool>,
    /// <p>Specify the TLS security policy that needs to be applied to the HTTPS endpoint of Elasticsearch domain. <br/> It can be one of the following values: <ul> <li><b>Policy-Min-TLS-1-0-2019-07: </b> TLS security policy which supports TLSv1.0 and higher.</li> <li><b>Policy-Min-TLS-1-2-2019-07: </b> TLS security policy which supports only TLSv1.2</li> </ul> </p>
    #[serde(rename = "TLSSecurityPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_security_policy: Option<String>,
}

/// <p>The configured endpoint options for the domain and their current status.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DomainEndpointOptionsStatus {
    /// <p>Options to configure endpoint for the Elasticsearch domain.</p>
    #[serde(rename = "Options")]
    pub options: DomainEndpointOptions,
    /// <p>The status of the endpoint options for the Elasticsearch domain. See <code>OptionStatus</code> for the status information that's included. </p>
    #[serde(rename = "Status")]
    pub status: OptionStatus,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DomainInfo {
    /// <p> Specifies the <code>DomainName</code>.</p>
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
}

/// <p>Options to enable, disable, and specify the properties of EBS storage volumes. For more information, see <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-createupdatedomains.html#es-createdomain-configure-ebs" target="_blank"> Configuring EBS-based Storage</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EBSOptions {
    /// <p>Specifies whether EBS-based storage is enabled.</p>
    #[serde(rename = "EBSEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_enabled: Option<bool>,
    /// <p>Specifies the IOPD for a Provisioned IOPS EBS volume (SSD).</p>
    #[serde(rename = "Iops")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iops: Option<i64>,
    /// <p> Integer to specify the size of an EBS volume.</p>
    #[serde(rename = "VolumeSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_size: Option<i64>,
    /// <p> Specifies the volume type for EBS-based storage.</p>
    #[serde(rename = "VolumeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_type: Option<String>,
}

/// <p> Status of the EBS options for the specified Elasticsearch domain.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EBSOptionsStatus {
    /// <p> Specifies the EBS options for the specified Elasticsearch domain.</p>
    #[serde(rename = "Options")]
    pub options: EBSOptions,
    /// <p> Specifies the status of the EBS options for the specified Elasticsearch domain.</p>
    #[serde(rename = "Status")]
    pub status: OptionStatus,
}

/// <p>Specifies the configuration for the domain cluster, such as the type and number of instances.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ElasticsearchClusterConfig {
    /// <p>Total number of dedicated master nodes, active and on standby, for the cluster.</p>
    #[serde(rename = "DedicatedMasterCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_master_count: Option<i64>,
    /// <p>A boolean value to indicate whether a dedicated master node is enabled. See <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-managedomains.html#es-managedomains-dedicatedmasternodes" target="_blank">About Dedicated Master Nodes</a> for more information.</p>
    #[serde(rename = "DedicatedMasterEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_master_enabled: Option<bool>,
    /// <p>The instance type for a dedicated master node.</p>
    #[serde(rename = "DedicatedMasterType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_master_type: Option<String>,
    /// <p>The number of instances in the specified domain cluster.</p>
    #[serde(rename = "InstanceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_count: Option<i64>,
    /// <p>The instance type for an Elasticsearch cluster. UltraWarm instance types are not supported for data instances.</p>
    #[serde(rename = "InstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// <p>The number of warm nodes in the cluster.</p>
    #[serde(rename = "WarmCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warm_count: Option<i64>,
    /// <p>True to enable warm storage.</p>
    #[serde(rename = "WarmEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warm_enabled: Option<bool>,
    /// <p>The instance type for the Elasticsearch cluster's warm nodes.</p>
    #[serde(rename = "WarmType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warm_type: Option<String>,
    /// <p>Specifies the zone awareness configuration for a domain when zone awareness is enabled.</p>
    #[serde(rename = "ZoneAwarenessConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_awareness_config: Option<ZoneAwarenessConfig>,
    /// <p>A boolean value to indicate whether zone awareness is enabled. See <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-managedomains.html#es-managedomains-zoneawareness" target="_blank">About Zone Awareness</a> for more information.</p>
    #[serde(rename = "ZoneAwarenessEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_awareness_enabled: Option<bool>,
}

/// <p> Specifies the configuration status for the specified Elasticsearch domain.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ElasticsearchClusterConfigStatus {
    /// <p> Specifies the cluster configuration for the specified Elasticsearch domain.</p>
    #[serde(rename = "Options")]
    pub options: ElasticsearchClusterConfig,
    /// <p> Specifies the status of the configuration for the specified Elasticsearch domain.</p>
    #[serde(rename = "Status")]
    pub status: OptionStatus,
}

/// <p>The configuration of an Elasticsearch domain.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ElasticsearchDomainConfig {
    /// <p>IAM access policy as a JSON-formatted string.</p>
    #[serde(rename = "AccessPolicies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_policies: Option<AccessPoliciesStatus>,
    /// <p>Specifies the <code>AdvancedOptions</code> for the domain. See <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-createupdatedomains.html#es-createdomain-configure-advanced-options" target="_blank">Configuring Advanced Options</a> for more information.</p>
    #[serde(rename = "AdvancedOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_options: Option<AdvancedOptionsStatus>,
    /// <p>The <code>CognitoOptions</code> for the specified domain. For more information, see <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-cognito-auth.html" target="_blank">Amazon Cognito Authentication for Kibana</a>.</p>
    #[serde(rename = "CognitoOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cognito_options: Option<CognitoOptionsStatus>,
    /// <p>Specifies the <code>DomainEndpointOptions</code> for the Elasticsearch domain.</p>
    #[serde(rename = "DomainEndpointOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_endpoint_options: Option<DomainEndpointOptionsStatus>,
    /// <p>Specifies the <code>EBSOptions</code> for the Elasticsearch domain.</p>
    #[serde(rename = "EBSOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_options: Option<EBSOptionsStatus>,
    /// <p>Specifies the <code>ElasticsearchClusterConfig</code> for the Elasticsearch domain.</p>
    #[serde(rename = "ElasticsearchClusterConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elasticsearch_cluster_config: Option<ElasticsearchClusterConfigStatus>,
    /// <p>String of format X.Y to specify version for the Elasticsearch domain.</p>
    #[serde(rename = "ElasticsearchVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elasticsearch_version: Option<ElasticsearchVersionStatus>,
    /// <p>Specifies the <code>EncryptionAtRestOptions</code> for the Elasticsearch domain.</p>
    #[serde(rename = "EncryptionAtRestOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_at_rest_options: Option<EncryptionAtRestOptionsStatus>,
    /// <p>Log publishing options for the given domain.</p>
    #[serde(rename = "LogPublishingOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_publishing_options: Option<LogPublishingOptionsStatus>,
    /// <p>Specifies the <code>NodeToNodeEncryptionOptions</code> for the Elasticsearch domain.</p>
    #[serde(rename = "NodeToNodeEncryptionOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_to_node_encryption_options: Option<NodeToNodeEncryptionOptionsStatus>,
    /// <p>Specifies the <code>SnapshotOptions</code> for the Elasticsearch domain.</p>
    #[serde(rename = "SnapshotOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_options: Option<SnapshotOptionsStatus>,
    /// <p>The <code>VPCOptions</code> for the specified domain. For more information, see <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-vpc.html" target="_blank">VPC Endpoints for Amazon Elasticsearch Service Domains</a>.</p>
    #[serde(rename = "VPCOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_options: Option<VPCDerivedInfoStatus>,
}

/// <p>The current status of an Elasticsearch domain.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ElasticsearchDomainStatus {
    /// <p>The Amazon resource name (ARN) of an Elasticsearch domain. See <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/index.html?Using_Identifiers.html" target="_blank">Identifiers for IAM Entities</a> in <i>Using AWS Identity and Access Management</i> for more information.</p>
    #[serde(rename = "ARN")]
    pub arn: String,
    /// <p> IAM access policy as a JSON-formatted string.</p>
    #[serde(rename = "AccessPolicies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_policies: Option<String>,
    /// <p>Specifies the status of the <code>AdvancedOptions</code></p>
    #[serde(rename = "AdvancedOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_options: Option<::std::collections::HashMap<String, String>>,
    /// <p>The <code>CognitoOptions</code> for the specified domain. For more information, see <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-cognito-auth.html" target="_blank">Amazon Cognito Authentication for Kibana</a>.</p>
    #[serde(rename = "CognitoOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cognito_options: Option<CognitoOptions>,
    /// <p>The domain creation status. <code>True</code> if the creation of an Elasticsearch domain is complete. <code>False</code> if domain creation is still in progress.</p>
    #[serde(rename = "Created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<bool>,
    /// <p>The domain deletion status. <code>True</code> if a delete request has been received for the domain but resource cleanup is still in progress. <code>False</code> if the domain has not been deleted. Once domain deletion is complete, the status of the domain is no longer returned.</p>
    #[serde(rename = "Deleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// <p>The current status of the Elasticsearch domain's endpoint options.</p>
    #[serde(rename = "DomainEndpointOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_endpoint_options: Option<DomainEndpointOptions>,
    /// <p>The unique identifier for the specified Elasticsearch domain.</p>
    #[serde(rename = "DomainId")]
    pub domain_id: String,
    /// <p>The name of an Elasticsearch domain. Domain names are unique across the domains owned by an account within an AWS region. Domain names start with a letter or number and can contain the following characters: a-z (lowercase), 0-9, and - (hyphen).</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// <p>The <code>EBSOptions</code> for the specified domain. See <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-createupdatedomains.html#es-createdomain-configure-ebs" target="_blank">Configuring EBS-based Storage</a> for more information.</p>
    #[serde(rename = "EBSOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_options: Option<EBSOptions>,
    /// <p>The type and number of instances in the domain cluster.</p>
    #[serde(rename = "ElasticsearchClusterConfig")]
    pub elasticsearch_cluster_config: ElasticsearchClusterConfig,
    #[serde(rename = "ElasticsearchVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elasticsearch_version: Option<String>,
    /// <p> Specifies the status of the <code>EncryptionAtRestOptions</code>.</p>
    #[serde(rename = "EncryptionAtRestOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_at_rest_options: Option<EncryptionAtRestOptions>,
    /// <p>The Elasticsearch domain endpoint that you use to submit index and search requests.</p>
    #[serde(rename = "Endpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    /// <p>Map containing the Elasticsearch domain endpoints used to submit index and search requests. Example <code>key, value</code>: <code>'vpc','vpc-endpoint-h2dsd34efgyghrtguk5gt6j2foh4.us-east-1.es.amazonaws.com'</code>.</p>
    #[serde(rename = "Endpoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<::std::collections::HashMap<String, String>>,
    /// <p>Log publishing options for the given domain.</p>
    #[serde(rename = "LogPublishingOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_publishing_options: Option<::std::collections::HashMap<String, LogPublishingOption>>,
    /// <p>Specifies the status of the <code>NodeToNodeEncryptionOptions</code>.</p>
    #[serde(rename = "NodeToNodeEncryptionOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_to_node_encryption_options: Option<NodeToNodeEncryptionOptions>,
    /// <p>The status of the Elasticsearch domain configuration. <code>True</code> if Amazon Elasticsearch Service is processing configuration changes. <code>False</code> if the configuration is active.</p>
    #[serde(rename = "Processing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing: Option<bool>,
    /// <p>The current status of the Elasticsearch domain's service software.</p>
    #[serde(rename = "ServiceSoftwareOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_software_options: Option<ServiceSoftwareOptions>,
    /// <p>Specifies the status of the <code>SnapshotOptions</code></p>
    #[serde(rename = "SnapshotOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_options: Option<SnapshotOptions>,
    /// <p>The status of an Elasticsearch domain version upgrade. <code>True</code> if Amazon Elasticsearch Service is undergoing a version upgrade. <code>False</code> if the configuration is active.</p>
    #[serde(rename = "UpgradeProcessing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgrade_processing: Option<bool>,
    /// <p>The <code>VPCOptions</code> for the specified domain. For more information, see <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-vpc.html" target="_blank">VPC Endpoints for Amazon Elasticsearch Service Domains</a>.</p>
    #[serde(rename = "VPCOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_options: Option<VPCDerivedInfo>,
}

/// <p> Status of the Elasticsearch version options for the specified Elasticsearch domain.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ElasticsearchVersionStatus {
    /// <p> Specifies the Elasticsearch version for the specified Elasticsearch domain.</p>
    #[serde(rename = "Options")]
    pub options: String,
    /// <p> Specifies the status of the Elasticsearch version options for the specified Elasticsearch domain.</p>
    #[serde(rename = "Status")]
    pub status: OptionStatus,
}

/// <p>Specifies the Encryption At Rest Options.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EncryptionAtRestOptions {
    /// <p>Specifies the option to enable Encryption At Rest.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p> Specifies the KMS Key ID for Encryption At Rest options.</p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
}

/// <p> Status of the Encryption At Rest options for the specified Elasticsearch domain.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EncryptionAtRestOptionsStatus {
    /// <p> Specifies the Encryption At Rest options for the specified Elasticsearch domain.</p>
    #[serde(rename = "Options")]
    pub options: EncryptionAtRestOptions,
    /// <p> Specifies the status of the Encryption At Rest options for the specified Elasticsearch domain.</p>
    #[serde(rename = "Status")]
    pub status: OptionStatus,
}

/// <p> Container for request parameters to <code> <a>GetCompatibleElasticsearchVersions</a> </code> operation. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetCompatibleElasticsearchVersionsRequest {
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
}

/// <p> Container for response returned by <code> <a>GetCompatibleElasticsearchVersions</a> </code> operation. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetCompatibleElasticsearchVersionsResponse {
    /// <p> A map of compatible Elasticsearch versions returned as part of the <code> <a>GetCompatibleElasticsearchVersions</a> </code> operation. </p>
    #[serde(rename = "CompatibleElasticsearchVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatible_elasticsearch_versions: Option<Vec<CompatibleVersionsMap>>,
}

/// <p> Container for request parameters to <code> <a>GetUpgradeHistory</a> </code> operation. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetUpgradeHistoryRequest {
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p> Container for response returned by <code> <a>GetUpgradeHistory</a> </code> operation. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetUpgradeHistoryResponse {
    /// <p>Pagination token that needs to be supplied to the next call to get the next page of results</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p> A list of <code> <a>UpgradeHistory</a> </code> objects corresponding to each Upgrade or Upgrade Eligibility Check performed on a domain returned as part of <code> <a>GetUpgradeHistoryResponse</a> </code> object. </p>
    #[serde(rename = "UpgradeHistories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgrade_histories: Option<Vec<UpgradeHistory>>,
}

/// <p> Container for request parameters to <code> <a>GetUpgradeStatus</a> </code> operation. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetUpgradeStatusRequest {
    #[serde(rename = "DomainName")]
    pub domain_name: String,
}

/// <p> Container for response returned by <code> <a>GetUpgradeStatus</a> </code> operation. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetUpgradeStatusResponse {
    /// <p> One of 4 statuses that a step can go through returned as part of the <code> <a>GetUpgradeStatusResponse</a> </code> object. The status can take one of the following values: <ul> <li>In Progress</li> <li>Succeeded</li> <li>Succeeded with Issues</li> <li>Failed</li> </ul> </p>
    #[serde(rename = "StepStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_status: Option<String>,
    /// <p>A string that describes the update briefly</p>
    #[serde(rename = "UpgradeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgrade_name: Option<String>,
    /// <p> Represents one of 3 steps that an Upgrade or Upgrade Eligibility Check does through: <ul> <li>PreUpgradeCheck</li> <li>Snapshot</li> <li>Upgrade</li> </ul> </p>
    #[serde(rename = "UpgradeStep")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgrade_step: Option<String>,
}

/// <p> InstanceCountLimits represents the limits on number of instances that be created in Amazon Elasticsearch for given InstanceType. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InstanceCountLimits {
    #[serde(rename = "MaximumInstanceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_instance_count: Option<i64>,
    #[serde(rename = "MinimumInstanceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_instance_count: Option<i64>,
}

/// <p>InstanceLimits represents the list of instance related attributes that are available for given InstanceType. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InstanceLimits {
    #[serde(rename = "InstanceCountLimits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_count_limits: Option<InstanceCountLimits>,
}

/// <p> Limits for given InstanceType and for each of it's role. <br/> Limits contains following <code> <a>StorageTypes,</a> </code> <code> <a>InstanceLimits</a> </code> and <code> <a>AdditionalLimits</a> </code> </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Limits {
    /// <p> List of additional limits that are specific to a given InstanceType and for each of it's <code> <a>InstanceRole</a> </code> . </p>
    #[serde(rename = "AdditionalLimits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_limits: Option<Vec<AdditionalLimit>>,
    #[serde(rename = "InstanceLimits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_limits: Option<InstanceLimits>,
    /// <p>StorageType represents the list of storage related types and attributes that are available for given InstanceType. </p>
    #[serde(rename = "StorageTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_types: Option<Vec<StorageType>>,
}

/// <p>The result of a <code>ListDomainNames</code> operation. Contains the names of all Elasticsearch domains owned by this account.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListDomainNamesResponse {
    /// <p>List of Elasticsearch domain names.</p>
    #[serde(rename = "DomainNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_names: Option<Vec<DomainInfo>>,
}

/// <p> Container for the parameters to the <code> <a>ListElasticsearchInstanceTypes</a> </code> operation. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListElasticsearchInstanceTypesRequest {
    /// <p>DomainName represents the name of the Domain that we are trying to modify. This should be present only if we are querying for list of available Elasticsearch instance types when modifying existing domain. </p>
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// <p>Version of Elasticsearch for which list of supported elasticsearch instance types are needed. </p>
    #[serde(rename = "ElasticsearchVersion")]
    pub elasticsearch_version: String,
    /// <p> Set this value to limit the number of results returned. Value provided must be greater than 30 else it wont be honored. </p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>NextToken should be sent in case if earlier API call produced result containing NextToken. It is used for pagination. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p> Container for the parameters returned by <code> <a>ListElasticsearchInstanceTypes</a> </code> operation. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListElasticsearchInstanceTypesResponse {
    /// <p> List of instance types supported by Amazon Elasticsearch service for given <code> <a>ElasticsearchVersion</a> </code> </p>
    #[serde(rename = "ElasticsearchInstanceTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elasticsearch_instance_types: Option<Vec<String>>,
    /// <p>In case if there are more results available NextToken would be present, make further request to the same API with received NextToken to paginate remaining results. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p> Container for the parameters to the <code> <a>ListElasticsearchVersions</a> </code> operation. <p> Use <code> <a>MaxResults</a> </code> to control the maximum number of results to retrieve in a single call. </p> <p> Use <code> <a>NextToken</a> </code> in response to retrieve more results. If the received response does not contain a NextToken, then there are no more results to retrieve. </p> </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListElasticsearchVersionsRequest {
    /// <p> Set this value to limit the number of results returned. Value provided must be greater than 10 else it wont be honored. </p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p> Container for the parameters for response received from <code> <a>ListElasticsearchVersions</a> </code> operation. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListElasticsearchVersionsResponse {
    #[serde(rename = "ElasticsearchVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elasticsearch_versions: Option<Vec<String>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Container for the parameters to the <code><a>ListTags</a></code> operation. Specify the <code>ARN</code> for the Elasticsearch domain to which the tags are attached that you want to view are attached.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsRequest {
    /// <p> Specify the <code>ARN</code> for the Elasticsearch domain to which the tags are attached that you want to view.</p>
    #[serde(rename = "ARN")]
    pub arn: String,
}

/// <p>The result of a <code>ListTags</code> operation. Contains tags for all requested Elasticsearch domains.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsResponse {
    /// <p> List of <code>Tag</code> for the requested Elasticsearch domain.</p>
    #[serde(rename = "TagList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<Tag>>,
}

/// <p>Log Publishing option that is set for given domain. <br/>Attributes and their details: <ul> <li>CloudWatchLogsLogGroupArn: ARN of the Cloudwatch log group to which log needs to be published.</li> <li>Enabled: Whether the log publishing for given log type is enabled or not</li> </ul> </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogPublishingOption {
    #[serde(rename = "CloudWatchLogsLogGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_log_group_arn: Option<String>,
    /// <p> Specifies whether given log publishing option is enabled or not.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// <p>The configured log publishing options for the domain and their current status.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LogPublishingOptionsStatus {
    /// <p>The log publishing options configured for the Elasticsearch domain.</p>
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<::std::collections::HashMap<String, LogPublishingOption>>,
    /// <p>The status of the log publishing options for the Elasticsearch domain. See <code>OptionStatus</code> for the status information that's included. </p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OptionStatus>,
}

/// <p>Specifies the node-to-node encryption options.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeToNodeEncryptionOptions {
    /// <p>Specify true to enable node-to-node encryption.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// <p>Status of the node-to-node encryption options for the specified Elasticsearch domain.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct NodeToNodeEncryptionOptionsStatus {
    /// <p>Specifies the node-to-node encryption options for the specified Elasticsearch domain.</p>
    #[serde(rename = "Options")]
    pub options: NodeToNodeEncryptionOptions,
    /// <p>Specifies the status of the node-to-node encryption options for the specified Elasticsearch domain.</p>
    #[serde(rename = "Status")]
    pub status: OptionStatus,
}

/// <p>Provides the current status of the entity.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct OptionStatus {
    /// <p>Timestamp which tells the creation date for the entity.</p>
    #[serde(rename = "CreationDate")]
    pub creation_date: f64,
    /// <p>Indicates whether the Elasticsearch domain is being deleted.</p>
    #[serde(rename = "PendingDeletion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_deletion: Option<bool>,
    /// <p>Provides the <code>OptionState</code> for the Elasticsearch domain.</p>
    #[serde(rename = "State")]
    pub state: String,
    /// <p>Timestamp which tells the last updated time for the entity.</p>
    #[serde(rename = "UpdateDate")]
    pub update_date: f64,
    /// <p>Specifies the latest version for the entity.</p>
    #[serde(rename = "UpdateVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_version: Option<i64>,
}

/// <p>Container for parameters to <code>PurchaseReservedElasticsearchInstanceOffering</code></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PurchaseReservedElasticsearchInstanceOfferingRequest {
    /// <p>The number of Elasticsearch instances to reserve.</p>
    #[serde(rename = "InstanceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_count: Option<i64>,
    /// <p>A customer-specified identifier to track this reservation.</p>
    #[serde(rename = "ReservationName")]
    pub reservation_name: String,
    /// <p>The ID of the reserved Elasticsearch instance offering to purchase.</p>
    #[serde(rename = "ReservedElasticsearchInstanceOfferingId")]
    pub reserved_elasticsearch_instance_offering_id: String,
}

/// <p>Represents the output of a <code>PurchaseReservedElasticsearchInstanceOffering</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PurchaseReservedElasticsearchInstanceOfferingResponse {
    /// <p>The customer-specified identifier used to track this reservation.</p>
    #[serde(rename = "ReservationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_name: Option<String>,
    /// <p>Details of the reserved Elasticsearch instance which was purchased.</p>
    #[serde(rename = "ReservedElasticsearchInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_elasticsearch_instance_id: Option<String>,
}

/// <p>Contains the specific price and frequency of a recurring charges for a reserved Elasticsearch instance, or for a reserved Elasticsearch instance offering.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RecurringCharge {
    /// <p>The monetary amount of the recurring charge.</p>
    #[serde(rename = "RecurringChargeAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_charge_amount: Option<f64>,
    /// <p>The frequency of the recurring charge.</p>
    #[serde(rename = "RecurringChargeFrequency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_charge_frequency: Option<String>,
}

/// <p>Container for the parameters to the <code><a>RemoveTags</a></code> operation. Specify the <code>ARN</code> for the Elasticsearch domain from which you want to remove the specified <code>TagKey</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RemoveTagsRequest {
    /// <p>Specifies the <code>ARN</code> for the Elasticsearch domain from which you want to delete the specified tags.</p>
    #[serde(rename = "ARN")]
    pub arn: String,
    /// <p>Specifies the <code>TagKey</code> list which you want to remove from the Elasticsearch domain.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

/// <p>Details of a reserved Elasticsearch instance.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ReservedElasticsearchInstance {
    /// <p>The currency code for the reserved Elasticsearch instance offering.</p>
    #[serde(rename = "CurrencyCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    /// <p>The duration, in seconds, for which the Elasticsearch instance is reserved.</p>
    #[serde(rename = "Duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// <p>The number of Elasticsearch instances that have been reserved.</p>
    #[serde(rename = "ElasticsearchInstanceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elasticsearch_instance_count: Option<i64>,
    /// <p>The Elasticsearch instance type offered by the reserved instance offering.</p>
    #[serde(rename = "ElasticsearchInstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elasticsearch_instance_type: Option<String>,
    /// <p>The upfront fixed charge you will paid to purchase the specific reserved Elasticsearch instance offering. </p>
    #[serde(rename = "FixedPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_price: Option<f64>,
    /// <p>The payment option as defined in the reserved Elasticsearch instance offering.</p>
    #[serde(rename = "PaymentOption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_option: Option<String>,
    /// <p>The charge to your account regardless of whether you are creating any domains using the instance offering.</p>
    #[serde(rename = "RecurringCharges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_charges: Option<Vec<RecurringCharge>>,
    /// <p>The customer-specified identifier to track this reservation.</p>
    #[serde(rename = "ReservationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_name: Option<String>,
    /// <p>The unique identifier for the reservation.</p>
    #[serde(rename = "ReservedElasticsearchInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_elasticsearch_instance_id: Option<String>,
    /// <p>The offering identifier.</p>
    #[serde(rename = "ReservedElasticsearchInstanceOfferingId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_elasticsearch_instance_offering_id: Option<String>,
    /// <p>The time the reservation started.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p>The state of the reserved Elasticsearch instance.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The rate you are charged for each hour for the domain that is using this reserved instance.</p>
    #[serde(rename = "UsagePrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_price: Option<f64>,
}

/// <p>Details of a reserved Elasticsearch instance offering.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ReservedElasticsearchInstanceOffering {
    /// <p>The currency code for the reserved Elasticsearch instance offering.</p>
    #[serde(rename = "CurrencyCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    /// <p>The duration, in seconds, for which the offering will reserve the Elasticsearch instance.</p>
    #[serde(rename = "Duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// <p>The Elasticsearch instance type offered by the reserved instance offering.</p>
    #[serde(rename = "ElasticsearchInstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elasticsearch_instance_type: Option<String>,
    /// <p>The upfront fixed charge you will pay to purchase the specific reserved Elasticsearch instance offering. </p>
    #[serde(rename = "FixedPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_price: Option<f64>,
    /// <p>Payment option for the reserved Elasticsearch instance offering</p>
    #[serde(rename = "PaymentOption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_option: Option<String>,
    /// <p>The charge to your account regardless of whether you are creating any domains using the instance offering.</p>
    #[serde(rename = "RecurringCharges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_charges: Option<Vec<RecurringCharge>>,
    /// <p>The Elasticsearch reserved instance offering identifier.</p>
    #[serde(rename = "ReservedElasticsearchInstanceOfferingId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_elasticsearch_instance_offering_id: Option<String>,
    /// <p>The rate you are charged for each hour the domain that is using the offering is running.</p>
    #[serde(rename = "UsagePrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_price: Option<f64>,
}

/// <p>The current options of an Elasticsearch domain service software options.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ServiceSoftwareOptions {
    /// <p>Timestamp, in Epoch time, until which you can manually request a service software update. After this date, we automatically update your service software.</p>
    #[serde(rename = "AutomatedUpdateDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automated_update_date: Option<f64>,
    /// <p><code>True</code> if you are able to cancel your service software version update. <code>False</code> if you are not able to cancel your service software version. </p>
    #[serde(rename = "Cancellable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellable: Option<bool>,
    /// <p>The current service software version that is present on the domain.</p>
    #[serde(rename = "CurrentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_version: Option<String>,
    /// <p>The description of the <code>UpdateStatus</code>.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The new service software version if one is available.</p>
    #[serde(rename = "NewVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_version: Option<String>,
    /// <p><code>True</code> if you are able to update you service software version. <code>False</code> if you are not able to update your service software version. </p>
    #[serde(rename = "UpdateAvailable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_available: Option<bool>,
    /// <p>The status of your service software update. This field can take the following values: <code>ELIGIBLE</code>, <code>PENDING_UPDATE</code>, <code>IN_PROGRESS</code>, <code>COMPLETED</code>, and <code>NOT_ELIGIBLE</code>.</p>
    #[serde(rename = "UpdateStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_status: Option<String>,
}

/// <p>Specifies the time, in UTC format, when the service takes a daily automated snapshot of the specified Elasticsearch domain. Default value is <code>0</code> hours.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SnapshotOptions {
    /// <p>Specifies the time, in UTC format, when the service takes a daily automated snapshot of the specified Elasticsearch domain. Default value is <code>0</code> hours.</p>
    #[serde(rename = "AutomatedSnapshotStartHour")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automated_snapshot_start_hour: Option<i64>,
}

/// <p>Status of a daily automated snapshot.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SnapshotOptionsStatus {
    /// <p>Specifies the daily snapshot options specified for the Elasticsearch domain.</p>
    #[serde(rename = "Options")]
    pub options: SnapshotOptions,
    /// <p>Specifies the status of a daily automated snapshot.</p>
    #[serde(rename = "Status")]
    pub status: OptionStatus,
}

/// <p>Container for the parameters to the <code><a>StartElasticsearchServiceSoftwareUpdate</a></code> operation. Specifies the name of the Elasticsearch domain that you wish to schedule a service software update on.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartElasticsearchServiceSoftwareUpdateRequest {
    /// <p>The name of the domain that you want to update to the latest service software.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
}

/// <p>The result of a <code>StartElasticsearchServiceSoftwareUpdate</code> operation. Contains the status of the update.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartElasticsearchServiceSoftwareUpdateResponse {
    /// <p>The current status of the Elasticsearch service software update.</p>
    #[serde(rename = "ServiceSoftwareOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_software_options: Option<ServiceSoftwareOptions>,
}

/// <p>StorageTypes represents the list of storage related types and their attributes that are available for given InstanceType. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StorageType {
    #[serde(rename = "StorageSubTypeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_sub_type_name: Option<String>,
    /// <p>List of limits that are applicable for given storage type. </p>
    #[serde(rename = "StorageTypeLimits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type_limits: Option<Vec<StorageTypeLimit>>,
    #[serde(rename = "StorageTypeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type_name: Option<String>,
}

/// <p>Limits that are applicable for given storage type. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StorageTypeLimit {
    /// <p> Name of storage limits that are applicable for given storage type. If <code> <a>StorageType</a> </code> is ebs, following storage options are applicable <ol> <li>MinimumVolumeSize</li> Minimum amount of volume size that is applicable for given storage type.It can be empty if it is not applicable. <li>MaximumVolumeSize</li> Maximum amount of volume size that is applicable for given storage type.It can be empty if it is not applicable. <li>MaximumIops</li> Maximum amount of Iops that is applicable for given storage type.It can be empty if it is not applicable. <li>MinimumIops</li> Minimum amount of Iops that is applicable for given storage type.It can be empty if it is not applicable. </ol> </p>
    #[serde(rename = "LimitName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_name: Option<String>,
    /// <p> Values for the <code> <a>StorageTypeLimit$LimitName</a> </code> . </p>
    #[serde(rename = "LimitValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_values: Option<Vec<String>>,
}

/// <p>Specifies a key value pair for a resource tag.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>Specifies the <code>TagKey</code>, the name of the tag. Tag keys must be unique for the Elasticsearch domain to which they are attached.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>Specifies the <code>TagValue</code>, the value assigned to the corresponding tag key. Tag values can be null and do not have to be unique in a tag set. For example, you can have a key value pair in a tag set of <code>project : Trinity</code> and <code>cost-center : Trinity</code></p>
    #[serde(rename = "Value")]
    pub value: String,
}

/// <p>Container for the parameters to the <code><a>UpdateElasticsearchDomain</a></code> operation. Specifies the type and number of instances in the domain cluster.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateElasticsearchDomainConfigRequest {
    /// <p>IAM access policy as a JSON-formatted string.</p>
    #[serde(rename = "AccessPolicies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_policies: Option<String>,
    /// <p>Modifies the advanced option to allow references to indices in an HTTP request body. Must be <code>false</code> when configuring access to individual sub-resources. By default, the value is <code>true</code>. See <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-createupdatedomains.html#es-createdomain-configure-advanced-options" target="_blank">Configuration Advanced Options</a> for more information.</p>
    #[serde(rename = "AdvancedOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_options: Option<::std::collections::HashMap<String, String>>,
    /// <p>Options to specify the Cognito user and identity pools for Kibana authentication. For more information, see <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-cognito-auth.html" target="_blank">Amazon Cognito Authentication for Kibana</a>.</p>
    #[serde(rename = "CognitoOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cognito_options: Option<CognitoOptions>,
    /// <p>Options to specify configuration that will be applied to the domain endpoint.</p>
    #[serde(rename = "DomainEndpointOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_endpoint_options: Option<DomainEndpointOptions>,
    /// <p>The name of the Elasticsearch domain that you are updating. </p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// <p>Specify the type and size of the EBS volume that you want to use. </p>
    #[serde(rename = "EBSOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_options: Option<EBSOptions>,
    /// <p>The type and number of instances to instantiate for the domain cluster.</p>
    #[serde(rename = "ElasticsearchClusterConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elasticsearch_cluster_config: Option<ElasticsearchClusterConfig>,
    /// <p>Map of <code>LogType</code> and <code>LogPublishingOption</code>, each containing options to publish a given type of Elasticsearch log.</p>
    #[serde(rename = "LogPublishingOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_publishing_options: Option<::std::collections::HashMap<String, LogPublishingOption>>,
    /// <p>Option to set the time, in UTC format, for the daily automated snapshot. Default value is <code>0</code> hours. </p>
    #[serde(rename = "SnapshotOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_options: Option<SnapshotOptions>,
    /// <p>Options to specify the subnets and security groups for VPC endpoint. For more information, see <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-vpc.html#es-creating-vpc" target="_blank">Creating a VPC</a> in <i>VPC Endpoints for Amazon Elasticsearch Service Domains</i></p>
    #[serde(rename = "VPCOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_options: Option<VPCOptions>,
}

/// <p>The result of an <code>UpdateElasticsearchDomain</code> request. Contains the status of the Elasticsearch domain being updated.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateElasticsearchDomainConfigResponse {
    /// <p>The status of the updated Elasticsearch domain. </p>
    #[serde(rename = "DomainConfig")]
    pub domain_config: ElasticsearchDomainConfig,
}

/// <p> Container for request parameters to <code> <a>UpgradeElasticsearchDomain</a> </code> operation. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpgradeElasticsearchDomainRequest {
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// <p> This flag, when set to True, indicates that an Upgrade Eligibility Check needs to be performed. This will not actually perform the Upgrade. </p>
    #[serde(rename = "PerformCheckOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perform_check_only: Option<bool>,
    /// <p>The version of Elasticsearch that you intend to upgrade the domain to.</p>
    #[serde(rename = "TargetVersion")]
    pub target_version: String,
}

/// <p> Container for response returned by <code> <a>UpgradeElasticsearchDomain</a> </code> operation. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpgradeElasticsearchDomainResponse {
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// <p> This flag, when set to True, indicates that an Upgrade Eligibility Check needs to be performed. This will not actually perform the Upgrade. </p>
    #[serde(rename = "PerformCheckOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perform_check_only: Option<bool>,
    /// <p>The version of Elasticsearch that you intend to upgrade the domain to.</p>
    #[serde(rename = "TargetVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_version: Option<String>,
}

/// <p>History of the last 10 Upgrades and Upgrade Eligibility Checks.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpgradeHistory {
    /// <p>UTC Timestamp at which the Upgrade API call was made in "yyyy-MM-ddTHH:mm:ssZ" format.</p>
    #[serde(rename = "StartTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<f64>,
    /// <p> A list of <code> <a>UpgradeStepItem</a> </code> s representing information about each step performed as pard of a specific Upgrade or Upgrade Eligibility Check. </p>
    #[serde(rename = "StepsList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps_list: Option<Vec<UpgradeStepItem>>,
    /// <p>A string that describes the update briefly</p>
    #[serde(rename = "UpgradeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgrade_name: Option<String>,
    /// <p> The overall status of the update. The status can take one of the following values: <ul> <li>In Progress</li> <li>Succeeded</li> <li>Succeeded with Issues</li> <li>Failed</li> </ul> </p>
    #[serde(rename = "UpgradeStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgrade_status: Option<String>,
}

/// <p>Represents a single step of the Upgrade or Upgrade Eligibility Check workflow.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpgradeStepItem {
    /// <p>A list of strings containing detailed information about the errors encountered in a particular step.</p>
    #[serde(rename = "Issues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issues: Option<Vec<String>>,
    /// <p>The Floating point value representing progress percentage of a particular step.</p>
    #[serde(rename = "ProgressPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_percent: Option<f64>,
    /// <p> Represents one of 3 steps that an Upgrade or Upgrade Eligibility Check does through: <ul> <li>PreUpgradeCheck</li> <li>Snapshot</li> <li>Upgrade</li> </ul> </p>
    #[serde(rename = "UpgradeStep")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgrade_step: Option<String>,
    /// <p> The status of a particular step during an upgrade. The status can take one of the following values: <ul> <li>In Progress</li> <li>Succeeded</li> <li>Succeeded with Issues</li> <li>Failed</li> </ul> </p>
    #[serde(rename = "UpgradeStepStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgrade_step_status: Option<String>,
}

/// <p>Options to specify the subnets and security groups for VPC endpoint. For more information, see <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-vpc.html" target="_blank"> VPC Endpoints for Amazon Elasticsearch Service Domains</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct VPCDerivedInfo {
    /// <p>The availability zones for the Elasticsearch domain. Exists only if the domain was created with VPCOptions.</p>
    #[serde(rename = "AvailabilityZones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<Vec<String>>,
    /// <p>Specifies the security groups for VPC endpoint.</p>
    #[serde(rename = "SecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    /// <p>Specifies the subnets for VPC endpoint.</p>
    #[serde(rename = "SubnetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    /// <p>The VPC Id for the Elasticsearch domain. Exists only if the domain was created with VPCOptions.</p>
    #[serde(rename = "VPCId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

/// <p> Status of the VPC options for the specified Elasticsearch domain.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct VPCDerivedInfoStatus {
    /// <p> Specifies the VPC options for the specified Elasticsearch domain.</p>
    #[serde(rename = "Options")]
    pub options: VPCDerivedInfo,
    /// <p> Specifies the status of the VPC options for the specified Elasticsearch domain.</p>
    #[serde(rename = "Status")]
    pub status: OptionStatus,
}

/// <p>Options to specify the subnets and security groups for VPC endpoint. For more information, see <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-vpc.html" target="_blank"> VPC Endpoints for Amazon Elasticsearch Service Domains</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct VPCOptions {
    /// <p>Specifies the security groups for VPC endpoint.</p>
    #[serde(rename = "SecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    /// <p>Specifies the subnets for VPC endpoint.</p>
    #[serde(rename = "SubnetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
}

/// <p>Specifies the zone awareness configuration for the domain cluster, such as the number of availability zones.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ZoneAwarenessConfig {
    /// <p>An integer value to indicate the number of availability zones for a domain when zone awareness is enabled. This should be equal to number of subnets if VPC endpoints is enabled</p>
    #[serde(rename = "AvailabilityZoneCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone_count: Option<i64>,
}

/// Errors returned by AddTags
#[derive(Debug, PartialEq)]
pub enum AddTagsError {
    /// <p>An error occurred while processing the request.</p>
    Base(String),
    /// <p>The request processing has failed because of an unknown error, exception or failure (the failure is internal to the service) . Gives http status code of 500.</p>
    Internal(String),
    /// <p>An exception for trying to create more than allowed resources or sub-resources. Gives http status code of 409.</p>
    LimitExceeded(String),
}

impl AddTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AddTagsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BaseException" => return RusotoError::Service(AddTagsError::Base(err.msg)),
                "InternalException" => {
                    return RusotoError::Service(AddTagsError::Internal(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(AddTagsError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AddTagsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AddTagsError::Base(ref cause) => write!(f, "{}", cause),
            AddTagsError::Internal(ref cause) => write!(f, "{}", cause),
            AddTagsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AddTagsError {}
/// Errors returned by CancelElasticsearchServiceSoftwareUpdate
#[derive(Debug, PartialEq)]
pub enum CancelElasticsearchServiceSoftwareUpdateError {
    /// <p>An error occurred while processing the request.</p>
    Base(String),
    /// <p>The request processing has failed because of an unknown error, exception or failure (the failure is internal to the service) . Gives http status code of 500.</p>
    Internal(String),
    /// <p>An exception for accessing or deleting a resource that does not exist. Gives http status code of 400.</p>
    ResourceNotFound(String),
}

impl CancelElasticsearchServiceSoftwareUpdateError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CancelElasticsearchServiceSoftwareUpdateError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BaseException" => {
                    return RusotoError::Service(
                        CancelElasticsearchServiceSoftwareUpdateError::Base(err.msg),
                    )
                }
                "InternalException" => {
                    return RusotoError::Service(
                        CancelElasticsearchServiceSoftwareUpdateError::Internal(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        CancelElasticsearchServiceSoftwareUpdateError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CancelElasticsearchServiceSoftwareUpdateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CancelElasticsearchServiceSoftwareUpdateError::Base(ref cause) => {
                write!(f, "{}", cause)
            }
            CancelElasticsearchServiceSoftwareUpdateError::Internal(ref cause) => {
                write!(f, "{}", cause)
            }
            CancelElasticsearchServiceSoftwareUpdateError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CancelElasticsearchServiceSoftwareUpdateError {}
/// Errors returned by CreateElasticsearchDomain
#[derive(Debug, PartialEq)]
pub enum CreateElasticsearchDomainError {
    /// <p>An error occurred while processing the request.</p>
    Base(String),
    /// <p>An error occured because the client wanted to access a not supported operation. Gives http status code of 409.</p>
    DisabledOperation(String),
    /// <p>The request processing has failed because of an unknown error, exception or failure (the failure is internal to the service) . Gives http status code of 500.</p>
    Internal(String),
    /// <p>An exception for trying to create or access sub-resource that is either invalid or not supported. Gives http status code of 409.</p>
    InvalidType(String),
    /// <p>An exception for trying to create more than allowed resources or sub-resources. Gives http status code of 409.</p>
    LimitExceeded(String),
    /// <p>An exception for creating a resource that already exists. Gives http status code of 400.</p>
    ResourceAlreadyExists(String),
}

impl CreateElasticsearchDomainError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateElasticsearchDomainError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BaseException" => {
                    return RusotoError::Service(CreateElasticsearchDomainError::Base(err.msg))
                }
                "DisabledOperationException" => {
                    return RusotoError::Service(CreateElasticsearchDomainError::DisabledOperation(
                        err.msg,
                    ))
                }
                "InternalException" => {
                    return RusotoError::Service(CreateElasticsearchDomainError::Internal(err.msg))
                }
                "InvalidTypeException" => {
                    return RusotoError::Service(CreateElasticsearchDomainError::InvalidType(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateElasticsearchDomainError::LimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(
                        CreateElasticsearchDomainError::ResourceAlreadyExists(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateElasticsearchDomainError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateElasticsearchDomainError::Base(ref cause) => write!(f, "{}", cause),
            CreateElasticsearchDomainError::DisabledOperation(ref cause) => write!(f, "{}", cause),
            CreateElasticsearchDomainError::Internal(ref cause) => write!(f, "{}", cause),
            CreateElasticsearchDomainError::InvalidType(ref cause) => write!(f, "{}", cause),
            CreateElasticsearchDomainError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateElasticsearchDomainError::ResourceAlreadyExists(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateElasticsearchDomainError {}
/// Errors returned by DeleteElasticsearchDomain
#[derive(Debug, PartialEq)]
pub enum DeleteElasticsearchDomainError {
    /// <p>An error occurred while processing the request.</p>
    Base(String),
    /// <p>The request processing has failed because of an unknown error, exception or failure (the failure is internal to the service) . Gives http status code of 500.</p>
    Internal(String),
    /// <p>An exception for accessing or deleting a resource that does not exist. Gives http status code of 400.</p>
    ResourceNotFound(String),
}

impl DeleteElasticsearchDomainError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteElasticsearchDomainError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BaseException" => {
                    return RusotoError::Service(DeleteElasticsearchDomainError::Base(err.msg))
                }
                "InternalException" => {
                    return RusotoError::Service(DeleteElasticsearchDomainError::Internal(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteElasticsearchDomainError::ResourceNotFound(
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
impl fmt::Display for DeleteElasticsearchDomainError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteElasticsearchDomainError::Base(ref cause) => write!(f, "{}", cause),
            DeleteElasticsearchDomainError::Internal(ref cause) => write!(f, "{}", cause),
            DeleteElasticsearchDomainError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteElasticsearchDomainError {}
/// Errors returned by DeleteElasticsearchServiceRole
#[derive(Debug, PartialEq)]
pub enum DeleteElasticsearchServiceRoleError {
    /// <p>An error occurred while processing the request.</p>
    Base(String),
    /// <p>The request processing has failed because of an unknown error, exception or failure (the failure is internal to the service) . Gives http status code of 500.</p>
    Internal(String),
}

impl DeleteElasticsearchServiceRoleError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteElasticsearchServiceRoleError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BaseException" => {
                    return RusotoError::Service(DeleteElasticsearchServiceRoleError::Base(err.msg))
                }
                "InternalException" => {
                    return RusotoError::Service(DeleteElasticsearchServiceRoleError::Internal(
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
impl fmt::Display for DeleteElasticsearchServiceRoleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteElasticsearchServiceRoleError::Base(ref cause) => write!(f, "{}", cause),
            DeleteElasticsearchServiceRoleError::Internal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteElasticsearchServiceRoleError {}
/// Errors returned by DescribeElasticsearchDomain
#[derive(Debug, PartialEq)]
pub enum DescribeElasticsearchDomainError {
    /// <p>An error occurred while processing the request.</p>
    Base(String),
    /// <p>The request processing has failed because of an unknown error, exception or failure (the failure is internal to the service) . Gives http status code of 500.</p>
    Internal(String),
    /// <p>An exception for accessing or deleting a resource that does not exist. Gives http status code of 400.</p>
    ResourceNotFound(String),
}

impl DescribeElasticsearchDomainError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeElasticsearchDomainError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BaseException" => {
                    return RusotoError::Service(DescribeElasticsearchDomainError::Base(err.msg))
                }
                "InternalException" => {
                    return RusotoError::Service(DescribeElasticsearchDomainError::Internal(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DescribeElasticsearchDomainError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeElasticsearchDomainError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeElasticsearchDomainError::Base(ref cause) => write!(f, "{}", cause),
            DescribeElasticsearchDomainError::Internal(ref cause) => write!(f, "{}", cause),
            DescribeElasticsearchDomainError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeElasticsearchDomainError {}
/// Errors returned by DescribeElasticsearchDomainConfig
#[derive(Debug, PartialEq)]
pub enum DescribeElasticsearchDomainConfigError {
    /// <p>An error occurred while processing the request.</p>
    Base(String),
    /// <p>The request processing has failed because of an unknown error, exception or failure (the failure is internal to the service) . Gives http status code of 500.</p>
    Internal(String),
    /// <p>An exception for accessing or deleting a resource that does not exist. Gives http status code of 400.</p>
    ResourceNotFound(String),
}

impl DescribeElasticsearchDomainConfigError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeElasticsearchDomainConfigError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BaseException" => {
                    return RusotoError::Service(DescribeElasticsearchDomainConfigError::Base(
                        err.msg,
                    ))
                }
                "InternalException" => {
                    return RusotoError::Service(DescribeElasticsearchDomainConfigError::Internal(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DescribeElasticsearchDomainConfigError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeElasticsearchDomainConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeElasticsearchDomainConfigError::Base(ref cause) => write!(f, "{}", cause),
            DescribeElasticsearchDomainConfigError::Internal(ref cause) => write!(f, "{}", cause),
            DescribeElasticsearchDomainConfigError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeElasticsearchDomainConfigError {}
/// Errors returned by DescribeElasticsearchDomains
#[derive(Debug, PartialEq)]
pub enum DescribeElasticsearchDomainsError {
    /// <p>An error occurred while processing the request.</p>
    Base(String),
    /// <p>The request processing has failed because of an unknown error, exception or failure (the failure is internal to the service) . Gives http status code of 500.</p>
    Internal(String),
}

impl DescribeElasticsearchDomainsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeElasticsearchDomainsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BaseException" => {
                    return RusotoError::Service(DescribeElasticsearchDomainsError::Base(err.msg))
                }
                "InternalException" => {
                    return RusotoError::Service(DescribeElasticsearchDomainsError::Internal(
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
impl fmt::Display for DescribeElasticsearchDomainsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeElasticsearchDomainsError::Base(ref cause) => write!(f, "{}", cause),
            DescribeElasticsearchDomainsError::Internal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeElasticsearchDomainsError {}
/// Errors returned by DescribeElasticsearchInstanceTypeLimits
#[derive(Debug, PartialEq)]
pub enum DescribeElasticsearchInstanceTypeLimitsError {
    /// <p>An error occurred while processing the request.</p>
    Base(String),
    /// <p>The request processing has failed because of an unknown error, exception or failure (the failure is internal to the service) . Gives http status code of 500.</p>
    Internal(String),
    /// <p>An exception for trying to create or access sub-resource that is either invalid or not supported. Gives http status code of 409.</p>
    InvalidType(String),
    /// <p>An exception for trying to create more than allowed resources or sub-resources. Gives http status code of 409.</p>
    LimitExceeded(String),
    /// <p>An exception for accessing or deleting a resource that does not exist. Gives http status code of 400.</p>
    ResourceNotFound(String),
}

impl DescribeElasticsearchInstanceTypeLimitsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeElasticsearchInstanceTypeLimitsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BaseException" => {
                    return RusotoError::Service(
                        DescribeElasticsearchInstanceTypeLimitsError::Base(err.msg),
                    )
                }
                "InternalException" => {
                    return RusotoError::Service(
                        DescribeElasticsearchInstanceTypeLimitsError::Internal(err.msg),
                    )
                }
                "InvalidTypeException" => {
                    return RusotoError::Service(
                        DescribeElasticsearchInstanceTypeLimitsError::InvalidType(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(
                        DescribeElasticsearchInstanceTypeLimitsError::LimitExceeded(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DescribeElasticsearchInstanceTypeLimitsError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeElasticsearchInstanceTypeLimitsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeElasticsearchInstanceTypeLimitsError::Base(ref cause) => write!(f, "{}", cause),
            DescribeElasticsearchInstanceTypeLimitsError::Internal(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeElasticsearchInstanceTypeLimitsError::InvalidType(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeElasticsearchInstanceTypeLimitsError::LimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeElasticsearchInstanceTypeLimitsError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeElasticsearchInstanceTypeLimitsError {}
/// Errors returned by DescribeReservedElasticsearchInstanceOfferings
#[derive(Debug, PartialEq)]
pub enum DescribeReservedElasticsearchInstanceOfferingsError {
    /// <p>An error occured because the client wanted to access a not supported operation. Gives http status code of 409.</p>
    DisabledOperation(String),
    /// <p>The request processing has failed because of an unknown error, exception or failure (the failure is internal to the service) . Gives http status code of 500.</p>
    Internal(String),
    /// <p>An exception for accessing or deleting a resource that does not exist. Gives http status code of 400.</p>
    ResourceNotFound(String),
}

impl DescribeReservedElasticsearchInstanceOfferingsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeReservedElasticsearchInstanceOfferingsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "DisabledOperationException" => {
                    return RusotoError::Service(
                        DescribeReservedElasticsearchInstanceOfferingsError::DisabledOperation(
                            err.msg,
                        ),
                    )
                }
                "InternalException" => {
                    return RusotoError::Service(
                        DescribeReservedElasticsearchInstanceOfferingsError::Internal(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DescribeReservedElasticsearchInstanceOfferingsError::ResourceNotFound(
                            err.msg,
                        ),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeReservedElasticsearchInstanceOfferingsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeReservedElasticsearchInstanceOfferingsError::DisabledOperation(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeReservedElasticsearchInstanceOfferingsError::Internal(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeReservedElasticsearchInstanceOfferingsError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeReservedElasticsearchInstanceOfferingsError {}
/// Errors returned by DescribeReservedElasticsearchInstances
#[derive(Debug, PartialEq)]
pub enum DescribeReservedElasticsearchInstancesError {
    /// <p>An error occured because the client wanted to access a not supported operation. Gives http status code of 409.</p>
    DisabledOperation(String),
    /// <p>The request processing has failed because of an unknown error, exception or failure (the failure is internal to the service) . Gives http status code of 500.</p>
    Internal(String),
    /// <p>An exception for accessing or deleting a resource that does not exist. Gives http status code of 400.</p>
    ResourceNotFound(String),
}

impl DescribeReservedElasticsearchInstancesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeReservedElasticsearchInstancesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "DisabledOperationException" => {
                    return RusotoError::Service(
                        DescribeReservedElasticsearchInstancesError::DisabledOperation(err.msg),
                    )
                }
                "InternalException" => {
                    return RusotoError::Service(
                        DescribeReservedElasticsearchInstancesError::Internal(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DescribeReservedElasticsearchInstancesError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeReservedElasticsearchInstancesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeReservedElasticsearchInstancesError::DisabledOperation(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeReservedElasticsearchInstancesError::Internal(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeReservedElasticsearchInstancesError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeReservedElasticsearchInstancesError {}
/// Errors returned by GetCompatibleElasticsearchVersions
#[derive(Debug, PartialEq)]
pub enum GetCompatibleElasticsearchVersionsError {
    /// <p>An error occurred while processing the request.</p>
    Base(String),
    /// <p>An error occured because the client wanted to access a not supported operation. Gives http status code of 409.</p>
    DisabledOperation(String),
    /// <p>The request processing has failed because of an unknown error, exception or failure (the failure is internal to the service) . Gives http status code of 500.</p>
    Internal(String),
    /// <p>An exception for accessing or deleting a resource that does not exist. Gives http status code of 400.</p>
    ResourceNotFound(String),
}

impl GetCompatibleElasticsearchVersionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetCompatibleElasticsearchVersionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BaseException" => {
                    return RusotoError::Service(GetCompatibleElasticsearchVersionsError::Base(
                        err.msg,
                    ))
                }
                "DisabledOperationException" => {
                    return RusotoError::Service(
                        GetCompatibleElasticsearchVersionsError::DisabledOperation(err.msg),
                    )
                }
                "InternalException" => {
                    return RusotoError::Service(GetCompatibleElasticsearchVersionsError::Internal(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        GetCompatibleElasticsearchVersionsError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetCompatibleElasticsearchVersionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetCompatibleElasticsearchVersionsError::Base(ref cause) => write!(f, "{}", cause),
            GetCompatibleElasticsearchVersionsError::DisabledOperation(ref cause) => {
                write!(f, "{}", cause)
            }
            GetCompatibleElasticsearchVersionsError::Internal(ref cause) => write!(f, "{}", cause),
            GetCompatibleElasticsearchVersionsError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetCompatibleElasticsearchVersionsError {}
/// Errors returned by GetUpgradeHistory
#[derive(Debug, PartialEq)]
pub enum GetUpgradeHistoryError {
    /// <p>An error occurred while processing the request.</p>
    Base(String),
    /// <p>An error occured because the client wanted to access a not supported operation. Gives http status code of 409.</p>
    DisabledOperation(String),
    /// <p>The request processing has failed because of an unknown error, exception or failure (the failure is internal to the service) . Gives http status code of 500.</p>
    Internal(String),
    /// <p>An exception for accessing or deleting a resource that does not exist. Gives http status code of 400.</p>
    ResourceNotFound(String),
}

impl GetUpgradeHistoryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetUpgradeHistoryError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BaseException" => {
                    return RusotoError::Service(GetUpgradeHistoryError::Base(err.msg))
                }
                "DisabledOperationException" => {
                    return RusotoError::Service(GetUpgradeHistoryError::DisabledOperation(err.msg))
                }
                "InternalException" => {
                    return RusotoError::Service(GetUpgradeHistoryError::Internal(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetUpgradeHistoryError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetUpgradeHistoryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetUpgradeHistoryError::Base(ref cause) => write!(f, "{}", cause),
            GetUpgradeHistoryError::DisabledOperation(ref cause) => write!(f, "{}", cause),
            GetUpgradeHistoryError::Internal(ref cause) => write!(f, "{}", cause),
            GetUpgradeHistoryError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetUpgradeHistoryError {}
/// Errors returned by GetUpgradeStatus
#[derive(Debug, PartialEq)]
pub enum GetUpgradeStatusError {
    /// <p>An error occurred while processing the request.</p>
    Base(String),
    /// <p>An error occured because the client wanted to access a not supported operation. Gives http status code of 409.</p>
    DisabledOperation(String),
    /// <p>The request processing has failed because of an unknown error, exception or failure (the failure is internal to the service) . Gives http status code of 500.</p>
    Internal(String),
    /// <p>An exception for accessing or deleting a resource that does not exist. Gives http status code of 400.</p>
    ResourceNotFound(String),
}

impl GetUpgradeStatusError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetUpgradeStatusError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BaseException" => {
                    return RusotoError::Service(GetUpgradeStatusError::Base(err.msg))
                }
                "DisabledOperationException" => {
                    return RusotoError::Service(GetUpgradeStatusError::DisabledOperation(err.msg))
                }
                "InternalException" => {
                    return RusotoError::Service(GetUpgradeStatusError::Internal(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetUpgradeStatusError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetUpgradeStatusError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetUpgradeStatusError::Base(ref cause) => write!(f, "{}", cause),
            GetUpgradeStatusError::DisabledOperation(ref cause) => write!(f, "{}", cause),
            GetUpgradeStatusError::Internal(ref cause) => write!(f, "{}", cause),
            GetUpgradeStatusError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetUpgradeStatusError {}
/// Errors returned by ListDomainNames
#[derive(Debug, PartialEq)]
pub enum ListDomainNamesError {
    /// <p>An error occurred while processing the request.</p>
    Base(String),
}

impl ListDomainNamesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDomainNamesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BaseException" => {
                    return RusotoError::Service(ListDomainNamesError::Base(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListDomainNamesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDomainNamesError::Base(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListDomainNamesError {}
/// Errors returned by ListElasticsearchInstanceTypes
#[derive(Debug, PartialEq)]
pub enum ListElasticsearchInstanceTypesError {
    /// <p>An error occurred while processing the request.</p>
    Base(String),
    /// <p>The request processing has failed because of an unknown error, exception or failure (the failure is internal to the service) . Gives http status code of 500.</p>
    Internal(String),
    /// <p>An exception for accessing or deleting a resource that does not exist. Gives http status code of 400.</p>
    ResourceNotFound(String),
}

impl ListElasticsearchInstanceTypesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListElasticsearchInstanceTypesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BaseException" => {
                    return RusotoError::Service(ListElasticsearchInstanceTypesError::Base(err.msg))
                }
                "InternalException" => {
                    return RusotoError::Service(ListElasticsearchInstanceTypesError::Internal(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        ListElasticsearchInstanceTypesError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListElasticsearchInstanceTypesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListElasticsearchInstanceTypesError::Base(ref cause) => write!(f, "{}", cause),
            ListElasticsearchInstanceTypesError::Internal(ref cause) => write!(f, "{}", cause),
            ListElasticsearchInstanceTypesError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListElasticsearchInstanceTypesError {}
/// Errors returned by ListElasticsearchVersions
#[derive(Debug, PartialEq)]
pub enum ListElasticsearchVersionsError {
    /// <p>An error occurred while processing the request.</p>
    Base(String),
    /// <p>The request processing has failed because of an unknown error, exception or failure (the failure is internal to the service) . Gives http status code of 500.</p>
    Internal(String),
    /// <p>An exception for accessing or deleting a resource that does not exist. Gives http status code of 400.</p>
    ResourceNotFound(String),
}

impl ListElasticsearchVersionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListElasticsearchVersionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BaseException" => {
                    return RusotoError::Service(ListElasticsearchVersionsError::Base(err.msg))
                }
                "InternalException" => {
                    return RusotoError::Service(ListElasticsearchVersionsError::Internal(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListElasticsearchVersionsError::ResourceNotFound(
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
impl fmt::Display for ListElasticsearchVersionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListElasticsearchVersionsError::Base(ref cause) => write!(f, "{}", cause),
            ListElasticsearchVersionsError::Internal(ref cause) => write!(f, "{}", cause),
            ListElasticsearchVersionsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListElasticsearchVersionsError {}
/// Errors returned by ListTags
#[derive(Debug, PartialEq)]
pub enum ListTagsError {
    /// <p>An error occurred while processing the request.</p>
    Base(String),
    /// <p>The request processing has failed because of an unknown error, exception or failure (the failure is internal to the service) . Gives http status code of 500.</p>
    Internal(String),
    /// <p>An exception for accessing or deleting a resource that does not exist. Gives http status code of 400.</p>
    ResourceNotFound(String),
}

impl ListTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BaseException" => return RusotoError::Service(ListTagsError::Base(err.msg)),
                "InternalException" => {
                    return RusotoError::Service(ListTagsError::Internal(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListTagsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListTagsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTagsError::Base(ref cause) => write!(f, "{}", cause),
            ListTagsError::Internal(ref cause) => write!(f, "{}", cause),
            ListTagsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsError {}
/// Errors returned by PurchaseReservedElasticsearchInstanceOffering
#[derive(Debug, PartialEq)]
pub enum PurchaseReservedElasticsearchInstanceOfferingError {
    /// <p>An error occured because the client wanted to access a not supported operation. Gives http status code of 409.</p>
    DisabledOperation(String),
    /// <p>The request processing has failed because of an unknown error, exception or failure (the failure is internal to the service) . Gives http status code of 500.</p>
    Internal(String),
    /// <p>An exception for trying to create more than allowed resources or sub-resources. Gives http status code of 409.</p>
    LimitExceeded(String),
    /// <p>An exception for creating a resource that already exists. Gives http status code of 400.</p>
    ResourceAlreadyExists(String),
    /// <p>An exception for accessing or deleting a resource that does not exist. Gives http status code of 400.</p>
    ResourceNotFound(String),
}

impl PurchaseReservedElasticsearchInstanceOfferingError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PurchaseReservedElasticsearchInstanceOfferingError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "DisabledOperationException" => {
                    return RusotoError::Service(
                        PurchaseReservedElasticsearchInstanceOfferingError::DisabledOperation(
                            err.msg,
                        ),
                    )
                }
                "InternalException" => {
                    return RusotoError::Service(
                        PurchaseReservedElasticsearchInstanceOfferingError::Internal(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(
                        PurchaseReservedElasticsearchInstanceOfferingError::LimitExceeded(err.msg),
                    )
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(
                        PurchaseReservedElasticsearchInstanceOfferingError::ResourceAlreadyExists(
                            err.msg,
                        ),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        PurchaseReservedElasticsearchInstanceOfferingError::ResourceNotFound(
                            err.msg,
                        ),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PurchaseReservedElasticsearchInstanceOfferingError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PurchaseReservedElasticsearchInstanceOfferingError::DisabledOperation(ref cause) => {
                write!(f, "{}", cause)
            }
            PurchaseReservedElasticsearchInstanceOfferingError::Internal(ref cause) => {
                write!(f, "{}", cause)
            }
            PurchaseReservedElasticsearchInstanceOfferingError::LimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            PurchaseReservedElasticsearchInstanceOfferingError::ResourceAlreadyExists(
                ref cause,
            ) => write!(f, "{}", cause),
            PurchaseReservedElasticsearchInstanceOfferingError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for PurchaseReservedElasticsearchInstanceOfferingError {}
/// Errors returned by RemoveTags
#[derive(Debug, PartialEq)]
pub enum RemoveTagsError {
    /// <p>An error occurred while processing the request.</p>
    Base(String),
    /// <p>The request processing has failed because of an unknown error, exception or failure (the failure is internal to the service) . Gives http status code of 500.</p>
    Internal(String),
}

impl RemoveTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RemoveTagsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BaseException" => return RusotoError::Service(RemoveTagsError::Base(err.msg)),
                "InternalException" => {
                    return RusotoError::Service(RemoveTagsError::Internal(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RemoveTagsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RemoveTagsError::Base(ref cause) => write!(f, "{}", cause),
            RemoveTagsError::Internal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RemoveTagsError {}
/// Errors returned by StartElasticsearchServiceSoftwareUpdate
#[derive(Debug, PartialEq)]
pub enum StartElasticsearchServiceSoftwareUpdateError {
    /// <p>An error occurred while processing the request.</p>
    Base(String),
    /// <p>The request processing has failed because of an unknown error, exception or failure (the failure is internal to the service) . Gives http status code of 500.</p>
    Internal(String),
    /// <p>An exception for accessing or deleting a resource that does not exist. Gives http status code of 400.</p>
    ResourceNotFound(String),
}

impl StartElasticsearchServiceSoftwareUpdateError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<StartElasticsearchServiceSoftwareUpdateError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BaseException" => {
                    return RusotoError::Service(
                        StartElasticsearchServiceSoftwareUpdateError::Base(err.msg),
                    )
                }
                "InternalException" => {
                    return RusotoError::Service(
                        StartElasticsearchServiceSoftwareUpdateError::Internal(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        StartElasticsearchServiceSoftwareUpdateError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartElasticsearchServiceSoftwareUpdateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartElasticsearchServiceSoftwareUpdateError::Base(ref cause) => write!(f, "{}", cause),
            StartElasticsearchServiceSoftwareUpdateError::Internal(ref cause) => {
                write!(f, "{}", cause)
            }
            StartElasticsearchServiceSoftwareUpdateError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for StartElasticsearchServiceSoftwareUpdateError {}
/// Errors returned by UpdateElasticsearchDomainConfig
#[derive(Debug, PartialEq)]
pub enum UpdateElasticsearchDomainConfigError {
    /// <p>An error occurred while processing the request.</p>
    Base(String),
    /// <p>The request processing has failed because of an unknown error, exception or failure (the failure is internal to the service) . Gives http status code of 500.</p>
    Internal(String),
    /// <p>An exception for trying to create or access sub-resource that is either invalid or not supported. Gives http status code of 409.</p>
    InvalidType(String),
    /// <p>An exception for trying to create more than allowed resources or sub-resources. Gives http status code of 409.</p>
    LimitExceeded(String),
    /// <p>An exception for accessing or deleting a resource that does not exist. Gives http status code of 400.</p>
    ResourceNotFound(String),
}

impl UpdateElasticsearchDomainConfigError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateElasticsearchDomainConfigError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BaseException" => {
                    return RusotoError::Service(UpdateElasticsearchDomainConfigError::Base(
                        err.msg,
                    ))
                }
                "InternalException" => {
                    return RusotoError::Service(UpdateElasticsearchDomainConfigError::Internal(
                        err.msg,
                    ))
                }
                "InvalidTypeException" => {
                    return RusotoError::Service(UpdateElasticsearchDomainConfigError::InvalidType(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(
                        UpdateElasticsearchDomainConfigError::LimitExceeded(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        UpdateElasticsearchDomainConfigError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateElasticsearchDomainConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateElasticsearchDomainConfigError::Base(ref cause) => write!(f, "{}", cause),
            UpdateElasticsearchDomainConfigError::Internal(ref cause) => write!(f, "{}", cause),
            UpdateElasticsearchDomainConfigError::InvalidType(ref cause) => write!(f, "{}", cause),
            UpdateElasticsearchDomainConfigError::LimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateElasticsearchDomainConfigError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateElasticsearchDomainConfigError {}
/// Errors returned by UpgradeElasticsearchDomain
#[derive(Debug, PartialEq)]
pub enum UpgradeElasticsearchDomainError {
    /// <p>An error occurred while processing the request.</p>
    Base(String),
    /// <p>An error occured because the client wanted to access a not supported operation. Gives http status code of 409.</p>
    DisabledOperation(String),
    /// <p>The request processing has failed because of an unknown error, exception or failure (the failure is internal to the service) . Gives http status code of 500.</p>
    Internal(String),
    /// <p>An exception for creating a resource that already exists. Gives http status code of 400.</p>
    ResourceAlreadyExists(String),
    /// <p>An exception for accessing or deleting a resource that does not exist. Gives http status code of 400.</p>
    ResourceNotFound(String),
}

impl UpgradeElasticsearchDomainError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpgradeElasticsearchDomainError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BaseException" => {
                    return RusotoError::Service(UpgradeElasticsearchDomainError::Base(err.msg))
                }
                "DisabledOperationException" => {
                    return RusotoError::Service(
                        UpgradeElasticsearchDomainError::DisabledOperation(err.msg),
                    )
                }
                "InternalException" => {
                    return RusotoError::Service(UpgradeElasticsearchDomainError::Internal(err.msg))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(
                        UpgradeElasticsearchDomainError::ResourceAlreadyExists(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpgradeElasticsearchDomainError::ResourceNotFound(
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
impl fmt::Display for UpgradeElasticsearchDomainError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpgradeElasticsearchDomainError::Base(ref cause) => write!(f, "{}", cause),
            UpgradeElasticsearchDomainError::DisabledOperation(ref cause) => write!(f, "{}", cause),
            UpgradeElasticsearchDomainError::Internal(ref cause) => write!(f, "{}", cause),
            UpgradeElasticsearchDomainError::ResourceAlreadyExists(ref cause) => {
                write!(f, "{}", cause)
            }
            UpgradeElasticsearchDomainError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpgradeElasticsearchDomainError {}
/// Trait representing the capabilities of the Amazon Elasticsearch Service API. Amazon Elasticsearch Service clients implement this trait.
pub trait Es {
    /// <p>Attaches tags to an existing Elasticsearch domain. Tags are a set of case-sensitive key value pairs. An Elasticsearch domain may have up to 10 tags. See <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-managedomains.html#es-managedomains-awsresorcetagging" target="_blank"> Tagging Amazon Elasticsearch Service Domains for more information.</a></p>
    fn add_tags(
        &self,
        input: AddTagsRequest,
    ) -> Pin<Box<dyn Future<Output = Result<(), RusotoError<AddTagsError>>> + Send + 'static>>;

    /// <p>Cancels a scheduled service software update for an Amazon ES domain. You can only perform this operation before the <code>AutomatedUpdateDate</code> and when the <code>UpdateStatus</code> is in the <code>PENDING_UPDATE</code> state.</p>
    fn cancel_elasticsearch_service_software_update(
        &self,
        input: CancelElasticsearchServiceSoftwareUpdateRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        CancelElasticsearchServiceSoftwareUpdateResponse,
                        RusotoError<CancelElasticsearchServiceSoftwareUpdateError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Creates a new Elasticsearch domain. For more information, see <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-createupdatedomains.html#es-createdomains" target="_blank">Creating Elasticsearch Domains</a> in the <i>Amazon Elasticsearch Service Developer Guide</i>.</p>
    fn create_elasticsearch_domain(
        &self,
        input: CreateElasticsearchDomainRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        CreateElasticsearchDomainResponse,
                        RusotoError<CreateElasticsearchDomainError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Permanently deletes the specified Elasticsearch domain and all of its data. Once a domain is deleted, it cannot be recovered.</p>
    fn delete_elasticsearch_domain(
        &self,
        input: DeleteElasticsearchDomainRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DeleteElasticsearchDomainResponse,
                        RusotoError<DeleteElasticsearchDomainError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Deletes the service-linked role that Elasticsearch Service uses to manage and maintain VPC domains. Role deletion will fail if any existing VPC domains use the role. You must delete any such Elasticsearch domains before deleting the role. See <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-vpc.html#es-enabling-slr" target="_blank">Deleting Elasticsearch Service Role</a> in <i>VPC Endpoints for Amazon Elasticsearch Service Domains</i>.</p>
    fn delete_elasticsearch_service_role(
        &self,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<(), RusotoError<DeleteElasticsearchServiceRoleError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Returns domain configuration information about the specified Elasticsearch domain, including the domain ID, domain endpoint, and domain ARN.</p>
    fn describe_elasticsearch_domain(
        &self,
        input: DescribeElasticsearchDomainRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DescribeElasticsearchDomainResponse,
                        RusotoError<DescribeElasticsearchDomainError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Provides cluster configuration information about the specified Elasticsearch domain, such as the state, creation date, update version, and update date for cluster options.</p>
    fn describe_elasticsearch_domain_config(
        &self,
        input: DescribeElasticsearchDomainConfigRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DescribeElasticsearchDomainConfigResponse,
                        RusotoError<DescribeElasticsearchDomainConfigError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Returns domain configuration information about the specified Elasticsearch domains, including the domain ID, domain endpoint, and domain ARN.</p>
    fn describe_elasticsearch_domains(
        &self,
        input: DescribeElasticsearchDomainsRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DescribeElasticsearchDomainsResponse,
                        RusotoError<DescribeElasticsearchDomainsError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p> Describe Elasticsearch Limits for a given InstanceType and ElasticsearchVersion. When modifying existing Domain, specify the <code> <a>DomainName</a> </code> to know what Limits are supported for modifying. </p>
    fn describe_elasticsearch_instance_type_limits(
        &self,
        input: DescribeElasticsearchInstanceTypeLimitsRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DescribeElasticsearchInstanceTypeLimitsResponse,
                        RusotoError<DescribeElasticsearchInstanceTypeLimitsError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Lists available reserved Elasticsearch instance offerings.</p>
    fn describe_reserved_elasticsearch_instance_offerings(
        &self,
        input: DescribeReservedElasticsearchInstanceOfferingsRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DescribeReservedElasticsearchInstanceOfferingsResponse,
                        RusotoError<DescribeReservedElasticsearchInstanceOfferingsError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Returns information about reserved Elasticsearch instances for this account.</p>
    fn describe_reserved_elasticsearch_instances(
        &self,
        input: DescribeReservedElasticsearchInstancesRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DescribeReservedElasticsearchInstancesResponse,
                        RusotoError<DescribeReservedElasticsearchInstancesError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p> Returns a list of upgrade compatible Elastisearch versions. You can optionally pass a <code> <a>DomainName</a> </code> to get all upgrade compatible Elasticsearch versions for that specific domain. </p>
    fn get_compatible_elasticsearch_versions(
        &self,
        input: GetCompatibleElasticsearchVersionsRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        GetCompatibleElasticsearchVersionsResponse,
                        RusotoError<GetCompatibleElasticsearchVersionsError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Retrieves the complete history of the last 10 upgrades that were performed on the domain.</p>
    fn get_upgrade_history(
        &self,
        input: GetUpgradeHistoryRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<GetUpgradeHistoryResponse, RusotoError<GetUpgradeHistoryError>>,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Retrieves the latest status of the last upgrade or upgrade eligibility check that was performed on the domain.</p>
    fn get_upgrade_status(
        &self,
        input: GetUpgradeStatusRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<GetUpgradeStatusResponse, RusotoError<GetUpgradeStatusError>>,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Returns the name of all Elasticsearch domains owned by the current user's account. </p>
    fn list_domain_names(
        &self,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<ListDomainNamesResponse, RusotoError<ListDomainNamesError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>List all Elasticsearch instance types that are supported for given ElasticsearchVersion</p>
    fn list_elasticsearch_instance_types(
        &self,
        input: ListElasticsearchInstanceTypesRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        ListElasticsearchInstanceTypesResponse,
                        RusotoError<ListElasticsearchInstanceTypesError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>List all supported Elasticsearch versions</p>
    fn list_elasticsearch_versions(
        &self,
        input: ListElasticsearchVersionsRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        ListElasticsearchVersionsResponse,
                        RusotoError<ListElasticsearchVersionsError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Returns all tags for the given Elasticsearch domain.</p>
    fn list_tags(
        &self,
        input: ListTagsRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<ListTagsResponse, RusotoError<ListTagsError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Allows you to purchase reserved Elasticsearch instances.</p>
    fn purchase_reserved_elasticsearch_instance_offering(
        &self,
        input: PurchaseReservedElasticsearchInstanceOfferingRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        PurchaseReservedElasticsearchInstanceOfferingResponse,
                        RusotoError<PurchaseReservedElasticsearchInstanceOfferingError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Removes the specified set of tags from the specified Elasticsearch domain.</p>
    fn remove_tags(
        &self,
        input: RemoveTagsRequest,
    ) -> Pin<Box<dyn Future<Output = Result<(), RusotoError<RemoveTagsError>>> + Send + 'static>>;

    /// <p>Schedules a service software update for an Amazon ES domain.</p>
    fn start_elasticsearch_service_software_update(
        &self,
        input: StartElasticsearchServiceSoftwareUpdateRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        StartElasticsearchServiceSoftwareUpdateResponse,
                        RusotoError<StartElasticsearchServiceSoftwareUpdateError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Modifies the cluster configuration of the specified Elasticsearch domain, setting as setting the instance type and the number of instances. </p>
    fn update_elasticsearch_domain_config(
        &self,
        input: UpdateElasticsearchDomainConfigRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        UpdateElasticsearchDomainConfigResponse,
                        RusotoError<UpdateElasticsearchDomainConfigError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Allows you to either upgrade your domain or perform an Upgrade eligibility check to a compatible Elasticsearch version.</p>
    fn upgrade_elasticsearch_domain(
        &self,
        input: UpgradeElasticsearchDomainRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        UpgradeElasticsearchDomainResponse,
                        RusotoError<UpgradeElasticsearchDomainError>,
                    >,
                > + Send
                + 'static,
        >,
    >;
}
/// A client for the Amazon Elasticsearch Service API.
#[derive(Clone)]
pub struct EsClient {
    client: Client,
    region: region::Region,
}

impl EsClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> EsClient {
        EsClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> EsClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        EsClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> EsClient {
        EsClient { client, region }
    }
}

impl Es for EsClient {
    /// <p>Attaches tags to an existing Elasticsearch domain. Tags are a set of case-sensitive key value pairs. An Elasticsearch domain may have up to 10 tags. See <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-managedomains.html#es-managedomains-awsresorcetagging" target="_blank"> Tagging Amazon Elasticsearch Service Domains for more information.</a></p>
    fn add_tags(
        &self,
        input: AddTagsRequest,
    ) -> Pin<Box<dyn Future<Output = Result<(), RusotoError<AddTagsError>>> + Send + 'static>> {
        let request_uri = "/2015-01-01/tags";

        let mut request = SignedRequest::new("POST", "es", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = ::std::mem::drop(response);

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(AddTagsError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Cancels a scheduled service software update for an Amazon ES domain. You can only perform this operation before the <code>AutomatedUpdateDate</code> and when the <code>UpdateStatus</code> is in the <code>PENDING_UPDATE</code> state.</p>
    fn cancel_elasticsearch_service_software_update(
        &self,
        input: CancelElasticsearchServiceSoftwareUpdateRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        CancelElasticsearchServiceSoftwareUpdateResponse,
                        RusotoError<CancelElasticsearchServiceSoftwareUpdateError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = "/2015-01-01/es/serviceSoftwareUpdate/cancel";

        let mut request = SignedRequest::new("POST", "es", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<CancelElasticsearchServiceSoftwareUpdateResponse, _>(
                )?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(CancelElasticsearchServiceSoftwareUpdateError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Creates a new Elasticsearch domain. For more information, see <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-createupdatedomains.html#es-createdomains" target="_blank">Creating Elasticsearch Domains</a> in the <i>Amazon Elasticsearch Service Developer Guide</i>.</p>
    fn create_elasticsearch_domain(
        &self,
        input: CreateElasticsearchDomainRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        CreateElasticsearchDomainResponse,
                        RusotoError<CreateElasticsearchDomainError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = "/2015-01-01/es/domain";

        let mut request = SignedRequest::new("POST", "es", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<CreateElasticsearchDomainResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(CreateElasticsearchDomainError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Permanently deletes the specified Elasticsearch domain and all of its data. Once a domain is deleted, it cannot be recovered.</p>
    fn delete_elasticsearch_domain(
        &self,
        input: DeleteElasticsearchDomainRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DeleteElasticsearchDomainResponse,
                        RusotoError<DeleteElasticsearchDomainError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = format!(
            "/2015-01-01/es/domain/{domain_name}",
            domain_name = input.domain_name
        );

        let mut request = SignedRequest::new("DELETE", "es", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<DeleteElasticsearchDomainResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(DeleteElasticsearchDomainError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Deletes the service-linked role that Elasticsearch Service uses to manage and maintain VPC domains. Role deletion will fail if any existing VPC domains use the role. You must delete any such Elasticsearch domains before deleting the role. See <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-vpc.html#es-enabling-slr" target="_blank">Deleting Elasticsearch Service Role</a> in <i>VPC Endpoints for Amazon Elasticsearch Service Domains</i>.</p>
    fn delete_elasticsearch_service_role(
        &self,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<(), RusotoError<DeleteElasticsearchServiceRoleError>>>
                + Send
                + 'static,
        >,
    > {
        let request_uri = "/2015-01-01/es/role";

        let mut request = SignedRequest::new("DELETE", "es", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = ::std::mem::drop(response);

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(DeleteElasticsearchServiceRoleError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Returns domain configuration information about the specified Elasticsearch domain, including the domain ID, domain endpoint, and domain ARN.</p>
    fn describe_elasticsearch_domain(
        &self,
        input: DescribeElasticsearchDomainRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DescribeElasticsearchDomainResponse,
                        RusotoError<DescribeElasticsearchDomainError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = format!(
            "/2015-01-01/es/domain/{domain_name}",
            domain_name = input.domain_name
        );

        let mut request = SignedRequest::new("GET", "es", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<DescribeElasticsearchDomainResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(DescribeElasticsearchDomainError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Provides cluster configuration information about the specified Elasticsearch domain, such as the state, creation date, update version, and update date for cluster options.</p>
    fn describe_elasticsearch_domain_config(
        &self,
        input: DescribeElasticsearchDomainConfigRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DescribeElasticsearchDomainConfigResponse,
                        RusotoError<DescribeElasticsearchDomainConfigError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = format!(
            "/2015-01-01/es/domain/{domain_name}/config",
            domain_name = input.domain_name
        );

        let mut request = SignedRequest::new("GET", "es", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<DescribeElasticsearchDomainConfigResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(DescribeElasticsearchDomainConfigError::from_response(
                    response,
                ))
            }
        }
        .boxed()
    }

    /// <p>Returns domain configuration information about the specified Elasticsearch domains, including the domain ID, domain endpoint, and domain ARN.</p>
    fn describe_elasticsearch_domains(
        &self,
        input: DescribeElasticsearchDomainsRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DescribeElasticsearchDomainsResponse,
                        RusotoError<DescribeElasticsearchDomainsError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = "/2015-01-01/es/domain-info";

        let mut request = SignedRequest::new("POST", "es", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<DescribeElasticsearchDomainsResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(DescribeElasticsearchDomainsError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p> Describe Elasticsearch Limits for a given InstanceType and ElasticsearchVersion. When modifying existing Domain, specify the <code> <a>DomainName</a> </code> to know what Limits are supported for modifying. </p>
    fn describe_elasticsearch_instance_type_limits(
        &self,
        input: DescribeElasticsearchInstanceTypeLimitsRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DescribeElasticsearchInstanceTypeLimitsResponse,
                        RusotoError<DescribeElasticsearchInstanceTypeLimitsError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = format!(
            "/2015-01-01/es/instanceTypeLimits/{elasticsearch_version}/{instance_type}",
            elasticsearch_version = input.elasticsearch_version,
            instance_type = input.instance_type
        );

        let mut request = SignedRequest::new("GET", "es", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.domain_name {
            params.put("domainName", x);
        }
        request.set_params(params);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<DescribeElasticsearchInstanceTypeLimitsResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(DescribeElasticsearchInstanceTypeLimitsError::from_response(
                    response,
                ))
            }
        }
        .boxed()
    }

    /// <p>Lists available reserved Elasticsearch instance offerings.</p>
    fn describe_reserved_elasticsearch_instance_offerings(
        &self,
        input: DescribeReservedElasticsearchInstanceOfferingsRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DescribeReservedElasticsearchInstanceOfferingsResponse,
                        RusotoError<DescribeReservedElasticsearchInstanceOfferingsError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = "/2015-01-01/es/reservedInstanceOfferings";

        let mut request = SignedRequest::new("GET", "es", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.reserved_elasticsearch_instance_offering_id {
            params.put("offeringId", x);
        }
        request.set_params(params);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<DescribeReservedElasticsearchInstanceOfferingsResponse, _>(
                )?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(DescribeReservedElasticsearchInstanceOfferingsError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Returns information about reserved Elasticsearch instances for this account.</p>
    fn describe_reserved_elasticsearch_instances(
        &self,
        input: DescribeReservedElasticsearchInstancesRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DescribeReservedElasticsearchInstancesResponse,
                        RusotoError<DescribeReservedElasticsearchInstancesError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = "/2015-01-01/es/reservedInstances";

        let mut request = SignedRequest::new("GET", "es", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.reserved_elasticsearch_instance_id {
            params.put("reservationId", x);
        }
        request.set_params(params);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<DescribeReservedElasticsearchInstancesResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(DescribeReservedElasticsearchInstancesError::from_response(
                    response,
                ))
            }
        }
        .boxed()
    }

    /// <p> Returns a list of upgrade compatible Elastisearch versions. You can optionally pass a <code> <a>DomainName</a> </code> to get all upgrade compatible Elasticsearch versions for that specific domain. </p>
    fn get_compatible_elasticsearch_versions(
        &self,
        input: GetCompatibleElasticsearchVersionsRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        GetCompatibleElasticsearchVersionsResponse,
                        RusotoError<GetCompatibleElasticsearchVersionsError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = "/2015-01-01/es/compatibleVersions";

        let mut request = SignedRequest::new("GET", "es", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.domain_name {
            params.put("domainName", x);
        }
        request.set_params(params);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<GetCompatibleElasticsearchVersionsResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(GetCompatibleElasticsearchVersionsError::from_response(
                    response,
                ))
            }
        }
        .boxed()
    }

    /// <p>Retrieves the complete history of the last 10 upgrades that were performed on the domain.</p>
    fn get_upgrade_history(
        &self,
        input: GetUpgradeHistoryRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<GetUpgradeHistoryResponse, RusotoError<GetUpgradeHistoryError>>,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = format!(
            "/2015-01-01/es/upgradeDomain/{domain_name}/history",
            domain_name = input.domain_name
        );

        let mut request = SignedRequest::new("GET", "es", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<GetUpgradeHistoryResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(GetUpgradeHistoryError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Retrieves the latest status of the last upgrade or upgrade eligibility check that was performed on the domain.</p>
    fn get_upgrade_status(
        &self,
        input: GetUpgradeStatusRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<GetUpgradeStatusResponse, RusotoError<GetUpgradeStatusError>>,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = format!(
            "/2015-01-01/es/upgradeDomain/{domain_name}/status",
            domain_name = input.domain_name
        );

        let mut request = SignedRequest::new("GET", "es", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<GetUpgradeStatusResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(GetUpgradeStatusError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Returns the name of all Elasticsearch domains owned by the current user's account. </p>
    fn list_domain_names(
        &self,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<ListDomainNamesResponse, RusotoError<ListDomainNamesError>>>
                + Send
                + 'static,
        >,
    > {
        let request_uri = "/2015-01-01/domain";

        let mut request = SignedRequest::new("GET", "es", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<ListDomainNamesResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(ListDomainNamesError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>List all Elasticsearch instance types that are supported for given ElasticsearchVersion</p>
    fn list_elasticsearch_instance_types(
        &self,
        input: ListElasticsearchInstanceTypesRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        ListElasticsearchInstanceTypesResponse,
                        RusotoError<ListElasticsearchInstanceTypesError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = format!(
            "/2015-01-01/es/instanceTypes/{elasticsearch_version}",
            elasticsearch_version = input.elasticsearch_version
        );

        let mut request = SignedRequest::new("GET", "es", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.domain_name {
            params.put("domainName", x);
        }
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<ListElasticsearchInstanceTypesResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(ListElasticsearchInstanceTypesError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>List all supported Elasticsearch versions</p>
    fn list_elasticsearch_versions(
        &self,
        input: ListElasticsearchVersionsRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        ListElasticsearchVersionsResponse,
                        RusotoError<ListElasticsearchVersionsError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = "/2015-01-01/es/versions";

        let mut request = SignedRequest::new("GET", "es", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<ListElasticsearchVersionsResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(ListElasticsearchVersionsError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Returns all tags for the given Elasticsearch domain.</p>
    fn list_tags(
        &self,
        input: ListTagsRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<ListTagsResponse, RusotoError<ListTagsError>>>
                + Send
                + 'static,
        >,
    > {
        let request_uri = "/2015-01-01/tags/";

        let mut request = SignedRequest::new("GET", "es", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        params.put("arn", &input.arn);
        request.set_params(params);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<ListTagsResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(ListTagsError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Allows you to purchase reserved Elasticsearch instances.</p>
    fn purchase_reserved_elasticsearch_instance_offering(
        &self,
        input: PurchaseReservedElasticsearchInstanceOfferingRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        PurchaseReservedElasticsearchInstanceOfferingResponse,
                        RusotoError<PurchaseReservedElasticsearchInstanceOfferingError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = "/2015-01-01/es/purchaseReservedInstanceOffering";

        let mut request = SignedRequest::new("POST", "es", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<PurchaseReservedElasticsearchInstanceOfferingResponse, _>(
                )?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(PurchaseReservedElasticsearchInstanceOfferingError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Removes the specified set of tags from the specified Elasticsearch domain.</p>
    fn remove_tags(
        &self,
        input: RemoveTagsRequest,
    ) -> Pin<Box<dyn Future<Output = Result<(), RusotoError<RemoveTagsError>>> + Send + 'static>>
    {
        let request_uri = "/2015-01-01/tags-removal";

        let mut request = SignedRequest::new("POST", "es", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = ::std::mem::drop(response);

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(RemoveTagsError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Schedules a service software update for an Amazon ES domain.</p>
    fn start_elasticsearch_service_software_update(
        &self,
        input: StartElasticsearchServiceSoftwareUpdateRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        StartElasticsearchServiceSoftwareUpdateResponse,
                        RusotoError<StartElasticsearchServiceSoftwareUpdateError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = "/2015-01-01/es/serviceSoftwareUpdate/start";

        let mut request = SignedRequest::new("POST", "es", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<StartElasticsearchServiceSoftwareUpdateResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(StartElasticsearchServiceSoftwareUpdateError::from_response(
                    response,
                ))
            }
        }
        .boxed()
    }

    /// <p>Modifies the cluster configuration of the specified Elasticsearch domain, setting as setting the instance type and the number of instances. </p>
    fn update_elasticsearch_domain_config(
        &self,
        input: UpdateElasticsearchDomainConfigRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        UpdateElasticsearchDomainConfigResponse,
                        RusotoError<UpdateElasticsearchDomainConfigError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = format!(
            "/2015-01-01/es/domain/{domain_name}/config",
            domain_name = input.domain_name
        );

        let mut request = SignedRequest::new("POST", "es", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<UpdateElasticsearchDomainConfigResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(UpdateElasticsearchDomainConfigError::from_response(
                    response,
                ))
            }
        }
        .boxed()
    }

    /// <p>Allows you to either upgrade your domain or perform an Upgrade eligibility check to a compatible Elasticsearch version.</p>
    fn upgrade_elasticsearch_domain(
        &self,
        input: UpgradeElasticsearchDomainRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        UpgradeElasticsearchDomainResponse,
                        RusotoError<UpgradeElasticsearchDomainError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = "/2015-01-01/es/upgradeDomain";

        let mut request = SignedRequest::new("POST", "es", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<UpgradeElasticsearchDomainResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(UpgradeElasticsearchDomainError::from_response(response))
            }
        }
        .boxed()
    }
}
