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
use rusoto_core::{Client, RusotoError, RusotoFuture};

use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
use serde_json;
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AssociateDomainRequest {
    /// <p>The ARN of an issued ACM certificate that is valid for the domain being associated.</p>
    #[serde(rename = "AcmCertificateArn")]
    pub acm_certificate_arn: String,
    /// <p>The name to display.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>The fully qualified domain name (FQDN).</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// <p>The Amazon Resource Name (ARN) of the fleet.</p>
    #[serde(rename = "FleetArn")]
    pub fleet_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AssociateDomainResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AssociateWebsiteAuthorizationProviderRequest {
    /// <p>The authorization provider type.</p>
    #[serde(rename = "AuthorizationProviderType")]
    pub authorization_provider_type: String,
    /// <p>The domain name of the authorization provider. This applies only to SAML-based authorization providers.</p>
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// <p>The ARN of the fleet.</p>
    #[serde(rename = "FleetArn")]
    pub fleet_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AssociateWebsiteAuthorizationProviderResponse {
    /// <p>A unique identifier for the authorization provider.</p>
    #[serde(rename = "AuthorizationProviderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_provider_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AssociateWebsiteCertificateAuthorityRequest {
    /// <p>The root certificate of the CA.</p>
    #[serde(rename = "Certificate")]
    pub certificate: String,
    /// <p>The certificate name to display.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>The ARN of the fleet.</p>
    #[serde(rename = "FleetArn")]
    pub fleet_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AssociateWebsiteCertificateAuthorityResponse {
    /// <p>A unique identifier for the CA.</p>
    #[serde(rename = "WebsiteCaId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website_ca_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateFleetRequest {
    /// <p>The fleet name to display.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>A unique name for the fleet.</p>
    #[serde(rename = "FleetName")]
    pub fleet_name: String,
    /// <p>The option to optimize for better performance by routing traffic through the closest AWS Region to users, which may be outside of your home Region.</p>
    #[serde(rename = "OptimizeForEndUserLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optimize_for_end_user_location: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateFleetResponse {
    /// <p>The ARN of the fleet.</p>
    #[serde(rename = "FleetArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteFleetRequest {
    /// <p>The ARN of the fleet.</p>
    #[serde(rename = "FleetArn")]
    pub fleet_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteFleetResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeAuditStreamConfigurationRequest {
    /// <p>The ARN of the fleet.</p>
    #[serde(rename = "FleetArn")]
    pub fleet_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeAuditStreamConfigurationResponse {
    /// <p>The ARN of the Amazon Kinesis data stream that will receive the audit events.</p>
    #[serde(rename = "AuditStreamArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_stream_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeCompanyNetworkConfigurationRequest {
    /// <p>The ARN of the fleet.</p>
    #[serde(rename = "FleetArn")]
    pub fleet_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeCompanyNetworkConfigurationResponse {
    /// <p>The security groups associated with access to the provided subnets.</p>
    #[serde(rename = "SecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    /// <p>The subnets used for X-ENI connections from Amazon WorkLink rendering containers.</p>
    #[serde(rename = "SubnetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    /// <p>The VPC with connectivity to associated websites.</p>
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeDevicePolicyConfigurationRequest {
    /// <p>The ARN of the fleet.</p>
    #[serde(rename = "FleetArn")]
    pub fleet_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeDevicePolicyConfigurationResponse {
    /// <p>The certificate chain, including intermediate certificates and the root certificate authority certificate used to issue device certificates.</p>
    #[serde(rename = "DeviceCaCertificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_ca_certificate: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeDeviceRequest {
    /// <p>A unique identifier for a registered user's device.</p>
    #[serde(rename = "DeviceId")]
    pub device_id: String,
    /// <p>The ARN of the fleet.</p>
    #[serde(rename = "FleetArn")]
    pub fleet_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeDeviceResponse {
    /// <p>The date that the device first signed in to Amazon WorkLink.</p>
    #[serde(rename = "FirstAccessedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_accessed_time: Option<f64>,
    /// <p>The date that the device last accessed Amazon WorkLink.</p>
    #[serde(rename = "LastAccessedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_accessed_time: Option<f64>,
    /// <p>The manufacturer of the device.</p>
    #[serde(rename = "Manufacturer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    /// <p>The model of the device.</p>
    #[serde(rename = "Model")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// <p>The operating system of the device.</p>
    #[serde(rename = "OperatingSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<String>,
    /// <p>The operating system version of the device.</p>
    #[serde(rename = "OperatingSystemVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system_version: Option<String>,
    /// <p>The operating system patch level of the device.</p>
    #[serde(rename = "PatchLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_level: Option<String>,
    /// <p>The current state of the device.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The user name associated with the device.</p>
    #[serde(rename = "Username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeDomainRequest {
    /// <p>The name of the domain.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// <p>The ARN of the fleet.</p>
    #[serde(rename = "FleetArn")]
    pub fleet_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeDomainResponse {
    /// <p>The ARN of an issued ACM certificate that is valid for the domain being associated.</p>
    #[serde(rename = "AcmCertificateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acm_certificate_arn: Option<String>,
    /// <p>The time that the domain was added.</p>
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    /// <p>The name to display.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>The name of the domain.</p>
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// <p>The current state for the domain.</p>
    #[serde(rename = "DomainStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeFleetMetadataRequest {
    /// <p>The ARN of the fleet.</p>
    #[serde(rename = "FleetArn")]
    pub fleet_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeFleetMetadataResponse {
    /// <p>The identifier used by users to sign in to the Amazon WorkLink app.</p>
    #[serde(rename = "CompanyCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_code: Option<String>,
    /// <p>The time that the fleet was created.</p>
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    /// <p>The name to display.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>The name of the fleet.</p>
    #[serde(rename = "FleetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_name: Option<String>,
    /// <p>The current state of the fleet.</p>
    #[serde(rename = "FleetStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_status: Option<String>,
    /// <p>The time that the fleet was last updated.</p>
    #[serde(rename = "LastUpdatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    /// <p>The option to optimize for better performance by routing traffic through the closest AWS Region to users, which may be outside of your home Region.</p>
    #[serde(rename = "OptimizeForEndUserLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optimize_for_end_user_location: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeIdentityProviderConfigurationRequest {
    /// <p>The ARN of the fleet.</p>
    #[serde(rename = "FleetArn")]
    pub fleet_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeIdentityProviderConfigurationResponse {
    /// <p>The SAML metadata document provided by the user’s identity provider.</p>
    #[serde(rename = "IdentityProviderSamlMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_provider_saml_metadata: Option<String>,
    /// <p>The type of identity provider.</p>
    #[serde(rename = "IdentityProviderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_provider_type: Option<String>,
    /// <p>The SAML metadata document uploaded to the user’s identity provider.</p>
    #[serde(rename = "ServiceProviderSamlMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_provider_saml_metadata: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeWebsiteCertificateAuthorityRequest {
    /// <p>The ARN of the fleet.</p>
    #[serde(rename = "FleetArn")]
    pub fleet_arn: String,
    /// <p>A unique identifier for the certificate authority.</p>
    #[serde(rename = "WebsiteCaId")]
    pub website_ca_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeWebsiteCertificateAuthorityResponse {
    /// <p>The root certificate of the certificate authority.</p>
    #[serde(rename = "Certificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    /// <p>The time that the certificate authority was added.</p>
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    /// <p>The certificate name to display.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

/// <p>The summary of devices.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeviceSummary {
    /// <p>The ID of the device.</p>
    #[serde(rename = "DeviceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    /// <p>The status of the device.</p>
    #[serde(rename = "DeviceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisassociateDomainRequest {
    /// <p>The name of the domain.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// <p>The ARN of the fleet.</p>
    #[serde(rename = "FleetArn")]
    pub fleet_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DisassociateDomainResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisassociateWebsiteAuthorizationProviderRequest {
    /// <p>A unique identifier for the authorization provider.</p>
    #[serde(rename = "AuthorizationProviderId")]
    pub authorization_provider_id: String,
    /// <p>The ARN of the fleet.</p>
    #[serde(rename = "FleetArn")]
    pub fleet_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DisassociateWebsiteAuthorizationProviderResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisassociateWebsiteCertificateAuthorityRequest {
    /// <p>The ARN of the fleet.</p>
    #[serde(rename = "FleetArn")]
    pub fleet_arn: String,
    /// <p>A unique identifier for the CA.</p>
    #[serde(rename = "WebsiteCaId")]
    pub website_ca_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DisassociateWebsiteCertificateAuthorityResponse {}

/// <p>The summary of the domain.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DomainSummary {
    /// <p>The time that the domain was created.</p>
    #[serde(rename = "CreatedTime")]
    pub created_time: f64,
    /// <p>The name to display.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>The name of the domain.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// <p>The status of the domain.</p>
    #[serde(rename = "DomainStatus")]
    pub domain_status: String,
}

/// <p>The summary of the fleet.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct FleetSummary {
    /// <p>The identifier used by users to sign into the Amazon WorkLink app.</p>
    #[serde(rename = "CompanyCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_code: Option<String>,
    /// <p>The time when the fleet was created.</p>
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    /// <p>The name to display.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>The ARN of the fleet.</p>
    #[serde(rename = "FleetArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_arn: Option<String>,
    /// <p>The name of the fleet.</p>
    #[serde(rename = "FleetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_name: Option<String>,
    /// <p>The status of the fleet.</p>
    #[serde(rename = "FleetStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_status: Option<String>,
    /// <p>The time when the fleet was last updated.</p>
    #[serde(rename = "LastUpdatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListDevicesRequest {
    /// <p>The ARN of the fleet.</p>
    #[serde(rename = "FleetArn")]
    pub fleet_arn: String,
    /// <p>The maximum number of results to be included in the next page.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The pagination token used to retrieve the next page of results for this operation. If this value is null, it retrieves the first page.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListDevicesResponse {
    /// <p>Information about the devices.</p>
    #[serde(rename = "Devices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<DeviceSummary>>,
    /// <p>The pagination token used to retrieve the next page of results for this operation. If there are no more pages, this value is null.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListDomainsRequest {
    /// <p>The ARN of the fleet.</p>
    #[serde(rename = "FleetArn")]
    pub fleet_arn: String,
    /// <p>The maximum number of results to be included in the next page.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The pagination token used to retrieve the next page of results for this operation. If this value is null, it retrieves the first page.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListDomainsResponse {
    /// <p>Information about the domains.</p>
    #[serde(rename = "Domains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domains: Option<Vec<DomainSummary>>,
    /// <p>The pagination token used to retrieve the next page of results for this operation. If there are no more pages, this value is null.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListFleetsRequest {
    /// <p>The maximum number of results to be included in the next page.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The pagination token used to retrieve the next page of results for this operation. If this value is null, it retrieves the first page.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListFleetsResponse {
    /// <p>The summary list of the fleets.</p>
    #[serde(rename = "FleetSummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_summary_list: Option<Vec<FleetSummary>>,
    /// <p>The pagination token used to retrieve the next page of results for this operation. If there are no more pages, this value is null.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListWebsiteAuthorizationProvidersRequest {
    /// <p>The ARN of the fleet.</p>
    #[serde(rename = "FleetArn")]
    pub fleet_arn: String,
    /// <p>The maximum number of results to be included in the next page.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The pagination token to use to retrieve the next page of results for this operation. If this value is null, it retrieves the first page.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListWebsiteAuthorizationProvidersResponse {
    /// <p>The pagination token to use to retrieve the next page of results for this operation. If this value is null, it retrieves the first page.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The website authorization providers.</p>
    #[serde(rename = "WebsiteAuthorizationProviders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website_authorization_providers: Option<Vec<WebsiteAuthorizationProviderSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListWebsiteCertificateAuthoritiesRequest {
    /// <p>The ARN of the fleet.</p>
    #[serde(rename = "FleetArn")]
    pub fleet_arn: String,
    /// <p>The maximum number of results to be included in the next page.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The pagination token used to retrieve the next page of results for this operation. If this value is null, it retrieves the first page.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListWebsiteCertificateAuthoritiesResponse {
    /// <p>The pagination token used to retrieve the next page of results for this operation. If there are no more pages, this value is null.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the certificates.</p>
    #[serde(rename = "WebsiteCertificateAuthorities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website_certificate_authorities: Option<Vec<WebsiteCaSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RestoreDomainAccessRequest {
    /// <p>The name of the domain.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// <p>The ARN of the fleet.</p>
    #[serde(rename = "FleetArn")]
    pub fleet_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RestoreDomainAccessResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RevokeDomainAccessRequest {
    /// <p>The name of the domain.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// <p>The ARN of the fleet.</p>
    #[serde(rename = "FleetArn")]
    pub fleet_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RevokeDomainAccessResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SignOutUserRequest {
    /// <p>The ARN of the fleet.</p>
    #[serde(rename = "FleetArn")]
    pub fleet_arn: String,
    /// <p>The name of the user.</p>
    #[serde(rename = "Username")]
    pub username: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SignOutUserResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateAuditStreamConfigurationRequest {
    /// <p>The ARN of the Amazon Kinesis data stream that receives the audit events.</p>
    #[serde(rename = "AuditStreamArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_stream_arn: Option<String>,
    /// <p>The ARN of the fleet.</p>
    #[serde(rename = "FleetArn")]
    pub fleet_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateAuditStreamConfigurationResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateCompanyNetworkConfigurationRequest {
    /// <p>The ARN of the fleet.</p>
    #[serde(rename = "FleetArn")]
    pub fleet_arn: String,
    /// <p>The security groups associated with access to the provided subnets.</p>
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Vec<String>,
    /// <p>The subnets used for X-ENI connections from Amazon WorkLink rendering containers.</p>
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Vec<String>,
    /// <p>The VPC with connectivity to associated websites.</p>
    #[serde(rename = "VpcId")]
    pub vpc_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateCompanyNetworkConfigurationResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateDevicePolicyConfigurationRequest {
    /// <p>The certificate chain, including intermediate certificates and the root certificate authority certificate used to issue device certificates.</p>
    #[serde(rename = "DeviceCaCertificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_ca_certificate: Option<String>,
    /// <p>The ARN of the fleet.</p>
    #[serde(rename = "FleetArn")]
    pub fleet_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateDevicePolicyConfigurationResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateDomainMetadataRequest {
    /// <p>The name to display.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>The name of the domain.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// <p>The ARN of the fleet.</p>
    #[serde(rename = "FleetArn")]
    pub fleet_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateDomainMetadataResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateFleetMetadataRequest {
    /// <p>The fleet name to display. The existing DisplayName is unset if null is passed.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>The ARN of the fleet.</p>
    #[serde(rename = "FleetArn")]
    pub fleet_arn: String,
    /// <p>The option to optimize for better performance by routing traffic through the closest AWS Region to users, which may be outside of your home Region.</p>
    #[serde(rename = "OptimizeForEndUserLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optimize_for_end_user_location: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateFleetMetadataResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateIdentityProviderConfigurationRequest {
    /// <p>The ARN of the fleet.</p>
    #[serde(rename = "FleetArn")]
    pub fleet_arn: String,
    /// <p>The SAML metadata document provided by the customer’s identity provider. The existing IdentityProviderSamlMetadata is unset if null is passed.</p>
    #[serde(rename = "IdentityProviderSamlMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_provider_saml_metadata: Option<String>,
    /// <p>The type of identity provider.</p>
    #[serde(rename = "IdentityProviderType")]
    pub identity_provider_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateIdentityProviderConfigurationResponse {}

/// <p>The summary of the website authorization provider.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct WebsiteAuthorizationProviderSummary {
    /// <p>A unique identifier for the authorization provider.</p>
    #[serde(rename = "AuthorizationProviderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_provider_id: Option<String>,
    /// <p>The authorization provider type.</p>
    #[serde(rename = "AuthorizationProviderType")]
    pub authorization_provider_type: String,
    /// <p>The time of creation.</p>
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    /// <p>The domain name of the authorization provider. This applies only to SAML-based authorization providers.</p>
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
}

/// <p>The summary of the certificate authority (CA).</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct WebsiteCaSummary {
    /// <p>The time when the CA was added.</p>
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    /// <p>The name to display.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>A unique identifier for the CA.</p>
    #[serde(rename = "WebsiteCaId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website_ca_id: Option<String>,
}

/// Errors returned by AssociateDomain
#[derive(Debug, PartialEq)]
pub enum AssociateDomainError {
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The resource already exists.</p>
    ResourceAlreadyExists(String),
    /// <p>The requested resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The number of requests exceeds the limit.</p>
    TooManyRequests(String),
    /// <p>You are not authorized to perform this action.</p>
    Unauthorized(String),
}

impl AssociateDomainError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AssociateDomainError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerErrorException" => {
                    return RusotoError::Service(AssociateDomainError::InternalServerError(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(AssociateDomainError::InvalidRequest(err.msg))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(AssociateDomainError::ResourceAlreadyExists(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AssociateDomainError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(AssociateDomainError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(AssociateDomainError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for AssociateDomainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AssociateDomainError {
    fn description(&self) -> &str {
        match *self {
            AssociateDomainError::InternalServerError(ref cause) => cause,
            AssociateDomainError::InvalidRequest(ref cause) => cause,
            AssociateDomainError::ResourceAlreadyExists(ref cause) => cause,
            AssociateDomainError::ResourceNotFound(ref cause) => cause,
            AssociateDomainError::TooManyRequests(ref cause) => cause,
            AssociateDomainError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by AssociateWebsiteAuthorizationProvider
#[derive(Debug, PartialEq)]
pub enum AssociateWebsiteAuthorizationProviderError {
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The resource already exists.</p>
    ResourceAlreadyExists(String),
    /// <p>The requested resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The number of requests exceeds the limit.</p>
    TooManyRequests(String),
    /// <p>You are not authorized to perform this action.</p>
    Unauthorized(String),
}

impl AssociateWebsiteAuthorizationProviderError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AssociateWebsiteAuthorizationProviderError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        AssociateWebsiteAuthorizationProviderError::InternalServerError(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        AssociateWebsiteAuthorizationProviderError::InvalidRequest(err.msg),
                    )
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(
                        AssociateWebsiteAuthorizationProviderError::ResourceAlreadyExists(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        AssociateWebsiteAuthorizationProviderError::ResourceNotFound(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        AssociateWebsiteAuthorizationProviderError::TooManyRequests(err.msg),
                    )
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(
                        AssociateWebsiteAuthorizationProviderError::Unauthorized(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for AssociateWebsiteAuthorizationProviderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AssociateWebsiteAuthorizationProviderError {
    fn description(&self) -> &str {
        match *self {
            AssociateWebsiteAuthorizationProviderError::InternalServerError(ref cause) => cause,
            AssociateWebsiteAuthorizationProviderError::InvalidRequest(ref cause) => cause,
            AssociateWebsiteAuthorizationProviderError::ResourceAlreadyExists(ref cause) => cause,
            AssociateWebsiteAuthorizationProviderError::ResourceNotFound(ref cause) => cause,
            AssociateWebsiteAuthorizationProviderError::TooManyRequests(ref cause) => cause,
            AssociateWebsiteAuthorizationProviderError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by AssociateWebsiteCertificateAuthority
#[derive(Debug, PartialEq)]
pub enum AssociateWebsiteCertificateAuthorityError {
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The resource already exists.</p>
    ResourceAlreadyExists(String),
    /// <p>The requested resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The number of requests exceeds the limit.</p>
    TooManyRequests(String),
    /// <p>You are not authorized to perform this action.</p>
    Unauthorized(String),
}

impl AssociateWebsiteCertificateAuthorityError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AssociateWebsiteCertificateAuthorityError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        AssociateWebsiteCertificateAuthorityError::InternalServerError(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        AssociateWebsiteCertificateAuthorityError::InvalidRequest(err.msg),
                    )
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(
                        AssociateWebsiteCertificateAuthorityError::ResourceAlreadyExists(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        AssociateWebsiteCertificateAuthorityError::ResourceNotFound(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        AssociateWebsiteCertificateAuthorityError::TooManyRequests(err.msg),
                    )
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(
                        AssociateWebsiteCertificateAuthorityError::Unauthorized(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for AssociateWebsiteCertificateAuthorityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AssociateWebsiteCertificateAuthorityError {
    fn description(&self) -> &str {
        match *self {
            AssociateWebsiteCertificateAuthorityError::InternalServerError(ref cause) => cause,
            AssociateWebsiteCertificateAuthorityError::InvalidRequest(ref cause) => cause,
            AssociateWebsiteCertificateAuthorityError::ResourceAlreadyExists(ref cause) => cause,
            AssociateWebsiteCertificateAuthorityError::ResourceNotFound(ref cause) => cause,
            AssociateWebsiteCertificateAuthorityError::TooManyRequests(ref cause) => cause,
            AssociateWebsiteCertificateAuthorityError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateFleet
#[derive(Debug, PartialEq)]
pub enum CreateFleetError {
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The resource already exists.</p>
    ResourceAlreadyExists(String),
    /// <p>The requested resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The number of requests exceeds the limit.</p>
    TooManyRequests(String),
    /// <p>You are not authorized to perform this action.</p>
    Unauthorized(String),
}

impl CreateFleetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateFleetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateFleetError::InternalServerError(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateFleetError::InvalidRequest(err.msg))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(CreateFleetError::ResourceAlreadyExists(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateFleetError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateFleetError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(CreateFleetError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateFleetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateFleetError {
    fn description(&self) -> &str {
        match *self {
            CreateFleetError::InternalServerError(ref cause) => cause,
            CreateFleetError::InvalidRequest(ref cause) => cause,
            CreateFleetError::ResourceAlreadyExists(ref cause) => cause,
            CreateFleetError::ResourceNotFound(ref cause) => cause,
            CreateFleetError::TooManyRequests(ref cause) => cause,
            CreateFleetError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteFleet
#[derive(Debug, PartialEq)]
pub enum DeleteFleetError {
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The requested resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The number of requests exceeds the limit.</p>
    TooManyRequests(String),
    /// <p>You are not authorized to perform this action.</p>
    Unauthorized(String),
}

impl DeleteFleetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteFleetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteFleetError::InternalServerError(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteFleetError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteFleetError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteFleetError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DeleteFleetError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteFleetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteFleetError {
    fn description(&self) -> &str {
        match *self {
            DeleteFleetError::InternalServerError(ref cause) => cause,
            DeleteFleetError::InvalidRequest(ref cause) => cause,
            DeleteFleetError::ResourceNotFound(ref cause) => cause,
            DeleteFleetError::TooManyRequests(ref cause) => cause,
            DeleteFleetError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeAuditStreamConfiguration
#[derive(Debug, PartialEq)]
pub enum DescribeAuditStreamConfigurationError {
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The requested resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The number of requests exceeds the limit.</p>
    TooManyRequests(String),
    /// <p>You are not authorized to perform this action.</p>
    Unauthorized(String),
}

impl DescribeAuditStreamConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeAuditStreamConfigurationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        DescribeAuditStreamConfigurationError::InternalServerError(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        DescribeAuditStreamConfigurationError::InvalidRequest(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DescribeAuditStreamConfigurationError::ResourceNotFound(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        DescribeAuditStreamConfigurationError::TooManyRequests(err.msg),
                    )
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(
                        DescribeAuditStreamConfigurationError::Unauthorized(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeAuditStreamConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeAuditStreamConfigurationError {
    fn description(&self) -> &str {
        match *self {
            DescribeAuditStreamConfigurationError::InternalServerError(ref cause) => cause,
            DescribeAuditStreamConfigurationError::InvalidRequest(ref cause) => cause,
            DescribeAuditStreamConfigurationError::ResourceNotFound(ref cause) => cause,
            DescribeAuditStreamConfigurationError::TooManyRequests(ref cause) => cause,
            DescribeAuditStreamConfigurationError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeCompanyNetworkConfiguration
#[derive(Debug, PartialEq)]
pub enum DescribeCompanyNetworkConfigurationError {
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The requested resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The number of requests exceeds the limit.</p>
    TooManyRequests(String),
    /// <p>You are not authorized to perform this action.</p>
    Unauthorized(String),
}

impl DescribeCompanyNetworkConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeCompanyNetworkConfigurationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        DescribeCompanyNetworkConfigurationError::InternalServerError(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        DescribeCompanyNetworkConfigurationError::InvalidRequest(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DescribeCompanyNetworkConfigurationError::ResourceNotFound(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        DescribeCompanyNetworkConfigurationError::TooManyRequests(err.msg),
                    )
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(
                        DescribeCompanyNetworkConfigurationError::Unauthorized(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeCompanyNetworkConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeCompanyNetworkConfigurationError {
    fn description(&self) -> &str {
        match *self {
            DescribeCompanyNetworkConfigurationError::InternalServerError(ref cause) => cause,
            DescribeCompanyNetworkConfigurationError::InvalidRequest(ref cause) => cause,
            DescribeCompanyNetworkConfigurationError::ResourceNotFound(ref cause) => cause,
            DescribeCompanyNetworkConfigurationError::TooManyRequests(ref cause) => cause,
            DescribeCompanyNetworkConfigurationError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeDevice
#[derive(Debug, PartialEq)]
pub enum DescribeDeviceError {
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The requested resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The number of requests exceeds the limit.</p>
    TooManyRequests(String),
    /// <p>You are not authorized to perform this action.</p>
    Unauthorized(String),
}

impl DescribeDeviceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeDeviceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerErrorException" => {
                    return RusotoError::Service(DescribeDeviceError::InternalServerError(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeDeviceError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeDeviceError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeDeviceError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DescribeDeviceError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeDeviceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeDeviceError {
    fn description(&self) -> &str {
        match *self {
            DescribeDeviceError::InternalServerError(ref cause) => cause,
            DescribeDeviceError::InvalidRequest(ref cause) => cause,
            DescribeDeviceError::ResourceNotFound(ref cause) => cause,
            DescribeDeviceError::TooManyRequests(ref cause) => cause,
            DescribeDeviceError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeDevicePolicyConfiguration
#[derive(Debug, PartialEq)]
pub enum DescribeDevicePolicyConfigurationError {
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The requested resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The number of requests exceeds the limit.</p>
    TooManyRequests(String),
    /// <p>You are not authorized to perform this action.</p>
    Unauthorized(String),
}

impl DescribeDevicePolicyConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeDevicePolicyConfigurationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        DescribeDevicePolicyConfigurationError::InternalServerError(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        DescribeDevicePolicyConfigurationError::InvalidRequest(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DescribeDevicePolicyConfigurationError::ResourceNotFound(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        DescribeDevicePolicyConfigurationError::TooManyRequests(err.msg),
                    )
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(
                        DescribeDevicePolicyConfigurationError::Unauthorized(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeDevicePolicyConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeDevicePolicyConfigurationError {
    fn description(&self) -> &str {
        match *self {
            DescribeDevicePolicyConfigurationError::InternalServerError(ref cause) => cause,
            DescribeDevicePolicyConfigurationError::InvalidRequest(ref cause) => cause,
            DescribeDevicePolicyConfigurationError::ResourceNotFound(ref cause) => cause,
            DescribeDevicePolicyConfigurationError::TooManyRequests(ref cause) => cause,
            DescribeDevicePolicyConfigurationError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeDomain
#[derive(Debug, PartialEq)]
pub enum DescribeDomainError {
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The requested resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The number of requests exceeds the limit.</p>
    TooManyRequests(String),
    /// <p>You are not authorized to perform this action.</p>
    Unauthorized(String),
}

impl DescribeDomainError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeDomainError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerErrorException" => {
                    return RusotoError::Service(DescribeDomainError::InternalServerError(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeDomainError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeDomainError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeDomainError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DescribeDomainError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeDomainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeDomainError {
    fn description(&self) -> &str {
        match *self {
            DescribeDomainError::InternalServerError(ref cause) => cause,
            DescribeDomainError::InvalidRequest(ref cause) => cause,
            DescribeDomainError::ResourceNotFound(ref cause) => cause,
            DescribeDomainError::TooManyRequests(ref cause) => cause,
            DescribeDomainError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeFleetMetadata
#[derive(Debug, PartialEq)]
pub enum DescribeFleetMetadataError {
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The requested resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The number of requests exceeds the limit.</p>
    TooManyRequests(String),
    /// <p>You are not authorized to perform this action.</p>
    Unauthorized(String),
}

impl DescribeFleetMetadataError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeFleetMetadataError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerErrorException" => {
                    return RusotoError::Service(DescribeFleetMetadataError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeFleetMetadataError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeFleetMetadataError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeFleetMetadataError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DescribeFleetMetadataError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeFleetMetadataError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeFleetMetadataError {
    fn description(&self) -> &str {
        match *self {
            DescribeFleetMetadataError::InternalServerError(ref cause) => cause,
            DescribeFleetMetadataError::InvalidRequest(ref cause) => cause,
            DescribeFleetMetadataError::ResourceNotFound(ref cause) => cause,
            DescribeFleetMetadataError::TooManyRequests(ref cause) => cause,
            DescribeFleetMetadataError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeIdentityProviderConfiguration
#[derive(Debug, PartialEq)]
pub enum DescribeIdentityProviderConfigurationError {
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The requested resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The number of requests exceeds the limit.</p>
    TooManyRequests(String),
    /// <p>You are not authorized to perform this action.</p>
    Unauthorized(String),
}

impl DescribeIdentityProviderConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeIdentityProviderConfigurationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        DescribeIdentityProviderConfigurationError::InternalServerError(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        DescribeIdentityProviderConfigurationError::InvalidRequest(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DescribeIdentityProviderConfigurationError::ResourceNotFound(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        DescribeIdentityProviderConfigurationError::TooManyRequests(err.msg),
                    )
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(
                        DescribeIdentityProviderConfigurationError::Unauthorized(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeIdentityProviderConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeIdentityProviderConfigurationError {
    fn description(&self) -> &str {
        match *self {
            DescribeIdentityProviderConfigurationError::InternalServerError(ref cause) => cause,
            DescribeIdentityProviderConfigurationError::InvalidRequest(ref cause) => cause,
            DescribeIdentityProviderConfigurationError::ResourceNotFound(ref cause) => cause,
            DescribeIdentityProviderConfigurationError::TooManyRequests(ref cause) => cause,
            DescribeIdentityProviderConfigurationError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeWebsiteCertificateAuthority
#[derive(Debug, PartialEq)]
pub enum DescribeWebsiteCertificateAuthorityError {
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The requested resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The number of requests exceeds the limit.</p>
    TooManyRequests(String),
    /// <p>You are not authorized to perform this action.</p>
    Unauthorized(String),
}

impl DescribeWebsiteCertificateAuthorityError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeWebsiteCertificateAuthorityError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        DescribeWebsiteCertificateAuthorityError::InternalServerError(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        DescribeWebsiteCertificateAuthorityError::InvalidRequest(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DescribeWebsiteCertificateAuthorityError::ResourceNotFound(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        DescribeWebsiteCertificateAuthorityError::TooManyRequests(err.msg),
                    )
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(
                        DescribeWebsiteCertificateAuthorityError::Unauthorized(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeWebsiteCertificateAuthorityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeWebsiteCertificateAuthorityError {
    fn description(&self) -> &str {
        match *self {
            DescribeWebsiteCertificateAuthorityError::InternalServerError(ref cause) => cause,
            DescribeWebsiteCertificateAuthorityError::InvalidRequest(ref cause) => cause,
            DescribeWebsiteCertificateAuthorityError::ResourceNotFound(ref cause) => cause,
            DescribeWebsiteCertificateAuthorityError::TooManyRequests(ref cause) => cause,
            DescribeWebsiteCertificateAuthorityError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by DisassociateDomain
#[derive(Debug, PartialEq)]
pub enum DisassociateDomainError {
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The requested resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The number of requests exceeds the limit.</p>
    TooManyRequests(String),
    /// <p>You are not authorized to perform this action.</p>
    Unauthorized(String),
}

impl DisassociateDomainError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisassociateDomainError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerErrorException" => {
                    return RusotoError::Service(DisassociateDomainError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DisassociateDomainError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DisassociateDomainError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DisassociateDomainError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DisassociateDomainError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DisassociateDomainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisassociateDomainError {
    fn description(&self) -> &str {
        match *self {
            DisassociateDomainError::InternalServerError(ref cause) => cause,
            DisassociateDomainError::InvalidRequest(ref cause) => cause,
            DisassociateDomainError::ResourceNotFound(ref cause) => cause,
            DisassociateDomainError::TooManyRequests(ref cause) => cause,
            DisassociateDomainError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by DisassociateWebsiteAuthorizationProvider
#[derive(Debug, PartialEq)]
pub enum DisassociateWebsiteAuthorizationProviderError {
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The resource already exists.</p>
    ResourceAlreadyExists(String),
    /// <p>The requested resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The number of requests exceeds the limit.</p>
    TooManyRequests(String),
    /// <p>You are not authorized to perform this action.</p>
    Unauthorized(String),
}

impl DisassociateWebsiteAuthorizationProviderError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisassociateWebsiteAuthorizationProviderError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        DisassociateWebsiteAuthorizationProviderError::InternalServerError(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        DisassociateWebsiteAuthorizationProviderError::InvalidRequest(err.msg),
                    )
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(
                        DisassociateWebsiteAuthorizationProviderError::ResourceAlreadyExists(
                            err.msg,
                        ),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DisassociateWebsiteAuthorizationProviderError::ResourceNotFound(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        DisassociateWebsiteAuthorizationProviderError::TooManyRequests(err.msg),
                    )
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(
                        DisassociateWebsiteAuthorizationProviderError::Unauthorized(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DisassociateWebsiteAuthorizationProviderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisassociateWebsiteAuthorizationProviderError {
    fn description(&self) -> &str {
        match *self {
            DisassociateWebsiteAuthorizationProviderError::InternalServerError(ref cause) => cause,
            DisassociateWebsiteAuthorizationProviderError::InvalidRequest(ref cause) => cause,
            DisassociateWebsiteAuthorizationProviderError::ResourceAlreadyExists(ref cause) => {
                cause
            }
            DisassociateWebsiteAuthorizationProviderError::ResourceNotFound(ref cause) => cause,
            DisassociateWebsiteAuthorizationProviderError::TooManyRequests(ref cause) => cause,
            DisassociateWebsiteAuthorizationProviderError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by DisassociateWebsiteCertificateAuthority
#[derive(Debug, PartialEq)]
pub enum DisassociateWebsiteCertificateAuthorityError {
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The requested resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The number of requests exceeds the limit.</p>
    TooManyRequests(String),
    /// <p>You are not authorized to perform this action.</p>
    Unauthorized(String),
}

impl DisassociateWebsiteCertificateAuthorityError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisassociateWebsiteCertificateAuthorityError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        DisassociateWebsiteCertificateAuthorityError::InternalServerError(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        DisassociateWebsiteCertificateAuthorityError::InvalidRequest(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DisassociateWebsiteCertificateAuthorityError::ResourceNotFound(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        DisassociateWebsiteCertificateAuthorityError::TooManyRequests(err.msg),
                    )
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(
                        DisassociateWebsiteCertificateAuthorityError::Unauthorized(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DisassociateWebsiteCertificateAuthorityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisassociateWebsiteCertificateAuthorityError {
    fn description(&self) -> &str {
        match *self {
            DisassociateWebsiteCertificateAuthorityError::InternalServerError(ref cause) => cause,
            DisassociateWebsiteCertificateAuthorityError::InvalidRequest(ref cause) => cause,
            DisassociateWebsiteCertificateAuthorityError::ResourceNotFound(ref cause) => cause,
            DisassociateWebsiteCertificateAuthorityError::TooManyRequests(ref cause) => cause,
            DisassociateWebsiteCertificateAuthorityError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by ListDevices
#[derive(Debug, PartialEq)]
pub enum ListDevicesError {
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The requested resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The number of requests exceeds the limit.</p>
    TooManyRequests(String),
    /// <p>You are not authorized to perform this action.</p>
    Unauthorized(String),
}

impl ListDevicesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDevicesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListDevicesError::InternalServerError(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListDevicesError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListDevicesError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListDevicesError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(ListDevicesError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListDevicesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListDevicesError {
    fn description(&self) -> &str {
        match *self {
            ListDevicesError::InternalServerError(ref cause) => cause,
            ListDevicesError::InvalidRequest(ref cause) => cause,
            ListDevicesError::ResourceNotFound(ref cause) => cause,
            ListDevicesError::TooManyRequests(ref cause) => cause,
            ListDevicesError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by ListDomains
#[derive(Debug, PartialEq)]
pub enum ListDomainsError {
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The number of requests exceeds the limit.</p>
    TooManyRequests(String),
    /// <p>You are not authorized to perform this action.</p>
    Unauthorized(String),
}

impl ListDomainsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDomainsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListDomainsError::InternalServerError(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListDomainsError::InvalidRequest(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListDomainsError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(ListDomainsError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListDomainsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListDomainsError {
    fn description(&self) -> &str {
        match *self {
            ListDomainsError::InternalServerError(ref cause) => cause,
            ListDomainsError::InvalidRequest(ref cause) => cause,
            ListDomainsError::TooManyRequests(ref cause) => cause,
            ListDomainsError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by ListFleets
#[derive(Debug, PartialEq)]
pub enum ListFleetsError {
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The number of requests exceeds the limit.</p>
    TooManyRequests(String),
    /// <p>You are not authorized to perform this action.</p>
    Unauthorized(String),
}

impl ListFleetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListFleetsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListFleetsError::InternalServerError(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListFleetsError::InvalidRequest(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListFleetsError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(ListFleetsError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListFleetsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListFleetsError {
    fn description(&self) -> &str {
        match *self {
            ListFleetsError::InternalServerError(ref cause) => cause,
            ListFleetsError::InvalidRequest(ref cause) => cause,
            ListFleetsError::TooManyRequests(ref cause) => cause,
            ListFleetsError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by ListWebsiteAuthorizationProviders
#[derive(Debug, PartialEq)]
pub enum ListWebsiteAuthorizationProvidersError {
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The requested resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The number of requests exceeds the limit.</p>
    TooManyRequests(String),
    /// <p>You are not authorized to perform this action.</p>
    Unauthorized(String),
}

impl ListWebsiteAuthorizationProvidersError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListWebsiteAuthorizationProvidersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        ListWebsiteAuthorizationProvidersError::InternalServerError(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        ListWebsiteAuthorizationProvidersError::InvalidRequest(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        ListWebsiteAuthorizationProvidersError::ResourceNotFound(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        ListWebsiteAuthorizationProvidersError::TooManyRequests(err.msg),
                    )
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(
                        ListWebsiteAuthorizationProvidersError::Unauthorized(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListWebsiteAuthorizationProvidersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListWebsiteAuthorizationProvidersError {
    fn description(&self) -> &str {
        match *self {
            ListWebsiteAuthorizationProvidersError::InternalServerError(ref cause) => cause,
            ListWebsiteAuthorizationProvidersError::InvalidRequest(ref cause) => cause,
            ListWebsiteAuthorizationProvidersError::ResourceNotFound(ref cause) => cause,
            ListWebsiteAuthorizationProvidersError::TooManyRequests(ref cause) => cause,
            ListWebsiteAuthorizationProvidersError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by ListWebsiteCertificateAuthorities
#[derive(Debug, PartialEq)]
pub enum ListWebsiteCertificateAuthoritiesError {
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The number of requests exceeds the limit.</p>
    TooManyRequests(String),
    /// <p>You are not authorized to perform this action.</p>
    Unauthorized(String),
}

impl ListWebsiteCertificateAuthoritiesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListWebsiteCertificateAuthoritiesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        ListWebsiteCertificateAuthoritiesError::InternalServerError(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        ListWebsiteCertificateAuthoritiesError::InvalidRequest(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        ListWebsiteCertificateAuthoritiesError::TooManyRequests(err.msg),
                    )
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(
                        ListWebsiteCertificateAuthoritiesError::Unauthorized(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListWebsiteCertificateAuthoritiesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListWebsiteCertificateAuthoritiesError {
    fn description(&self) -> &str {
        match *self {
            ListWebsiteCertificateAuthoritiesError::InternalServerError(ref cause) => cause,
            ListWebsiteCertificateAuthoritiesError::InvalidRequest(ref cause) => cause,
            ListWebsiteCertificateAuthoritiesError::TooManyRequests(ref cause) => cause,
            ListWebsiteCertificateAuthoritiesError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by RestoreDomainAccess
#[derive(Debug, PartialEq)]
pub enum RestoreDomainAccessError {
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The requested resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The number of requests exceeds the limit.</p>
    TooManyRequests(String),
    /// <p>You are not authorized to perform this action.</p>
    Unauthorized(String),
}

impl RestoreDomainAccessError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RestoreDomainAccessError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerErrorException" => {
                    return RusotoError::Service(RestoreDomainAccessError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(RestoreDomainAccessError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(RestoreDomainAccessError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(RestoreDomainAccessError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(RestoreDomainAccessError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for RestoreDomainAccessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RestoreDomainAccessError {
    fn description(&self) -> &str {
        match *self {
            RestoreDomainAccessError::InternalServerError(ref cause) => cause,
            RestoreDomainAccessError::InvalidRequest(ref cause) => cause,
            RestoreDomainAccessError::ResourceNotFound(ref cause) => cause,
            RestoreDomainAccessError::TooManyRequests(ref cause) => cause,
            RestoreDomainAccessError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by RevokeDomainAccess
#[derive(Debug, PartialEq)]
pub enum RevokeDomainAccessError {
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The requested resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The number of requests exceeds the limit.</p>
    TooManyRequests(String),
    /// <p>You are not authorized to perform this action.</p>
    Unauthorized(String),
}

impl RevokeDomainAccessError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RevokeDomainAccessError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerErrorException" => {
                    return RusotoError::Service(RevokeDomainAccessError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(RevokeDomainAccessError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(RevokeDomainAccessError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(RevokeDomainAccessError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(RevokeDomainAccessError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for RevokeDomainAccessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RevokeDomainAccessError {
    fn description(&self) -> &str {
        match *self {
            RevokeDomainAccessError::InternalServerError(ref cause) => cause,
            RevokeDomainAccessError::InvalidRequest(ref cause) => cause,
            RevokeDomainAccessError::ResourceNotFound(ref cause) => cause,
            RevokeDomainAccessError::TooManyRequests(ref cause) => cause,
            RevokeDomainAccessError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by SignOutUser
#[derive(Debug, PartialEq)]
pub enum SignOutUserError {
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The requested resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The number of requests exceeds the limit.</p>
    TooManyRequests(String),
    /// <p>You are not authorized to perform this action.</p>
    Unauthorized(String),
}

impl SignOutUserError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SignOutUserError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerErrorException" => {
                    return RusotoError::Service(SignOutUserError::InternalServerError(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(SignOutUserError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(SignOutUserError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(SignOutUserError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(SignOutUserError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for SignOutUserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SignOutUserError {
    fn description(&self) -> &str {
        match *self {
            SignOutUserError::InternalServerError(ref cause) => cause,
            SignOutUserError::InvalidRequest(ref cause) => cause,
            SignOutUserError::ResourceNotFound(ref cause) => cause,
            SignOutUserError::TooManyRequests(ref cause) => cause,
            SignOutUserError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateAuditStreamConfiguration
#[derive(Debug, PartialEq)]
pub enum UpdateAuditStreamConfigurationError {
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The requested resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The number of requests exceeds the limit.</p>
    TooManyRequests(String),
    /// <p>You are not authorized to perform this action.</p>
    Unauthorized(String),
}

impl UpdateAuditStreamConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateAuditStreamConfigurationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        UpdateAuditStreamConfigurationError::InternalServerError(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        UpdateAuditStreamConfigurationError::InvalidRequest(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        UpdateAuditStreamConfigurationError::ResourceNotFound(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        UpdateAuditStreamConfigurationError::TooManyRequests(err.msg),
                    )
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(UpdateAuditStreamConfigurationError::Unauthorized(
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
impl fmt::Display for UpdateAuditStreamConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateAuditStreamConfigurationError {
    fn description(&self) -> &str {
        match *self {
            UpdateAuditStreamConfigurationError::InternalServerError(ref cause) => cause,
            UpdateAuditStreamConfigurationError::InvalidRequest(ref cause) => cause,
            UpdateAuditStreamConfigurationError::ResourceNotFound(ref cause) => cause,
            UpdateAuditStreamConfigurationError::TooManyRequests(ref cause) => cause,
            UpdateAuditStreamConfigurationError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateCompanyNetworkConfiguration
#[derive(Debug, PartialEq)]
pub enum UpdateCompanyNetworkConfigurationError {
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The requested resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The number of requests exceeds the limit.</p>
    TooManyRequests(String),
    /// <p>You are not authorized to perform this action.</p>
    Unauthorized(String),
}

impl UpdateCompanyNetworkConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateCompanyNetworkConfigurationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        UpdateCompanyNetworkConfigurationError::InternalServerError(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        UpdateCompanyNetworkConfigurationError::InvalidRequest(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        UpdateCompanyNetworkConfigurationError::ResourceNotFound(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        UpdateCompanyNetworkConfigurationError::TooManyRequests(err.msg),
                    )
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(
                        UpdateCompanyNetworkConfigurationError::Unauthorized(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateCompanyNetworkConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateCompanyNetworkConfigurationError {
    fn description(&self) -> &str {
        match *self {
            UpdateCompanyNetworkConfigurationError::InternalServerError(ref cause) => cause,
            UpdateCompanyNetworkConfigurationError::InvalidRequest(ref cause) => cause,
            UpdateCompanyNetworkConfigurationError::ResourceNotFound(ref cause) => cause,
            UpdateCompanyNetworkConfigurationError::TooManyRequests(ref cause) => cause,
            UpdateCompanyNetworkConfigurationError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateDevicePolicyConfiguration
#[derive(Debug, PartialEq)]
pub enum UpdateDevicePolicyConfigurationError {
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The requested resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The number of requests exceeds the limit.</p>
    TooManyRequests(String),
    /// <p>You are not authorized to perform this action.</p>
    Unauthorized(String),
}

impl UpdateDevicePolicyConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateDevicePolicyConfigurationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        UpdateDevicePolicyConfigurationError::InternalServerError(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        UpdateDevicePolicyConfigurationError::InvalidRequest(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        UpdateDevicePolicyConfigurationError::ResourceNotFound(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        UpdateDevicePolicyConfigurationError::TooManyRequests(err.msg),
                    )
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(
                        UpdateDevicePolicyConfigurationError::Unauthorized(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateDevicePolicyConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateDevicePolicyConfigurationError {
    fn description(&self) -> &str {
        match *self {
            UpdateDevicePolicyConfigurationError::InternalServerError(ref cause) => cause,
            UpdateDevicePolicyConfigurationError::InvalidRequest(ref cause) => cause,
            UpdateDevicePolicyConfigurationError::ResourceNotFound(ref cause) => cause,
            UpdateDevicePolicyConfigurationError::TooManyRequests(ref cause) => cause,
            UpdateDevicePolicyConfigurationError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateDomainMetadata
#[derive(Debug, PartialEq)]
pub enum UpdateDomainMetadataError {
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The requested resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The number of requests exceeds the limit.</p>
    TooManyRequests(String),
    /// <p>You are not authorized to perform this action.</p>
    Unauthorized(String),
}

impl UpdateDomainMetadataError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateDomainMetadataError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateDomainMetadataError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateDomainMetadataError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateDomainMetadataError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateDomainMetadataError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(UpdateDomainMetadataError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateDomainMetadataError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateDomainMetadataError {
    fn description(&self) -> &str {
        match *self {
            UpdateDomainMetadataError::InternalServerError(ref cause) => cause,
            UpdateDomainMetadataError::InvalidRequest(ref cause) => cause,
            UpdateDomainMetadataError::ResourceNotFound(ref cause) => cause,
            UpdateDomainMetadataError::TooManyRequests(ref cause) => cause,
            UpdateDomainMetadataError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateFleetMetadata
#[derive(Debug, PartialEq)]
pub enum UpdateFleetMetadataError {
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The requested resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The number of requests exceeds the limit.</p>
    TooManyRequests(String),
    /// <p>You are not authorized to perform this action.</p>
    Unauthorized(String),
}

impl UpdateFleetMetadataError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateFleetMetadataError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateFleetMetadataError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateFleetMetadataError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateFleetMetadataError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateFleetMetadataError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(UpdateFleetMetadataError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateFleetMetadataError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateFleetMetadataError {
    fn description(&self) -> &str {
        match *self {
            UpdateFleetMetadataError::InternalServerError(ref cause) => cause,
            UpdateFleetMetadataError::InvalidRequest(ref cause) => cause,
            UpdateFleetMetadataError::ResourceNotFound(ref cause) => cause,
            UpdateFleetMetadataError::TooManyRequests(ref cause) => cause,
            UpdateFleetMetadataError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateIdentityProviderConfiguration
#[derive(Debug, PartialEq)]
pub enum UpdateIdentityProviderConfigurationError {
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The requested resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The number of requests exceeds the limit.</p>
    TooManyRequests(String),
    /// <p>You are not authorized to perform this action.</p>
    Unauthorized(String),
}

impl UpdateIdentityProviderConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateIdentityProviderConfigurationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        UpdateIdentityProviderConfigurationError::InternalServerError(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        UpdateIdentityProviderConfigurationError::InvalidRequest(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        UpdateIdentityProviderConfigurationError::ResourceNotFound(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        UpdateIdentityProviderConfigurationError::TooManyRequests(err.msg),
                    )
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(
                        UpdateIdentityProviderConfigurationError::Unauthorized(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateIdentityProviderConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateIdentityProviderConfigurationError {
    fn description(&self) -> &str {
        match *self {
            UpdateIdentityProviderConfigurationError::InternalServerError(ref cause) => cause,
            UpdateIdentityProviderConfigurationError::InvalidRequest(ref cause) => cause,
            UpdateIdentityProviderConfigurationError::ResourceNotFound(ref cause) => cause,
            UpdateIdentityProviderConfigurationError::TooManyRequests(ref cause) => cause,
            UpdateIdentityProviderConfigurationError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the WorkLink API. WorkLink clients implement this trait.
pub trait Worklink {
    /// <p>Specifies a domain to be associated to Amazon WorkLink.</p>
    fn associate_domain(
        &self,
        input: AssociateDomainRequest,
    ) -> RusotoFuture<AssociateDomainResponse, AssociateDomainError>;

    /// <p>Associates a website authorization provider with a specified fleet. This is used to authorize users against associated websites in the company network.</p>
    fn associate_website_authorization_provider(
        &self,
        input: AssociateWebsiteAuthorizationProviderRequest,
    ) -> RusotoFuture<
        AssociateWebsiteAuthorizationProviderResponse,
        AssociateWebsiteAuthorizationProviderError,
    >;

    /// <p>Imports the root certificate of a certificate authority (CA) used to obtain TLS certificates used by associated websites within the company network.</p>
    fn associate_website_certificate_authority(
        &self,
        input: AssociateWebsiteCertificateAuthorityRequest,
    ) -> RusotoFuture<
        AssociateWebsiteCertificateAuthorityResponse,
        AssociateWebsiteCertificateAuthorityError,
    >;

    /// <p>Creates a fleet. A fleet consists of resources and the configuration that delivers associated websites to authorized users who download and set up the Amazon WorkLink app.</p>
    fn create_fleet(
        &self,
        input: CreateFleetRequest,
    ) -> RusotoFuture<CreateFleetResponse, CreateFleetError>;

    /// <p>Deletes a fleet. Prevents users from accessing previously associated websites. </p>
    fn delete_fleet(
        &self,
        input: DeleteFleetRequest,
    ) -> RusotoFuture<DeleteFleetResponse, DeleteFleetError>;

    /// <p>Describes the configuration for delivering audit streams to the customer account.</p>
    fn describe_audit_stream_configuration(
        &self,
        input: DescribeAuditStreamConfigurationRequest,
    ) -> RusotoFuture<DescribeAuditStreamConfigurationResponse, DescribeAuditStreamConfigurationError>;

    /// <p>Describes the networking configuration to access the internal websites associated with the specified fleet.</p>
    fn describe_company_network_configuration(
        &self,
        input: DescribeCompanyNetworkConfigurationRequest,
    ) -> RusotoFuture<
        DescribeCompanyNetworkConfigurationResponse,
        DescribeCompanyNetworkConfigurationError,
    >;

    /// <p>Provides information about a user's device.</p>
    fn describe_device(
        &self,
        input: DescribeDeviceRequest,
    ) -> RusotoFuture<DescribeDeviceResponse, DescribeDeviceError>;

    /// <p>Describes the device policy configuration for the specified fleet.</p>
    fn describe_device_policy_configuration(
        &self,
        input: DescribeDevicePolicyConfigurationRequest,
    ) -> RusotoFuture<
        DescribeDevicePolicyConfigurationResponse,
        DescribeDevicePolicyConfigurationError,
    >;

    /// <p>Provides information about the domain.</p>
    fn describe_domain(
        &self,
        input: DescribeDomainRequest,
    ) -> RusotoFuture<DescribeDomainResponse, DescribeDomainError>;

    /// <p>Provides basic information for the specified fleet, excluding identity provider, networking, and device configuration details.</p>
    fn describe_fleet_metadata(
        &self,
        input: DescribeFleetMetadataRequest,
    ) -> RusotoFuture<DescribeFleetMetadataResponse, DescribeFleetMetadataError>;

    /// <p>Describes the identity provider configuration of the specified fleet.</p>
    fn describe_identity_provider_configuration(
        &self,
        input: DescribeIdentityProviderConfigurationRequest,
    ) -> RusotoFuture<
        DescribeIdentityProviderConfigurationResponse,
        DescribeIdentityProviderConfigurationError,
    >;

    /// <p>Provides information about the certificate authority.</p>
    fn describe_website_certificate_authority(
        &self,
        input: DescribeWebsiteCertificateAuthorityRequest,
    ) -> RusotoFuture<
        DescribeWebsiteCertificateAuthorityResponse,
        DescribeWebsiteCertificateAuthorityError,
    >;

    /// <p>Disassociates a domain from Amazon WorkLink. End users lose the ability to access the domain with Amazon WorkLink. </p>
    fn disassociate_domain(
        &self,
        input: DisassociateDomainRequest,
    ) -> RusotoFuture<DisassociateDomainResponse, DisassociateDomainError>;

    /// <p>Disassociates a website authorization provider from a specified fleet. After the disassociation, users can't load any associated websites that require this authorization provider.</p>
    fn disassociate_website_authorization_provider(
        &self,
        input: DisassociateWebsiteAuthorizationProviderRequest,
    ) -> RusotoFuture<
        DisassociateWebsiteAuthorizationProviderResponse,
        DisassociateWebsiteAuthorizationProviderError,
    >;

    /// <p>Removes a certificate authority (CA).</p>
    fn disassociate_website_certificate_authority(
        &self,
        input: DisassociateWebsiteCertificateAuthorityRequest,
    ) -> RusotoFuture<
        DisassociateWebsiteCertificateAuthorityResponse,
        DisassociateWebsiteCertificateAuthorityError,
    >;

    /// <p>Retrieves a list of devices registered with the specified fleet.</p>
    fn list_devices(
        &self,
        input: ListDevicesRequest,
    ) -> RusotoFuture<ListDevicesResponse, ListDevicesError>;

    /// <p>Retrieves a list of domains associated to a specified fleet.</p>
    fn list_domains(
        &self,
        input: ListDomainsRequest,
    ) -> RusotoFuture<ListDomainsResponse, ListDomainsError>;

    /// <p>Retrieves a list of fleets for the current account and Region.</p>
    fn list_fleets(
        &self,
        input: ListFleetsRequest,
    ) -> RusotoFuture<ListFleetsResponse, ListFleetsError>;

    /// <p>Retrieves a list of website authorization providers associated with a specified fleet.</p>
    fn list_website_authorization_providers(
        &self,
        input: ListWebsiteAuthorizationProvidersRequest,
    ) -> RusotoFuture<
        ListWebsiteAuthorizationProvidersResponse,
        ListWebsiteAuthorizationProvidersError,
    >;

    /// <p>Retrieves a list of certificate authorities added for the current account and Region.</p>
    fn list_website_certificate_authorities(
        &self,
        input: ListWebsiteCertificateAuthoritiesRequest,
    ) -> RusotoFuture<
        ListWebsiteCertificateAuthoritiesResponse,
        ListWebsiteCertificateAuthoritiesError,
    >;

    /// <p>Moves a domain to ACTIVE status if it was in the INACTIVE status.</p>
    fn restore_domain_access(
        &self,
        input: RestoreDomainAccessRequest,
    ) -> RusotoFuture<RestoreDomainAccessResponse, RestoreDomainAccessError>;

    /// <p>Moves a domain to INACTIVE status if it was in the ACTIVE status.</p>
    fn revoke_domain_access(
        &self,
        input: RevokeDomainAccessRequest,
    ) -> RusotoFuture<RevokeDomainAccessResponse, RevokeDomainAccessError>;

    /// <p>Signs the user out from all of their devices. The user can sign in again if they have valid credentials.</p>
    fn sign_out_user(
        &self,
        input: SignOutUserRequest,
    ) -> RusotoFuture<SignOutUserResponse, SignOutUserError>;

    /// <p>Updates the audit stream configuration for the fleet.</p>
    fn update_audit_stream_configuration(
        &self,
        input: UpdateAuditStreamConfigurationRequest,
    ) -> RusotoFuture<UpdateAuditStreamConfigurationResponse, UpdateAuditStreamConfigurationError>;

    /// <p>Updates the company network configuration for the fleet.</p>
    fn update_company_network_configuration(
        &self,
        input: UpdateCompanyNetworkConfigurationRequest,
    ) -> RusotoFuture<
        UpdateCompanyNetworkConfigurationResponse,
        UpdateCompanyNetworkConfigurationError,
    >;

    /// <p>Updates the device policy configuration for the fleet.</p>
    fn update_device_policy_configuration(
        &self,
        input: UpdateDevicePolicyConfigurationRequest,
    ) -> RusotoFuture<UpdateDevicePolicyConfigurationResponse, UpdateDevicePolicyConfigurationError>;

    /// <p>Updates domain metadata, such as DisplayName.</p>
    fn update_domain_metadata(
        &self,
        input: UpdateDomainMetadataRequest,
    ) -> RusotoFuture<UpdateDomainMetadataResponse, UpdateDomainMetadataError>;

    /// <p>Updates fleet metadata, such as DisplayName.</p>
    fn update_fleet_metadata(
        &self,
        input: UpdateFleetMetadataRequest,
    ) -> RusotoFuture<UpdateFleetMetadataResponse, UpdateFleetMetadataError>;

    /// <p>Updates the identity provider configuration for the fleet.</p>
    fn update_identity_provider_configuration(
        &self,
        input: UpdateIdentityProviderConfigurationRequest,
    ) -> RusotoFuture<
        UpdateIdentityProviderConfigurationResponse,
        UpdateIdentityProviderConfigurationError,
    >;
}
/// A client for the WorkLink API.
#[derive(Clone)]
pub struct WorklinkClient {
    client: Client,
    region: region::Region,
}

impl WorklinkClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> WorklinkClient {
        WorklinkClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> WorklinkClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        WorklinkClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }
}

impl Worklink for WorklinkClient {
    /// <p>Specifies a domain to be associated to Amazon WorkLink.</p>
    fn associate_domain(
        &self,
        input: AssociateDomainRequest,
    ) -> RusotoFuture<AssociateDomainResponse, AssociateDomainError> {
        let request_uri = "/associateDomain";

        let mut request = SignedRequest::new("POST", "worklink", &self.region, &request_uri);
        request.set_content_type("application/json".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<AssociateDomainResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(AssociateDomainError::from_response(response))),
                )
            }
        })
    }

    /// <p>Associates a website authorization provider with a specified fleet. This is used to authorize users against associated websites in the company network.</p>
    fn associate_website_authorization_provider(
        &self,
        input: AssociateWebsiteAuthorizationProviderRequest,
    ) -> RusotoFuture<
        AssociateWebsiteAuthorizationProviderResponse,
        AssociateWebsiteAuthorizationProviderError,
    > {
        let request_uri = "/associateWebsiteAuthorizationProvider";

        let mut request = SignedRequest::new("POST", "worklink", &self.region, &request_uri);
        request.set_content_type("application/json".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<AssociateWebsiteAuthorizationProviderResponse, _>(
                    )?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AssociateWebsiteAuthorizationProviderError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Imports the root certificate of a certificate authority (CA) used to obtain TLS certificates used by associated websites within the company network.</p>
    fn associate_website_certificate_authority(
        &self,
        input: AssociateWebsiteCertificateAuthorityRequest,
    ) -> RusotoFuture<
        AssociateWebsiteCertificateAuthorityResponse,
        AssociateWebsiteCertificateAuthorityError,
    > {
        let request_uri = "/associateWebsiteCertificateAuthority";

        let mut request = SignedRequest::new("POST", "worklink", &self.region, &request_uri);
        request.set_content_type("application/json".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<AssociateWebsiteCertificateAuthorityResponse, _>(
                    )?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AssociateWebsiteCertificateAuthorityError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Creates a fleet. A fleet consists of resources and the configuration that delivers associated websites to authorized users who download and set up the Amazon WorkLink app.</p>
    fn create_fleet(
        &self,
        input: CreateFleetRequest,
    ) -> RusotoFuture<CreateFleetResponse, CreateFleetError> {
        let request_uri = "/createFleet";

        let mut request = SignedRequest::new("POST", "worklink", &self.region, &request_uri);
        request.set_content_type("application/json".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateFleetResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateFleetError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a fleet. Prevents users from accessing previously associated websites. </p>
    fn delete_fleet(
        &self,
        input: DeleteFleetRequest,
    ) -> RusotoFuture<DeleteFleetResponse, DeleteFleetError> {
        let request_uri = "/deleteFleet";

        let mut request = SignedRequest::new("POST", "worklink", &self.region, &request_uri);
        request.set_content_type("application/json".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteFleetResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteFleetError::from_response(response))),
                )
            }
        })
    }

    /// <p>Describes the configuration for delivering audit streams to the customer account.</p>
    fn describe_audit_stream_configuration(
        &self,
        input: DescribeAuditStreamConfigurationRequest,
    ) -> RusotoFuture<DescribeAuditStreamConfigurationResponse, DescribeAuditStreamConfigurationError>
    {
        let request_uri = "/describeAuditStreamConfiguration";

        let mut request = SignedRequest::new("POST", "worklink", &self.region, &request_uri);
        request.set_content_type("application/json".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeAuditStreamConfigurationResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeAuditStreamConfigurationError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Describes the networking configuration to access the internal websites associated with the specified fleet.</p>
    fn describe_company_network_configuration(
        &self,
        input: DescribeCompanyNetworkConfigurationRequest,
    ) -> RusotoFuture<
        DescribeCompanyNetworkConfigurationResponse,
        DescribeCompanyNetworkConfigurationError,
    > {
        let request_uri = "/describeCompanyNetworkConfiguration";

        let mut request = SignedRequest::new("POST", "worklink", &self.region, &request_uri);
        request.set_content_type("application/json".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeCompanyNetworkConfigurationResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeCompanyNetworkConfigurationError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Provides information about a user's device.</p>
    fn describe_device(
        &self,
        input: DescribeDeviceRequest,
    ) -> RusotoFuture<DescribeDeviceResponse, DescribeDeviceError> {
        let request_uri = "/describeDevice";

        let mut request = SignedRequest::new("POST", "worklink", &self.region, &request_uri);
        request.set_content_type("application/json".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeDeviceResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeDeviceError::from_response(response))),
                )
            }
        })
    }

    /// <p>Describes the device policy configuration for the specified fleet.</p>
    fn describe_device_policy_configuration(
        &self,
        input: DescribeDevicePolicyConfigurationRequest,
    ) -> RusotoFuture<
        DescribeDevicePolicyConfigurationResponse,
        DescribeDevicePolicyConfigurationError,
    > {
        let request_uri = "/describeDevicePolicyConfiguration";

        let mut request = SignedRequest::new("POST", "worklink", &self.region, &request_uri);
        request.set_content_type("application/json".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeDevicePolicyConfigurationResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeDevicePolicyConfigurationError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Provides information about the domain.</p>
    fn describe_domain(
        &self,
        input: DescribeDomainRequest,
    ) -> RusotoFuture<DescribeDomainResponse, DescribeDomainError> {
        let request_uri = "/describeDomain";

        let mut request = SignedRequest::new("POST", "worklink", &self.region, &request_uri);
        request.set_content_type("application/json".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeDomainResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeDomainError::from_response(response))),
                )
            }
        })
    }

    /// <p>Provides basic information for the specified fleet, excluding identity provider, networking, and device configuration details.</p>
    fn describe_fleet_metadata(
        &self,
        input: DescribeFleetMetadataRequest,
    ) -> RusotoFuture<DescribeFleetMetadataResponse, DescribeFleetMetadataError> {
        let request_uri = "/describeFleetMetadata";

        let mut request = SignedRequest::new("POST", "worklink", &self.region, &request_uri);
        request.set_content_type("application/json".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeFleetMetadataResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DescribeFleetMetadataError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Describes the identity provider configuration of the specified fleet.</p>
    fn describe_identity_provider_configuration(
        &self,
        input: DescribeIdentityProviderConfigurationRequest,
    ) -> RusotoFuture<
        DescribeIdentityProviderConfigurationResponse,
        DescribeIdentityProviderConfigurationError,
    > {
        let request_uri = "/describeIdentityProviderConfiguration";

        let mut request = SignedRequest::new("POST", "worklink", &self.region, &request_uri);
        request.set_content_type("application/json".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeIdentityProviderConfigurationResponse, _>(
                    )?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeIdentityProviderConfigurationError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Provides information about the certificate authority.</p>
    fn describe_website_certificate_authority(
        &self,
        input: DescribeWebsiteCertificateAuthorityRequest,
    ) -> RusotoFuture<
        DescribeWebsiteCertificateAuthorityResponse,
        DescribeWebsiteCertificateAuthorityError,
    > {
        let request_uri = "/describeWebsiteCertificateAuthority";

        let mut request = SignedRequest::new("POST", "worklink", &self.region, &request_uri);
        request.set_content_type("application/json".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeWebsiteCertificateAuthorityResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeWebsiteCertificateAuthorityError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Disassociates a domain from Amazon WorkLink. End users lose the ability to access the domain with Amazon WorkLink. </p>
    fn disassociate_domain(
        &self,
        input: DisassociateDomainRequest,
    ) -> RusotoFuture<DisassociateDomainResponse, DisassociateDomainError> {
        let request_uri = "/disassociateDomain";

        let mut request = SignedRequest::new("POST", "worklink", &self.region, &request_uri);
        request.set_content_type("application/json".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DisassociateDomainResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DisassociateDomainError::from_response(response))),
                )
            }
        })
    }

    /// <p>Disassociates a website authorization provider from a specified fleet. After the disassociation, users can't load any associated websites that require this authorization provider.</p>
    fn disassociate_website_authorization_provider(
        &self,
        input: DisassociateWebsiteAuthorizationProviderRequest,
    ) -> RusotoFuture<
        DisassociateWebsiteAuthorizationProviderResponse,
        DisassociateWebsiteAuthorizationProviderError,
    > {
        let request_uri = "/disassociateWebsiteAuthorizationProvider";

        let mut request = SignedRequest::new("POST", "worklink", &self.region, &request_uri);
        request.set_content_type("application/json".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DisassociateWebsiteAuthorizationProviderResponse, _>(
                    )?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DisassociateWebsiteAuthorizationProviderError::from_response(response))
                }))
            }
        })
    }

    /// <p>Removes a certificate authority (CA).</p>
    fn disassociate_website_certificate_authority(
        &self,
        input: DisassociateWebsiteCertificateAuthorityRequest,
    ) -> RusotoFuture<
        DisassociateWebsiteCertificateAuthorityResponse,
        DisassociateWebsiteCertificateAuthorityError,
    > {
        let request_uri = "/disassociateWebsiteCertificateAuthority";

        let mut request = SignedRequest::new("POST", "worklink", &self.region, &request_uri);
        request.set_content_type("application/json".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DisassociateWebsiteCertificateAuthorityResponse, _>(
                    )?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DisassociateWebsiteCertificateAuthorityError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Retrieves a list of devices registered with the specified fleet.</p>
    fn list_devices(
        &self,
        input: ListDevicesRequest,
    ) -> RusotoFuture<ListDevicesResponse, ListDevicesError> {
        let request_uri = "/listDevices";

        let mut request = SignedRequest::new("POST", "worklink", &self.region, &request_uri);
        request.set_content_type("application/json".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListDevicesResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListDevicesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves a list of domains associated to a specified fleet.</p>
    fn list_domains(
        &self,
        input: ListDomainsRequest,
    ) -> RusotoFuture<ListDomainsResponse, ListDomainsError> {
        let request_uri = "/listDomains";

        let mut request = SignedRequest::new("POST", "worklink", &self.region, &request_uri);
        request.set_content_type("application/json".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListDomainsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListDomainsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves a list of fleets for the current account and Region.</p>
    fn list_fleets(
        &self,
        input: ListFleetsRequest,
    ) -> RusotoFuture<ListFleetsResponse, ListFleetsError> {
        let request_uri = "/listFleets";

        let mut request = SignedRequest::new("POST", "worklink", &self.region, &request_uri);
        request.set_content_type("application/json".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListFleetsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListFleetsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves a list of website authorization providers associated with a specified fleet.</p>
    fn list_website_authorization_providers(
        &self,
        input: ListWebsiteAuthorizationProvidersRequest,
    ) -> RusotoFuture<
        ListWebsiteAuthorizationProvidersResponse,
        ListWebsiteAuthorizationProvidersError,
    > {
        let request_uri = "/listWebsiteAuthorizationProviders";

        let mut request = SignedRequest::new("POST", "worklink", &self.region, &request_uri);
        request.set_content_type("application/json".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListWebsiteAuthorizationProvidersResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListWebsiteAuthorizationProvidersError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Retrieves a list of certificate authorities added for the current account and Region.</p>
    fn list_website_certificate_authorities(
        &self,
        input: ListWebsiteCertificateAuthoritiesRequest,
    ) -> RusotoFuture<
        ListWebsiteCertificateAuthoritiesResponse,
        ListWebsiteCertificateAuthoritiesError,
    > {
        let request_uri = "/listWebsiteCertificateAuthorities";

        let mut request = SignedRequest::new("POST", "worklink", &self.region, &request_uri);
        request.set_content_type("application/json".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListWebsiteCertificateAuthoritiesResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListWebsiteCertificateAuthoritiesError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Moves a domain to ACTIVE status if it was in the INACTIVE status.</p>
    fn restore_domain_access(
        &self,
        input: RestoreDomainAccessRequest,
    ) -> RusotoFuture<RestoreDomainAccessResponse, RestoreDomainAccessError> {
        let request_uri = "/restoreDomainAccess";

        let mut request = SignedRequest::new("POST", "worklink", &self.region, &request_uri);
        request.set_content_type("application/json".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<RestoreDomainAccessResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(RestoreDomainAccessError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Moves a domain to INACTIVE status if it was in the ACTIVE status.</p>
    fn revoke_domain_access(
        &self,
        input: RevokeDomainAccessRequest,
    ) -> RusotoFuture<RevokeDomainAccessResponse, RevokeDomainAccessError> {
        let request_uri = "/revokeDomainAccess";

        let mut request = SignedRequest::new("POST", "worklink", &self.region, &request_uri);
        request.set_content_type("application/json".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<RevokeDomainAccessResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(RevokeDomainAccessError::from_response(response))),
                )
            }
        })
    }

    /// <p>Signs the user out from all of their devices. The user can sign in again if they have valid credentials.</p>
    fn sign_out_user(
        &self,
        input: SignOutUserRequest,
    ) -> RusotoFuture<SignOutUserResponse, SignOutUserError> {
        let request_uri = "/signOutUser";

        let mut request = SignedRequest::new("POST", "worklink", &self.region, &request_uri);
        request.set_content_type("application/json".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<SignOutUserResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(SignOutUserError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates the audit stream configuration for the fleet.</p>
    fn update_audit_stream_configuration(
        &self,
        input: UpdateAuditStreamConfigurationRequest,
    ) -> RusotoFuture<UpdateAuditStreamConfigurationResponse, UpdateAuditStreamConfigurationError>
    {
        let request_uri = "/updateAuditStreamConfiguration";

        let mut request = SignedRequest::new("POST", "worklink", &self.region, &request_uri);
        request.set_content_type("application/json".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateAuditStreamConfigurationResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateAuditStreamConfigurationError::from_response(response))
                }))
            }
        })
    }

    /// <p>Updates the company network configuration for the fleet.</p>
    fn update_company_network_configuration(
        &self,
        input: UpdateCompanyNetworkConfigurationRequest,
    ) -> RusotoFuture<
        UpdateCompanyNetworkConfigurationResponse,
        UpdateCompanyNetworkConfigurationError,
    > {
        let request_uri = "/updateCompanyNetworkConfiguration";

        let mut request = SignedRequest::new("POST", "worklink", &self.region, &request_uri);
        request.set_content_type("application/json".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateCompanyNetworkConfigurationResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateCompanyNetworkConfigurationError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Updates the device policy configuration for the fleet.</p>
    fn update_device_policy_configuration(
        &self,
        input: UpdateDevicePolicyConfigurationRequest,
    ) -> RusotoFuture<UpdateDevicePolicyConfigurationResponse, UpdateDevicePolicyConfigurationError>
    {
        let request_uri = "/updateDevicePolicyConfiguration";

        let mut request = SignedRequest::new("POST", "worklink", &self.region, &request_uri);
        request.set_content_type("application/json".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateDevicePolicyConfigurationResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateDevicePolicyConfigurationError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Updates domain metadata, such as DisplayName.</p>
    fn update_domain_metadata(
        &self,
        input: UpdateDomainMetadataRequest,
    ) -> RusotoFuture<UpdateDomainMetadataResponse, UpdateDomainMetadataError> {
        let request_uri = "/updateDomainMetadata";

        let mut request = SignedRequest::new("POST", "worklink", &self.region, &request_uri);
        request.set_content_type("application/json".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateDomainMetadataResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateDomainMetadataError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Updates fleet metadata, such as DisplayName.</p>
    fn update_fleet_metadata(
        &self,
        input: UpdateFleetMetadataRequest,
    ) -> RusotoFuture<UpdateFleetMetadataResponse, UpdateFleetMetadataError> {
        let request_uri = "/UpdateFleetMetadata";

        let mut request = SignedRequest::new("POST", "worklink", &self.region, &request_uri);
        request.set_content_type("application/json".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateFleetMetadataResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateFleetMetadataError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Updates the identity provider configuration for the fleet.</p>
    fn update_identity_provider_configuration(
        &self,
        input: UpdateIdentityProviderConfigurationRequest,
    ) -> RusotoFuture<
        UpdateIdentityProviderConfigurationResponse,
        UpdateIdentityProviderConfigurationError,
    > {
        let request_uri = "/updateIdentityProviderConfiguration";

        let mut request = SignedRequest::new("POST", "worklink", &self.region, &request_uri);
        request.set_content_type("application/json".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateIdentityProviderConfigurationResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateIdentityProviderConfigurationError::from_response(
                        response,
                    ))
                }))
            }
        })
    }
}
