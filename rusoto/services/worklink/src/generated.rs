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

use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociateDomainResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociateWebsiteAuthorizationProviderResponse {
    /// <p>A unique identifier for the authorization provider.</p>
    #[serde(rename = "AuthorizationProviderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_provider_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociateWebsiteCertificateAuthorityResponse {
    /// <p>A unique identifier for the CA.</p>
    #[serde(rename = "WebsiteCaId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website_ca_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateFleetResponse {
    /// <p>The ARN of the fleet.</p>
    #[serde(rename = "FleetArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteFleetRequest {
    /// <p>The ARN of the fleet.</p>
    #[serde(rename = "FleetArn")]
    pub fleet_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteFleetResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeAuditStreamConfigurationRequest {
    /// <p>The ARN of the fleet.</p>
    #[serde(rename = "FleetArn")]
    pub fleet_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeAuditStreamConfigurationResponse {
    /// <p>The ARN of the Amazon Kinesis data stream that will receive the audit events.</p>
    #[serde(rename = "AuditStreamArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_stream_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeCompanyNetworkConfigurationRequest {
    /// <p>The ARN of the fleet.</p>
    #[serde(rename = "FleetArn")]
    pub fleet_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDevicePolicyConfigurationRequest {
    /// <p>The ARN of the fleet.</p>
    #[serde(rename = "FleetArn")]
    pub fleet_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeDevicePolicyConfigurationResponse {
    /// <p>The certificate chain, including intermediate certificates and the root certificate authority certificate used to issue device certificates.</p>
    #[serde(rename = "DeviceCaCertificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_ca_certificate: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDeviceRequest {
    /// <p>A unique identifier for a registered user's device.</p>
    #[serde(rename = "DeviceId")]
    pub device_id: String,
    /// <p>The ARN of the fleet.</p>
    #[serde(rename = "FleetArn")]
    pub fleet_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDomainRequest {
    /// <p>The name of the domain.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// <p>The ARN of the fleet.</p>
    #[serde(rename = "FleetArn")]
    pub fleet_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeFleetMetadataRequest {
    /// <p>The ARN of the fleet.</p>
    #[serde(rename = "FleetArn")]
    pub fleet_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeIdentityProviderConfigurationRequest {
    /// <p>The ARN of the fleet.</p>
    #[serde(rename = "FleetArn")]
    pub fleet_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeWebsiteCertificateAuthorityRequest {
    /// <p>The ARN of the fleet.</p>
    #[serde(rename = "FleetArn")]
    pub fleet_arn: String,
    /// <p>A unique identifier for the certificate authority.</p>
    #[serde(rename = "WebsiteCaId")]
    pub website_ca_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateDomainRequest {
    /// <p>The name of the domain.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// <p>The ARN of the fleet.</p>
    #[serde(rename = "FleetArn")]
    pub fleet_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisassociateDomainResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateWebsiteAuthorizationProviderRequest {
    /// <p>A unique identifier for the authorization provider.</p>
    #[serde(rename = "AuthorizationProviderId")]
    pub authorization_provider_id: String,
    /// <p>The ARN of the fleet.</p>
    #[serde(rename = "FleetArn")]
    pub fleet_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisassociateWebsiteAuthorizationProviderResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateWebsiteCertificateAuthorityRequest {
    /// <p>The ARN of the fleet.</p>
    #[serde(rename = "FleetArn")]
    pub fleet_arn: String,
    /// <p>A unique identifier for the CA.</p>
    #[serde(rename = "WebsiteCaId")]
    pub website_ca_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisassociateWebsiteCertificateAuthorityResponse {}

/// <p>The summary of the domain.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RestoreDomainAccessRequest {
    /// <p>The name of the domain.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// <p>The ARN of the fleet.</p>
    #[serde(rename = "FleetArn")]
    pub fleet_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RestoreDomainAccessResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RevokeDomainAccessRequest {
    /// <p>The name of the domain.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// <p>The ARN of the fleet.</p>
    #[serde(rename = "FleetArn")]
    pub fleet_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RevokeDomainAccessResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SignOutUserRequest {
    /// <p>The ARN of the fleet.</p>
    #[serde(rename = "FleetArn")]
    pub fleet_arn: String,
    /// <p>The name of the user.</p>
    #[serde(rename = "Username")]
    pub username: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SignOutUserResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateAuditStreamConfigurationResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateCompanyNetworkConfigurationResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateDevicePolicyConfigurationResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateDomainMetadataResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateFleetMetadataResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateIdentityProviderConfigurationResponse {}

/// <p>The summary of the website authorization provider.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AssociateDomainError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociateDomainError::InternalServerError(ref cause) => write!(f, "{}", cause),
            AssociateDomainError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            AssociateDomainError::ResourceAlreadyExists(ref cause) => write!(f, "{}", cause),
            AssociateDomainError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            AssociateDomainError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            AssociateDomainError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AssociateDomainError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AssociateWebsiteAuthorizationProviderError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociateWebsiteAuthorizationProviderError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateWebsiteAuthorizationProviderError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateWebsiteAuthorizationProviderError::ResourceAlreadyExists(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateWebsiteAuthorizationProviderError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateWebsiteAuthorizationProviderError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateWebsiteAuthorizationProviderError::Unauthorized(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for AssociateWebsiteAuthorizationProviderError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AssociateWebsiteCertificateAuthorityError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociateWebsiteCertificateAuthorityError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateWebsiteCertificateAuthorityError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateWebsiteCertificateAuthorityError::ResourceAlreadyExists(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateWebsiteCertificateAuthorityError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateWebsiteCertificateAuthorityError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateWebsiteCertificateAuthorityError::Unauthorized(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for AssociateWebsiteCertificateAuthorityError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateFleetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateFleetError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreateFleetError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            CreateFleetError::ResourceAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateFleetError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateFleetError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            CreateFleetError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateFleetError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteFleetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteFleetError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteFleetError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DeleteFleetError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteFleetError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            DeleteFleetError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteFleetError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeAuditStreamConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeAuditStreamConfigurationError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeAuditStreamConfigurationError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeAuditStreamConfigurationError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeAuditStreamConfigurationError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeAuditStreamConfigurationError::Unauthorized(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeAuditStreamConfigurationError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeCompanyNetworkConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeCompanyNetworkConfigurationError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeCompanyNetworkConfigurationError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeCompanyNetworkConfigurationError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeCompanyNetworkConfigurationError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeCompanyNetworkConfigurationError::Unauthorized(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeCompanyNetworkConfigurationError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeDeviceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeDeviceError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DescribeDeviceError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribeDeviceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeDeviceError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            DescribeDeviceError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeDeviceError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeDevicePolicyConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeDevicePolicyConfigurationError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeDevicePolicyConfigurationError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeDevicePolicyConfigurationError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeDevicePolicyConfigurationError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeDevicePolicyConfigurationError::Unauthorized(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeDevicePolicyConfigurationError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeDomainError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeDomainError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DescribeDomainError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribeDomainError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeDomainError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            DescribeDomainError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeDomainError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeFleetMetadataError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeFleetMetadataError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DescribeFleetMetadataError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribeFleetMetadataError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeFleetMetadataError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            DescribeFleetMetadataError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeFleetMetadataError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeIdentityProviderConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeIdentityProviderConfigurationError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeIdentityProviderConfigurationError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeIdentityProviderConfigurationError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeIdentityProviderConfigurationError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeIdentityProviderConfigurationError::Unauthorized(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeIdentityProviderConfigurationError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeWebsiteCertificateAuthorityError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeWebsiteCertificateAuthorityError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeWebsiteCertificateAuthorityError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeWebsiteCertificateAuthorityError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeWebsiteCertificateAuthorityError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeWebsiteCertificateAuthorityError::Unauthorized(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeWebsiteCertificateAuthorityError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DisassociateDomainError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateDomainError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DisassociateDomainError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DisassociateDomainError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DisassociateDomainError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            DisassociateDomainError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DisassociateDomainError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DisassociateWebsiteAuthorizationProviderError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateWebsiteAuthorizationProviderError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateWebsiteAuthorizationProviderError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateWebsiteAuthorizationProviderError::ResourceAlreadyExists(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateWebsiteAuthorizationProviderError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateWebsiteAuthorizationProviderError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateWebsiteAuthorizationProviderError::Unauthorized(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DisassociateWebsiteAuthorizationProviderError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DisassociateWebsiteCertificateAuthorityError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateWebsiteCertificateAuthorityError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateWebsiteCertificateAuthorityError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateWebsiteCertificateAuthorityError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateWebsiteCertificateAuthorityError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateWebsiteCertificateAuthorityError::Unauthorized(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DisassociateWebsiteCertificateAuthorityError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListDevicesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDevicesError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListDevicesError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListDevicesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListDevicesError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            ListDevicesError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListDevicesError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListDomainsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDomainsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListDomainsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListDomainsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            ListDomainsError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListDomainsError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListFleetsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListFleetsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListFleetsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListFleetsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            ListFleetsError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListFleetsError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListWebsiteAuthorizationProvidersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListWebsiteAuthorizationProvidersError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            ListWebsiteAuthorizationProvidersError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            ListWebsiteAuthorizationProvidersError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            ListWebsiteAuthorizationProvidersError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
            ListWebsiteAuthorizationProvidersError::Unauthorized(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListWebsiteAuthorizationProvidersError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListWebsiteCertificateAuthoritiesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListWebsiteCertificateAuthoritiesError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            ListWebsiteCertificateAuthoritiesError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            ListWebsiteCertificateAuthoritiesError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
            ListWebsiteCertificateAuthoritiesError::Unauthorized(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListWebsiteCertificateAuthoritiesError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RestoreDomainAccessError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RestoreDomainAccessError::InternalServerError(ref cause) => write!(f, "{}", cause),
            RestoreDomainAccessError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            RestoreDomainAccessError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            RestoreDomainAccessError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            RestoreDomainAccessError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RestoreDomainAccessError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RevokeDomainAccessError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RevokeDomainAccessError::InternalServerError(ref cause) => write!(f, "{}", cause),
            RevokeDomainAccessError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            RevokeDomainAccessError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            RevokeDomainAccessError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            RevokeDomainAccessError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RevokeDomainAccessError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for SignOutUserError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SignOutUserError::InternalServerError(ref cause) => write!(f, "{}", cause),
            SignOutUserError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            SignOutUserError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            SignOutUserError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            SignOutUserError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SignOutUserError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateAuditStreamConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateAuditStreamConfigurationError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateAuditStreamConfigurationError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateAuditStreamConfigurationError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateAuditStreamConfigurationError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateAuditStreamConfigurationError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateAuditStreamConfigurationError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateCompanyNetworkConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateCompanyNetworkConfigurationError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateCompanyNetworkConfigurationError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateCompanyNetworkConfigurationError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateCompanyNetworkConfigurationError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateCompanyNetworkConfigurationError::Unauthorized(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateCompanyNetworkConfigurationError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateDevicePolicyConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateDevicePolicyConfigurationError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateDevicePolicyConfigurationError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateDevicePolicyConfigurationError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateDevicePolicyConfigurationError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateDevicePolicyConfigurationError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateDevicePolicyConfigurationError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateDomainMetadataError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateDomainMetadataError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateDomainMetadataError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UpdateDomainMetadataError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateDomainMetadataError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            UpdateDomainMetadataError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateDomainMetadataError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateFleetMetadataError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateFleetMetadataError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateFleetMetadataError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UpdateFleetMetadataError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateFleetMetadataError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            UpdateFleetMetadataError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateFleetMetadataError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateIdentityProviderConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateIdentityProviderConfigurationError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateIdentityProviderConfigurationError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateIdentityProviderConfigurationError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateIdentityProviderConfigurationError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateIdentityProviderConfigurationError::Unauthorized(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateIdentityProviderConfigurationError {}
/// Trait representing the capabilities of the WorkLink API. WorkLink clients implement this trait.
#[async_trait]
pub trait Worklink {
    /// <p>Specifies a domain to be associated to Amazon WorkLink.</p>
    async fn associate_domain(
        &self,
        input: AssociateDomainRequest,
    ) -> Result<AssociateDomainResponse, RusotoError<AssociateDomainError>>;

    /// <p>Associates a website authorization provider with a specified fleet. This is used to authorize users against associated websites in the company network.</p>
    async fn associate_website_authorization_provider(
        &self,
        input: AssociateWebsiteAuthorizationProviderRequest,
    ) -> Result<
        AssociateWebsiteAuthorizationProviderResponse,
        RusotoError<AssociateWebsiteAuthorizationProviderError>,
    >;

    /// <p>Imports the root certificate of a certificate authority (CA) used to obtain TLS certificates used by associated websites within the company network.</p>
    async fn associate_website_certificate_authority(
        &self,
        input: AssociateWebsiteCertificateAuthorityRequest,
    ) -> Result<
        AssociateWebsiteCertificateAuthorityResponse,
        RusotoError<AssociateWebsiteCertificateAuthorityError>,
    >;

    /// <p>Creates a fleet. A fleet consists of resources and the configuration that delivers associated websites to authorized users who download and set up the Amazon WorkLink app.</p>
    async fn create_fleet(
        &self,
        input: CreateFleetRequest,
    ) -> Result<CreateFleetResponse, RusotoError<CreateFleetError>>;

    /// <p>Deletes a fleet. Prevents users from accessing previously associated websites. </p>
    async fn delete_fleet(
        &self,
        input: DeleteFleetRequest,
    ) -> Result<DeleteFleetResponse, RusotoError<DeleteFleetError>>;

    /// <p>Describes the configuration for delivering audit streams to the customer account.</p>
    async fn describe_audit_stream_configuration(
        &self,
        input: DescribeAuditStreamConfigurationRequest,
    ) -> Result<
        DescribeAuditStreamConfigurationResponse,
        RusotoError<DescribeAuditStreamConfigurationError>,
    >;

    /// <p>Describes the networking configuration to access the internal websites associated with the specified fleet.</p>
    async fn describe_company_network_configuration(
        &self,
        input: DescribeCompanyNetworkConfigurationRequest,
    ) -> Result<
        DescribeCompanyNetworkConfigurationResponse,
        RusotoError<DescribeCompanyNetworkConfigurationError>,
    >;

    /// <p>Provides information about a user's device.</p>
    async fn describe_device(
        &self,
        input: DescribeDeviceRequest,
    ) -> Result<DescribeDeviceResponse, RusotoError<DescribeDeviceError>>;

    /// <p>Describes the device policy configuration for the specified fleet.</p>
    async fn describe_device_policy_configuration(
        &self,
        input: DescribeDevicePolicyConfigurationRequest,
    ) -> Result<
        DescribeDevicePolicyConfigurationResponse,
        RusotoError<DescribeDevicePolicyConfigurationError>,
    >;

    /// <p>Provides information about the domain.</p>
    async fn describe_domain(
        &self,
        input: DescribeDomainRequest,
    ) -> Result<DescribeDomainResponse, RusotoError<DescribeDomainError>>;

    /// <p>Provides basic information for the specified fleet, excluding identity provider, networking, and device configuration details.</p>
    async fn describe_fleet_metadata(
        &self,
        input: DescribeFleetMetadataRequest,
    ) -> Result<DescribeFleetMetadataResponse, RusotoError<DescribeFleetMetadataError>>;

    /// <p>Describes the identity provider configuration of the specified fleet.</p>
    async fn describe_identity_provider_configuration(
        &self,
        input: DescribeIdentityProviderConfigurationRequest,
    ) -> Result<
        DescribeIdentityProviderConfigurationResponse,
        RusotoError<DescribeIdentityProviderConfigurationError>,
    >;

    /// <p>Provides information about the certificate authority.</p>
    async fn describe_website_certificate_authority(
        &self,
        input: DescribeWebsiteCertificateAuthorityRequest,
    ) -> Result<
        DescribeWebsiteCertificateAuthorityResponse,
        RusotoError<DescribeWebsiteCertificateAuthorityError>,
    >;

    /// <p>Disassociates a domain from Amazon WorkLink. End users lose the ability to access the domain with Amazon WorkLink. </p>
    async fn disassociate_domain(
        &self,
        input: DisassociateDomainRequest,
    ) -> Result<DisassociateDomainResponse, RusotoError<DisassociateDomainError>>;

    /// <p>Disassociates a website authorization provider from a specified fleet. After the disassociation, users can't load any associated websites that require this authorization provider.</p>
    async fn disassociate_website_authorization_provider(
        &self,
        input: DisassociateWebsiteAuthorizationProviderRequest,
    ) -> Result<
        DisassociateWebsiteAuthorizationProviderResponse,
        RusotoError<DisassociateWebsiteAuthorizationProviderError>,
    >;

    /// <p>Removes a certificate authority (CA).</p>
    async fn disassociate_website_certificate_authority(
        &self,
        input: DisassociateWebsiteCertificateAuthorityRequest,
    ) -> Result<
        DisassociateWebsiteCertificateAuthorityResponse,
        RusotoError<DisassociateWebsiteCertificateAuthorityError>,
    >;

    /// <p>Retrieves a list of devices registered with the specified fleet.</p>
    async fn list_devices(
        &self,
        input: ListDevicesRequest,
    ) -> Result<ListDevicesResponse, RusotoError<ListDevicesError>>;

    /// <p>Retrieves a list of domains associated to a specified fleet.</p>
    async fn list_domains(
        &self,
        input: ListDomainsRequest,
    ) -> Result<ListDomainsResponse, RusotoError<ListDomainsError>>;

    /// <p>Retrieves a list of fleets for the current account and Region.</p>
    async fn list_fleets(
        &self,
        input: ListFleetsRequest,
    ) -> Result<ListFleetsResponse, RusotoError<ListFleetsError>>;

    /// <p>Retrieves a list of website authorization providers associated with a specified fleet.</p>
    async fn list_website_authorization_providers(
        &self,
        input: ListWebsiteAuthorizationProvidersRequest,
    ) -> Result<
        ListWebsiteAuthorizationProvidersResponse,
        RusotoError<ListWebsiteAuthorizationProvidersError>,
    >;

    /// <p>Retrieves a list of certificate authorities added for the current account and Region.</p>
    async fn list_website_certificate_authorities(
        &self,
        input: ListWebsiteCertificateAuthoritiesRequest,
    ) -> Result<
        ListWebsiteCertificateAuthoritiesResponse,
        RusotoError<ListWebsiteCertificateAuthoritiesError>,
    >;

    /// <p>Moves a domain to ACTIVE status if it was in the INACTIVE status.</p>
    async fn restore_domain_access(
        &self,
        input: RestoreDomainAccessRequest,
    ) -> Result<RestoreDomainAccessResponse, RusotoError<RestoreDomainAccessError>>;

    /// <p>Moves a domain to INACTIVE status if it was in the ACTIVE status.</p>
    async fn revoke_domain_access(
        &self,
        input: RevokeDomainAccessRequest,
    ) -> Result<RevokeDomainAccessResponse, RusotoError<RevokeDomainAccessError>>;

    /// <p>Signs the user out from all of their devices. The user can sign in again if they have valid credentials.</p>
    async fn sign_out_user(
        &self,
        input: SignOutUserRequest,
    ) -> Result<SignOutUserResponse, RusotoError<SignOutUserError>>;

    /// <p>Updates the audit stream configuration for the fleet.</p>
    async fn update_audit_stream_configuration(
        &self,
        input: UpdateAuditStreamConfigurationRequest,
    ) -> Result<
        UpdateAuditStreamConfigurationResponse,
        RusotoError<UpdateAuditStreamConfigurationError>,
    >;

    /// <p>Updates the company network configuration for the fleet.</p>
    async fn update_company_network_configuration(
        &self,
        input: UpdateCompanyNetworkConfigurationRequest,
    ) -> Result<
        UpdateCompanyNetworkConfigurationResponse,
        RusotoError<UpdateCompanyNetworkConfigurationError>,
    >;

    /// <p>Updates the device policy configuration for the fleet.</p>
    async fn update_device_policy_configuration(
        &self,
        input: UpdateDevicePolicyConfigurationRequest,
    ) -> Result<
        UpdateDevicePolicyConfigurationResponse,
        RusotoError<UpdateDevicePolicyConfigurationError>,
    >;

    /// <p>Updates domain metadata, such as DisplayName.</p>
    async fn update_domain_metadata(
        &self,
        input: UpdateDomainMetadataRequest,
    ) -> Result<UpdateDomainMetadataResponse, RusotoError<UpdateDomainMetadataError>>;

    /// <p>Updates fleet metadata, such as DisplayName.</p>
    async fn update_fleet_metadata(
        &self,
        input: UpdateFleetMetadataRequest,
    ) -> Result<UpdateFleetMetadataResponse, RusotoError<UpdateFleetMetadataError>>;

    /// <p>Updates the identity provider configuration for the fleet.</p>
    async fn update_identity_provider_configuration(
        &self,
        input: UpdateIdentityProviderConfigurationRequest,
    ) -> Result<
        UpdateIdentityProviderConfigurationResponse,
        RusotoError<UpdateIdentityProviderConfigurationError>,
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
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        WorklinkClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> WorklinkClient {
        WorklinkClient { client, region }
    }
}

#[async_trait]
impl Worklink for WorklinkClient {
    /// <p>Specifies a domain to be associated to Amazon WorkLink.</p>
    async fn associate_domain(
        &self,
        input: AssociateDomainRequest,
    ) -> Result<AssociateDomainResponse, RusotoError<AssociateDomainError>> {
        let request_uri = "/associateDomain";

        let mut request = SignedRequest::new("POST", "worklink", &self.region, &request_uri);
        request.set_content_type("application/json".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<AssociateDomainResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(AssociateDomainError::from_response(response))
        }
    }

    /// <p>Associates a website authorization provider with a specified fleet. This is used to authorize users against associated websites in the company network.</p>
    async fn associate_website_authorization_provider(
        &self,
        input: AssociateWebsiteAuthorizationProviderRequest,
    ) -> Result<
        AssociateWebsiteAuthorizationProviderResponse,
        RusotoError<AssociateWebsiteAuthorizationProviderError>,
    > {
        let request_uri = "/associateWebsiteAuthorizationProvider";

        let mut request = SignedRequest::new("POST", "worklink", &self.region, &request_uri);
        request.set_content_type("application/json".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<AssociateWebsiteAuthorizationProviderResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(AssociateWebsiteAuthorizationProviderError::from_response(
                response,
            ))
        }
    }

    /// <p>Imports the root certificate of a certificate authority (CA) used to obtain TLS certificates used by associated websites within the company network.</p>
    async fn associate_website_certificate_authority(
        &self,
        input: AssociateWebsiteCertificateAuthorityRequest,
    ) -> Result<
        AssociateWebsiteCertificateAuthorityResponse,
        RusotoError<AssociateWebsiteCertificateAuthorityError>,
    > {
        let request_uri = "/associateWebsiteCertificateAuthority";

        let mut request = SignedRequest::new("POST", "worklink", &self.region, &request_uri);
        request.set_content_type("application/json".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<AssociateWebsiteCertificateAuthorityResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(AssociateWebsiteCertificateAuthorityError::from_response(
                response,
            ))
        }
    }

    /// <p>Creates a fleet. A fleet consists of resources and the configuration that delivers associated websites to authorized users who download and set up the Amazon WorkLink app.</p>
    async fn create_fleet(
        &self,
        input: CreateFleetRequest,
    ) -> Result<CreateFleetResponse, RusotoError<CreateFleetError>> {
        let request_uri = "/createFleet";

        let mut request = SignedRequest::new("POST", "worklink", &self.region, &request_uri);
        request.set_content_type("application/json".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateFleetResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateFleetError::from_response(response))
        }
    }

    /// <p>Deletes a fleet. Prevents users from accessing previously associated websites. </p>
    async fn delete_fleet(
        &self,
        input: DeleteFleetRequest,
    ) -> Result<DeleteFleetResponse, RusotoError<DeleteFleetError>> {
        let request_uri = "/deleteFleet";

        let mut request = SignedRequest::new("POST", "worklink", &self.region, &request_uri);
        request.set_content_type("application/json".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteFleetResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteFleetError::from_response(response))
        }
    }

    /// <p>Describes the configuration for delivering audit streams to the customer account.</p>
    async fn describe_audit_stream_configuration(
        &self,
        input: DescribeAuditStreamConfigurationRequest,
    ) -> Result<
        DescribeAuditStreamConfigurationResponse,
        RusotoError<DescribeAuditStreamConfigurationError>,
    > {
        let request_uri = "/describeAuditStreamConfiguration";

        let mut request = SignedRequest::new("POST", "worklink", &self.region, &request_uri);
        request.set_content_type("application/json".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeAuditStreamConfigurationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeAuditStreamConfigurationError::from_response(
                response,
            ))
        }
    }

    /// <p>Describes the networking configuration to access the internal websites associated with the specified fleet.</p>
    async fn describe_company_network_configuration(
        &self,
        input: DescribeCompanyNetworkConfigurationRequest,
    ) -> Result<
        DescribeCompanyNetworkConfigurationResponse,
        RusotoError<DescribeCompanyNetworkConfigurationError>,
    > {
        let request_uri = "/describeCompanyNetworkConfiguration";

        let mut request = SignedRequest::new("POST", "worklink", &self.region, &request_uri);
        request.set_content_type("application/json".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeCompanyNetworkConfigurationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeCompanyNetworkConfigurationError::from_response(
                response,
            ))
        }
    }

    /// <p>Provides information about a user's device.</p>
    async fn describe_device(
        &self,
        input: DescribeDeviceRequest,
    ) -> Result<DescribeDeviceResponse, RusotoError<DescribeDeviceError>> {
        let request_uri = "/describeDevice";

        let mut request = SignedRequest::new("POST", "worklink", &self.region, &request_uri);
        request.set_content_type("application/json".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeDeviceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeDeviceError::from_response(response))
        }
    }

    /// <p>Describes the device policy configuration for the specified fleet.</p>
    async fn describe_device_policy_configuration(
        &self,
        input: DescribeDevicePolicyConfigurationRequest,
    ) -> Result<
        DescribeDevicePolicyConfigurationResponse,
        RusotoError<DescribeDevicePolicyConfigurationError>,
    > {
        let request_uri = "/describeDevicePolicyConfiguration";

        let mut request = SignedRequest::new("POST", "worklink", &self.region, &request_uri);
        request.set_content_type("application/json".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeDevicePolicyConfigurationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeDevicePolicyConfigurationError::from_response(
                response,
            ))
        }
    }

    /// <p>Provides information about the domain.</p>
    async fn describe_domain(
        &self,
        input: DescribeDomainRequest,
    ) -> Result<DescribeDomainResponse, RusotoError<DescribeDomainError>> {
        let request_uri = "/describeDomain";

        let mut request = SignedRequest::new("POST", "worklink", &self.region, &request_uri);
        request.set_content_type("application/json".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeDomainResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeDomainError::from_response(response))
        }
    }

    /// <p>Provides basic information for the specified fleet, excluding identity provider, networking, and device configuration details.</p>
    async fn describe_fleet_metadata(
        &self,
        input: DescribeFleetMetadataRequest,
    ) -> Result<DescribeFleetMetadataResponse, RusotoError<DescribeFleetMetadataError>> {
        let request_uri = "/describeFleetMetadata";

        let mut request = SignedRequest::new("POST", "worklink", &self.region, &request_uri);
        request.set_content_type("application/json".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeFleetMetadataResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeFleetMetadataError::from_response(response))
        }
    }

    /// <p>Describes the identity provider configuration of the specified fleet.</p>
    async fn describe_identity_provider_configuration(
        &self,
        input: DescribeIdentityProviderConfigurationRequest,
    ) -> Result<
        DescribeIdentityProviderConfigurationResponse,
        RusotoError<DescribeIdentityProviderConfigurationError>,
    > {
        let request_uri = "/describeIdentityProviderConfiguration";

        let mut request = SignedRequest::new("POST", "worklink", &self.region, &request_uri);
        request.set_content_type("application/json".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeIdentityProviderConfigurationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeIdentityProviderConfigurationError::from_response(
                response,
            ))
        }
    }

    /// <p>Provides information about the certificate authority.</p>
    async fn describe_website_certificate_authority(
        &self,
        input: DescribeWebsiteCertificateAuthorityRequest,
    ) -> Result<
        DescribeWebsiteCertificateAuthorityResponse,
        RusotoError<DescribeWebsiteCertificateAuthorityError>,
    > {
        let request_uri = "/describeWebsiteCertificateAuthority";

        let mut request = SignedRequest::new("POST", "worklink", &self.region, &request_uri);
        request.set_content_type("application/json".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeWebsiteCertificateAuthorityResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeWebsiteCertificateAuthorityError::from_response(
                response,
            ))
        }
    }

    /// <p>Disassociates a domain from Amazon WorkLink. End users lose the ability to access the domain with Amazon WorkLink. </p>
    async fn disassociate_domain(
        &self,
        input: DisassociateDomainRequest,
    ) -> Result<DisassociateDomainResponse, RusotoError<DisassociateDomainError>> {
        let request_uri = "/disassociateDomain";

        let mut request = SignedRequest::new("POST", "worklink", &self.region, &request_uri);
        request.set_content_type("application/json".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DisassociateDomainResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DisassociateDomainError::from_response(response))
        }
    }

    /// <p>Disassociates a website authorization provider from a specified fleet. After the disassociation, users can't load any associated websites that require this authorization provider.</p>
    async fn disassociate_website_authorization_provider(
        &self,
        input: DisassociateWebsiteAuthorizationProviderRequest,
    ) -> Result<
        DisassociateWebsiteAuthorizationProviderResponse,
        RusotoError<DisassociateWebsiteAuthorizationProviderError>,
    > {
        let request_uri = "/disassociateWebsiteAuthorizationProvider";

        let mut request = SignedRequest::new("POST", "worklink", &self.region, &request_uri);
        request.set_content_type("application/json".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DisassociateWebsiteAuthorizationProviderResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DisassociateWebsiteAuthorizationProviderError::from_response(response))
        }
    }

    /// <p>Removes a certificate authority (CA).</p>
    async fn disassociate_website_certificate_authority(
        &self,
        input: DisassociateWebsiteCertificateAuthorityRequest,
    ) -> Result<
        DisassociateWebsiteCertificateAuthorityResponse,
        RusotoError<DisassociateWebsiteCertificateAuthorityError>,
    > {
        let request_uri = "/disassociateWebsiteCertificateAuthority";

        let mut request = SignedRequest::new("POST", "worklink", &self.region, &request_uri);
        request.set_content_type("application/json".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DisassociateWebsiteCertificateAuthorityResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DisassociateWebsiteCertificateAuthorityError::from_response(
                response,
            ))
        }
    }

    /// <p>Retrieves a list of devices registered with the specified fleet.</p>
    async fn list_devices(
        &self,
        input: ListDevicesRequest,
    ) -> Result<ListDevicesResponse, RusotoError<ListDevicesError>> {
        let request_uri = "/listDevices";

        let mut request = SignedRequest::new("POST", "worklink", &self.region, &request_uri);
        request.set_content_type("application/json".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListDevicesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListDevicesError::from_response(response))
        }
    }

    /// <p>Retrieves a list of domains associated to a specified fleet.</p>
    async fn list_domains(
        &self,
        input: ListDomainsRequest,
    ) -> Result<ListDomainsResponse, RusotoError<ListDomainsError>> {
        let request_uri = "/listDomains";

        let mut request = SignedRequest::new("POST", "worklink", &self.region, &request_uri);
        request.set_content_type("application/json".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListDomainsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListDomainsError::from_response(response))
        }
    }

    /// <p>Retrieves a list of fleets for the current account and Region.</p>
    async fn list_fleets(
        &self,
        input: ListFleetsRequest,
    ) -> Result<ListFleetsResponse, RusotoError<ListFleetsError>> {
        let request_uri = "/listFleets";

        let mut request = SignedRequest::new("POST", "worklink", &self.region, &request_uri);
        request.set_content_type("application/json".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListFleetsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListFleetsError::from_response(response))
        }
    }

    /// <p>Retrieves a list of website authorization providers associated with a specified fleet.</p>
    async fn list_website_authorization_providers(
        &self,
        input: ListWebsiteAuthorizationProvidersRequest,
    ) -> Result<
        ListWebsiteAuthorizationProvidersResponse,
        RusotoError<ListWebsiteAuthorizationProvidersError>,
    > {
        let request_uri = "/listWebsiteAuthorizationProviders";

        let mut request = SignedRequest::new("POST", "worklink", &self.region, &request_uri);
        request.set_content_type("application/json".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListWebsiteAuthorizationProvidersResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListWebsiteAuthorizationProvidersError::from_response(
                response,
            ))
        }
    }

    /// <p>Retrieves a list of certificate authorities added for the current account and Region.</p>
    async fn list_website_certificate_authorities(
        &self,
        input: ListWebsiteCertificateAuthoritiesRequest,
    ) -> Result<
        ListWebsiteCertificateAuthoritiesResponse,
        RusotoError<ListWebsiteCertificateAuthoritiesError>,
    > {
        let request_uri = "/listWebsiteCertificateAuthorities";

        let mut request = SignedRequest::new("POST", "worklink", &self.region, &request_uri);
        request.set_content_type("application/json".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListWebsiteCertificateAuthoritiesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListWebsiteCertificateAuthoritiesError::from_response(
                response,
            ))
        }
    }

    /// <p>Moves a domain to ACTIVE status if it was in the INACTIVE status.</p>
    async fn restore_domain_access(
        &self,
        input: RestoreDomainAccessRequest,
    ) -> Result<RestoreDomainAccessResponse, RusotoError<RestoreDomainAccessError>> {
        let request_uri = "/restoreDomainAccess";

        let mut request = SignedRequest::new("POST", "worklink", &self.region, &request_uri);
        request.set_content_type("application/json".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<RestoreDomainAccessResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(RestoreDomainAccessError::from_response(response))
        }
    }

    /// <p>Moves a domain to INACTIVE status if it was in the ACTIVE status.</p>
    async fn revoke_domain_access(
        &self,
        input: RevokeDomainAccessRequest,
    ) -> Result<RevokeDomainAccessResponse, RusotoError<RevokeDomainAccessError>> {
        let request_uri = "/revokeDomainAccess";

        let mut request = SignedRequest::new("POST", "worklink", &self.region, &request_uri);
        request.set_content_type("application/json".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<RevokeDomainAccessResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(RevokeDomainAccessError::from_response(response))
        }
    }

    /// <p>Signs the user out from all of their devices. The user can sign in again if they have valid credentials.</p>
    async fn sign_out_user(
        &self,
        input: SignOutUserRequest,
    ) -> Result<SignOutUserResponse, RusotoError<SignOutUserError>> {
        let request_uri = "/signOutUser";

        let mut request = SignedRequest::new("POST", "worklink", &self.region, &request_uri);
        request.set_content_type("application/json".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<SignOutUserResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(SignOutUserError::from_response(response))
        }
    }

    /// <p>Updates the audit stream configuration for the fleet.</p>
    async fn update_audit_stream_configuration(
        &self,
        input: UpdateAuditStreamConfigurationRequest,
    ) -> Result<
        UpdateAuditStreamConfigurationResponse,
        RusotoError<UpdateAuditStreamConfigurationError>,
    > {
        let request_uri = "/updateAuditStreamConfiguration";

        let mut request = SignedRequest::new("POST", "worklink", &self.region, &request_uri);
        request.set_content_type("application/json".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateAuditStreamConfigurationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateAuditStreamConfigurationError::from_response(response))
        }
    }

    /// <p>Updates the company network configuration for the fleet.</p>
    async fn update_company_network_configuration(
        &self,
        input: UpdateCompanyNetworkConfigurationRequest,
    ) -> Result<
        UpdateCompanyNetworkConfigurationResponse,
        RusotoError<UpdateCompanyNetworkConfigurationError>,
    > {
        let request_uri = "/updateCompanyNetworkConfiguration";

        let mut request = SignedRequest::new("POST", "worklink", &self.region, &request_uri);
        request.set_content_type("application/json".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateCompanyNetworkConfigurationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateCompanyNetworkConfigurationError::from_response(
                response,
            ))
        }
    }

    /// <p>Updates the device policy configuration for the fleet.</p>
    async fn update_device_policy_configuration(
        &self,
        input: UpdateDevicePolicyConfigurationRequest,
    ) -> Result<
        UpdateDevicePolicyConfigurationResponse,
        RusotoError<UpdateDevicePolicyConfigurationError>,
    > {
        let request_uri = "/updateDevicePolicyConfiguration";

        let mut request = SignedRequest::new("POST", "worklink", &self.region, &request_uri);
        request.set_content_type("application/json".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateDevicePolicyConfigurationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateDevicePolicyConfigurationError::from_response(
                response,
            ))
        }
    }

    /// <p>Updates domain metadata, such as DisplayName.</p>
    async fn update_domain_metadata(
        &self,
        input: UpdateDomainMetadataRequest,
    ) -> Result<UpdateDomainMetadataResponse, RusotoError<UpdateDomainMetadataError>> {
        let request_uri = "/updateDomainMetadata";

        let mut request = SignedRequest::new("POST", "worklink", &self.region, &request_uri);
        request.set_content_type("application/json".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateDomainMetadataResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateDomainMetadataError::from_response(response))
        }
    }

    /// <p>Updates fleet metadata, such as DisplayName.</p>
    async fn update_fleet_metadata(
        &self,
        input: UpdateFleetMetadataRequest,
    ) -> Result<UpdateFleetMetadataResponse, RusotoError<UpdateFleetMetadataError>> {
        let request_uri = "/UpdateFleetMetadata";

        let mut request = SignedRequest::new("POST", "worklink", &self.region, &request_uri);
        request.set_content_type("application/json".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateFleetMetadataResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateFleetMetadataError::from_response(response))
        }
    }

    /// <p>Updates the identity provider configuration for the fleet.</p>
    async fn update_identity_provider_configuration(
        &self,
        input: UpdateIdentityProviderConfigurationRequest,
    ) -> Result<
        UpdateIdentityProviderConfigurationResponse,
        RusotoError<UpdateIdentityProviderConfigurationError>,
    > {
        let request_uri = "/updateIdentityProviderConfiguration";

        let mut request = SignedRequest::new("POST", "worklink", &self.region, &request_uri);
        request.set_content_type("application/json".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateIdentityProviderConfigurationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateIdentityProviderConfigurationError::from_response(
                response,
            ))
        }
    }
}
