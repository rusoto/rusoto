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
/// <p>Container for the parameters to the <code><a>AcceptInboundCrossClusterSearchConnection</a></code> operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AcceptInboundCrossClusterSearchConnectionRequest {
    /// <p>The id of the inbound connection that you want to accept.</p>
    #[serde(rename = "CrossClusterSearchConnectionId")]
    pub cross_cluster_search_connection_id: String,
}

/// <p>The result of a <code><a>AcceptInboundCrossClusterSearchConnection</a></code> operation. Contains details of accepted inbound connection.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AcceptInboundCrossClusterSearchConnectionResponse {
    /// <p>Specifies the <code><a>InboundCrossClusterSearchConnection</a></code> of accepted inbound connection. </p>
    #[serde(rename = "CrossClusterSearchConnection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_cluster_search_connection: Option<InboundCrossClusterSearchConnection>,
}

/// <p>The configured access rules for the domain's document and search endpoints, and the current status of those rules.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AdvancedOptionsStatus {
    /// <p> Specifies the status of advanced options for the specified Elasticsearch domain.</p>
    #[serde(rename = "Options")]
    pub options: ::std::collections::HashMap<String, String>,
    /// <p> Specifies the status of <code>OptionStatus</code> for advanced options for the specified Elasticsearch domain.</p>
    #[serde(rename = "Status")]
    pub status: OptionStatus,
}

/// <p>Specifies the advanced security configuration: whether advanced security is enabled, whether the internal database option is enabled.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AdvancedSecurityOptions {
    /// <p>True if advanced security is enabled.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>True if the internal user database is enabled.</p>
    #[serde(rename = "InternalUserDatabaseEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_user_database_enabled: Option<bool>,
    /// <p>Describes the SAML application configured for a domain.</p>
    #[serde(rename = "SAMLOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saml_options: Option<SAMLOptionsOutput>,
}

/// <p>Specifies the advanced security configuration: whether advanced security is enabled, whether the internal database option is enabled, master username and password (if internal database is enabled), and master user ARN (if IAM is enabled).</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AdvancedSecurityOptionsInput {
    /// <p>True if advanced security is enabled.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>True if the internal user database is enabled.</p>
    #[serde(rename = "InternalUserDatabaseEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_user_database_enabled: Option<bool>,
    /// <p>Credentials for the master user: username and password, ARN, or both.</p>
    #[serde(rename = "MasterUserOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_options: Option<MasterUserOptions>,
    /// <p>Specifies the SAML application configuration for the domain.</p>
    #[serde(rename = "SAMLOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saml_options: Option<SAMLOptionsInput>,
}

/// <p> Specifies the status of advanced security options for the specified Elasticsearch domain.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AdvancedSecurityOptionsStatus {
    /// <p> Specifies advanced security options for the specified Elasticsearch domain.</p>
    #[serde(rename = "Options")]
    pub options: AdvancedSecurityOptions,
    /// <p> Status of the advanced security options for the specified Elasticsearch domain.</p>
    #[serde(rename = "Status")]
    pub status: OptionStatus,
}

/// <p> Container for request parameters to <code> <a>AssociatePackage</a> </code> operation. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssociatePackageRequest {
    /// <p>Name of the domain that you want to associate the package with.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// <p>Internal ID of the package that you want to associate with a domain. Use <code>DescribePackages</code> to find this value.</p>
    #[serde(rename = "PackageID")]
    pub package_id: String,
}

/// <p> Container for response returned by <code> <a>AssociatePackage</a> </code> operation. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociatePackageResponse {
    /// <p><code>DomainPackageDetails</code></p>
    #[serde(rename = "DomainPackageDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_package_details: Option<DomainPackageDetails>,
}

/// <p>Container for the parameters to the <code><a>CancelElasticsearchServiceSoftwareUpdate</a></code> operation. Specifies the name of the Elasticsearch domain that you wish to cancel a service software update on.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CancelElasticsearchServiceSoftwareUpdateRequest {
    /// <p>The name of the domain that you want to stop the latest service software update on.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
}

/// <p>The result of a <code>CancelElasticsearchServiceSoftwareUpdate</code> operation. Contains the status of the update.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CancelElasticsearchServiceSoftwareUpdateResponse {
    /// <p>The current status of the Elasticsearch service software update.</p>
    #[serde(rename = "ServiceSoftwareOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_software_options: Option<ServiceSoftwareOptions>,
}

/// <p>Options to specify the Cognito user and identity pools for Kibana authentication. For more information, see <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-cognito-auth.html" target="_blank">Amazon Cognito Authentication for Kibana</a>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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
    /// <p>Specifies advanced security options.</p>
    #[serde(rename = "AdvancedSecurityOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_security_options: Option<AdvancedSecurityOptionsInput>,
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
    pub log_publishing_options: Option<::std::collections::HashMap<LogType, LogPublishingOption>>,
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateElasticsearchDomainResponse {
    /// <p>The status of the newly created Elasticsearch domain. </p>
    #[serde(rename = "DomainStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_status: Option<ElasticsearchDomainStatus>,
}

/// <p>Container for the parameters to the <code><a>CreateOutboundCrossClusterSearchConnection</a></code> operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateOutboundCrossClusterSearchConnectionRequest {
    /// <p>Specifies the connection alias that will be used by the customer for this connection.</p>
    #[serde(rename = "ConnectionAlias")]
    pub connection_alias: String,
    /// <p>Specifies the <code><a>DomainInformation</a></code> for the destination Elasticsearch domain.</p>
    #[serde(rename = "DestinationDomainInfo")]
    pub destination_domain_info: DomainInformation,
    /// <p>Specifies the <code><a>DomainInformation</a></code> for the source Elasticsearch domain.</p>
    #[serde(rename = "SourceDomainInfo")]
    pub source_domain_info: DomainInformation,
}

/// <p>The result of a <code><a>CreateOutboundCrossClusterSearchConnection</a></code> request. Contains the details of the newly created cross-cluster search connection.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateOutboundCrossClusterSearchConnectionResponse {
    /// <p>Specifies the connection alias provided during the create connection request.</p>
    #[serde(rename = "ConnectionAlias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_alias: Option<String>,
    /// <p>Specifies the <code><a>OutboundCrossClusterSearchConnectionStatus</a></code> for the newly created connection.</p>
    #[serde(rename = "ConnectionStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_status: Option<OutboundCrossClusterSearchConnectionStatus>,
    /// <p>Unique id for the created outbound connection, which is used for subsequent operations on connection.</p>
    #[serde(rename = "CrossClusterSearchConnectionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_cluster_search_connection_id: Option<String>,
    /// <p>Specifies the <code><a>DomainInformation</a></code> for the destination Elasticsearch domain.</p>
    #[serde(rename = "DestinationDomainInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_domain_info: Option<DomainInformation>,
    /// <p>Specifies the <code><a>DomainInformation</a></code> for the source Elasticsearch domain.</p>
    #[serde(rename = "SourceDomainInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_domain_info: Option<DomainInformation>,
}

/// <p> Container for request parameters to <code> <a>CreatePackage</a> </code> operation. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreatePackageRequest {
    /// <p>Description of the package.</p>
    #[serde(rename = "PackageDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_description: Option<String>,
    /// <p>Unique identifier for the package.</p>
    #[serde(rename = "PackageName")]
    pub package_name: String,
    /// <p>The customer S3 location <code>PackageSource</code> for importing the package.</p>
    #[serde(rename = "PackageSource")]
    pub package_source: PackageSource,
    /// <p>Type of package. Currently supports only TXT-DICTIONARY.</p>
    #[serde(rename = "PackageType")]
    pub package_type: PackageType,
}

/// <p> Container for response returned by <code> <a>CreatePackage</a> </code> operation. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreatePackageResponse {
    /// <p>Information about the package <code>PackageDetails</code>.</p>
    #[serde(rename = "PackageDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_details: Option<PackageDetails>,
}

/// <p>Container for the parameters to the <code><a>DeleteElasticsearchDomain</a></code> operation. Specifies the name of the Elasticsearch domain that you want to delete.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteElasticsearchDomainRequest {
    /// <p>The name of the Elasticsearch domain that you want to permanently delete.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
}

/// <p>The result of a <code>DeleteElasticsearchDomain</code> request. Contains the status of the pending deletion, or no status if the domain and all of its resources have been deleted.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteElasticsearchDomainResponse {
    /// <p>The status of the Elasticsearch domain being deleted.</p>
    #[serde(rename = "DomainStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_status: Option<ElasticsearchDomainStatus>,
}

/// <p>Container for the parameters to the <code><a>DeleteInboundCrossClusterSearchConnection</a></code> operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteInboundCrossClusterSearchConnectionRequest {
    /// <p>The id of the inbound connection that you want to permanently delete.</p>
    #[serde(rename = "CrossClusterSearchConnectionId")]
    pub cross_cluster_search_connection_id: String,
}

/// <p>The result of a <code><a>DeleteInboundCrossClusterSearchConnection</a></code> operation. Contains details of deleted inbound connection.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteInboundCrossClusterSearchConnectionResponse {
    /// <p>Specifies the <code><a>InboundCrossClusterSearchConnection</a></code> of deleted inbound connection. </p>
    #[serde(rename = "CrossClusterSearchConnection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_cluster_search_connection: Option<InboundCrossClusterSearchConnection>,
}

/// <p>Container for the parameters to the <code><a>DeleteOutboundCrossClusterSearchConnection</a></code> operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteOutboundCrossClusterSearchConnectionRequest {
    /// <p>The id of the outbound connection that you want to permanently delete.</p>
    #[serde(rename = "CrossClusterSearchConnectionId")]
    pub cross_cluster_search_connection_id: String,
}

/// <p>The result of a <code><a>DeleteOutboundCrossClusterSearchConnection</a></code> operation. Contains details of deleted outbound connection.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteOutboundCrossClusterSearchConnectionResponse {
    /// <p>Specifies the <code><a>OutboundCrossClusterSearchConnection</a></code> of deleted outbound connection. </p>
    #[serde(rename = "CrossClusterSearchConnection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_cluster_search_connection: Option<OutboundCrossClusterSearchConnection>,
}

/// <p> Container for request parameters to <code> <a>DeletePackage</a> </code> operation. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeletePackageRequest {
    /// <p>Internal ID of the package that you want to delete. Use <code>DescribePackages</code> to find this value.</p>
    #[serde(rename = "PackageID")]
    pub package_id: String,
}

/// <p> Container for response parameters to <code> <a>DeletePackage</a> </code> operation. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeletePackageResponse {
    /// <p><code>PackageDetails</code></p>
    #[serde(rename = "PackageDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_details: Option<PackageDetails>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownDeploymentStatus {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum DeploymentStatus {
    Completed,
    Eligible,
    InProgress,
    NotEligible,
    PendingUpdate,
    #[doc(hidden)]
    UnknownVariant(UnknownDeploymentStatus),
}

impl Default for DeploymentStatus {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for DeploymentStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for DeploymentStatus {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for DeploymentStatus {
    fn into(self) -> String {
        match self {
            DeploymentStatus::Completed => "COMPLETED".to_string(),
            DeploymentStatus::Eligible => "ELIGIBLE".to_string(),
            DeploymentStatus::InProgress => "IN_PROGRESS".to_string(),
            DeploymentStatus::NotEligible => "NOT_ELIGIBLE".to_string(),
            DeploymentStatus::PendingUpdate => "PENDING_UPDATE".to_string(),
            DeploymentStatus::UnknownVariant(UnknownDeploymentStatus { name: original }) => {
                original
            }
        }
    }
}

impl<'a> Into<&'a str> for &'a DeploymentStatus {
    fn into(self) -> &'a str {
        match self {
            DeploymentStatus::Completed => &"COMPLETED",
            DeploymentStatus::Eligible => &"ELIGIBLE",
            DeploymentStatus::InProgress => &"IN_PROGRESS",
            DeploymentStatus::NotEligible => &"NOT_ELIGIBLE",
            DeploymentStatus::PendingUpdate => &"PENDING_UPDATE",
            DeploymentStatus::UnknownVariant(UnknownDeploymentStatus { name: original }) => {
                original
            }
        }
    }
}

impl From<&str> for DeploymentStatus {
    fn from(name: &str) -> Self {
        match name {
            "COMPLETED" => DeploymentStatus::Completed,
            "ELIGIBLE" => DeploymentStatus::Eligible,
            "IN_PROGRESS" => DeploymentStatus::InProgress,
            "NOT_ELIGIBLE" => DeploymentStatus::NotEligible,
            "PENDING_UPDATE" => DeploymentStatus::PendingUpdate,
            _ => DeploymentStatus::UnknownVariant(UnknownDeploymentStatus {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for DeploymentStatus {
    fn from(name: String) -> Self {
        match &*name {
            "COMPLETED" => DeploymentStatus::Completed,
            "ELIGIBLE" => DeploymentStatus::Eligible,
            "IN_PROGRESS" => DeploymentStatus::InProgress,
            "NOT_ELIGIBLE" => DeploymentStatus::NotEligible,
            "PENDING_UPDATE" => DeploymentStatus::PendingUpdate,
            _ => DeploymentStatus::UnknownVariant(UnknownDeploymentStatus { name }),
        }
    }
}

impl ::std::str::FromStr for DeploymentStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for DeploymentStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for DeploymentStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p> Container for the parameters to the <code>DescribeElasticsearchDomainConfig</code> operation. Specifies the domain name for which you want configuration information.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeElasticsearchDomainConfigRequest {
    /// <p>The Elasticsearch domain that you want to get information about.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
}

/// <p>The result of a <code>DescribeElasticsearchDomainConfig</code> request. Contains the configuration information of the requested domain.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeElasticsearchDomainConfigResponse {
    /// <p>The configuration information of the domain requested in the <code>DescribeElasticsearchDomainConfig</code> request.</p>
    #[serde(rename = "DomainConfig")]
    pub domain_config: ElasticsearchDomainConfig,
}

/// <p>Container for the parameters to the <code><a>DescribeElasticsearchDomain</a></code> operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeElasticsearchDomainRequest {
    /// <p>The name of the Elasticsearch domain for which you want information.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
}

/// <p>The result of a <code>DescribeElasticsearchDomain</code> request. Contains the status of the domain specified in the request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeElasticsearchDomainResponse {
    /// <p>The current status of the Elasticsearch domain.</p>
    #[serde(rename = "DomainStatus")]
    pub domain_status: ElasticsearchDomainStatus,
}

/// <p>Container for the parameters to the <code><a>DescribeElasticsearchDomains</a></code> operation. By default, the API returns the status of all Elasticsearch domains.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeElasticsearchDomainsRequest {
    /// <p>The Elasticsearch domains for which you want information.</p>
    #[serde(rename = "DomainNames")]
    pub domain_names: Vec<String>,
}

/// <p>The result of a <code>DescribeElasticsearchDomains</code> request. Contains the status of the specified domains or all domains owned by the account.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeElasticsearchDomainsResponse {
    /// <p>The status of the domains requested in the <code>DescribeElasticsearchDomains</code> request.</p>
    #[serde(rename = "DomainStatusList")]
    pub domain_status_list: Vec<ElasticsearchDomainStatus>,
}

/// <p> Container for the parameters to <code> <a>DescribeElasticsearchInstanceTypeLimits</a> </code> operation. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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
    pub instance_type: ESPartitionInstanceType,
}

/// <p> Container for the parameters received from <code> <a>DescribeElasticsearchInstanceTypeLimits</a> </code> operation. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeElasticsearchInstanceTypeLimitsResponse {
    #[serde(rename = "LimitsByRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits_by_role: Option<::std::collections::HashMap<String, Limits>>,
}

/// <p>Container for the parameters to the <code><a>DescribeInboundCrossClusterSearchConnections</a></code> operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeInboundCrossClusterSearchConnectionsRequest {
    /// <p> A list of filters used to match properties for inbound cross-cluster search connection. Available <code><a>Filter</a></code> names for this operation are: <ul> <li>cross-cluster-search-connection-id</li> <li>source-domain-info.domain-name</li> <li>source-domain-info.owner-id</li> <li>source-domain-info.region</li> <li>destination-domain-info.domain-name</li> </ul> </p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>Set this value to limit the number of results returned. If not specified, defaults to 100.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p> NextToken is sent in case the earlier API call results contain the NextToken. It is used for pagination.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>The result of a <code><a>DescribeInboundCrossClusterSearchConnections</a></code> request. Contains the list of connections matching the filter criteria.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeInboundCrossClusterSearchConnectionsResponse {
    /// <p>Consists of list of <code><a>InboundCrossClusterSearchConnection</a></code> matching the specified filter criteria.</p>
    #[serde(rename = "CrossClusterSearchConnections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_cluster_search_connections: Option<Vec<InboundCrossClusterSearchConnection>>,
    /// <p>If more results are available and NextToken is present, make the next request to the same API with the received NextToken to paginate the remaining results. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Container for the parameters to the <code><a>DescribeOutboundCrossClusterSearchConnections</a></code> operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeOutboundCrossClusterSearchConnectionsRequest {
    /// <p> A list of filters used to match properties for outbound cross-cluster search connection. Available <code><a>Filter</a></code> names for this operation are: <ul> <li>cross-cluster-search-connection-id</li> <li>destination-domain-info.domain-name</li> <li>destination-domain-info.owner-id</li> <li>destination-domain-info.region</li> <li>source-domain-info.domain-name</li> </ul> </p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>Set this value to limit the number of results returned. If not specified, defaults to 100.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p> NextToken is sent in case the earlier API call results contain the NextToken. It is used for pagination.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>The result of a <code><a>DescribeOutboundCrossClusterSearchConnections</a></code> request. Contains the list of connections matching the filter criteria.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeOutboundCrossClusterSearchConnectionsResponse {
    /// <p>Consists of list of <code><a>OutboundCrossClusterSearchConnection</a></code> matching the specified filter criteria.</p>
    #[serde(rename = "CrossClusterSearchConnections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_cluster_search_connections: Option<Vec<OutboundCrossClusterSearchConnection>>,
    /// <p>If more results are available and NextToken is present, make the next request to the same API with the received NextToken to paginate the remaining results. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Filter to apply in <code>DescribePackage</code> response.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribePackagesFilter {
    /// <p>Any field from <code>PackageDetails</code>.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<DescribePackagesFilterName>,
    /// <p>A list of values for the specified field.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Vec<String>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownDescribePackagesFilterName {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum DescribePackagesFilterName {
    PackageID,
    PackageName,
    PackageStatus,
    #[doc(hidden)]
    UnknownVariant(UnknownDescribePackagesFilterName),
}

impl Default for DescribePackagesFilterName {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for DescribePackagesFilterName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for DescribePackagesFilterName {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for DescribePackagesFilterName {
    fn into(self) -> String {
        match self {
            DescribePackagesFilterName::PackageID => "PackageID".to_string(),
            DescribePackagesFilterName::PackageName => "PackageName".to_string(),
            DescribePackagesFilterName::PackageStatus => "PackageStatus".to_string(),
            DescribePackagesFilterName::UnknownVariant(UnknownDescribePackagesFilterName {
                name: original,
            }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a DescribePackagesFilterName {
    fn into(self) -> &'a str {
        match self {
            DescribePackagesFilterName::PackageID => &"PackageID",
            DescribePackagesFilterName::PackageName => &"PackageName",
            DescribePackagesFilterName::PackageStatus => &"PackageStatus",
            DescribePackagesFilterName::UnknownVariant(UnknownDescribePackagesFilterName {
                name: original,
            }) => original,
        }
    }
}

impl From<&str> for DescribePackagesFilterName {
    fn from(name: &str) -> Self {
        match name {
            "PackageID" => DescribePackagesFilterName::PackageID,
            "PackageName" => DescribePackagesFilterName::PackageName,
            "PackageStatus" => DescribePackagesFilterName::PackageStatus,
            _ => DescribePackagesFilterName::UnknownVariant(UnknownDescribePackagesFilterName {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for DescribePackagesFilterName {
    fn from(name: String) -> Self {
        match &*name {
            "PackageID" => DescribePackagesFilterName::PackageID,
            "PackageName" => DescribePackagesFilterName::PackageName,
            "PackageStatus" => DescribePackagesFilterName::PackageStatus,
            _ => DescribePackagesFilterName::UnknownVariant(UnknownDescribePackagesFilterName {
                name,
            }),
        }
    }
}

impl ::std::str::FromStr for DescribePackagesFilterName {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for DescribePackagesFilterName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

#[cfg(feature = "deserialize_structs")]
impl<'de> Deserialize<'de> for DescribePackagesFilterName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p> Container for request parameters to <code> <a>DescribePackage</a> </code> operation. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribePackagesRequest {
    /// <p>Only returns packages that match the <code>DescribePackagesFilterList</code> values.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<DescribePackagesFilter>>,
    /// <p>Limits results to a maximum number of packages.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Used for pagination. Only necessary if a previous API call includes a non-null NextToken value. If provided, returns results for the next page.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p> Container for response returned by <code> <a>DescribePackages</a> </code> operation. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribePackagesResponse {
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>List of <code>PackageDetails</code> objects.</p>
    #[serde(rename = "PackageDetailsList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_details_list: Option<Vec<PackageDetails>>,
}

/// <p>Container for parameters to <code>DescribeReservedElasticsearchInstanceOfferings</code></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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

/// <p> Container for request parameters to <code> <a>DissociatePackage</a> </code> operation. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DissociatePackageRequest {
    /// <p>Name of the domain that you want to associate the package with.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// <p>Internal ID of the package that you want to associate with a domain. Use <code>DescribePackages</code> to find this value.</p>
    #[serde(rename = "PackageID")]
    pub package_id: String,
}

/// <p> Container for response returned by <code> <a>DissociatePackage</a> </code> operation. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DissociatePackageResponse {
    /// <p><code>DomainPackageDetails</code></p>
    #[serde(rename = "DomainPackageDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_package_details: Option<DomainPackageDetails>,
}

/// <p>Options to configure endpoint for the Elasticsearch domain.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DomainEndpointOptions {
    /// <p>Specify the fully qualified domain for your custom endpoint.</p>
    #[serde(rename = "CustomEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_endpoint: Option<String>,
    /// <p>Specify ACM certificate ARN for your custom endpoint.</p>
    #[serde(rename = "CustomEndpointCertificateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_endpoint_certificate_arn: Option<String>,
    /// <p>Specify if custom endpoint should be enabled for the Elasticsearch domain.</p>
    #[serde(rename = "CustomEndpointEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_endpoint_enabled: Option<bool>,
    /// <p>Specify if only HTTPS endpoint should be enabled for the Elasticsearch domain.</p>
    #[serde(rename = "EnforceHTTPS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforce_https: Option<bool>,
    /// <p>Specify the TLS security policy that needs to be applied to the HTTPS endpoint of Elasticsearch domain. <br/> It can be one of the following values: <ul> <li><b>Policy-Min-TLS-1-0-2019-07: </b> TLS security policy which supports TLSv1.0 and higher.</li> <li><b>Policy-Min-TLS-1-2-2019-07: </b> TLS security policy which supports only TLSv1.2</li> </ul> </p>
    #[serde(rename = "TLSSecurityPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_security_policy: Option<TLSSecurityPolicy>,
}

/// <p>The configured endpoint options for the domain and their current status.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DomainEndpointOptionsStatus {
    /// <p>Options to configure endpoint for the Elasticsearch domain.</p>
    #[serde(rename = "Options")]
    pub options: DomainEndpointOptions,
    /// <p>The status of the endpoint options for the Elasticsearch domain. See <code>OptionStatus</code> for the status information that's included. </p>
    #[serde(rename = "Status")]
    pub status: OptionStatus,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DomainInfo {
    /// <p> Specifies the <code>DomainName</code>.</p>
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DomainInformation {
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    #[serde(rename = "OwnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(rename = "Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

/// <p>Information on a package that is associated with a domain.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DomainPackageDetails {
    /// <p>Name of the domain you've associated a package with.</p>
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// <p>State of the association. Values are ASSOCIATING/ASSOCIATION_FAILED/ACTIVE/DISSOCIATING/DISSOCIATION_FAILED.</p>
    #[serde(rename = "DomainPackageStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_package_status: Option<DomainPackageStatus>,
    /// <p>Additional information if the package is in an error state. Null otherwise.</p>
    #[serde(rename = "ErrorDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_details: Option<ErrorDetails>,
    /// <p>Timestamp of the most-recent update to the association status.</p>
    #[serde(rename = "LastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<f64>,
    /// <p>Internal ID of the package.</p>
    #[serde(rename = "PackageID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_id: Option<String>,
    /// <p>User specified name of the package.</p>
    #[serde(rename = "PackageName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_name: Option<String>,
    /// <p>Currently supports only TXT-DICTIONARY.</p>
    #[serde(rename = "PackageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_type: Option<PackageType>,
    #[serde(rename = "PackageVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_version: Option<String>,
    /// <p>The relative path on Amazon ES nodes, which can be used as synonym_path when the package is synonym file.</p>
    #[serde(rename = "ReferencePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_path: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownDomainPackageStatus {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum DomainPackageStatus {
    Active,
    Associating,
    AssociationFailed,
    Dissociating,
    DissociationFailed,
    #[doc(hidden)]
    UnknownVariant(UnknownDomainPackageStatus),
}

impl Default for DomainPackageStatus {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for DomainPackageStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for DomainPackageStatus {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for DomainPackageStatus {
    fn into(self) -> String {
        match self {
            DomainPackageStatus::Active => "ACTIVE".to_string(),
            DomainPackageStatus::Associating => "ASSOCIATING".to_string(),
            DomainPackageStatus::AssociationFailed => "ASSOCIATION_FAILED".to_string(),
            DomainPackageStatus::Dissociating => "DISSOCIATING".to_string(),
            DomainPackageStatus::DissociationFailed => "DISSOCIATION_FAILED".to_string(),
            DomainPackageStatus::UnknownVariant(UnknownDomainPackageStatus { name: original }) => {
                original
            }
        }
    }
}

impl<'a> Into<&'a str> for &'a DomainPackageStatus {
    fn into(self) -> &'a str {
        match self {
            DomainPackageStatus::Active => &"ACTIVE",
            DomainPackageStatus::Associating => &"ASSOCIATING",
            DomainPackageStatus::AssociationFailed => &"ASSOCIATION_FAILED",
            DomainPackageStatus::Dissociating => &"DISSOCIATING",
            DomainPackageStatus::DissociationFailed => &"DISSOCIATION_FAILED",
            DomainPackageStatus::UnknownVariant(UnknownDomainPackageStatus { name: original }) => {
                original
            }
        }
    }
}

impl From<&str> for DomainPackageStatus {
    fn from(name: &str) -> Self {
        match name {
            "ACTIVE" => DomainPackageStatus::Active,
            "ASSOCIATING" => DomainPackageStatus::Associating,
            "ASSOCIATION_FAILED" => DomainPackageStatus::AssociationFailed,
            "DISSOCIATING" => DomainPackageStatus::Dissociating,
            "DISSOCIATION_FAILED" => DomainPackageStatus::DissociationFailed,
            _ => DomainPackageStatus::UnknownVariant(UnknownDomainPackageStatus {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for DomainPackageStatus {
    fn from(name: String) -> Self {
        match &*name {
            "ACTIVE" => DomainPackageStatus::Active,
            "ASSOCIATING" => DomainPackageStatus::Associating,
            "ASSOCIATION_FAILED" => DomainPackageStatus::AssociationFailed,
            "DISSOCIATING" => DomainPackageStatus::Dissociating,
            "DISSOCIATION_FAILED" => DomainPackageStatus::DissociationFailed,
            _ => DomainPackageStatus::UnknownVariant(UnknownDomainPackageStatus { name }),
        }
    }
}

impl ::std::str::FromStr for DomainPackageStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for DomainPackageStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for DomainPackageStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Options to enable, disable, and specify the properties of EBS storage volumes. For more information, see <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-createupdatedomains.html#es-createdomain-configure-ebs" target="_blank"> Configuring EBS-based Storage</a>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
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
    pub volume_type: Option<VolumeType>,
}

/// <p> Status of the EBS options for the specified Elasticsearch domain.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EBSOptionsStatus {
    /// <p> Specifies the EBS options for the specified Elasticsearch domain.</p>
    #[serde(rename = "Options")]
    pub options: EBSOptions,
    /// <p> Specifies the status of the EBS options for the specified Elasticsearch domain.</p>
    #[serde(rename = "Status")]
    pub status: OptionStatus,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownESPartitionInstanceType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ESPartitionInstanceType {
    C42XlargeElasticsearch,
    C44XlargeElasticsearch,
    C48XlargeElasticsearch,
    C4LargeElasticsearch,
    C4XlargeElasticsearch,
    C518XlargeElasticsearch,
    C52XlargeElasticsearch,
    C54XlargeElasticsearch,
    C59XlargeElasticsearch,
    C5LargeElasticsearch,
    C5XlargeElasticsearch,
    D22XlargeElasticsearch,
    D24XlargeElasticsearch,
    D28XlargeElasticsearch,
    D2XlargeElasticsearch,
    I22XlargeElasticsearch,
    I2XlargeElasticsearch,
    I316XlargeElasticsearch,
    I32XlargeElasticsearch,
    I34XlargeElasticsearch,
    I38XlargeElasticsearch,
    I3LargeElasticsearch,
    I3XlargeElasticsearch,
    M32XlargeElasticsearch,
    M3LargeElasticsearch,
    M3MediumElasticsearch,
    M3XlargeElasticsearch,
    M410XlargeElasticsearch,
    M42XlargeElasticsearch,
    M44XlargeElasticsearch,
    M4LargeElasticsearch,
    M4XlargeElasticsearch,
    M512XlargeElasticsearch,
    M52XlargeElasticsearch,
    M54XlargeElasticsearch,
    M5LargeElasticsearch,
    M5XlargeElasticsearch,
    R32XlargeElasticsearch,
    R34XlargeElasticsearch,
    R38XlargeElasticsearch,
    R3LargeElasticsearch,
    R3XlargeElasticsearch,
    R416XlargeElasticsearch,
    R42XlargeElasticsearch,
    R44XlargeElasticsearch,
    R48XlargeElasticsearch,
    R4LargeElasticsearch,
    R4XlargeElasticsearch,
    R512XlargeElasticsearch,
    R52XlargeElasticsearch,
    R54XlargeElasticsearch,
    R5LargeElasticsearch,
    R5XlargeElasticsearch,
    T2MediumElasticsearch,
    T2MicroElasticsearch,
    T2SmallElasticsearch,
    Ultrawarm1LargeElasticsearch,
    Ultrawarm1MediumElasticsearch,
    #[doc(hidden)]
    UnknownVariant(UnknownESPartitionInstanceType),
}

impl Default for ESPartitionInstanceType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for ESPartitionInstanceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for ESPartitionInstanceType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for ESPartitionInstanceType {
    fn into(self) -> String {
        match self {
            ESPartitionInstanceType::C42XlargeElasticsearch => {
                "c4.2xlarge.elasticsearch".to_string()
            }
            ESPartitionInstanceType::C44XlargeElasticsearch => {
                "c4.4xlarge.elasticsearch".to_string()
            }
            ESPartitionInstanceType::C48XlargeElasticsearch => {
                "c4.8xlarge.elasticsearch".to_string()
            }
            ESPartitionInstanceType::C4LargeElasticsearch => "c4.large.elasticsearch".to_string(),
            ESPartitionInstanceType::C4XlargeElasticsearch => "c4.xlarge.elasticsearch".to_string(),
            ESPartitionInstanceType::C518XlargeElasticsearch => {
                "c5.18xlarge.elasticsearch".to_string()
            }
            ESPartitionInstanceType::C52XlargeElasticsearch => {
                "c5.2xlarge.elasticsearch".to_string()
            }
            ESPartitionInstanceType::C54XlargeElasticsearch => {
                "c5.4xlarge.elasticsearch".to_string()
            }
            ESPartitionInstanceType::C59XlargeElasticsearch => {
                "c5.9xlarge.elasticsearch".to_string()
            }
            ESPartitionInstanceType::C5LargeElasticsearch => "c5.large.elasticsearch".to_string(),
            ESPartitionInstanceType::C5XlargeElasticsearch => "c5.xlarge.elasticsearch".to_string(),
            ESPartitionInstanceType::D22XlargeElasticsearch => {
                "d2.2xlarge.elasticsearch".to_string()
            }
            ESPartitionInstanceType::D24XlargeElasticsearch => {
                "d2.4xlarge.elasticsearch".to_string()
            }
            ESPartitionInstanceType::D28XlargeElasticsearch => {
                "d2.8xlarge.elasticsearch".to_string()
            }
            ESPartitionInstanceType::D2XlargeElasticsearch => "d2.xlarge.elasticsearch".to_string(),
            ESPartitionInstanceType::I22XlargeElasticsearch => {
                "i2.2xlarge.elasticsearch".to_string()
            }
            ESPartitionInstanceType::I2XlargeElasticsearch => "i2.xlarge.elasticsearch".to_string(),
            ESPartitionInstanceType::I316XlargeElasticsearch => {
                "i3.16xlarge.elasticsearch".to_string()
            }
            ESPartitionInstanceType::I32XlargeElasticsearch => {
                "i3.2xlarge.elasticsearch".to_string()
            }
            ESPartitionInstanceType::I34XlargeElasticsearch => {
                "i3.4xlarge.elasticsearch".to_string()
            }
            ESPartitionInstanceType::I38XlargeElasticsearch => {
                "i3.8xlarge.elasticsearch".to_string()
            }
            ESPartitionInstanceType::I3LargeElasticsearch => "i3.large.elasticsearch".to_string(),
            ESPartitionInstanceType::I3XlargeElasticsearch => "i3.xlarge.elasticsearch".to_string(),
            ESPartitionInstanceType::M32XlargeElasticsearch => {
                "m3.2xlarge.elasticsearch".to_string()
            }
            ESPartitionInstanceType::M3LargeElasticsearch => "m3.large.elasticsearch".to_string(),
            ESPartitionInstanceType::M3MediumElasticsearch => "m3.medium.elasticsearch".to_string(),
            ESPartitionInstanceType::M3XlargeElasticsearch => "m3.xlarge.elasticsearch".to_string(),
            ESPartitionInstanceType::M410XlargeElasticsearch => {
                "m4.10xlarge.elasticsearch".to_string()
            }
            ESPartitionInstanceType::M42XlargeElasticsearch => {
                "m4.2xlarge.elasticsearch".to_string()
            }
            ESPartitionInstanceType::M44XlargeElasticsearch => {
                "m4.4xlarge.elasticsearch".to_string()
            }
            ESPartitionInstanceType::M4LargeElasticsearch => "m4.large.elasticsearch".to_string(),
            ESPartitionInstanceType::M4XlargeElasticsearch => "m4.xlarge.elasticsearch".to_string(),
            ESPartitionInstanceType::M512XlargeElasticsearch => {
                "m5.12xlarge.elasticsearch".to_string()
            }
            ESPartitionInstanceType::M52XlargeElasticsearch => {
                "m5.2xlarge.elasticsearch".to_string()
            }
            ESPartitionInstanceType::M54XlargeElasticsearch => {
                "m5.4xlarge.elasticsearch".to_string()
            }
            ESPartitionInstanceType::M5LargeElasticsearch => "m5.large.elasticsearch".to_string(),
            ESPartitionInstanceType::M5XlargeElasticsearch => "m5.xlarge.elasticsearch".to_string(),
            ESPartitionInstanceType::R32XlargeElasticsearch => {
                "r3.2xlarge.elasticsearch".to_string()
            }
            ESPartitionInstanceType::R34XlargeElasticsearch => {
                "r3.4xlarge.elasticsearch".to_string()
            }
            ESPartitionInstanceType::R38XlargeElasticsearch => {
                "r3.8xlarge.elasticsearch".to_string()
            }
            ESPartitionInstanceType::R3LargeElasticsearch => "r3.large.elasticsearch".to_string(),
            ESPartitionInstanceType::R3XlargeElasticsearch => "r3.xlarge.elasticsearch".to_string(),
            ESPartitionInstanceType::R416XlargeElasticsearch => {
                "r4.16xlarge.elasticsearch".to_string()
            }
            ESPartitionInstanceType::R42XlargeElasticsearch => {
                "r4.2xlarge.elasticsearch".to_string()
            }
            ESPartitionInstanceType::R44XlargeElasticsearch => {
                "r4.4xlarge.elasticsearch".to_string()
            }
            ESPartitionInstanceType::R48XlargeElasticsearch => {
                "r4.8xlarge.elasticsearch".to_string()
            }
            ESPartitionInstanceType::R4LargeElasticsearch => "r4.large.elasticsearch".to_string(),
            ESPartitionInstanceType::R4XlargeElasticsearch => "r4.xlarge.elasticsearch".to_string(),
            ESPartitionInstanceType::R512XlargeElasticsearch => {
                "r5.12xlarge.elasticsearch".to_string()
            }
            ESPartitionInstanceType::R52XlargeElasticsearch => {
                "r5.2xlarge.elasticsearch".to_string()
            }
            ESPartitionInstanceType::R54XlargeElasticsearch => {
                "r5.4xlarge.elasticsearch".to_string()
            }
            ESPartitionInstanceType::R5LargeElasticsearch => "r5.large.elasticsearch".to_string(),
            ESPartitionInstanceType::R5XlargeElasticsearch => "r5.xlarge.elasticsearch".to_string(),
            ESPartitionInstanceType::T2MediumElasticsearch => "t2.medium.elasticsearch".to_string(),
            ESPartitionInstanceType::T2MicroElasticsearch => "t2.micro.elasticsearch".to_string(),
            ESPartitionInstanceType::T2SmallElasticsearch => "t2.small.elasticsearch".to_string(),
            ESPartitionInstanceType::Ultrawarm1LargeElasticsearch => {
                "ultrawarm1.large.elasticsearch".to_string()
            }
            ESPartitionInstanceType::Ultrawarm1MediumElasticsearch => {
                "ultrawarm1.medium.elasticsearch".to_string()
            }
            ESPartitionInstanceType::UnknownVariant(UnknownESPartitionInstanceType {
                name: original,
            }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a ESPartitionInstanceType {
    fn into(self) -> &'a str {
        match self {
            ESPartitionInstanceType::C42XlargeElasticsearch => &"c4.2xlarge.elasticsearch",
            ESPartitionInstanceType::C44XlargeElasticsearch => &"c4.4xlarge.elasticsearch",
            ESPartitionInstanceType::C48XlargeElasticsearch => &"c4.8xlarge.elasticsearch",
            ESPartitionInstanceType::C4LargeElasticsearch => &"c4.large.elasticsearch",
            ESPartitionInstanceType::C4XlargeElasticsearch => &"c4.xlarge.elasticsearch",
            ESPartitionInstanceType::C518XlargeElasticsearch => &"c5.18xlarge.elasticsearch",
            ESPartitionInstanceType::C52XlargeElasticsearch => &"c5.2xlarge.elasticsearch",
            ESPartitionInstanceType::C54XlargeElasticsearch => &"c5.4xlarge.elasticsearch",
            ESPartitionInstanceType::C59XlargeElasticsearch => &"c5.9xlarge.elasticsearch",
            ESPartitionInstanceType::C5LargeElasticsearch => &"c5.large.elasticsearch",
            ESPartitionInstanceType::C5XlargeElasticsearch => &"c5.xlarge.elasticsearch",
            ESPartitionInstanceType::D22XlargeElasticsearch => &"d2.2xlarge.elasticsearch",
            ESPartitionInstanceType::D24XlargeElasticsearch => &"d2.4xlarge.elasticsearch",
            ESPartitionInstanceType::D28XlargeElasticsearch => &"d2.8xlarge.elasticsearch",
            ESPartitionInstanceType::D2XlargeElasticsearch => &"d2.xlarge.elasticsearch",
            ESPartitionInstanceType::I22XlargeElasticsearch => &"i2.2xlarge.elasticsearch",
            ESPartitionInstanceType::I2XlargeElasticsearch => &"i2.xlarge.elasticsearch",
            ESPartitionInstanceType::I316XlargeElasticsearch => &"i3.16xlarge.elasticsearch",
            ESPartitionInstanceType::I32XlargeElasticsearch => &"i3.2xlarge.elasticsearch",
            ESPartitionInstanceType::I34XlargeElasticsearch => &"i3.4xlarge.elasticsearch",
            ESPartitionInstanceType::I38XlargeElasticsearch => &"i3.8xlarge.elasticsearch",
            ESPartitionInstanceType::I3LargeElasticsearch => &"i3.large.elasticsearch",
            ESPartitionInstanceType::I3XlargeElasticsearch => &"i3.xlarge.elasticsearch",
            ESPartitionInstanceType::M32XlargeElasticsearch => &"m3.2xlarge.elasticsearch",
            ESPartitionInstanceType::M3LargeElasticsearch => &"m3.large.elasticsearch",
            ESPartitionInstanceType::M3MediumElasticsearch => &"m3.medium.elasticsearch",
            ESPartitionInstanceType::M3XlargeElasticsearch => &"m3.xlarge.elasticsearch",
            ESPartitionInstanceType::M410XlargeElasticsearch => &"m4.10xlarge.elasticsearch",
            ESPartitionInstanceType::M42XlargeElasticsearch => &"m4.2xlarge.elasticsearch",
            ESPartitionInstanceType::M44XlargeElasticsearch => &"m4.4xlarge.elasticsearch",
            ESPartitionInstanceType::M4LargeElasticsearch => &"m4.large.elasticsearch",
            ESPartitionInstanceType::M4XlargeElasticsearch => &"m4.xlarge.elasticsearch",
            ESPartitionInstanceType::M512XlargeElasticsearch => &"m5.12xlarge.elasticsearch",
            ESPartitionInstanceType::M52XlargeElasticsearch => &"m5.2xlarge.elasticsearch",
            ESPartitionInstanceType::M54XlargeElasticsearch => &"m5.4xlarge.elasticsearch",
            ESPartitionInstanceType::M5LargeElasticsearch => &"m5.large.elasticsearch",
            ESPartitionInstanceType::M5XlargeElasticsearch => &"m5.xlarge.elasticsearch",
            ESPartitionInstanceType::R32XlargeElasticsearch => &"r3.2xlarge.elasticsearch",
            ESPartitionInstanceType::R34XlargeElasticsearch => &"r3.4xlarge.elasticsearch",
            ESPartitionInstanceType::R38XlargeElasticsearch => &"r3.8xlarge.elasticsearch",
            ESPartitionInstanceType::R3LargeElasticsearch => &"r3.large.elasticsearch",
            ESPartitionInstanceType::R3XlargeElasticsearch => &"r3.xlarge.elasticsearch",
            ESPartitionInstanceType::R416XlargeElasticsearch => &"r4.16xlarge.elasticsearch",
            ESPartitionInstanceType::R42XlargeElasticsearch => &"r4.2xlarge.elasticsearch",
            ESPartitionInstanceType::R44XlargeElasticsearch => &"r4.4xlarge.elasticsearch",
            ESPartitionInstanceType::R48XlargeElasticsearch => &"r4.8xlarge.elasticsearch",
            ESPartitionInstanceType::R4LargeElasticsearch => &"r4.large.elasticsearch",
            ESPartitionInstanceType::R4XlargeElasticsearch => &"r4.xlarge.elasticsearch",
            ESPartitionInstanceType::R512XlargeElasticsearch => &"r5.12xlarge.elasticsearch",
            ESPartitionInstanceType::R52XlargeElasticsearch => &"r5.2xlarge.elasticsearch",
            ESPartitionInstanceType::R54XlargeElasticsearch => &"r5.4xlarge.elasticsearch",
            ESPartitionInstanceType::R5LargeElasticsearch => &"r5.large.elasticsearch",
            ESPartitionInstanceType::R5XlargeElasticsearch => &"r5.xlarge.elasticsearch",
            ESPartitionInstanceType::T2MediumElasticsearch => &"t2.medium.elasticsearch",
            ESPartitionInstanceType::T2MicroElasticsearch => &"t2.micro.elasticsearch",
            ESPartitionInstanceType::T2SmallElasticsearch => &"t2.small.elasticsearch",
            ESPartitionInstanceType::Ultrawarm1LargeElasticsearch => {
                &"ultrawarm1.large.elasticsearch"
            }
            ESPartitionInstanceType::Ultrawarm1MediumElasticsearch => {
                &"ultrawarm1.medium.elasticsearch"
            }
            ESPartitionInstanceType::UnknownVariant(UnknownESPartitionInstanceType {
                name: original,
            }) => original,
        }
    }
}

impl From<&str> for ESPartitionInstanceType {
    fn from(name: &str) -> Self {
        match name {
            "c4.2xlarge.elasticsearch" => ESPartitionInstanceType::C42XlargeElasticsearch,
            "c4.4xlarge.elasticsearch" => ESPartitionInstanceType::C44XlargeElasticsearch,
            "c4.8xlarge.elasticsearch" => ESPartitionInstanceType::C48XlargeElasticsearch,
            "c4.large.elasticsearch" => ESPartitionInstanceType::C4LargeElasticsearch,
            "c4.xlarge.elasticsearch" => ESPartitionInstanceType::C4XlargeElasticsearch,
            "c5.18xlarge.elasticsearch" => ESPartitionInstanceType::C518XlargeElasticsearch,
            "c5.2xlarge.elasticsearch" => ESPartitionInstanceType::C52XlargeElasticsearch,
            "c5.4xlarge.elasticsearch" => ESPartitionInstanceType::C54XlargeElasticsearch,
            "c5.9xlarge.elasticsearch" => ESPartitionInstanceType::C59XlargeElasticsearch,
            "c5.large.elasticsearch" => ESPartitionInstanceType::C5LargeElasticsearch,
            "c5.xlarge.elasticsearch" => ESPartitionInstanceType::C5XlargeElasticsearch,
            "d2.2xlarge.elasticsearch" => ESPartitionInstanceType::D22XlargeElasticsearch,
            "d2.4xlarge.elasticsearch" => ESPartitionInstanceType::D24XlargeElasticsearch,
            "d2.8xlarge.elasticsearch" => ESPartitionInstanceType::D28XlargeElasticsearch,
            "d2.xlarge.elasticsearch" => ESPartitionInstanceType::D2XlargeElasticsearch,
            "i2.2xlarge.elasticsearch" => ESPartitionInstanceType::I22XlargeElasticsearch,
            "i2.xlarge.elasticsearch" => ESPartitionInstanceType::I2XlargeElasticsearch,
            "i3.16xlarge.elasticsearch" => ESPartitionInstanceType::I316XlargeElasticsearch,
            "i3.2xlarge.elasticsearch" => ESPartitionInstanceType::I32XlargeElasticsearch,
            "i3.4xlarge.elasticsearch" => ESPartitionInstanceType::I34XlargeElasticsearch,
            "i3.8xlarge.elasticsearch" => ESPartitionInstanceType::I38XlargeElasticsearch,
            "i3.large.elasticsearch" => ESPartitionInstanceType::I3LargeElasticsearch,
            "i3.xlarge.elasticsearch" => ESPartitionInstanceType::I3XlargeElasticsearch,
            "m3.2xlarge.elasticsearch" => ESPartitionInstanceType::M32XlargeElasticsearch,
            "m3.large.elasticsearch" => ESPartitionInstanceType::M3LargeElasticsearch,
            "m3.medium.elasticsearch" => ESPartitionInstanceType::M3MediumElasticsearch,
            "m3.xlarge.elasticsearch" => ESPartitionInstanceType::M3XlargeElasticsearch,
            "m4.10xlarge.elasticsearch" => ESPartitionInstanceType::M410XlargeElasticsearch,
            "m4.2xlarge.elasticsearch" => ESPartitionInstanceType::M42XlargeElasticsearch,
            "m4.4xlarge.elasticsearch" => ESPartitionInstanceType::M44XlargeElasticsearch,
            "m4.large.elasticsearch" => ESPartitionInstanceType::M4LargeElasticsearch,
            "m4.xlarge.elasticsearch" => ESPartitionInstanceType::M4XlargeElasticsearch,
            "m5.12xlarge.elasticsearch" => ESPartitionInstanceType::M512XlargeElasticsearch,
            "m5.2xlarge.elasticsearch" => ESPartitionInstanceType::M52XlargeElasticsearch,
            "m5.4xlarge.elasticsearch" => ESPartitionInstanceType::M54XlargeElasticsearch,
            "m5.large.elasticsearch" => ESPartitionInstanceType::M5LargeElasticsearch,
            "m5.xlarge.elasticsearch" => ESPartitionInstanceType::M5XlargeElasticsearch,
            "r3.2xlarge.elasticsearch" => ESPartitionInstanceType::R32XlargeElasticsearch,
            "r3.4xlarge.elasticsearch" => ESPartitionInstanceType::R34XlargeElasticsearch,
            "r3.8xlarge.elasticsearch" => ESPartitionInstanceType::R38XlargeElasticsearch,
            "r3.large.elasticsearch" => ESPartitionInstanceType::R3LargeElasticsearch,
            "r3.xlarge.elasticsearch" => ESPartitionInstanceType::R3XlargeElasticsearch,
            "r4.16xlarge.elasticsearch" => ESPartitionInstanceType::R416XlargeElasticsearch,
            "r4.2xlarge.elasticsearch" => ESPartitionInstanceType::R42XlargeElasticsearch,
            "r4.4xlarge.elasticsearch" => ESPartitionInstanceType::R44XlargeElasticsearch,
            "r4.8xlarge.elasticsearch" => ESPartitionInstanceType::R48XlargeElasticsearch,
            "r4.large.elasticsearch" => ESPartitionInstanceType::R4LargeElasticsearch,
            "r4.xlarge.elasticsearch" => ESPartitionInstanceType::R4XlargeElasticsearch,
            "r5.12xlarge.elasticsearch" => ESPartitionInstanceType::R512XlargeElasticsearch,
            "r5.2xlarge.elasticsearch" => ESPartitionInstanceType::R52XlargeElasticsearch,
            "r5.4xlarge.elasticsearch" => ESPartitionInstanceType::R54XlargeElasticsearch,
            "r5.large.elasticsearch" => ESPartitionInstanceType::R5LargeElasticsearch,
            "r5.xlarge.elasticsearch" => ESPartitionInstanceType::R5XlargeElasticsearch,
            "t2.medium.elasticsearch" => ESPartitionInstanceType::T2MediumElasticsearch,
            "t2.micro.elasticsearch" => ESPartitionInstanceType::T2MicroElasticsearch,
            "t2.small.elasticsearch" => ESPartitionInstanceType::T2SmallElasticsearch,
            "ultrawarm1.large.elasticsearch" => {
                ESPartitionInstanceType::Ultrawarm1LargeElasticsearch
            }
            "ultrawarm1.medium.elasticsearch" => {
                ESPartitionInstanceType::Ultrawarm1MediumElasticsearch
            }
            _ => ESPartitionInstanceType::UnknownVariant(UnknownESPartitionInstanceType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for ESPartitionInstanceType {
    fn from(name: String) -> Self {
        match &*name {
            "c4.2xlarge.elasticsearch" => ESPartitionInstanceType::C42XlargeElasticsearch,
            "c4.4xlarge.elasticsearch" => ESPartitionInstanceType::C44XlargeElasticsearch,
            "c4.8xlarge.elasticsearch" => ESPartitionInstanceType::C48XlargeElasticsearch,
            "c4.large.elasticsearch" => ESPartitionInstanceType::C4LargeElasticsearch,
            "c4.xlarge.elasticsearch" => ESPartitionInstanceType::C4XlargeElasticsearch,
            "c5.18xlarge.elasticsearch" => ESPartitionInstanceType::C518XlargeElasticsearch,
            "c5.2xlarge.elasticsearch" => ESPartitionInstanceType::C52XlargeElasticsearch,
            "c5.4xlarge.elasticsearch" => ESPartitionInstanceType::C54XlargeElasticsearch,
            "c5.9xlarge.elasticsearch" => ESPartitionInstanceType::C59XlargeElasticsearch,
            "c5.large.elasticsearch" => ESPartitionInstanceType::C5LargeElasticsearch,
            "c5.xlarge.elasticsearch" => ESPartitionInstanceType::C5XlargeElasticsearch,
            "d2.2xlarge.elasticsearch" => ESPartitionInstanceType::D22XlargeElasticsearch,
            "d2.4xlarge.elasticsearch" => ESPartitionInstanceType::D24XlargeElasticsearch,
            "d2.8xlarge.elasticsearch" => ESPartitionInstanceType::D28XlargeElasticsearch,
            "d2.xlarge.elasticsearch" => ESPartitionInstanceType::D2XlargeElasticsearch,
            "i2.2xlarge.elasticsearch" => ESPartitionInstanceType::I22XlargeElasticsearch,
            "i2.xlarge.elasticsearch" => ESPartitionInstanceType::I2XlargeElasticsearch,
            "i3.16xlarge.elasticsearch" => ESPartitionInstanceType::I316XlargeElasticsearch,
            "i3.2xlarge.elasticsearch" => ESPartitionInstanceType::I32XlargeElasticsearch,
            "i3.4xlarge.elasticsearch" => ESPartitionInstanceType::I34XlargeElasticsearch,
            "i3.8xlarge.elasticsearch" => ESPartitionInstanceType::I38XlargeElasticsearch,
            "i3.large.elasticsearch" => ESPartitionInstanceType::I3LargeElasticsearch,
            "i3.xlarge.elasticsearch" => ESPartitionInstanceType::I3XlargeElasticsearch,
            "m3.2xlarge.elasticsearch" => ESPartitionInstanceType::M32XlargeElasticsearch,
            "m3.large.elasticsearch" => ESPartitionInstanceType::M3LargeElasticsearch,
            "m3.medium.elasticsearch" => ESPartitionInstanceType::M3MediumElasticsearch,
            "m3.xlarge.elasticsearch" => ESPartitionInstanceType::M3XlargeElasticsearch,
            "m4.10xlarge.elasticsearch" => ESPartitionInstanceType::M410XlargeElasticsearch,
            "m4.2xlarge.elasticsearch" => ESPartitionInstanceType::M42XlargeElasticsearch,
            "m4.4xlarge.elasticsearch" => ESPartitionInstanceType::M44XlargeElasticsearch,
            "m4.large.elasticsearch" => ESPartitionInstanceType::M4LargeElasticsearch,
            "m4.xlarge.elasticsearch" => ESPartitionInstanceType::M4XlargeElasticsearch,
            "m5.12xlarge.elasticsearch" => ESPartitionInstanceType::M512XlargeElasticsearch,
            "m5.2xlarge.elasticsearch" => ESPartitionInstanceType::M52XlargeElasticsearch,
            "m5.4xlarge.elasticsearch" => ESPartitionInstanceType::M54XlargeElasticsearch,
            "m5.large.elasticsearch" => ESPartitionInstanceType::M5LargeElasticsearch,
            "m5.xlarge.elasticsearch" => ESPartitionInstanceType::M5XlargeElasticsearch,
            "r3.2xlarge.elasticsearch" => ESPartitionInstanceType::R32XlargeElasticsearch,
            "r3.4xlarge.elasticsearch" => ESPartitionInstanceType::R34XlargeElasticsearch,
            "r3.8xlarge.elasticsearch" => ESPartitionInstanceType::R38XlargeElasticsearch,
            "r3.large.elasticsearch" => ESPartitionInstanceType::R3LargeElasticsearch,
            "r3.xlarge.elasticsearch" => ESPartitionInstanceType::R3XlargeElasticsearch,
            "r4.16xlarge.elasticsearch" => ESPartitionInstanceType::R416XlargeElasticsearch,
            "r4.2xlarge.elasticsearch" => ESPartitionInstanceType::R42XlargeElasticsearch,
            "r4.4xlarge.elasticsearch" => ESPartitionInstanceType::R44XlargeElasticsearch,
            "r4.8xlarge.elasticsearch" => ESPartitionInstanceType::R48XlargeElasticsearch,
            "r4.large.elasticsearch" => ESPartitionInstanceType::R4LargeElasticsearch,
            "r4.xlarge.elasticsearch" => ESPartitionInstanceType::R4XlargeElasticsearch,
            "r5.12xlarge.elasticsearch" => ESPartitionInstanceType::R512XlargeElasticsearch,
            "r5.2xlarge.elasticsearch" => ESPartitionInstanceType::R52XlargeElasticsearch,
            "r5.4xlarge.elasticsearch" => ESPartitionInstanceType::R54XlargeElasticsearch,
            "r5.large.elasticsearch" => ESPartitionInstanceType::R5LargeElasticsearch,
            "r5.xlarge.elasticsearch" => ESPartitionInstanceType::R5XlargeElasticsearch,
            "t2.medium.elasticsearch" => ESPartitionInstanceType::T2MediumElasticsearch,
            "t2.micro.elasticsearch" => ESPartitionInstanceType::T2MicroElasticsearch,
            "t2.small.elasticsearch" => ESPartitionInstanceType::T2SmallElasticsearch,
            "ultrawarm1.large.elasticsearch" => {
                ESPartitionInstanceType::Ultrawarm1LargeElasticsearch
            }
            "ultrawarm1.medium.elasticsearch" => {
                ESPartitionInstanceType::Ultrawarm1MediumElasticsearch
            }
            _ => ESPartitionInstanceType::UnknownVariant(UnknownESPartitionInstanceType { name }),
        }
    }
}

impl ::std::str::FromStr for ESPartitionInstanceType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for ESPartitionInstanceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ESPartitionInstanceType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownESWarmPartitionInstanceType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ESWarmPartitionInstanceType {
    Ultrawarm1LargeElasticsearch,
    Ultrawarm1MediumElasticsearch,
    #[doc(hidden)]
    UnknownVariant(UnknownESWarmPartitionInstanceType),
}

impl Default for ESWarmPartitionInstanceType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for ESWarmPartitionInstanceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for ESWarmPartitionInstanceType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for ESWarmPartitionInstanceType {
    fn into(self) -> String {
        match self {
            ESWarmPartitionInstanceType::Ultrawarm1LargeElasticsearch => {
                "ultrawarm1.large.elasticsearch".to_string()
            }
            ESWarmPartitionInstanceType::Ultrawarm1MediumElasticsearch => {
                "ultrawarm1.medium.elasticsearch".to_string()
            }
            ESWarmPartitionInstanceType::UnknownVariant(UnknownESWarmPartitionInstanceType {
                name: original,
            }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a ESWarmPartitionInstanceType {
    fn into(self) -> &'a str {
        match self {
            ESWarmPartitionInstanceType::Ultrawarm1LargeElasticsearch => {
                &"ultrawarm1.large.elasticsearch"
            }
            ESWarmPartitionInstanceType::Ultrawarm1MediumElasticsearch => {
                &"ultrawarm1.medium.elasticsearch"
            }
            ESWarmPartitionInstanceType::UnknownVariant(UnknownESWarmPartitionInstanceType {
                name: original,
            }) => original,
        }
    }
}

impl From<&str> for ESWarmPartitionInstanceType {
    fn from(name: &str) -> Self {
        match name {
            "ultrawarm1.large.elasticsearch" => {
                ESWarmPartitionInstanceType::Ultrawarm1LargeElasticsearch
            }
            "ultrawarm1.medium.elasticsearch" => {
                ESWarmPartitionInstanceType::Ultrawarm1MediumElasticsearch
            }
            _ => ESWarmPartitionInstanceType::UnknownVariant(UnknownESWarmPartitionInstanceType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for ESWarmPartitionInstanceType {
    fn from(name: String) -> Self {
        match &*name {
            "ultrawarm1.large.elasticsearch" => {
                ESWarmPartitionInstanceType::Ultrawarm1LargeElasticsearch
            }
            "ultrawarm1.medium.elasticsearch" => {
                ESWarmPartitionInstanceType::Ultrawarm1MediumElasticsearch
            }
            _ => ESWarmPartitionInstanceType::UnknownVariant(UnknownESWarmPartitionInstanceType {
                name,
            }),
        }
    }
}

impl ::std::str::FromStr for ESWarmPartitionInstanceType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for ESWarmPartitionInstanceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ESWarmPartitionInstanceType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Specifies the configuration for the domain cluster, such as the type and number of instances.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
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
    pub dedicated_master_type: Option<ESPartitionInstanceType>,
    /// <p>The number of instances in the specified domain cluster.</p>
    #[serde(rename = "InstanceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_count: Option<i64>,
    /// <p>The instance type for an Elasticsearch cluster. UltraWarm instance types are not supported for data instances.</p>
    #[serde(rename = "InstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<ESPartitionInstanceType>,
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
    pub warm_type: Option<ESWarmPartitionInstanceType>,
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
    /// <p>Specifies <code>AdvancedSecurityOptions</code> for the domain. </p>
    #[serde(rename = "AdvancedSecurityOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_security_options: Option<AdvancedSecurityOptionsStatus>,
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
    /// <p>The current status of the Elasticsearch domain's advanced security options.</p>
    #[serde(rename = "AdvancedSecurityOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_security_options: Option<AdvancedSecurityOptions>,
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
    pub log_publishing_options: Option<::std::collections::HashMap<LogType, LogPublishingOption>>,
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EncryptionAtRestOptionsStatus {
    /// <p> Specifies the Encryption At Rest options for the specified Elasticsearch domain.</p>
    #[serde(rename = "Options")]
    pub options: EncryptionAtRestOptions,
    /// <p> Specifies the status of the Encryption At Rest options for the specified Elasticsearch domain.</p>
    #[serde(rename = "Status")]
    pub status: OptionStatus,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ErrorDetails {
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "ErrorType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<String>,
}

/// <p> A filter used to limit results when describing inbound or outbound cross-cluster search connections. Multiple values can be specified per filter. A cross-cluster search connection must match at least one of the specified values for it to be returned from an operation. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Filter {
    /// <p> Specifies the name of the filter. </p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p> Contains one or more values for the filter. </p>
    #[serde(rename = "Values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// <p> Container for request parameters to <code> <a>GetCompatibleElasticsearchVersions</a> </code> operation. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetCompatibleElasticsearchVersionsRequest {
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
}

/// <p> Container for response returned by <code> <a>GetCompatibleElasticsearchVersions</a> </code> operation. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetCompatibleElasticsearchVersionsResponse {
    /// <p> A map of compatible Elasticsearch versions returned as part of the <code> <a>GetCompatibleElasticsearchVersions</a> </code> operation. </p>
    #[serde(rename = "CompatibleElasticsearchVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatible_elasticsearch_versions: Option<Vec<CompatibleVersionsMap>>,
}

/// <p> Container for request parameters to <code> <a>GetPackageVersionHistory</a> </code> operation. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetPackageVersionHistoryRequest {
    /// <p>Limits results to a maximum number of versions.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Used for pagination. Only necessary if a previous API call includes a non-null NextToken value. If provided, returns results for the next page.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Returns an audit history of versions of the package.</p>
    #[serde(rename = "PackageID")]
    pub package_id: String,
}

/// <p> Container for response returned by <code> <a>GetPackageVersionHistory</a> </code> operation. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetPackageVersionHistoryResponse {
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PackageID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_id: Option<String>,
    /// <p>List of <code>PackageVersionHistory</code> objects.</p>
    #[serde(rename = "PackageVersionHistoryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_version_history_list: Option<Vec<PackageVersionHistory>>,
}

/// <p> Container for request parameters to <code> <a>GetUpgradeHistory</a> </code> operation. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetUpgradeStatusRequest {
    #[serde(rename = "DomainName")]
    pub domain_name: String,
}

/// <p> Container for response returned by <code> <a>GetUpgradeStatus</a> </code> operation. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetUpgradeStatusResponse {
    /// <p> One of 4 statuses that a step can go through returned as part of the <code> <a>GetUpgradeStatusResponse</a> </code> object. The status can take one of the following values: <ul> <li>In Progress</li> <li>Succeeded</li> <li>Succeeded with Issues</li> <li>Failed</li> </ul> </p>
    #[serde(rename = "StepStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_status: Option<UpgradeStatus>,
    /// <p>A string that describes the update briefly</p>
    #[serde(rename = "UpgradeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgrade_name: Option<String>,
    /// <p> Represents one of 3 steps that an Upgrade or Upgrade Eligibility Check does through: <ul> <li>PreUpgradeCheck</li> <li>Snapshot</li> <li>Upgrade</li> </ul> </p>
    #[serde(rename = "UpgradeStep")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgrade_step: Option<UpgradeStep>,
}

/// <p>Specifies details of an inbound connection.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InboundCrossClusterSearchConnection {
    /// <p>Specifies the <code><a>InboundCrossClusterSearchConnectionStatus</a></code> for the outbound connection.</p>
    #[serde(rename = "ConnectionStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_status: Option<InboundCrossClusterSearchConnectionStatus>,
    /// <p>Specifies the connection id for the inbound cross-cluster search connection.</p>
    #[serde(rename = "CrossClusterSearchConnectionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_cluster_search_connection_id: Option<String>,
    /// <p>Specifies the <code><a>DomainInformation</a></code> for the destination Elasticsearch domain.</p>
    #[serde(rename = "DestinationDomainInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_domain_info: Option<DomainInformation>,
    /// <p>Specifies the <code><a>DomainInformation</a></code> for the source Elasticsearch domain.</p>
    #[serde(rename = "SourceDomainInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_domain_info: Option<DomainInformation>,
}

/// <p>Specifies the coonection status of an inbound cross-cluster search connection.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InboundCrossClusterSearchConnectionStatus {
    /// <p>Specifies verbose information for the inbound connection status.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p><p>The state code for inbound connection. This can be one of the following:</p> <ul> <li>PENDING_ACCEPTANCE: Inbound connection is not yet accepted by destination domain owner.</li> <li>APPROVED: Inbound connection is pending acceptance by destination domain owner.</li> <li>REJECTING: Inbound connection rejection is in process.</li> <li>REJECTED: Inbound connection is rejected.</li> <li>DELETING: Inbound connection deletion is in progress.</li> <li>DELETED: Inbound connection is deleted and cannot be used further.</li> </ul></p>
    #[serde(rename = "StatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<InboundCrossClusterSearchConnectionStatusCode>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownInboundCrossClusterSearchConnectionStatusCode {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum InboundCrossClusterSearchConnectionStatusCode {
    Approved,
    Deleted,
    Deleting,
    PendingAcceptance,
    Rejected,
    Rejecting,
    #[doc(hidden)]
    UnknownVariant(UnknownInboundCrossClusterSearchConnectionStatusCode),
}

impl Default for InboundCrossClusterSearchConnectionStatusCode {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for InboundCrossClusterSearchConnectionStatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for InboundCrossClusterSearchConnectionStatusCode {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for InboundCrossClusterSearchConnectionStatusCode {
    fn into(self) -> String {
        match self {
            InboundCrossClusterSearchConnectionStatusCode::Approved => "APPROVED".to_string(),
            InboundCrossClusterSearchConnectionStatusCode::Deleted => "DELETED".to_string(),
            InboundCrossClusterSearchConnectionStatusCode::Deleting => "DELETING".to_string(),
            InboundCrossClusterSearchConnectionStatusCode::PendingAcceptance => {
                "PENDING_ACCEPTANCE".to_string()
            }
            InboundCrossClusterSearchConnectionStatusCode::Rejected => "REJECTED".to_string(),
            InboundCrossClusterSearchConnectionStatusCode::Rejecting => "REJECTING".to_string(),
            InboundCrossClusterSearchConnectionStatusCode::UnknownVariant(
                UnknownInboundCrossClusterSearchConnectionStatusCode { name: original },
            ) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a InboundCrossClusterSearchConnectionStatusCode {
    fn into(self) -> &'a str {
        match self {
            InboundCrossClusterSearchConnectionStatusCode::Approved => &"APPROVED",
            InboundCrossClusterSearchConnectionStatusCode::Deleted => &"DELETED",
            InboundCrossClusterSearchConnectionStatusCode::Deleting => &"DELETING",
            InboundCrossClusterSearchConnectionStatusCode::PendingAcceptance => {
                &"PENDING_ACCEPTANCE"
            }
            InboundCrossClusterSearchConnectionStatusCode::Rejected => &"REJECTED",
            InboundCrossClusterSearchConnectionStatusCode::Rejecting => &"REJECTING",
            InboundCrossClusterSearchConnectionStatusCode::UnknownVariant(
                UnknownInboundCrossClusterSearchConnectionStatusCode { name: original },
            ) => original,
        }
    }
}

impl From<&str> for InboundCrossClusterSearchConnectionStatusCode {
    fn from(name: &str) -> Self {
        match name {
            "APPROVED" => InboundCrossClusterSearchConnectionStatusCode::Approved,
            "DELETED" => InboundCrossClusterSearchConnectionStatusCode::Deleted,
            "DELETING" => InboundCrossClusterSearchConnectionStatusCode::Deleting,
            "PENDING_ACCEPTANCE" => {
                InboundCrossClusterSearchConnectionStatusCode::PendingAcceptance
            }
            "REJECTED" => InboundCrossClusterSearchConnectionStatusCode::Rejected,
            "REJECTING" => InboundCrossClusterSearchConnectionStatusCode::Rejecting,
            _ => InboundCrossClusterSearchConnectionStatusCode::UnknownVariant(
                UnknownInboundCrossClusterSearchConnectionStatusCode {
                    name: name.to_owned(),
                },
            ),
        }
    }
}

impl From<String> for InboundCrossClusterSearchConnectionStatusCode {
    fn from(name: String) -> Self {
        match &*name {
            "APPROVED" => InboundCrossClusterSearchConnectionStatusCode::Approved,
            "DELETED" => InboundCrossClusterSearchConnectionStatusCode::Deleted,
            "DELETING" => InboundCrossClusterSearchConnectionStatusCode::Deleting,
            "PENDING_ACCEPTANCE" => {
                InboundCrossClusterSearchConnectionStatusCode::PendingAcceptance
            }
            "REJECTED" => InboundCrossClusterSearchConnectionStatusCode::Rejected,
            "REJECTING" => InboundCrossClusterSearchConnectionStatusCode::Rejecting,
            _ => InboundCrossClusterSearchConnectionStatusCode::UnknownVariant(
                UnknownInboundCrossClusterSearchConnectionStatusCode { name },
            ),
        }
    }
}

impl ::std::str::FromStr for InboundCrossClusterSearchConnectionStatusCode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for InboundCrossClusterSearchConnectionStatusCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for InboundCrossClusterSearchConnectionStatusCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p> InstanceCountLimits represents the limits on number of instances that be created in Amazon Elasticsearch for given InstanceType. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InstanceLimits {
    #[serde(rename = "InstanceCountLimits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_count_limits: Option<InstanceCountLimits>,
}

/// <p> Limits for given InstanceType and for each of it's role. <br/> Limits contains following <code> <a>StorageTypes,</a> </code> <code> <a>InstanceLimits</a> </code> and <code> <a>AdditionalLimits</a> </code> </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListDomainNamesResponse {
    /// <p>List of Elasticsearch domain names.</p>
    #[serde(rename = "DomainNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_names: Option<Vec<DomainInfo>>,
}

/// <p> Container for request parameters to <code> <a>ListDomainsForPackage</a> </code> operation. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListDomainsForPackageRequest {
    /// <p>Limits results to a maximum number of domains.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Used for pagination. Only necessary if a previous API call includes a non-null NextToken value. If provided, returns results for the next page.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The package for which to list domains.</p>
    #[serde(rename = "PackageID")]
    pub package_id: String,
}

/// <p> Container for response parameters to <code> <a>ListDomainsForPackage</a> </code> operation. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListDomainsForPackageResponse {
    /// <p>List of <code>DomainPackageDetails</code> objects.</p>
    #[serde(rename = "DomainPackageDetailsList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_package_details_list: Option<Vec<DomainPackageDetails>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p> Container for the parameters to the <code> <a>ListElasticsearchInstanceTypes</a> </code> operation. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListElasticsearchInstanceTypesResponse {
    /// <p> List of instance types supported by Amazon Elasticsearch service for given <code> <a>ElasticsearchVersion</a> </code> </p>
    #[serde(rename = "ElasticsearchInstanceTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elasticsearch_instance_types: Option<Vec<ESPartitionInstanceType>>,
    /// <p>In case if there are more results available NextToken would be present, make further request to the same API with received NextToken to paginate remaining results. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p> Container for the parameters to the <code> <a>ListElasticsearchVersions</a> </code> operation. <p> Use <code> <a>MaxResults</a> </code> to control the maximum number of results to retrieve in a single call. </p> <p> Use <code> <a>NextToken</a> </code> in response to retrieve more results. If the received response does not contain a NextToken, then there are no more results to retrieve. </p> </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListElasticsearchVersionsResponse {
    #[serde(rename = "ElasticsearchVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elasticsearch_versions: Option<Vec<String>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p> Container for request parameters to <code> <a>ListPackagesForDomain</a> </code> operation. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListPackagesForDomainRequest {
    /// <p>The name of the domain for which you want to list associated packages.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// <p>Limits results to a maximum number of packages.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Used for pagination. Only necessary if a previous API call includes a non-null NextToken value. If provided, returns results for the next page.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p> Container for response parameters to <code> <a>ListPackagesForDomain</a> </code> operation. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListPackagesForDomainResponse {
    /// <p>List of <code>DomainPackageDetails</code> objects.</p>
    #[serde(rename = "DomainPackageDetailsList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_package_details_list: Option<Vec<DomainPackageDetails>>,
    /// <p>Pagination token that needs to be supplied to the next call to get the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Container for the parameters to the <code><a>ListTags</a></code> operation. Specify the <code>ARN</code> for the Elasticsearch domain to which the tags are attached that you want to view are attached.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsRequest {
    /// <p> Specify the <code>ARN</code> for the Elasticsearch domain to which the tags are attached that you want to view.</p>
    #[serde(rename = "ARN")]
    pub arn: String,
}

/// <p>The result of a <code>ListTags</code> operation. Contains tags for all requested Elasticsearch domains.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsResponse {
    /// <p> List of <code>Tag</code> for the requested Elasticsearch domain.</p>
    #[serde(rename = "TagList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<Tag>>,
}

/// <p>Log Publishing option that is set for given domain. <br/>Attributes and their details: <ul> <li>CloudWatchLogsLogGroupArn: ARN of the Cloudwatch log group to which log needs to be published.</li> <li>Enabled: Whether the log publishing for given log type is enabled or not</li> </ul> </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LogPublishingOptionsStatus {
    /// <p>The log publishing options configured for the Elasticsearch domain.</p>
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<::std::collections::HashMap<LogType, LogPublishingOption>>,
    /// <p>The status of the log publishing options for the Elasticsearch domain. See <code>OptionStatus</code> for the status information that's included. </p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OptionStatus>,
}

/// <p>Type of Log File, it can be one of the following: <ul> <li>INDEX_SLOW_LOGS: Index slow logs contain insert requests that took more time than configured index query log threshold to execute.</li> <li>SEARCH_SLOW_LOGS: Search slow logs contain search queries that took more time than configured search query log threshold to execute.</li> <li>ES_APPLICATION_LOGS: Elasticsearch application logs contain information about errors and warnings raised during the operation of the service and can be useful for troubleshooting.</li> <li>AUDIT_LOGS: Audit logs contain records of user requests for access from the domain.</li> </ul> </p>

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownLogType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum LogType {
    AuditLogs,
    EsApplicationLogs,
    IndexSlowLogs,
    SearchSlowLogs,
    #[doc(hidden)]
    UnknownVariant(UnknownLogType),
}

impl Default for LogType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for LogType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for LogType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for LogType {
    fn into(self) -> String {
        match self {
            LogType::AuditLogs => "AUDIT_LOGS".to_string(),
            LogType::EsApplicationLogs => "ES_APPLICATION_LOGS".to_string(),
            LogType::IndexSlowLogs => "INDEX_SLOW_LOGS".to_string(),
            LogType::SearchSlowLogs => "SEARCH_SLOW_LOGS".to_string(),
            LogType::UnknownVariant(UnknownLogType { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a LogType {
    fn into(self) -> &'a str {
        match self {
            LogType::AuditLogs => &"AUDIT_LOGS",
            LogType::EsApplicationLogs => &"ES_APPLICATION_LOGS",
            LogType::IndexSlowLogs => &"INDEX_SLOW_LOGS",
            LogType::SearchSlowLogs => &"SEARCH_SLOW_LOGS",
            LogType::UnknownVariant(UnknownLogType { name: original }) => original,
        }
    }
}

impl From<&str> for LogType {
    fn from(name: &str) -> Self {
        match name {
            "AUDIT_LOGS" => LogType::AuditLogs,
            "ES_APPLICATION_LOGS" => LogType::EsApplicationLogs,
            "INDEX_SLOW_LOGS" => LogType::IndexSlowLogs,
            "SEARCH_SLOW_LOGS" => LogType::SearchSlowLogs,
            _ => LogType::UnknownVariant(UnknownLogType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for LogType {
    fn from(name: String) -> Self {
        match &*name {
            "AUDIT_LOGS" => LogType::AuditLogs,
            "ES_APPLICATION_LOGS" => LogType::EsApplicationLogs,
            "INDEX_SLOW_LOGS" => LogType::IndexSlowLogs,
            "SEARCH_SLOW_LOGS" => LogType::SearchSlowLogs,
            _ => LogType::UnknownVariant(UnknownLogType { name }),
        }
    }
}

impl ::std::str::FromStr for LogType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for LogType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for LogType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Credentials for the master user: username and password, ARN, or both.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct MasterUserOptions {
    /// <p>ARN for the master user (if IAM is enabled).</p>
    #[serde(rename = "MasterUserARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_arn: Option<String>,
    /// <p>The master user's username, which is stored in the Amazon Elasticsearch Service domain's internal database.</p>
    #[serde(rename = "MasterUserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_name: Option<String>,
    /// <p>The master user's password, which is stored in the Amazon Elasticsearch Service domain's internal database.</p>
    #[serde(rename = "MasterUserPassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_password: Option<String>,
}

/// <p>Specifies the node-to-node encryption options.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct NodeToNodeEncryptionOptions {
    /// <p>Specify true to enable node-to-node encryption.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// <p>Status of the node-to-node encryption options for the specified Elasticsearch domain.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct NodeToNodeEncryptionOptionsStatus {
    /// <p>Specifies the node-to-node encryption options for the specified Elasticsearch domain.</p>
    #[serde(rename = "Options")]
    pub options: NodeToNodeEncryptionOptions,
    /// <p>Specifies the status of the node-to-node encryption options for the specified Elasticsearch domain.</p>
    #[serde(rename = "Status")]
    pub status: OptionStatus,
}

/// <p><p>The state of a requested change. One of the following:</p> <ul> <li>Processing: The request change is still in-process.</li> <li>Active: The request change is processed and deployed to the Elasticsearch domain.</li> </ul></p>

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownOptionState {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum OptionState {
    Active,
    Processing,
    RequiresIndexDocuments,
    #[doc(hidden)]
    UnknownVariant(UnknownOptionState),
}

impl Default for OptionState {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for OptionState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for OptionState {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for OptionState {
    fn into(self) -> String {
        match self {
            OptionState::Active => "Active".to_string(),
            OptionState::Processing => "Processing".to_string(),
            OptionState::RequiresIndexDocuments => "RequiresIndexDocuments".to_string(),
            OptionState::UnknownVariant(UnknownOptionState { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a OptionState {
    fn into(self) -> &'a str {
        match self {
            OptionState::Active => &"Active",
            OptionState::Processing => &"Processing",
            OptionState::RequiresIndexDocuments => &"RequiresIndexDocuments",
            OptionState::UnknownVariant(UnknownOptionState { name: original }) => original,
        }
    }
}

impl From<&str> for OptionState {
    fn from(name: &str) -> Self {
        match name {
            "Active" => OptionState::Active,
            "Processing" => OptionState::Processing,
            "RequiresIndexDocuments" => OptionState::RequiresIndexDocuments,
            _ => OptionState::UnknownVariant(UnknownOptionState {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for OptionState {
    fn from(name: String) -> Self {
        match &*name {
            "Active" => OptionState::Active,
            "Processing" => OptionState::Processing,
            "RequiresIndexDocuments" => OptionState::RequiresIndexDocuments,
            _ => OptionState::UnknownVariant(UnknownOptionState { name }),
        }
    }
}

impl ::std::str::FromStr for OptionState {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for OptionState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for OptionState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Provides the current status of the entity.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
    pub state: OptionState,
    /// <p>Timestamp which tells the last updated time for the entity.</p>
    #[serde(rename = "UpdateDate")]
    pub update_date: f64,
    /// <p>Specifies the latest version for the entity.</p>
    #[serde(rename = "UpdateVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_version: Option<i64>,
}

/// <p>Specifies details of an outbound connection.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct OutboundCrossClusterSearchConnection {
    /// <p>Specifies the connection alias for the outbound cross-cluster search connection.</p>
    #[serde(rename = "ConnectionAlias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_alias: Option<String>,
    /// <p>Specifies the <code><a>OutboundCrossClusterSearchConnectionStatus</a></code> for the outbound connection.</p>
    #[serde(rename = "ConnectionStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_status: Option<OutboundCrossClusterSearchConnectionStatus>,
    /// <p>Specifies the connection id for the outbound cross-cluster search connection.</p>
    #[serde(rename = "CrossClusterSearchConnectionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_cluster_search_connection_id: Option<String>,
    /// <p>Specifies the <code><a>DomainInformation</a></code> for the destination Elasticsearch domain.</p>
    #[serde(rename = "DestinationDomainInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_domain_info: Option<DomainInformation>,
    /// <p>Specifies the <code><a>DomainInformation</a></code> for the source Elasticsearch domain.</p>
    #[serde(rename = "SourceDomainInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_domain_info: Option<DomainInformation>,
}

/// <p>Specifies the connection status of an outbound cross-cluster search connection.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct OutboundCrossClusterSearchConnectionStatus {
    /// <p>Specifies verbose information for the outbound connection status.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p><p>The state code for outbound connection. This can be one of the following:</p> <ul> <li>VALIDATING: The outbound connection request is being validated.</li> <li>VALIDATION<em>FAILED: Validation failed for the connection request.</li> <li>PENDING</em>ACCEPTANCE: Outbound connection request is validated and is not yet accepted by destination domain owner.</li> <li>PROVISIONING: Outbound connection request is in process.</li> <li>ACTIVE: Outbound connection is active and ready to use.</li> <li>REJECTED: Outbound connection request is rejected by destination domain owner.</li> <li>DELETING: Outbound connection deletion is in progress.</li> <li>DELETED: Outbound connection is deleted and cannot be used further.</li> </ul></p>
    #[serde(rename = "StatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<OutboundCrossClusterSearchConnectionStatusCode>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownOutboundCrossClusterSearchConnectionStatusCode {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum OutboundCrossClusterSearchConnectionStatusCode {
    Active,
    Deleted,
    Deleting,
    PendingAcceptance,
    Provisioning,
    Rejected,
    Validating,
    ValidationFailed,
    #[doc(hidden)]
    UnknownVariant(UnknownOutboundCrossClusterSearchConnectionStatusCode),
}

impl Default for OutboundCrossClusterSearchConnectionStatusCode {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for OutboundCrossClusterSearchConnectionStatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for OutboundCrossClusterSearchConnectionStatusCode {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for OutboundCrossClusterSearchConnectionStatusCode {
    fn into(self) -> String {
        match self {
            OutboundCrossClusterSearchConnectionStatusCode::Active => "ACTIVE".to_string(),
            OutboundCrossClusterSearchConnectionStatusCode::Deleted => "DELETED".to_string(),
            OutboundCrossClusterSearchConnectionStatusCode::Deleting => "DELETING".to_string(),
            OutboundCrossClusterSearchConnectionStatusCode::PendingAcceptance => {
                "PENDING_ACCEPTANCE".to_string()
            }
            OutboundCrossClusterSearchConnectionStatusCode::Provisioning => {
                "PROVISIONING".to_string()
            }
            OutboundCrossClusterSearchConnectionStatusCode::Rejected => "REJECTED".to_string(),
            OutboundCrossClusterSearchConnectionStatusCode::Validating => "VALIDATING".to_string(),
            OutboundCrossClusterSearchConnectionStatusCode::ValidationFailed => {
                "VALIDATION_FAILED".to_string()
            }
            OutboundCrossClusterSearchConnectionStatusCode::UnknownVariant(
                UnknownOutboundCrossClusterSearchConnectionStatusCode { name: original },
            ) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a OutboundCrossClusterSearchConnectionStatusCode {
    fn into(self) -> &'a str {
        match self {
            OutboundCrossClusterSearchConnectionStatusCode::Active => &"ACTIVE",
            OutboundCrossClusterSearchConnectionStatusCode::Deleted => &"DELETED",
            OutboundCrossClusterSearchConnectionStatusCode::Deleting => &"DELETING",
            OutboundCrossClusterSearchConnectionStatusCode::PendingAcceptance => {
                &"PENDING_ACCEPTANCE"
            }
            OutboundCrossClusterSearchConnectionStatusCode::Provisioning => &"PROVISIONING",
            OutboundCrossClusterSearchConnectionStatusCode::Rejected => &"REJECTED",
            OutboundCrossClusterSearchConnectionStatusCode::Validating => &"VALIDATING",
            OutboundCrossClusterSearchConnectionStatusCode::ValidationFailed => {
                &"VALIDATION_FAILED"
            }
            OutboundCrossClusterSearchConnectionStatusCode::UnknownVariant(
                UnknownOutboundCrossClusterSearchConnectionStatusCode { name: original },
            ) => original,
        }
    }
}

impl From<&str> for OutboundCrossClusterSearchConnectionStatusCode {
    fn from(name: &str) -> Self {
        match name {
            "ACTIVE" => OutboundCrossClusterSearchConnectionStatusCode::Active,
            "DELETED" => OutboundCrossClusterSearchConnectionStatusCode::Deleted,
            "DELETING" => OutboundCrossClusterSearchConnectionStatusCode::Deleting,
            "PENDING_ACCEPTANCE" => {
                OutboundCrossClusterSearchConnectionStatusCode::PendingAcceptance
            }
            "PROVISIONING" => OutboundCrossClusterSearchConnectionStatusCode::Provisioning,
            "REJECTED" => OutboundCrossClusterSearchConnectionStatusCode::Rejected,
            "VALIDATING" => OutboundCrossClusterSearchConnectionStatusCode::Validating,
            "VALIDATION_FAILED" => OutboundCrossClusterSearchConnectionStatusCode::ValidationFailed,
            _ => OutboundCrossClusterSearchConnectionStatusCode::UnknownVariant(
                UnknownOutboundCrossClusterSearchConnectionStatusCode {
                    name: name.to_owned(),
                },
            ),
        }
    }
}

impl From<String> for OutboundCrossClusterSearchConnectionStatusCode {
    fn from(name: String) -> Self {
        match &*name {
            "ACTIVE" => OutboundCrossClusterSearchConnectionStatusCode::Active,
            "DELETED" => OutboundCrossClusterSearchConnectionStatusCode::Deleted,
            "DELETING" => OutboundCrossClusterSearchConnectionStatusCode::Deleting,
            "PENDING_ACCEPTANCE" => {
                OutboundCrossClusterSearchConnectionStatusCode::PendingAcceptance
            }
            "PROVISIONING" => OutboundCrossClusterSearchConnectionStatusCode::Provisioning,
            "REJECTED" => OutboundCrossClusterSearchConnectionStatusCode::Rejected,
            "VALIDATING" => OutboundCrossClusterSearchConnectionStatusCode::Validating,
            "VALIDATION_FAILED" => OutboundCrossClusterSearchConnectionStatusCode::ValidationFailed,
            _ => OutboundCrossClusterSearchConnectionStatusCode::UnknownVariant(
                UnknownOutboundCrossClusterSearchConnectionStatusCode { name },
            ),
        }
    }
}

impl ::std::str::FromStr for OutboundCrossClusterSearchConnectionStatusCode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for OutboundCrossClusterSearchConnectionStatusCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for OutboundCrossClusterSearchConnectionStatusCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Basic information about a package.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PackageDetails {
    #[serde(rename = "AvailablePackageVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_package_version: Option<String>,
    /// <p>Timestamp which tells creation date of the package.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>Additional information if the package is in an error state. Null otherwise.</p>
    #[serde(rename = "ErrorDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_details: Option<ErrorDetails>,
    #[serde(rename = "LastUpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    /// <p>User-specified description of the package.</p>
    #[serde(rename = "PackageDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_description: Option<String>,
    /// <p>Internal ID of the package.</p>
    #[serde(rename = "PackageID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_id: Option<String>,
    /// <p>User specified name of the package.</p>
    #[serde(rename = "PackageName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_name: Option<String>,
    /// <p>Current state of the package. Values are COPYING/COPY_FAILED/AVAILABLE/DELETING/DELETE_FAILED</p>
    #[serde(rename = "PackageStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_status: Option<PackageStatus>,
    /// <p>Currently supports only TXT-DICTIONARY.</p>
    #[serde(rename = "PackageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_type: Option<PackageType>,
}

/// <p>The S3 location for importing the package specified as <code>S3BucketName</code> and <code>S3Key</code></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PackageSource {
    /// <p>Name of the bucket containing the package.</p>
    #[serde(rename = "S3BucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_name: Option<String>,
    /// <p>Key (file name) of the package.</p>
    #[serde(rename = "S3Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownPackageStatus {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum PackageStatus {
    Available,
    Copying,
    CopyFailed,
    Deleted,
    DeleteFailed,
    Deleting,
    Validating,
    ValidationFailed,
    #[doc(hidden)]
    UnknownVariant(UnknownPackageStatus),
}

impl Default for PackageStatus {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for PackageStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for PackageStatus {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for PackageStatus {
    fn into(self) -> String {
        match self {
            PackageStatus::Available => "AVAILABLE".to_string(),
            PackageStatus::Copying => "COPYING".to_string(),
            PackageStatus::CopyFailed => "COPY_FAILED".to_string(),
            PackageStatus::Deleted => "DELETED".to_string(),
            PackageStatus::DeleteFailed => "DELETE_FAILED".to_string(),
            PackageStatus::Deleting => "DELETING".to_string(),
            PackageStatus::Validating => "VALIDATING".to_string(),
            PackageStatus::ValidationFailed => "VALIDATION_FAILED".to_string(),
            PackageStatus::UnknownVariant(UnknownPackageStatus { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a PackageStatus {
    fn into(self) -> &'a str {
        match self {
            PackageStatus::Available => &"AVAILABLE",
            PackageStatus::Copying => &"COPYING",
            PackageStatus::CopyFailed => &"COPY_FAILED",
            PackageStatus::Deleted => &"DELETED",
            PackageStatus::DeleteFailed => &"DELETE_FAILED",
            PackageStatus::Deleting => &"DELETING",
            PackageStatus::Validating => &"VALIDATING",
            PackageStatus::ValidationFailed => &"VALIDATION_FAILED",
            PackageStatus::UnknownVariant(UnknownPackageStatus { name: original }) => original,
        }
    }
}

impl From<&str> for PackageStatus {
    fn from(name: &str) -> Self {
        match name {
            "AVAILABLE" => PackageStatus::Available,
            "COPYING" => PackageStatus::Copying,
            "COPY_FAILED" => PackageStatus::CopyFailed,
            "DELETED" => PackageStatus::Deleted,
            "DELETE_FAILED" => PackageStatus::DeleteFailed,
            "DELETING" => PackageStatus::Deleting,
            "VALIDATING" => PackageStatus::Validating,
            "VALIDATION_FAILED" => PackageStatus::ValidationFailed,
            _ => PackageStatus::UnknownVariant(UnknownPackageStatus {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for PackageStatus {
    fn from(name: String) -> Self {
        match &*name {
            "AVAILABLE" => PackageStatus::Available,
            "COPYING" => PackageStatus::Copying,
            "COPY_FAILED" => PackageStatus::CopyFailed,
            "DELETED" => PackageStatus::Deleted,
            "DELETE_FAILED" => PackageStatus::DeleteFailed,
            "DELETING" => PackageStatus::Deleting,
            "VALIDATING" => PackageStatus::Validating,
            "VALIDATION_FAILED" => PackageStatus::ValidationFailed,
            _ => PackageStatus::UnknownVariant(UnknownPackageStatus { name }),
        }
    }
}

impl ::std::str::FromStr for PackageStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for PackageStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for PackageStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownPackageType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum PackageType {
    TxtDictionary,
    #[doc(hidden)]
    UnknownVariant(UnknownPackageType),
}

impl Default for PackageType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for PackageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for PackageType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for PackageType {
    fn into(self) -> String {
        match self {
            PackageType::TxtDictionary => "TXT-DICTIONARY".to_string(),
            PackageType::UnknownVariant(UnknownPackageType { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a PackageType {
    fn into(self) -> &'a str {
        match self {
            PackageType::TxtDictionary => &"TXT-DICTIONARY",
            PackageType::UnknownVariant(UnknownPackageType { name: original }) => original,
        }
    }
}

impl From<&str> for PackageType {
    fn from(name: &str) -> Self {
        match name {
            "TXT-DICTIONARY" => PackageType::TxtDictionary,
            _ => PackageType::UnknownVariant(UnknownPackageType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for PackageType {
    fn from(name: String) -> Self {
        match &*name {
            "TXT-DICTIONARY" => PackageType::TxtDictionary,
            _ => PackageType::UnknownVariant(UnknownPackageType { name }),
        }
    }
}

impl ::std::str::FromStr for PackageType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for PackageType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for PackageType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Details of a package version.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PackageVersionHistory {
    /// <p>A message associated with the version.</p>
    #[serde(rename = "CommitMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_message: Option<String>,
    /// <p>Timestamp which tells creation time of the package version.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>Version of the package.</p>
    #[serde(rename = "PackageVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_version: Option<String>,
}

/// <p>Container for parameters to <code>PurchaseReservedElasticsearchInstanceOffering</code></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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

/// <p>Container for the parameters to the <code><a>RejectInboundCrossClusterSearchConnection</a></code> operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RejectInboundCrossClusterSearchConnectionRequest {
    /// <p>The id of the inbound connection that you want to reject.</p>
    #[serde(rename = "CrossClusterSearchConnectionId")]
    pub cross_cluster_search_connection_id: String,
}

/// <p>The result of a <code><a>RejectInboundCrossClusterSearchConnection</a></code> operation. Contains details of rejected inbound connection.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RejectInboundCrossClusterSearchConnectionResponse {
    /// <p>Specifies the <code><a>InboundCrossClusterSearchConnection</a></code> of rejected inbound connection. </p>
    #[serde(rename = "CrossClusterSearchConnection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_cluster_search_connection: Option<InboundCrossClusterSearchConnection>,
}

/// <p>Container for the parameters to the <code><a>RemoveTags</a></code> operation. Specify the <code>ARN</code> for the Elasticsearch domain from which you want to remove the specified <code>TagKey</code>.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
    pub elasticsearch_instance_type: Option<ESPartitionInstanceType>,
    /// <p>The upfront fixed charge you will paid to purchase the specific reserved Elasticsearch instance offering. </p>
    #[serde(rename = "FixedPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_price: Option<f64>,
    /// <p>The payment option as defined in the reserved Elasticsearch instance offering.</p>
    #[serde(rename = "PaymentOption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_option: Option<ReservedElasticsearchInstancePaymentOption>,
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
    pub elasticsearch_instance_type: Option<ESPartitionInstanceType>,
    /// <p>The upfront fixed charge you will pay to purchase the specific reserved Elasticsearch instance offering. </p>
    #[serde(rename = "FixedPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_price: Option<f64>,
    /// <p>Payment option for the reserved Elasticsearch instance offering</p>
    #[serde(rename = "PaymentOption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_option: Option<ReservedElasticsearchInstancePaymentOption>,
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

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownReservedElasticsearchInstancePaymentOption {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ReservedElasticsearchInstancePaymentOption {
    AllUpfront,
    NoUpfront,
    PartialUpfront,
    #[doc(hidden)]
    UnknownVariant(UnknownReservedElasticsearchInstancePaymentOption),
}

impl Default for ReservedElasticsearchInstancePaymentOption {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for ReservedElasticsearchInstancePaymentOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for ReservedElasticsearchInstancePaymentOption {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for ReservedElasticsearchInstancePaymentOption {
    fn into(self) -> String {
        match self {
            ReservedElasticsearchInstancePaymentOption::AllUpfront => "ALL_UPFRONT".to_string(),
            ReservedElasticsearchInstancePaymentOption::NoUpfront => "NO_UPFRONT".to_string(),
            ReservedElasticsearchInstancePaymentOption::PartialUpfront => {
                "PARTIAL_UPFRONT".to_string()
            }
            ReservedElasticsearchInstancePaymentOption::UnknownVariant(
                UnknownReservedElasticsearchInstancePaymentOption { name: original },
            ) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a ReservedElasticsearchInstancePaymentOption {
    fn into(self) -> &'a str {
        match self {
            ReservedElasticsearchInstancePaymentOption::AllUpfront => &"ALL_UPFRONT",
            ReservedElasticsearchInstancePaymentOption::NoUpfront => &"NO_UPFRONT",
            ReservedElasticsearchInstancePaymentOption::PartialUpfront => &"PARTIAL_UPFRONT",
            ReservedElasticsearchInstancePaymentOption::UnknownVariant(
                UnknownReservedElasticsearchInstancePaymentOption { name: original },
            ) => original,
        }
    }
}

impl From<&str> for ReservedElasticsearchInstancePaymentOption {
    fn from(name: &str) -> Self {
        match name {
            "ALL_UPFRONT" => ReservedElasticsearchInstancePaymentOption::AllUpfront,
            "NO_UPFRONT" => ReservedElasticsearchInstancePaymentOption::NoUpfront,
            "PARTIAL_UPFRONT" => ReservedElasticsearchInstancePaymentOption::PartialUpfront,
            _ => ReservedElasticsearchInstancePaymentOption::UnknownVariant(
                UnknownReservedElasticsearchInstancePaymentOption {
                    name: name.to_owned(),
                },
            ),
        }
    }
}

impl From<String> for ReservedElasticsearchInstancePaymentOption {
    fn from(name: String) -> Self {
        match &*name {
            "ALL_UPFRONT" => ReservedElasticsearchInstancePaymentOption::AllUpfront,
            "NO_UPFRONT" => ReservedElasticsearchInstancePaymentOption::NoUpfront,
            "PARTIAL_UPFRONT" => ReservedElasticsearchInstancePaymentOption::PartialUpfront,
            _ => ReservedElasticsearchInstancePaymentOption::UnknownVariant(
                UnknownReservedElasticsearchInstancePaymentOption { name },
            ),
        }
    }
}

impl ::std::str::FromStr for ReservedElasticsearchInstancePaymentOption {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for ReservedElasticsearchInstancePaymentOption {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ReservedElasticsearchInstancePaymentOption {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Specifies the SAML Identity Provider's information.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SAMLIdp {
    /// <p>The unique Entity ID of the application in SAML Identity Provider.</p>
    #[serde(rename = "EntityId")]
    pub entity_id: String,
    /// <p>The Metadata of the SAML application in xml format.</p>
    #[serde(rename = "MetadataContent")]
    pub metadata_content: String,
}

/// <p>Specifies the SAML application configuration for the domain.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SAMLOptionsInput {
    /// <p>True if SAML is enabled.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>Specifies the SAML Identity Provider's information.</p>
    #[serde(rename = "Idp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idp: Option<SAMLIdp>,
    /// <p>The backend role to which the SAML master user is mapped to.</p>
    #[serde(rename = "MasterBackendRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_backend_role: Option<String>,
    /// <p>The SAML master username, which is stored in the Amazon Elasticsearch Service domain's internal database.</p>
    #[serde(rename = "MasterUserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_name: Option<String>,
    /// <p>The key to use for matching the SAML Roles attribute.</p>
    #[serde(rename = "RolesKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles_key: Option<String>,
    /// <p>The duration, in minutes, after which a user session becomes inactive. Acceptable values are between 1 and 1440, and the default value is 60.</p>
    #[serde(rename = "SessionTimeoutMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_timeout_minutes: Option<i64>,
    /// <p>The key to use for matching the SAML Subject attribute.</p>
    #[serde(rename = "SubjectKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_key: Option<String>,
}

/// <p>Describes the SAML application configured for the domain.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SAMLOptionsOutput {
    /// <p>True if SAML is enabled.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>Describes the SAML Identity Provider's information.</p>
    #[serde(rename = "Idp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idp: Option<SAMLIdp>,
    /// <p>The key used for matching the SAML Roles attribute.</p>
    #[serde(rename = "RolesKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles_key: Option<String>,
    /// <p>The duration, in minutes, after which a user session becomes inactive.</p>
    #[serde(rename = "SessionTimeoutMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_timeout_minutes: Option<i64>,
    /// <p>The key used for matching the SAML Subject attribute.</p>
    #[serde(rename = "SubjectKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_key: Option<String>,
}

/// <p>The current options of an Elasticsearch domain service software options.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
    /// <p><code>True</code> if a service software is never automatically updated. <code>False</code> if a service software is automatically updated after <code>AutomatedUpdateDate</code>. </p>
    #[serde(rename = "OptionalDeployment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional_deployment: Option<bool>,
    /// <p><code>True</code> if you are able to update you service software version. <code>False</code> if you are not able to update your service software version. </p>
    #[serde(rename = "UpdateAvailable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_available: Option<bool>,
    /// <p>The status of your service software update. This field can take the following values: <code>ELIGIBLE</code>, <code>PENDING_UPDATE</code>, <code>IN_PROGRESS</code>, <code>COMPLETED</code>, and <code>NOT_ELIGIBLE</code>.</p>
    #[serde(rename = "UpdateStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_status: Option<DeploymentStatus>,
}

/// <p>Specifies the time, in UTC format, when the service takes a daily automated snapshot of the specified Elasticsearch domain. Default value is <code>0</code> hours.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SnapshotOptions {
    /// <p>Specifies the time, in UTC format, when the service takes a daily automated snapshot of the specified Elasticsearch domain. Default value is <code>0</code> hours.</p>
    #[serde(rename = "AutomatedSnapshotStartHour")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automated_snapshot_start_hour: Option<i64>,
}

/// <p>Status of a daily automated snapshot.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartElasticsearchServiceSoftwareUpdateRequest {
    /// <p>The name of the domain that you want to update to the latest service software.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
}

/// <p>The result of a <code>StartElasticsearchServiceSoftwareUpdate</code> operation. Contains the status of the update.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartElasticsearchServiceSoftwareUpdateResponse {
    /// <p>The current status of the Elasticsearch service software update.</p>
    #[serde(rename = "ServiceSoftwareOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_software_options: Option<ServiceSoftwareOptions>,
}

/// <p>StorageTypes represents the list of storage related types and their attributes that are available for given InstanceType. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownTLSSecurityPolicy {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum TLSSecurityPolicy {
    PolicyMinTLS10201907,
    PolicyMinTLS12201907,
    #[doc(hidden)]
    UnknownVariant(UnknownTLSSecurityPolicy),
}

impl Default for TLSSecurityPolicy {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for TLSSecurityPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for TLSSecurityPolicy {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for TLSSecurityPolicy {
    fn into(self) -> String {
        match self {
            TLSSecurityPolicy::PolicyMinTLS10201907 => "Policy-Min-TLS-1-0-2019-07".to_string(),
            TLSSecurityPolicy::PolicyMinTLS12201907 => "Policy-Min-TLS-1-2-2019-07".to_string(),
            TLSSecurityPolicy::UnknownVariant(UnknownTLSSecurityPolicy { name: original }) => {
                original
            }
        }
    }
}

impl<'a> Into<&'a str> for &'a TLSSecurityPolicy {
    fn into(self) -> &'a str {
        match self {
            TLSSecurityPolicy::PolicyMinTLS10201907 => &"Policy-Min-TLS-1-0-2019-07",
            TLSSecurityPolicy::PolicyMinTLS12201907 => &"Policy-Min-TLS-1-2-2019-07",
            TLSSecurityPolicy::UnknownVariant(UnknownTLSSecurityPolicy { name: original }) => {
                original
            }
        }
    }
}

impl From<&str> for TLSSecurityPolicy {
    fn from(name: &str) -> Self {
        match name {
            "Policy-Min-TLS-1-0-2019-07" => TLSSecurityPolicy::PolicyMinTLS10201907,
            "Policy-Min-TLS-1-2-2019-07" => TLSSecurityPolicy::PolicyMinTLS12201907,
            _ => TLSSecurityPolicy::UnknownVariant(UnknownTLSSecurityPolicy {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for TLSSecurityPolicy {
    fn from(name: String) -> Self {
        match &*name {
            "Policy-Min-TLS-1-0-2019-07" => TLSSecurityPolicy::PolicyMinTLS10201907,
            "Policy-Min-TLS-1-2-2019-07" => TLSSecurityPolicy::PolicyMinTLS12201907,
            _ => TLSSecurityPolicy::UnknownVariant(UnknownTLSSecurityPolicy { name }),
        }
    }
}

impl ::std::str::FromStr for TLSSecurityPolicy {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for TLSSecurityPolicy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for TLSSecurityPolicy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Specifies a key value pair for a resource tag.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Tag {
    /// <p>Specifies the <code>TagKey</code>, the name of the tag. Tag keys must be unique for the Elasticsearch domain to which they are attached.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>Specifies the <code>TagValue</code>, the value assigned to the corresponding tag key. Tag values can be null and do not have to be unique in a tag set. For example, you can have a key value pair in a tag set of <code>project : Trinity</code> and <code>cost-center : Trinity</code></p>
    #[serde(rename = "Value")]
    pub value: String,
}

/// <p>Container for the parameters to the <code><a>UpdateElasticsearchDomain</a></code> operation. Specifies the type and number of instances in the domain cluster.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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
    /// <p>Specifies advanced security options.</p>
    #[serde(rename = "AdvancedSecurityOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_security_options: Option<AdvancedSecurityOptionsInput>,
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
    pub log_publishing_options: Option<::std::collections::HashMap<LogType, LogPublishingOption>>,
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateElasticsearchDomainConfigResponse {
    /// <p>The status of the updated Elasticsearch domain. </p>
    #[serde(rename = "DomainConfig")]
    pub domain_config: ElasticsearchDomainConfig,
}

/// <p> Container for request parameters to <code> <a>UpdatePackage</a> </code> operation. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdatePackageRequest {
    /// <p>An info message for the new version which will be shown as part of <code>GetPackageVersionHistoryResponse</code>.</p>
    #[serde(rename = "CommitMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_message: Option<String>,
    /// <p>New description of the package.</p>
    #[serde(rename = "PackageDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_description: Option<String>,
    /// <p>Unique identifier for the package.</p>
    #[serde(rename = "PackageID")]
    pub package_id: String,
    #[serde(rename = "PackageSource")]
    pub package_source: PackageSource,
}

/// <p> Container for response returned by <code> <a>UpdatePackage</a> </code> operation. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdatePackageResponse {
    /// <p>Information about the package <code>PackageDetails</code>.</p>
    #[serde(rename = "PackageDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_details: Option<PackageDetails>,
}

/// <p> Container for request parameters to <code> <a>UpgradeElasticsearchDomain</a> </code> operation. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
    pub upgrade_status: Option<UpgradeStatus>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownUpgradeStatus {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum UpgradeStatus {
    Failed,
    InProgress,
    Succeeded,
    SucceededWithIssues,
    #[doc(hidden)]
    UnknownVariant(UnknownUpgradeStatus),
}

impl Default for UpgradeStatus {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for UpgradeStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for UpgradeStatus {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for UpgradeStatus {
    fn into(self) -> String {
        match self {
            UpgradeStatus::Failed => "FAILED".to_string(),
            UpgradeStatus::InProgress => "IN_PROGRESS".to_string(),
            UpgradeStatus::Succeeded => "SUCCEEDED".to_string(),
            UpgradeStatus::SucceededWithIssues => "SUCCEEDED_WITH_ISSUES".to_string(),
            UpgradeStatus::UnknownVariant(UnknownUpgradeStatus { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a UpgradeStatus {
    fn into(self) -> &'a str {
        match self {
            UpgradeStatus::Failed => &"FAILED",
            UpgradeStatus::InProgress => &"IN_PROGRESS",
            UpgradeStatus::Succeeded => &"SUCCEEDED",
            UpgradeStatus::SucceededWithIssues => &"SUCCEEDED_WITH_ISSUES",
            UpgradeStatus::UnknownVariant(UnknownUpgradeStatus { name: original }) => original,
        }
    }
}

impl From<&str> for UpgradeStatus {
    fn from(name: &str) -> Self {
        match name {
            "FAILED" => UpgradeStatus::Failed,
            "IN_PROGRESS" => UpgradeStatus::InProgress,
            "SUCCEEDED" => UpgradeStatus::Succeeded,
            "SUCCEEDED_WITH_ISSUES" => UpgradeStatus::SucceededWithIssues,
            _ => UpgradeStatus::UnknownVariant(UnknownUpgradeStatus {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for UpgradeStatus {
    fn from(name: String) -> Self {
        match &*name {
            "FAILED" => UpgradeStatus::Failed,
            "IN_PROGRESS" => UpgradeStatus::InProgress,
            "SUCCEEDED" => UpgradeStatus::Succeeded,
            "SUCCEEDED_WITH_ISSUES" => UpgradeStatus::SucceededWithIssues,
            _ => UpgradeStatus::UnknownVariant(UnknownUpgradeStatus { name }),
        }
    }
}

impl ::std::str::FromStr for UpgradeStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for UpgradeStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for UpgradeStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownUpgradeStep {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum UpgradeStep {
    PreUpgradeCheck,
    Snapshot,
    Upgrade,
    #[doc(hidden)]
    UnknownVariant(UnknownUpgradeStep),
}

impl Default for UpgradeStep {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for UpgradeStep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for UpgradeStep {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for UpgradeStep {
    fn into(self) -> String {
        match self {
            UpgradeStep::PreUpgradeCheck => "PRE_UPGRADE_CHECK".to_string(),
            UpgradeStep::Snapshot => "SNAPSHOT".to_string(),
            UpgradeStep::Upgrade => "UPGRADE".to_string(),
            UpgradeStep::UnknownVariant(UnknownUpgradeStep { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a UpgradeStep {
    fn into(self) -> &'a str {
        match self {
            UpgradeStep::PreUpgradeCheck => &"PRE_UPGRADE_CHECK",
            UpgradeStep::Snapshot => &"SNAPSHOT",
            UpgradeStep::Upgrade => &"UPGRADE",
            UpgradeStep::UnknownVariant(UnknownUpgradeStep { name: original }) => original,
        }
    }
}

impl From<&str> for UpgradeStep {
    fn from(name: &str) -> Self {
        match name {
            "PRE_UPGRADE_CHECK" => UpgradeStep::PreUpgradeCheck,
            "SNAPSHOT" => UpgradeStep::Snapshot,
            "UPGRADE" => UpgradeStep::Upgrade,
            _ => UpgradeStep::UnknownVariant(UnknownUpgradeStep {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for UpgradeStep {
    fn from(name: String) -> Self {
        match &*name {
            "PRE_UPGRADE_CHECK" => UpgradeStep::PreUpgradeCheck,
            "SNAPSHOT" => UpgradeStep::Snapshot,
            "UPGRADE" => UpgradeStep::Upgrade,
            _ => UpgradeStep::UnknownVariant(UnknownUpgradeStep { name }),
        }
    }
}

impl ::std::str::FromStr for UpgradeStep {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for UpgradeStep {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for UpgradeStep {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Represents a single step of the Upgrade or Upgrade Eligibility Check workflow.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
    pub upgrade_step: Option<UpgradeStep>,
    /// <p> The status of a particular step during an upgrade. The status can take one of the following values: <ul> <li>In Progress</li> <li>Succeeded</li> <li>Succeeded with Issues</li> <li>Failed</li> </ul> </p>
    #[serde(rename = "UpgradeStepStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgrade_step_status: Option<UpgradeStatus>,
}

/// <p>Options to specify the subnets and security groups for VPC endpoint. For more information, see <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-vpc.html" target="_blank"> VPC Endpoints for Amazon Elasticsearch Service Domains</a>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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

/// <p> The type of EBS volume, standard, gp2, or io1. See <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-createupdatedomains.html#es-createdomain-configure-ebs" target="_blank">Configuring EBS-based Storage</a>for more information.</p>

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownVolumeType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum VolumeType {
    Gp2,
    Io1,
    Standard,
    #[doc(hidden)]
    UnknownVariant(UnknownVolumeType),
}

impl Default for VolumeType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for VolumeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for VolumeType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for VolumeType {
    fn into(self) -> String {
        match self {
            VolumeType::Gp2 => "gp2".to_string(),
            VolumeType::Io1 => "io1".to_string(),
            VolumeType::Standard => "standard".to_string(),
            VolumeType::UnknownVariant(UnknownVolumeType { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a VolumeType {
    fn into(self) -> &'a str {
        match self {
            VolumeType::Gp2 => &"gp2",
            VolumeType::Io1 => &"io1",
            VolumeType::Standard => &"standard",
            VolumeType::UnknownVariant(UnknownVolumeType { name: original }) => original,
        }
    }
}

impl From<&str> for VolumeType {
    fn from(name: &str) -> Self {
        match name {
            "gp2" => VolumeType::Gp2,
            "io1" => VolumeType::Io1,
            "standard" => VolumeType::Standard,
            _ => VolumeType::UnknownVariant(UnknownVolumeType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for VolumeType {
    fn from(name: String) -> Self {
        match &*name {
            "gp2" => VolumeType::Gp2,
            "io1" => VolumeType::Io1,
            "standard" => VolumeType::Standard,
            _ => VolumeType::UnknownVariant(UnknownVolumeType { name }),
        }
    }
}

impl ::std::str::FromStr for VolumeType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for VolumeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for VolumeType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Specifies the zone awareness configuration for the domain cluster, such as the number of availability zones.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ZoneAwarenessConfig {
    /// <p>An integer value to indicate the number of availability zones for a domain when zone awareness is enabled. This should be equal to number of subnets if VPC endpoints is enabled</p>
    #[serde(rename = "AvailabilityZoneCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone_count: Option<i64>,
}

/// Errors returned by AcceptInboundCrossClusterSearchConnection
#[derive(Debug, PartialEq)]
pub enum AcceptInboundCrossClusterSearchConnectionError {
    /// <p>An error occured because the client wanted to access a not supported operation. Gives http status code of 409.</p>
    DisabledOperation(String),
    /// <p>An exception for trying to create more than allowed resources or sub-resources. Gives http status code of 409.</p>
    LimitExceeded(String),
    /// <p>An exception for accessing or deleting a resource that does not exist. Gives http status code of 400.</p>
    ResourceNotFound(String),
}

impl AcceptInboundCrossClusterSearchConnectionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AcceptInboundCrossClusterSearchConnectionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "DisabledOperationException" => {
                    return RusotoError::Service(
                        AcceptInboundCrossClusterSearchConnectionError::DisabledOperation(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(
                        AcceptInboundCrossClusterSearchConnectionError::LimitExceeded(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        AcceptInboundCrossClusterSearchConnectionError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AcceptInboundCrossClusterSearchConnectionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AcceptInboundCrossClusterSearchConnectionError::DisabledOperation(ref cause) => {
                write!(f, "{}", cause)
            }
            AcceptInboundCrossClusterSearchConnectionError::LimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            AcceptInboundCrossClusterSearchConnectionError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for AcceptInboundCrossClusterSearchConnectionError {}
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
/// Errors returned by AssociatePackage
#[derive(Debug, PartialEq)]
pub enum AssociatePackageError {
    /// <p>An error occurred because user does not have permissions to access the resource. Returns HTTP status code 403.</p>
    AccessDenied(String),
    /// <p>An error occurred while processing the request.</p>
    Base(String),
    /// <p>An error occurred because the client attempts to remove a resource that is currently in use. Returns HTTP status code 409.</p>
    Conflict(String),
    /// <p>The request processing has failed because of an unknown error, exception or failure (the failure is internal to the service) . Gives http status code of 500.</p>
    Internal(String),
    /// <p>An exception for accessing or deleting a resource that does not exist. Gives http status code of 400.</p>
    ResourceNotFound(String),
}

impl AssociatePackageError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AssociatePackageError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(AssociatePackageError::AccessDenied(err.msg))
                }
                "BaseException" => {
                    return RusotoError::Service(AssociatePackageError::Base(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(AssociatePackageError::Conflict(err.msg))
                }
                "InternalException" => {
                    return RusotoError::Service(AssociatePackageError::Internal(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AssociatePackageError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AssociatePackageError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociatePackageError::AccessDenied(ref cause) => write!(f, "{}", cause),
            AssociatePackageError::Base(ref cause) => write!(f, "{}", cause),
            AssociatePackageError::Conflict(ref cause) => write!(f, "{}", cause),
            AssociatePackageError::Internal(ref cause) => write!(f, "{}", cause),
            AssociatePackageError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AssociatePackageError {}
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
/// Errors returned by CreateOutboundCrossClusterSearchConnection
#[derive(Debug, PartialEq)]
pub enum CreateOutboundCrossClusterSearchConnectionError {
    /// <p>An error occured because the client wanted to access a not supported operation. Gives http status code of 409.</p>
    DisabledOperation(String),
    /// <p>The request processing has failed because of an unknown error, exception or failure (the failure is internal to the service) . Gives http status code of 500.</p>
    Internal(String),
    /// <p>An exception for trying to create more than allowed resources or sub-resources. Gives http status code of 409.</p>
    LimitExceeded(String),
    /// <p>An exception for creating a resource that already exists. Gives http status code of 400.</p>
    ResourceAlreadyExists(String),
}

impl CreateOutboundCrossClusterSearchConnectionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateOutboundCrossClusterSearchConnectionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "DisabledOperationException" => {
                    return RusotoError::Service(
                        CreateOutboundCrossClusterSearchConnectionError::DisabledOperation(err.msg),
                    )
                }
                "InternalException" => {
                    return RusotoError::Service(
                        CreateOutboundCrossClusterSearchConnectionError::Internal(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(
                        CreateOutboundCrossClusterSearchConnectionError::LimitExceeded(err.msg),
                    )
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(
                        CreateOutboundCrossClusterSearchConnectionError::ResourceAlreadyExists(
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
impl fmt::Display for CreateOutboundCrossClusterSearchConnectionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateOutboundCrossClusterSearchConnectionError::DisabledOperation(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateOutboundCrossClusterSearchConnectionError::Internal(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateOutboundCrossClusterSearchConnectionError::LimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateOutboundCrossClusterSearchConnectionError::ResourceAlreadyExists(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateOutboundCrossClusterSearchConnectionError {}
/// Errors returned by CreatePackage
#[derive(Debug, PartialEq)]
pub enum CreatePackageError {
    /// <p>An error occurred because user does not have permissions to access the resource. Returns HTTP status code 403.</p>
    AccessDenied(String),
    /// <p>An error occurred while processing the request.</p>
    Base(String),
    /// <p>The request processing has failed because of an unknown error, exception or failure (the failure is internal to the service) . Gives http status code of 500.</p>
    Internal(String),
    /// <p>An exception for trying to create or access sub-resource that is either invalid or not supported. Gives http status code of 409.</p>
    InvalidType(String),
    /// <p>An exception for trying to create more than allowed resources or sub-resources. Gives http status code of 409.</p>
    LimitExceeded(String),
    /// <p>An exception for creating a resource that already exists. Gives http status code of 400.</p>
    ResourceAlreadyExists(String),
}

impl CreatePackageError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreatePackageError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreatePackageError::AccessDenied(err.msg))
                }
                "BaseException" => return RusotoError::Service(CreatePackageError::Base(err.msg)),
                "InternalException" => {
                    return RusotoError::Service(CreatePackageError::Internal(err.msg))
                }
                "InvalidTypeException" => {
                    return RusotoError::Service(CreatePackageError::InvalidType(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreatePackageError::LimitExceeded(err.msg))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(CreatePackageError::ResourceAlreadyExists(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreatePackageError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreatePackageError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreatePackageError::Base(ref cause) => write!(f, "{}", cause),
            CreatePackageError::Internal(ref cause) => write!(f, "{}", cause),
            CreatePackageError::InvalidType(ref cause) => write!(f, "{}", cause),
            CreatePackageError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreatePackageError::ResourceAlreadyExists(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreatePackageError {}
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
/// Errors returned by DeleteInboundCrossClusterSearchConnection
#[derive(Debug, PartialEq)]
pub enum DeleteInboundCrossClusterSearchConnectionError {
    /// <p>An error occured because the client wanted to access a not supported operation. Gives http status code of 409.</p>
    DisabledOperation(String),
    /// <p>An exception for accessing or deleting a resource that does not exist. Gives http status code of 400.</p>
    ResourceNotFound(String),
}

impl DeleteInboundCrossClusterSearchConnectionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteInboundCrossClusterSearchConnectionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "DisabledOperationException" => {
                    return RusotoError::Service(
                        DeleteInboundCrossClusterSearchConnectionError::DisabledOperation(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DeleteInboundCrossClusterSearchConnectionError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteInboundCrossClusterSearchConnectionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteInboundCrossClusterSearchConnectionError::DisabledOperation(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteInboundCrossClusterSearchConnectionError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteInboundCrossClusterSearchConnectionError {}
/// Errors returned by DeleteOutboundCrossClusterSearchConnection
#[derive(Debug, PartialEq)]
pub enum DeleteOutboundCrossClusterSearchConnectionError {
    /// <p>An error occured because the client wanted to access a not supported operation. Gives http status code of 409.</p>
    DisabledOperation(String),
    /// <p>An exception for accessing or deleting a resource that does not exist. Gives http status code of 400.</p>
    ResourceNotFound(String),
}

impl DeleteOutboundCrossClusterSearchConnectionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteOutboundCrossClusterSearchConnectionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "DisabledOperationException" => {
                    return RusotoError::Service(
                        DeleteOutboundCrossClusterSearchConnectionError::DisabledOperation(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DeleteOutboundCrossClusterSearchConnectionError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteOutboundCrossClusterSearchConnectionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteOutboundCrossClusterSearchConnectionError::DisabledOperation(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteOutboundCrossClusterSearchConnectionError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteOutboundCrossClusterSearchConnectionError {}
/// Errors returned by DeletePackage
#[derive(Debug, PartialEq)]
pub enum DeletePackageError {
    /// <p>An error occurred because user does not have permissions to access the resource. Returns HTTP status code 403.</p>
    AccessDenied(String),
    /// <p>An error occurred while processing the request.</p>
    Base(String),
    /// <p>An error occurred because the client attempts to remove a resource that is currently in use. Returns HTTP status code 409.</p>
    Conflict(String),
    /// <p>The request processing has failed because of an unknown error, exception or failure (the failure is internal to the service) . Gives http status code of 500.</p>
    Internal(String),
    /// <p>An exception for accessing or deleting a resource that does not exist. Gives http status code of 400.</p>
    ResourceNotFound(String),
}

impl DeletePackageError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeletePackageError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeletePackageError::AccessDenied(err.msg))
                }
                "BaseException" => return RusotoError::Service(DeletePackageError::Base(err.msg)),
                "ConflictException" => {
                    return RusotoError::Service(DeletePackageError::Conflict(err.msg))
                }
                "InternalException" => {
                    return RusotoError::Service(DeletePackageError::Internal(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeletePackageError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeletePackageError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeletePackageError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeletePackageError::Base(ref cause) => write!(f, "{}", cause),
            DeletePackageError::Conflict(ref cause) => write!(f, "{}", cause),
            DeletePackageError::Internal(ref cause) => write!(f, "{}", cause),
            DeletePackageError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeletePackageError {}
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
/// Errors returned by DescribeInboundCrossClusterSearchConnections
#[derive(Debug, PartialEq)]
pub enum DescribeInboundCrossClusterSearchConnectionsError {
    /// <p>An error occured because the client wanted to access a not supported operation. Gives http status code of 409.</p>
    DisabledOperation(String),
    /// <p>The request processing has failed because of invalid pagination token provided by customer. Returns an HTTP status code of 400. </p>
    InvalidPaginationToken(String),
}

impl DescribeInboundCrossClusterSearchConnectionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeInboundCrossClusterSearchConnectionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "DisabledOperationException" => {
                    return RusotoError::Service(
                        DescribeInboundCrossClusterSearchConnectionsError::DisabledOperation(
                            err.msg,
                        ),
                    )
                }
                "InvalidPaginationTokenException" => {
                    return RusotoError::Service(
                        DescribeInboundCrossClusterSearchConnectionsError::InvalidPaginationToken(
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
impl fmt::Display for DescribeInboundCrossClusterSearchConnectionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeInboundCrossClusterSearchConnectionsError::DisabledOperation(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeInboundCrossClusterSearchConnectionsError::InvalidPaginationToken(
                ref cause,
            ) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeInboundCrossClusterSearchConnectionsError {}
/// Errors returned by DescribeOutboundCrossClusterSearchConnections
#[derive(Debug, PartialEq)]
pub enum DescribeOutboundCrossClusterSearchConnectionsError {
    /// <p>An error occured because the client wanted to access a not supported operation. Gives http status code of 409.</p>
    DisabledOperation(String),
    /// <p>The request processing has failed because of invalid pagination token provided by customer. Returns an HTTP status code of 400. </p>
    InvalidPaginationToken(String),
}

impl DescribeOutboundCrossClusterSearchConnectionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeOutboundCrossClusterSearchConnectionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "DisabledOperationException" => {
                    return RusotoError::Service(
                        DescribeOutboundCrossClusterSearchConnectionsError::DisabledOperation(
                            err.msg,
                        ),
                    )
                }
                "InvalidPaginationTokenException" => {
                    return RusotoError::Service(
                        DescribeOutboundCrossClusterSearchConnectionsError::InvalidPaginationToken(
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
impl fmt::Display for DescribeOutboundCrossClusterSearchConnectionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeOutboundCrossClusterSearchConnectionsError::DisabledOperation(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeOutboundCrossClusterSearchConnectionsError::InvalidPaginationToken(
                ref cause,
            ) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeOutboundCrossClusterSearchConnectionsError {}
/// Errors returned by DescribePackages
#[derive(Debug, PartialEq)]
pub enum DescribePackagesError {
    /// <p>An error occurred because user does not have permissions to access the resource. Returns HTTP status code 403.</p>
    AccessDenied(String),
    /// <p>An error occurred while processing the request.</p>
    Base(String),
    /// <p>The request processing has failed because of an unknown error, exception or failure (the failure is internal to the service) . Gives http status code of 500.</p>
    Internal(String),
    /// <p>An exception for accessing or deleting a resource that does not exist. Gives http status code of 400.</p>
    ResourceNotFound(String),
}

impl DescribePackagesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribePackagesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribePackagesError::AccessDenied(err.msg))
                }
                "BaseException" => {
                    return RusotoError::Service(DescribePackagesError::Base(err.msg))
                }
                "InternalException" => {
                    return RusotoError::Service(DescribePackagesError::Internal(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribePackagesError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribePackagesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribePackagesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DescribePackagesError::Base(ref cause) => write!(f, "{}", cause),
            DescribePackagesError::Internal(ref cause) => write!(f, "{}", cause),
            DescribePackagesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribePackagesError {}
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
/// Errors returned by DissociatePackage
#[derive(Debug, PartialEq)]
pub enum DissociatePackageError {
    /// <p>An error occurred because user does not have permissions to access the resource. Returns HTTP status code 403.</p>
    AccessDenied(String),
    /// <p>An error occurred while processing the request.</p>
    Base(String),
    /// <p>An error occurred because the client attempts to remove a resource that is currently in use. Returns HTTP status code 409.</p>
    Conflict(String),
    /// <p>The request processing has failed because of an unknown error, exception or failure (the failure is internal to the service) . Gives http status code of 500.</p>
    Internal(String),
    /// <p>An exception for accessing or deleting a resource that does not exist. Gives http status code of 400.</p>
    ResourceNotFound(String),
}

impl DissociatePackageError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DissociatePackageError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DissociatePackageError::AccessDenied(err.msg))
                }
                "BaseException" => {
                    return RusotoError::Service(DissociatePackageError::Base(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DissociatePackageError::Conflict(err.msg))
                }
                "InternalException" => {
                    return RusotoError::Service(DissociatePackageError::Internal(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DissociatePackageError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DissociatePackageError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DissociatePackageError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DissociatePackageError::Base(ref cause) => write!(f, "{}", cause),
            DissociatePackageError::Conflict(ref cause) => write!(f, "{}", cause),
            DissociatePackageError::Internal(ref cause) => write!(f, "{}", cause),
            DissociatePackageError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DissociatePackageError {}
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
/// Errors returned by GetPackageVersionHistory
#[derive(Debug, PartialEq)]
pub enum GetPackageVersionHistoryError {
    /// <p>An error occurred because user does not have permissions to access the resource. Returns HTTP status code 403.</p>
    AccessDenied(String),
    /// <p>An error occurred while processing the request.</p>
    Base(String),
    /// <p>The request processing has failed because of an unknown error, exception or failure (the failure is internal to the service) . Gives http status code of 500.</p>
    Internal(String),
    /// <p>An exception for accessing or deleting a resource that does not exist. Gives http status code of 400.</p>
    ResourceNotFound(String),
}

impl GetPackageVersionHistoryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetPackageVersionHistoryError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetPackageVersionHistoryError::AccessDenied(
                        err.msg,
                    ))
                }
                "BaseException" => {
                    return RusotoError::Service(GetPackageVersionHistoryError::Base(err.msg))
                }
                "InternalException" => {
                    return RusotoError::Service(GetPackageVersionHistoryError::Internal(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetPackageVersionHistoryError::ResourceNotFound(
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
impl fmt::Display for GetPackageVersionHistoryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetPackageVersionHistoryError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetPackageVersionHistoryError::Base(ref cause) => write!(f, "{}", cause),
            GetPackageVersionHistoryError::Internal(ref cause) => write!(f, "{}", cause),
            GetPackageVersionHistoryError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetPackageVersionHistoryError {}
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
/// Errors returned by ListDomainsForPackage
#[derive(Debug, PartialEq)]
pub enum ListDomainsForPackageError {
    /// <p>An error occurred because user does not have permissions to access the resource. Returns HTTP status code 403.</p>
    AccessDenied(String),
    /// <p>An error occurred while processing the request.</p>
    Base(String),
    /// <p>The request processing has failed because of an unknown error, exception or failure (the failure is internal to the service) . Gives http status code of 500.</p>
    Internal(String),
    /// <p>An exception for accessing or deleting a resource that does not exist. Gives http status code of 400.</p>
    ResourceNotFound(String),
}

impl ListDomainsForPackageError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDomainsForPackageError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListDomainsForPackageError::AccessDenied(err.msg))
                }
                "BaseException" => {
                    return RusotoError::Service(ListDomainsForPackageError::Base(err.msg))
                }
                "InternalException" => {
                    return RusotoError::Service(ListDomainsForPackageError::Internal(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListDomainsForPackageError::ResourceNotFound(
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
impl fmt::Display for ListDomainsForPackageError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDomainsForPackageError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListDomainsForPackageError::Base(ref cause) => write!(f, "{}", cause),
            ListDomainsForPackageError::Internal(ref cause) => write!(f, "{}", cause),
            ListDomainsForPackageError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListDomainsForPackageError {}
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
/// Errors returned by ListPackagesForDomain
#[derive(Debug, PartialEq)]
pub enum ListPackagesForDomainError {
    /// <p>An error occurred because user does not have permissions to access the resource. Returns HTTP status code 403.</p>
    AccessDenied(String),
    /// <p>An error occurred while processing the request.</p>
    Base(String),
    /// <p>The request processing has failed because of an unknown error, exception or failure (the failure is internal to the service) . Gives http status code of 500.</p>
    Internal(String),
    /// <p>An exception for accessing or deleting a resource that does not exist. Gives http status code of 400.</p>
    ResourceNotFound(String),
}

impl ListPackagesForDomainError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListPackagesForDomainError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListPackagesForDomainError::AccessDenied(err.msg))
                }
                "BaseException" => {
                    return RusotoError::Service(ListPackagesForDomainError::Base(err.msg))
                }
                "InternalException" => {
                    return RusotoError::Service(ListPackagesForDomainError::Internal(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListPackagesForDomainError::ResourceNotFound(
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
impl fmt::Display for ListPackagesForDomainError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListPackagesForDomainError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListPackagesForDomainError::Base(ref cause) => write!(f, "{}", cause),
            ListPackagesForDomainError::Internal(ref cause) => write!(f, "{}", cause),
            ListPackagesForDomainError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListPackagesForDomainError {}
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
/// Errors returned by RejectInboundCrossClusterSearchConnection
#[derive(Debug, PartialEq)]
pub enum RejectInboundCrossClusterSearchConnectionError {
    /// <p>An error occured because the client wanted to access a not supported operation. Gives http status code of 409.</p>
    DisabledOperation(String),
    /// <p>An exception for accessing or deleting a resource that does not exist. Gives http status code of 400.</p>
    ResourceNotFound(String),
}

impl RejectInboundCrossClusterSearchConnectionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<RejectInboundCrossClusterSearchConnectionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "DisabledOperationException" => {
                    return RusotoError::Service(
                        RejectInboundCrossClusterSearchConnectionError::DisabledOperation(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        RejectInboundCrossClusterSearchConnectionError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RejectInboundCrossClusterSearchConnectionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RejectInboundCrossClusterSearchConnectionError::DisabledOperation(ref cause) => {
                write!(f, "{}", cause)
            }
            RejectInboundCrossClusterSearchConnectionError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for RejectInboundCrossClusterSearchConnectionError {}
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
/// Errors returned by UpdatePackage
#[derive(Debug, PartialEq)]
pub enum UpdatePackageError {
    /// <p>An error occurred because user does not have permissions to access the resource. Returns HTTP status code 403.</p>
    AccessDenied(String),
    /// <p>An error occurred while processing the request.</p>
    Base(String),
    /// <p>The request processing has failed because of an unknown error, exception or failure (the failure is internal to the service) . Gives http status code of 500.</p>
    Internal(String),
    /// <p>An exception for trying to create more than allowed resources or sub-resources. Gives http status code of 409.</p>
    LimitExceeded(String),
    /// <p>An exception for accessing or deleting a resource that does not exist. Gives http status code of 400.</p>
    ResourceNotFound(String),
}

impl UpdatePackageError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdatePackageError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdatePackageError::AccessDenied(err.msg))
                }
                "BaseException" => return RusotoError::Service(UpdatePackageError::Base(err.msg)),
                "InternalException" => {
                    return RusotoError::Service(UpdatePackageError::Internal(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdatePackageError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdatePackageError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdatePackageError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdatePackageError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdatePackageError::Base(ref cause) => write!(f, "{}", cause),
            UpdatePackageError::Internal(ref cause) => write!(f, "{}", cause),
            UpdatePackageError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdatePackageError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdatePackageError {}
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
#[async_trait]
pub trait Es {
    /// <p>Allows the destination domain owner to accept an inbound cross-cluster search connection request.</p>
    async fn accept_inbound_cross_cluster_search_connection(
        &self,
        input: AcceptInboundCrossClusterSearchConnectionRequest,
    ) -> Result<
        AcceptInboundCrossClusterSearchConnectionResponse,
        RusotoError<AcceptInboundCrossClusterSearchConnectionError>,
    >;

    /// <p>Attaches tags to an existing Elasticsearch domain. Tags are a set of case-sensitive key value pairs. An Elasticsearch domain may have up to 10 tags. See <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-managedomains.html#es-managedomains-awsresorcetagging" target="_blank"> Tagging Amazon Elasticsearch Service Domains for more information.</a></p>
    async fn add_tags(&self, input: AddTagsRequest) -> Result<(), RusotoError<AddTagsError>>;

    /// <p>Associates a package with an Amazon ES domain.</p>
    async fn associate_package(
        &self,
        input: AssociatePackageRequest,
    ) -> Result<AssociatePackageResponse, RusotoError<AssociatePackageError>>;

    /// <p>Cancels a scheduled service software update for an Amazon ES domain. You can only perform this operation before the <code>AutomatedUpdateDate</code> and when the <code>UpdateStatus</code> is in the <code>PENDING_UPDATE</code> state.</p>
    async fn cancel_elasticsearch_service_software_update(
        &self,
        input: CancelElasticsearchServiceSoftwareUpdateRequest,
    ) -> Result<
        CancelElasticsearchServiceSoftwareUpdateResponse,
        RusotoError<CancelElasticsearchServiceSoftwareUpdateError>,
    >;

    /// <p>Creates a new Elasticsearch domain. For more information, see <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-createupdatedomains.html#es-createdomains" target="_blank">Creating Elasticsearch Domains</a> in the <i>Amazon Elasticsearch Service Developer Guide</i>.</p>
    async fn create_elasticsearch_domain(
        &self,
        input: CreateElasticsearchDomainRequest,
    ) -> Result<CreateElasticsearchDomainResponse, RusotoError<CreateElasticsearchDomainError>>;

    /// <p>Creates a new cross-cluster search connection from a source domain to a destination domain.</p>
    async fn create_outbound_cross_cluster_search_connection(
        &self,
        input: CreateOutboundCrossClusterSearchConnectionRequest,
    ) -> Result<
        CreateOutboundCrossClusterSearchConnectionResponse,
        RusotoError<CreateOutboundCrossClusterSearchConnectionError>,
    >;

    /// <p>Create a package for use with Amazon ES domains.</p>
    async fn create_package(
        &self,
        input: CreatePackageRequest,
    ) -> Result<CreatePackageResponse, RusotoError<CreatePackageError>>;

    /// <p>Permanently deletes the specified Elasticsearch domain and all of its data. Once a domain is deleted, it cannot be recovered.</p>
    async fn delete_elasticsearch_domain(
        &self,
        input: DeleteElasticsearchDomainRequest,
    ) -> Result<DeleteElasticsearchDomainResponse, RusotoError<DeleteElasticsearchDomainError>>;

    /// <p>Deletes the service-linked role that Elasticsearch Service uses to manage and maintain VPC domains. Role deletion will fail if any existing VPC domains use the role. You must delete any such Elasticsearch domains before deleting the role. See <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-vpc.html#es-enabling-slr" target="_blank">Deleting Elasticsearch Service Role</a> in <i>VPC Endpoints for Amazon Elasticsearch Service Domains</i>.</p>
    async fn delete_elasticsearch_service_role(
        &self,
    ) -> Result<(), RusotoError<DeleteElasticsearchServiceRoleError>>;

    /// <p>Allows the destination domain owner to delete an existing inbound cross-cluster search connection.</p>
    async fn delete_inbound_cross_cluster_search_connection(
        &self,
        input: DeleteInboundCrossClusterSearchConnectionRequest,
    ) -> Result<
        DeleteInboundCrossClusterSearchConnectionResponse,
        RusotoError<DeleteInboundCrossClusterSearchConnectionError>,
    >;

    /// <p>Allows the source domain owner to delete an existing outbound cross-cluster search connection.</p>
    async fn delete_outbound_cross_cluster_search_connection(
        &self,
        input: DeleteOutboundCrossClusterSearchConnectionRequest,
    ) -> Result<
        DeleteOutboundCrossClusterSearchConnectionResponse,
        RusotoError<DeleteOutboundCrossClusterSearchConnectionError>,
    >;

    /// <p>Delete the package.</p>
    async fn delete_package(
        &self,
        input: DeletePackageRequest,
    ) -> Result<DeletePackageResponse, RusotoError<DeletePackageError>>;

    /// <p>Returns domain configuration information about the specified Elasticsearch domain, including the domain ID, domain endpoint, and domain ARN.</p>
    async fn describe_elasticsearch_domain(
        &self,
        input: DescribeElasticsearchDomainRequest,
    ) -> Result<DescribeElasticsearchDomainResponse, RusotoError<DescribeElasticsearchDomainError>>;

    /// <p>Provides cluster configuration information about the specified Elasticsearch domain, such as the state, creation date, update version, and update date for cluster options.</p>
    async fn describe_elasticsearch_domain_config(
        &self,
        input: DescribeElasticsearchDomainConfigRequest,
    ) -> Result<
        DescribeElasticsearchDomainConfigResponse,
        RusotoError<DescribeElasticsearchDomainConfigError>,
    >;

    /// <p>Returns domain configuration information about the specified Elasticsearch domains, including the domain ID, domain endpoint, and domain ARN.</p>
    async fn describe_elasticsearch_domains(
        &self,
        input: DescribeElasticsearchDomainsRequest,
    ) -> Result<DescribeElasticsearchDomainsResponse, RusotoError<DescribeElasticsearchDomainsError>>;

    /// <p> Describe Elasticsearch Limits for a given InstanceType and ElasticsearchVersion. When modifying existing Domain, specify the <code> <a>DomainName</a> </code> to know what Limits are supported for modifying. </p>
    async fn describe_elasticsearch_instance_type_limits(
        &self,
        input: DescribeElasticsearchInstanceTypeLimitsRequest,
    ) -> Result<
        DescribeElasticsearchInstanceTypeLimitsResponse,
        RusotoError<DescribeElasticsearchInstanceTypeLimitsError>,
    >;

    /// <p>Lists all the inbound cross-cluster search connections for a destination domain.</p>
    async fn describe_inbound_cross_cluster_search_connections(
        &self,
        input: DescribeInboundCrossClusterSearchConnectionsRequest,
    ) -> Result<
        DescribeInboundCrossClusterSearchConnectionsResponse,
        RusotoError<DescribeInboundCrossClusterSearchConnectionsError>,
    >;

    /// <p>Lists all the outbound cross-cluster search connections for a source domain.</p>
    async fn describe_outbound_cross_cluster_search_connections(
        &self,
        input: DescribeOutboundCrossClusterSearchConnectionsRequest,
    ) -> Result<
        DescribeOutboundCrossClusterSearchConnectionsResponse,
        RusotoError<DescribeOutboundCrossClusterSearchConnectionsError>,
    >;

    /// <p>Describes all packages available to Amazon ES. Includes options for filtering, limiting the number of results, and pagination.</p>
    async fn describe_packages(
        &self,
        input: DescribePackagesRequest,
    ) -> Result<DescribePackagesResponse, RusotoError<DescribePackagesError>>;

    /// <p>Lists available reserved Elasticsearch instance offerings.</p>
    async fn describe_reserved_elasticsearch_instance_offerings(
        &self,
        input: DescribeReservedElasticsearchInstanceOfferingsRequest,
    ) -> Result<
        DescribeReservedElasticsearchInstanceOfferingsResponse,
        RusotoError<DescribeReservedElasticsearchInstanceOfferingsError>,
    >;

    /// <p>Returns information about reserved Elasticsearch instances for this account.</p>
    async fn describe_reserved_elasticsearch_instances(
        &self,
        input: DescribeReservedElasticsearchInstancesRequest,
    ) -> Result<
        DescribeReservedElasticsearchInstancesResponse,
        RusotoError<DescribeReservedElasticsearchInstancesError>,
    >;

    /// <p>Dissociates a package from the Amazon ES domain.</p>
    async fn dissociate_package(
        &self,
        input: DissociatePackageRequest,
    ) -> Result<DissociatePackageResponse, RusotoError<DissociatePackageError>>;

    /// <p> Returns a list of upgrade compatible Elastisearch versions. You can optionally pass a <code> <a>DomainName</a> </code> to get all upgrade compatible Elasticsearch versions for that specific domain. </p>
    async fn get_compatible_elasticsearch_versions(
        &self,
        input: GetCompatibleElasticsearchVersionsRequest,
    ) -> Result<
        GetCompatibleElasticsearchVersionsResponse,
        RusotoError<GetCompatibleElasticsearchVersionsError>,
    >;

    /// <p>Returns a list of versions of the package, along with their creation time and commit message.</p>
    async fn get_package_version_history(
        &self,
        input: GetPackageVersionHistoryRequest,
    ) -> Result<GetPackageVersionHistoryResponse, RusotoError<GetPackageVersionHistoryError>>;

    /// <p>Retrieves the complete history of the last 10 upgrades that were performed on the domain.</p>
    async fn get_upgrade_history(
        &self,
        input: GetUpgradeHistoryRequest,
    ) -> Result<GetUpgradeHistoryResponse, RusotoError<GetUpgradeHistoryError>>;

    /// <p>Retrieves the latest status of the last upgrade or upgrade eligibility check that was performed on the domain.</p>
    async fn get_upgrade_status(
        &self,
        input: GetUpgradeStatusRequest,
    ) -> Result<GetUpgradeStatusResponse, RusotoError<GetUpgradeStatusError>>;

    /// <p>Returns the name of all Elasticsearch domains owned by the current user's account. </p>
    async fn list_domain_names(
        &self,
    ) -> Result<ListDomainNamesResponse, RusotoError<ListDomainNamesError>>;

    /// <p>Lists all Amazon ES domains associated with the package.</p>
    async fn list_domains_for_package(
        &self,
        input: ListDomainsForPackageRequest,
    ) -> Result<ListDomainsForPackageResponse, RusotoError<ListDomainsForPackageError>>;

    /// <p>List all Elasticsearch instance types that are supported for given ElasticsearchVersion</p>
    async fn list_elasticsearch_instance_types(
        &self,
        input: ListElasticsearchInstanceTypesRequest,
    ) -> Result<
        ListElasticsearchInstanceTypesResponse,
        RusotoError<ListElasticsearchInstanceTypesError>,
    >;

    /// <p>List all supported Elasticsearch versions</p>
    async fn list_elasticsearch_versions(
        &self,
        input: ListElasticsearchVersionsRequest,
    ) -> Result<ListElasticsearchVersionsResponse, RusotoError<ListElasticsearchVersionsError>>;

    /// <p>Lists all packages associated with the Amazon ES domain.</p>
    async fn list_packages_for_domain(
        &self,
        input: ListPackagesForDomainRequest,
    ) -> Result<ListPackagesForDomainResponse, RusotoError<ListPackagesForDomainError>>;

    /// <p>Returns all tags for the given Elasticsearch domain.</p>
    async fn list_tags(
        &self,
        input: ListTagsRequest,
    ) -> Result<ListTagsResponse, RusotoError<ListTagsError>>;

    /// <p>Allows you to purchase reserved Elasticsearch instances.</p>
    async fn purchase_reserved_elasticsearch_instance_offering(
        &self,
        input: PurchaseReservedElasticsearchInstanceOfferingRequest,
    ) -> Result<
        PurchaseReservedElasticsearchInstanceOfferingResponse,
        RusotoError<PurchaseReservedElasticsearchInstanceOfferingError>,
    >;

    /// <p>Allows the destination domain owner to reject an inbound cross-cluster search connection request.</p>
    async fn reject_inbound_cross_cluster_search_connection(
        &self,
        input: RejectInboundCrossClusterSearchConnectionRequest,
    ) -> Result<
        RejectInboundCrossClusterSearchConnectionResponse,
        RusotoError<RejectInboundCrossClusterSearchConnectionError>,
    >;

    /// <p>Removes the specified set of tags from the specified Elasticsearch domain.</p>
    async fn remove_tags(
        &self,
        input: RemoveTagsRequest,
    ) -> Result<(), RusotoError<RemoveTagsError>>;

    /// <p>Schedules a service software update for an Amazon ES domain.</p>
    async fn start_elasticsearch_service_software_update(
        &self,
        input: StartElasticsearchServiceSoftwareUpdateRequest,
    ) -> Result<
        StartElasticsearchServiceSoftwareUpdateResponse,
        RusotoError<StartElasticsearchServiceSoftwareUpdateError>,
    >;

    /// <p>Modifies the cluster configuration of the specified Elasticsearch domain, setting as setting the instance type and the number of instances. </p>
    async fn update_elasticsearch_domain_config(
        &self,
        input: UpdateElasticsearchDomainConfigRequest,
    ) -> Result<
        UpdateElasticsearchDomainConfigResponse,
        RusotoError<UpdateElasticsearchDomainConfigError>,
    >;

    /// <p>Updates a package for use with Amazon ES domains.</p>
    async fn update_package(
        &self,
        input: UpdatePackageRequest,
    ) -> Result<UpdatePackageResponse, RusotoError<UpdatePackageError>>;

    /// <p>Allows you to either upgrade your domain or perform an Upgrade eligibility check to a compatible Elasticsearch version.</p>
    async fn upgrade_elasticsearch_domain(
        &self,
        input: UpgradeElasticsearchDomainRequest,
    ) -> Result<UpgradeElasticsearchDomainResponse, RusotoError<UpgradeElasticsearchDomainError>>;
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

#[async_trait]
impl Es for EsClient {
    /// <p>Allows the destination domain owner to accept an inbound cross-cluster search connection request.</p>
    #[allow(unused_mut)]
    async fn accept_inbound_cross_cluster_search_connection(
        &self,
        input: AcceptInboundCrossClusterSearchConnectionRequest,
    ) -> Result<
        AcceptInboundCrossClusterSearchConnectionResponse,
        RusotoError<AcceptInboundCrossClusterSearchConnectionError>,
    > {
        let request_uri = format!(
            "/2015-01-01/es/ccs/inboundConnection/{connection_id}/accept",
            connection_id = input.cross_cluster_search_connection_id
        );

        let mut request = SignedRequest::new("PUT", "es", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<AcceptInboundCrossClusterSearchConnectionResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(AcceptInboundCrossClusterSearchConnectionError::from_response(response))
        }
    }

    /// <p>Attaches tags to an existing Elasticsearch domain. Tags are a set of case-sensitive key value pairs. An Elasticsearch domain may have up to 10 tags. See <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-managedomains.html#es-managedomains-awsresorcetagging" target="_blank"> Tagging Amazon Elasticsearch Service Domains for more information.</a></p>
    #[allow(unused_mut)]
    async fn add_tags(&self, input: AddTagsRequest) -> Result<(), RusotoError<AddTagsError>> {
        let request_uri = "/2015-01-01/tags";

        let mut request = SignedRequest::new("POST", "es", &self.region, &request_uri);
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
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(AddTagsError::from_response(response))
        }
    }

    /// <p>Associates a package with an Amazon ES domain.</p>
    #[allow(unused_mut)]
    async fn associate_package(
        &self,
        input: AssociatePackageRequest,
    ) -> Result<AssociatePackageResponse, RusotoError<AssociatePackageError>> {
        let request_uri = format!(
            "/2015-01-01/packages/associate/{package_id}/{domain_name}",
            domain_name = input.domain_name,
            package_id = input.package_id
        );

        let mut request = SignedRequest::new("POST", "es", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<AssociatePackageResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(AssociatePackageError::from_response(response))
        }
    }

    /// <p>Cancels a scheduled service software update for an Amazon ES domain. You can only perform this operation before the <code>AutomatedUpdateDate</code> and when the <code>UpdateStatus</code> is in the <code>PENDING_UPDATE</code> state.</p>
    #[allow(unused_mut)]
    async fn cancel_elasticsearch_service_software_update(
        &self,
        input: CancelElasticsearchServiceSoftwareUpdateRequest,
    ) -> Result<
        CancelElasticsearchServiceSoftwareUpdateResponse,
        RusotoError<CancelElasticsearchServiceSoftwareUpdateError>,
    > {
        let request_uri = "/2015-01-01/es/serviceSoftwareUpdate/cancel";

        let mut request = SignedRequest::new("POST", "es", &self.region, &request_uri);
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
                .deserialize::<CancelElasticsearchServiceSoftwareUpdateResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CancelElasticsearchServiceSoftwareUpdateError::from_response(response))
        }
    }

    /// <p>Creates a new Elasticsearch domain. For more information, see <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-createupdatedomains.html#es-createdomains" target="_blank">Creating Elasticsearch Domains</a> in the <i>Amazon Elasticsearch Service Developer Guide</i>.</p>
    #[allow(unused_mut)]
    async fn create_elasticsearch_domain(
        &self,
        input: CreateElasticsearchDomainRequest,
    ) -> Result<CreateElasticsearchDomainResponse, RusotoError<CreateElasticsearchDomainError>>
    {
        let request_uri = "/2015-01-01/es/domain";

        let mut request = SignedRequest::new("POST", "es", &self.region, &request_uri);
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
                .deserialize::<CreateElasticsearchDomainResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateElasticsearchDomainError::from_response(response))
        }
    }

    /// <p>Creates a new cross-cluster search connection from a source domain to a destination domain.</p>
    #[allow(unused_mut)]
    async fn create_outbound_cross_cluster_search_connection(
        &self,
        input: CreateOutboundCrossClusterSearchConnectionRequest,
    ) -> Result<
        CreateOutboundCrossClusterSearchConnectionResponse,
        RusotoError<CreateOutboundCrossClusterSearchConnectionError>,
    > {
        let request_uri = "/2015-01-01/es/ccs/outboundConnection";

        let mut request = SignedRequest::new("POST", "es", &self.region, &request_uri);
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
                .deserialize::<CreateOutboundCrossClusterSearchConnectionResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateOutboundCrossClusterSearchConnectionError::from_response(response))
        }
    }

    /// <p>Create a package for use with Amazon ES domains.</p>
    #[allow(unused_mut)]
    async fn create_package(
        &self,
        input: CreatePackageRequest,
    ) -> Result<CreatePackageResponse, RusotoError<CreatePackageError>> {
        let request_uri = "/2015-01-01/packages";

        let mut request = SignedRequest::new("POST", "es", &self.region, &request_uri);
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
                .deserialize::<CreatePackageResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreatePackageError::from_response(response))
        }
    }

    /// <p>Permanently deletes the specified Elasticsearch domain and all of its data. Once a domain is deleted, it cannot be recovered.</p>
    #[allow(unused_mut)]
    async fn delete_elasticsearch_domain(
        &self,
        input: DeleteElasticsearchDomainRequest,
    ) -> Result<DeleteElasticsearchDomainResponse, RusotoError<DeleteElasticsearchDomainError>>
    {
        let request_uri = format!(
            "/2015-01-01/es/domain/{domain_name}",
            domain_name = input.domain_name
        );

        let mut request = SignedRequest::new("DELETE", "es", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteElasticsearchDomainResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteElasticsearchDomainError::from_response(response))
        }
    }

    /// <p>Deletes the service-linked role that Elasticsearch Service uses to manage and maintain VPC domains. Role deletion will fail if any existing VPC domains use the role. You must delete any such Elasticsearch domains before deleting the role. See <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-vpc.html#es-enabling-slr" target="_blank">Deleting Elasticsearch Service Role</a> in <i>VPC Endpoints for Amazon Elasticsearch Service Domains</i>.</p>
    #[allow(unused_mut)]
    async fn delete_elasticsearch_service_role(
        &self,
    ) -> Result<(), RusotoError<DeleteElasticsearchServiceRoleError>> {
        let request_uri = "/2015-01-01/es/role";

        let mut request = SignedRequest::new("DELETE", "es", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteElasticsearchServiceRoleError::from_response(response))
        }
    }

    /// <p>Allows the destination domain owner to delete an existing inbound cross-cluster search connection.</p>
    #[allow(unused_mut)]
    async fn delete_inbound_cross_cluster_search_connection(
        &self,
        input: DeleteInboundCrossClusterSearchConnectionRequest,
    ) -> Result<
        DeleteInboundCrossClusterSearchConnectionResponse,
        RusotoError<DeleteInboundCrossClusterSearchConnectionError>,
    > {
        let request_uri = format!(
            "/2015-01-01/es/ccs/inboundConnection/{connection_id}",
            connection_id = input.cross_cluster_search_connection_id
        );

        let mut request = SignedRequest::new("DELETE", "es", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteInboundCrossClusterSearchConnectionResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteInboundCrossClusterSearchConnectionError::from_response(response))
        }
    }

    /// <p>Allows the source domain owner to delete an existing outbound cross-cluster search connection.</p>
    #[allow(unused_mut)]
    async fn delete_outbound_cross_cluster_search_connection(
        &self,
        input: DeleteOutboundCrossClusterSearchConnectionRequest,
    ) -> Result<
        DeleteOutboundCrossClusterSearchConnectionResponse,
        RusotoError<DeleteOutboundCrossClusterSearchConnectionError>,
    > {
        let request_uri = format!(
            "/2015-01-01/es/ccs/outboundConnection/{connection_id}",
            connection_id = input.cross_cluster_search_connection_id
        );

        let mut request = SignedRequest::new("DELETE", "es", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteOutboundCrossClusterSearchConnectionResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteOutboundCrossClusterSearchConnectionError::from_response(response))
        }
    }

    /// <p>Delete the package.</p>
    #[allow(unused_mut)]
    async fn delete_package(
        &self,
        input: DeletePackageRequest,
    ) -> Result<DeletePackageResponse, RusotoError<DeletePackageError>> {
        let request_uri = format!(
            "/2015-01-01/packages/{package_id}",
            package_id = input.package_id
        );

        let mut request = SignedRequest::new("DELETE", "es", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeletePackageResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeletePackageError::from_response(response))
        }
    }

    /// <p>Returns domain configuration information about the specified Elasticsearch domain, including the domain ID, domain endpoint, and domain ARN.</p>
    #[allow(unused_mut)]
    async fn describe_elasticsearch_domain(
        &self,
        input: DescribeElasticsearchDomainRequest,
    ) -> Result<DescribeElasticsearchDomainResponse, RusotoError<DescribeElasticsearchDomainError>>
    {
        let request_uri = format!(
            "/2015-01-01/es/domain/{domain_name}",
            domain_name = input.domain_name
        );

        let mut request = SignedRequest::new("GET", "es", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeElasticsearchDomainResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeElasticsearchDomainError::from_response(response))
        }
    }

    /// <p>Provides cluster configuration information about the specified Elasticsearch domain, such as the state, creation date, update version, and update date for cluster options.</p>
    #[allow(unused_mut)]
    async fn describe_elasticsearch_domain_config(
        &self,
        input: DescribeElasticsearchDomainConfigRequest,
    ) -> Result<
        DescribeElasticsearchDomainConfigResponse,
        RusotoError<DescribeElasticsearchDomainConfigError>,
    > {
        let request_uri = format!(
            "/2015-01-01/es/domain/{domain_name}/config",
            domain_name = input.domain_name
        );

        let mut request = SignedRequest::new("GET", "es", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
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

    /// <p>Returns domain configuration information about the specified Elasticsearch domains, including the domain ID, domain endpoint, and domain ARN.</p>
    #[allow(unused_mut)]
    async fn describe_elasticsearch_domains(
        &self,
        input: DescribeElasticsearchDomainsRequest,
    ) -> Result<DescribeElasticsearchDomainsResponse, RusotoError<DescribeElasticsearchDomainsError>>
    {
        let request_uri = "/2015-01-01/es/domain-info";

        let mut request = SignedRequest::new("POST", "es", &self.region, &request_uri);
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
                .deserialize::<DescribeElasticsearchDomainsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeElasticsearchDomainsError::from_response(response))
        }
    }

    /// <p> Describe Elasticsearch Limits for a given InstanceType and ElasticsearchVersion. When modifying existing Domain, specify the <code> <a>DomainName</a> </code> to know what Limits are supported for modifying. </p>
    #[allow(unused_mut)]
    async fn describe_elasticsearch_instance_type_limits(
        &self,
        input: DescribeElasticsearchInstanceTypeLimitsRequest,
    ) -> Result<
        DescribeElasticsearchInstanceTypeLimitsResponse,
        RusotoError<DescribeElasticsearchInstanceTypeLimitsError>,
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
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

    /// <p>Lists all the inbound cross-cluster search connections for a destination domain.</p>
    #[allow(unused_mut)]
    async fn describe_inbound_cross_cluster_search_connections(
        &self,
        input: DescribeInboundCrossClusterSearchConnectionsRequest,
    ) -> Result<
        DescribeInboundCrossClusterSearchConnectionsResponse,
        RusotoError<DescribeInboundCrossClusterSearchConnectionsError>,
    > {
        let request_uri = "/2015-01-01/es/ccs/inboundConnection/search";

        let mut request = SignedRequest::new("POST", "es", &self.region, &request_uri);
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
                .deserialize::<DescribeInboundCrossClusterSearchConnectionsResponse, _>(
            )?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeInboundCrossClusterSearchConnectionsError::from_response(response))
        }
    }

    /// <p>Lists all the outbound cross-cluster search connections for a source domain.</p>
    #[allow(unused_mut)]
    async fn describe_outbound_cross_cluster_search_connections(
        &self,
        input: DescribeOutboundCrossClusterSearchConnectionsRequest,
    ) -> Result<
        DescribeOutboundCrossClusterSearchConnectionsResponse,
        RusotoError<DescribeOutboundCrossClusterSearchConnectionsError>,
    > {
        let request_uri = "/2015-01-01/es/ccs/outboundConnection/search";

        let mut request = SignedRequest::new("POST", "es", &self.region, &request_uri);
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
                .deserialize::<DescribeOutboundCrossClusterSearchConnectionsResponse, _>(
            )?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeOutboundCrossClusterSearchConnectionsError::from_response(response))
        }
    }

    /// <p>Describes all packages available to Amazon ES. Includes options for filtering, limiting the number of results, and pagination.</p>
    #[allow(unused_mut)]
    async fn describe_packages(
        &self,
        input: DescribePackagesRequest,
    ) -> Result<DescribePackagesResponse, RusotoError<DescribePackagesError>> {
        let request_uri = "/2015-01-01/packages/describe";

        let mut request = SignedRequest::new("POST", "es", &self.region, &request_uri);
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
                .deserialize::<DescribePackagesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribePackagesError::from_response(response))
        }
    }

    /// <p>Lists available reserved Elasticsearch instance offerings.</p>
    #[allow(unused_mut)]
    async fn describe_reserved_elasticsearch_instance_offerings(
        &self,
        input: DescribeReservedElasticsearchInstanceOfferingsRequest,
    ) -> Result<
        DescribeReservedElasticsearchInstanceOfferingsResponse,
        RusotoError<DescribeReservedElasticsearchInstanceOfferingsError>,
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeReservedElasticsearchInstanceOfferingsResponse, _>(
            )?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeReservedElasticsearchInstanceOfferingsError::from_response(response))
        }
    }

    /// <p>Returns information about reserved Elasticsearch instances for this account.</p>
    #[allow(unused_mut)]
    async fn describe_reserved_elasticsearch_instances(
        &self,
        input: DescribeReservedElasticsearchInstancesRequest,
    ) -> Result<
        DescribeReservedElasticsearchInstancesResponse,
        RusotoError<DescribeReservedElasticsearchInstancesError>,
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
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

    /// <p>Dissociates a package from the Amazon ES domain.</p>
    #[allow(unused_mut)]
    async fn dissociate_package(
        &self,
        input: DissociatePackageRequest,
    ) -> Result<DissociatePackageResponse, RusotoError<DissociatePackageError>> {
        let request_uri = format!(
            "/2015-01-01/packages/dissociate/{package_id}/{domain_name}",
            domain_name = input.domain_name,
            package_id = input.package_id
        );

        let mut request = SignedRequest::new("POST", "es", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DissociatePackageResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DissociatePackageError::from_response(response))
        }
    }

    /// <p> Returns a list of upgrade compatible Elastisearch versions. You can optionally pass a <code> <a>DomainName</a> </code> to get all upgrade compatible Elasticsearch versions for that specific domain. </p>
    #[allow(unused_mut)]
    async fn get_compatible_elasticsearch_versions(
        &self,
        input: GetCompatibleElasticsearchVersionsRequest,
    ) -> Result<
        GetCompatibleElasticsearchVersionsResponse,
        RusotoError<GetCompatibleElasticsearchVersionsError>,
    > {
        let request_uri = "/2015-01-01/es/compatibleVersions";

        let mut request = SignedRequest::new("GET", "es", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.domain_name {
            params.put("domainName", x);
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
                .deserialize::<GetCompatibleElasticsearchVersionsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetCompatibleElasticsearchVersionsError::from_response(
                response,
            ))
        }
    }

    /// <p>Returns a list of versions of the package, along with their creation time and commit message.</p>
    #[allow(unused_mut)]
    async fn get_package_version_history(
        &self,
        input: GetPackageVersionHistoryRequest,
    ) -> Result<GetPackageVersionHistoryResponse, RusotoError<GetPackageVersionHistoryError>> {
        let request_uri = format!(
            "/2015-01-01/packages/{package_id}/history",
            package_id = input.package_id
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetPackageVersionHistoryResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetPackageVersionHistoryError::from_response(response))
        }
    }

    /// <p>Retrieves the complete history of the last 10 upgrades that were performed on the domain.</p>
    #[allow(unused_mut)]
    async fn get_upgrade_history(
        &self,
        input: GetUpgradeHistoryRequest,
    ) -> Result<GetUpgradeHistoryResponse, RusotoError<GetUpgradeHistoryError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetUpgradeHistoryResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetUpgradeHistoryError::from_response(response))
        }
    }

    /// <p>Retrieves the latest status of the last upgrade or upgrade eligibility check that was performed on the domain.</p>
    #[allow(unused_mut)]
    async fn get_upgrade_status(
        &self,
        input: GetUpgradeStatusRequest,
    ) -> Result<GetUpgradeStatusResponse, RusotoError<GetUpgradeStatusError>> {
        let request_uri = format!(
            "/2015-01-01/es/upgradeDomain/{domain_name}/status",
            domain_name = input.domain_name
        );

        let mut request = SignedRequest::new("GET", "es", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetUpgradeStatusResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetUpgradeStatusError::from_response(response))
        }
    }

    /// <p>Returns the name of all Elasticsearch domains owned by the current user's account. </p>
    #[allow(unused_mut)]
    async fn list_domain_names(
        &self,
    ) -> Result<ListDomainNamesResponse, RusotoError<ListDomainNamesError>> {
        let request_uri = "/2015-01-01/domain";

        let mut request = SignedRequest::new("GET", "es", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListDomainNamesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListDomainNamesError::from_response(response))
        }
    }

    /// <p>Lists all Amazon ES domains associated with the package.</p>
    #[allow(unused_mut)]
    async fn list_domains_for_package(
        &self,
        input: ListDomainsForPackageRequest,
    ) -> Result<ListDomainsForPackageResponse, RusotoError<ListDomainsForPackageError>> {
        let request_uri = format!(
            "/2015-01-01/packages/{package_id}/domains",
            package_id = input.package_id
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListDomainsForPackageResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListDomainsForPackageError::from_response(response))
        }
    }

    /// <p>List all Elasticsearch instance types that are supported for given ElasticsearchVersion</p>
    #[allow(unused_mut)]
    async fn list_elasticsearch_instance_types(
        &self,
        input: ListElasticsearchInstanceTypesRequest,
    ) -> Result<
        ListElasticsearchInstanceTypesResponse,
        RusotoError<ListElasticsearchInstanceTypesError>,
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListElasticsearchInstanceTypesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListElasticsearchInstanceTypesError::from_response(response))
        }
    }

    /// <p>List all supported Elasticsearch versions</p>
    #[allow(unused_mut)]
    async fn list_elasticsearch_versions(
        &self,
        input: ListElasticsearchVersionsRequest,
    ) -> Result<ListElasticsearchVersionsResponse, RusotoError<ListElasticsearchVersionsError>>
    {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListElasticsearchVersionsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListElasticsearchVersionsError::from_response(response))
        }
    }

    /// <p>Lists all packages associated with the Amazon ES domain.</p>
    #[allow(unused_mut)]
    async fn list_packages_for_domain(
        &self,
        input: ListPackagesForDomainRequest,
    ) -> Result<ListPackagesForDomainResponse, RusotoError<ListPackagesForDomainError>> {
        let request_uri = format!(
            "/2015-01-01/domain/{domain_name}/packages",
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListPackagesForDomainResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListPackagesForDomainError::from_response(response))
        }
    }

    /// <p>Returns all tags for the given Elasticsearch domain.</p>
    #[allow(unused_mut)]
    async fn list_tags(
        &self,
        input: ListTagsRequest,
    ) -> Result<ListTagsResponse, RusotoError<ListTagsError>> {
        let request_uri = "/2015-01-01/tags/";

        let mut request = SignedRequest::new("GET", "es", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        params.put("arn", &input.arn);
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListTagsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListTagsError::from_response(response))
        }
    }

    /// <p>Allows you to purchase reserved Elasticsearch instances.</p>
    #[allow(unused_mut)]
    async fn purchase_reserved_elasticsearch_instance_offering(
        &self,
        input: PurchaseReservedElasticsearchInstanceOfferingRequest,
    ) -> Result<
        PurchaseReservedElasticsearchInstanceOfferingResponse,
        RusotoError<PurchaseReservedElasticsearchInstanceOfferingError>,
    > {
        let request_uri = "/2015-01-01/es/purchaseReservedInstanceOffering";

        let mut request = SignedRequest::new("POST", "es", &self.region, &request_uri);
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
                .deserialize::<PurchaseReservedElasticsearchInstanceOfferingResponse, _>(
            )?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PurchaseReservedElasticsearchInstanceOfferingError::from_response(response))
        }
    }

    /// <p>Allows the destination domain owner to reject an inbound cross-cluster search connection request.</p>
    #[allow(unused_mut)]
    async fn reject_inbound_cross_cluster_search_connection(
        &self,
        input: RejectInboundCrossClusterSearchConnectionRequest,
    ) -> Result<
        RejectInboundCrossClusterSearchConnectionResponse,
        RusotoError<RejectInboundCrossClusterSearchConnectionError>,
    > {
        let request_uri = format!(
            "/2015-01-01/es/ccs/inboundConnection/{connection_id}/reject",
            connection_id = input.cross_cluster_search_connection_id
        );

        let mut request = SignedRequest::new("PUT", "es", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<RejectInboundCrossClusterSearchConnectionResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(RejectInboundCrossClusterSearchConnectionError::from_response(response))
        }
    }

    /// <p>Removes the specified set of tags from the specified Elasticsearch domain.</p>
    #[allow(unused_mut)]
    async fn remove_tags(
        &self,
        input: RemoveTagsRequest,
    ) -> Result<(), RusotoError<RemoveTagsError>> {
        let request_uri = "/2015-01-01/tags-removal";

        let mut request = SignedRequest::new("POST", "es", &self.region, &request_uri);
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
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(RemoveTagsError::from_response(response))
        }
    }

    /// <p>Schedules a service software update for an Amazon ES domain.</p>
    #[allow(unused_mut)]
    async fn start_elasticsearch_service_software_update(
        &self,
        input: StartElasticsearchServiceSoftwareUpdateRequest,
    ) -> Result<
        StartElasticsearchServiceSoftwareUpdateResponse,
        RusotoError<StartElasticsearchServiceSoftwareUpdateError>,
    > {
        let request_uri = "/2015-01-01/es/serviceSoftwareUpdate/start";

        let mut request = SignedRequest::new("POST", "es", &self.region, &request_uri);
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
                .deserialize::<StartElasticsearchServiceSoftwareUpdateResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(StartElasticsearchServiceSoftwareUpdateError::from_response(
                response,
            ))
        }
    }

    /// <p>Modifies the cluster configuration of the specified Elasticsearch domain, setting as setting the instance type and the number of instances. </p>
    #[allow(unused_mut)]
    async fn update_elasticsearch_domain_config(
        &self,
        input: UpdateElasticsearchDomainConfigRequest,
    ) -> Result<
        UpdateElasticsearchDomainConfigResponse,
        RusotoError<UpdateElasticsearchDomainConfigError>,
    > {
        let request_uri = format!(
            "/2015-01-01/es/domain/{domain_name}/config",
            domain_name = input.domain_name
        );

        let mut request = SignedRequest::new("POST", "es", &self.region, &request_uri);
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
                .deserialize::<UpdateElasticsearchDomainConfigResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateElasticsearchDomainConfigError::from_response(
                response,
            ))
        }
    }

    /// <p>Updates a package for use with Amazon ES domains.</p>
    #[allow(unused_mut)]
    async fn update_package(
        &self,
        input: UpdatePackageRequest,
    ) -> Result<UpdatePackageResponse, RusotoError<UpdatePackageError>> {
        let request_uri = "/2015-01-01/packages/update";

        let mut request = SignedRequest::new("POST", "es", &self.region, &request_uri);
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
                .deserialize::<UpdatePackageResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdatePackageError::from_response(response))
        }
    }

    /// <p>Allows you to either upgrade your domain or perform an Upgrade eligibility check to a compatible Elasticsearch version.</p>
    #[allow(unused_mut)]
    async fn upgrade_elasticsearch_domain(
        &self,
        input: UpgradeElasticsearchDomainRequest,
    ) -> Result<UpgradeElasticsearchDomainResponse, RusotoError<UpgradeElasticsearchDomainError>>
    {
        let request_uri = "/2015-01-01/es/upgradeDomain";

        let mut request = SignedRequest::new("POST", "es", &self.region, &request_uri);
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
                .deserialize::<UpgradeElasticsearchDomainResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpgradeElasticsearchDomainError::from_response(response))
        }
    }
}
