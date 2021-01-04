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
use rusoto_core::request::HttpResponse;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

impl LicenseManagerClient {
    fn new_signed_request(&self, http_method: &str, request_uri: &str) -> SignedRequest {
        let mut request =
            SignedRequest::new(http_method, "license-manager", &self.region, request_uri);

        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request
    }

    async fn sign_and_dispatch<E>(
        &self,
        request: SignedRequest,
        from_response: fn(BufferedHttpResponse) -> RusotoError<E>,
    ) -> Result<HttpResponse, RusotoError<E>> {
        let mut response = self.client.sign_and_dispatch(request).await?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(from_response(response));
        }

        Ok(response)
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AcceptGrantRequest {
    /// <p>Amazon Resource Name (ARN) of the grant.</p>
    #[serde(rename = "GrantArn")]
    pub grant_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AcceptGrantResponse {
    /// <p>Grant ARN.</p>
    #[serde(rename = "GrantArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_arn: Option<String>,
    /// <p>Grant status.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>Grant version.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>Describes automated discovery.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AutomatedDiscoveryInformation {
    /// <p>Time that automated discovery last ran.</p>
    #[serde(rename = "LastRunTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_run_time: Option<f64>,
}

/// <p>Details about a borrow configuration.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct BorrowConfiguration {
    /// <p>Indicates whether early check-ins are allowed.</p>
    #[serde(rename = "AllowEarlyCheckIn")]
    pub allow_early_check_in: bool,
    /// <p>Maximum time for the borrow configuration, in minutes.</p>
    #[serde(rename = "MaxTimeToLiveInMinutes")]
    pub max_time_to_live_in_minutes: i64,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CheckInLicenseRequest {
    /// <p>License beneficiary.</p>
    #[serde(rename = "Beneficiary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary: Option<String>,
    /// <p>License consumption token.</p>
    #[serde(rename = "LicenseConsumptionToken")]
    pub license_consumption_token: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CheckInLicenseResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CheckoutBorrowLicenseRequest {
    /// <p>Information about constraints.</p>
    #[serde(rename = "CheckoutMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_metadata: Option<Vec<Metadata>>,
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "ClientToken")]
    pub client_token: String,
    /// <p>Digital signature method. The possible value is JSON Web Signature (JWS) algorithm PS384. For more information, see <a href="https://tools.ietf.org/html/rfc7518#section-3.5">RFC 7518 Digital Signature with RSASSA-PSS</a>.</p>
    #[serde(rename = "DigitalSignatureMethod")]
    pub digital_signature_method: String,
    /// <p>License entitlements. Partial checkouts are not supported.</p>
    #[serde(rename = "Entitlements")]
    pub entitlements: Vec<EntitlementData>,
    /// <p>Amazon Resource Name (ARN) of the license. The license must use the borrow consumption configuration.</p>
    #[serde(rename = "LicenseArn")]
    pub license_arn: String,
    /// <p>Node ID.</p>
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CheckoutBorrowLicenseResponse {
    /// <p>Information about constraints.</p>
    #[serde(rename = "CheckoutMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_metadata: Option<Vec<Metadata>>,
    /// <p>Allowed license entitlements.</p>
    #[serde(rename = "EntitlementsAllowed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlements_allowed: Option<Vec<EntitlementData>>,
    /// <p>Date and time at which the license checkout expires.</p>
    #[serde(rename = "Expiration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<String>,
    /// <p>Date and time at which the license checkout is issued.</p>
    #[serde(rename = "IssuedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_at: Option<String>,
    /// <p>Amazon Resource Name (ARN) of the license.</p>
    #[serde(rename = "LicenseArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_arn: Option<String>,
    /// <p>License consumption token.</p>
    #[serde(rename = "LicenseConsumptionToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_consumption_token: Option<String>,
    /// <p>Node ID.</p>
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// <p>Signed token.</p>
    #[serde(rename = "SignedToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signed_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CheckoutLicenseRequest {
    /// <p>License beneficiary.</p>
    #[serde(rename = "Beneficiary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary: Option<String>,
    /// <p>Checkout type.</p>
    #[serde(rename = "CheckoutType")]
    pub checkout_type: String,
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "ClientToken")]
    pub client_token: String,
    /// <p>License entitlements.</p>
    #[serde(rename = "Entitlements")]
    pub entitlements: Vec<EntitlementData>,
    /// <p>Key fingerprint identifying the license.</p>
    #[serde(rename = "KeyFingerprint")]
    pub key_fingerprint: String,
    /// <p>Node ID.</p>
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// <p>Product SKU.</p>
    #[serde(rename = "ProductSKU")]
    pub product_sku: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CheckoutLicenseResponse {
    /// <p>Checkout type.</p>
    #[serde(rename = "CheckoutType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_type: Option<String>,
    /// <p>Allowed license entitlements.</p>
    #[serde(rename = "EntitlementsAllowed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlements_allowed: Option<Vec<EntitlementData>>,
    /// <p>Date and time at which the license checkout expires.</p>
    #[serde(rename = "Expiration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<String>,
    /// <p>Date and time at which the license checkout is issued.</p>
    #[serde(rename = "IssuedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_at: Option<String>,
    /// <p>License consumption token.</p>
    #[serde(rename = "LicenseConsumptionToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_consumption_token: Option<String>,
    /// <p>Node ID.</p>
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// <p>Signed token.</p>
    #[serde(rename = "SignedToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signed_token: Option<String>,
}

/// <p>Details about license consumption.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ConsumedLicenseSummary {
    /// <p>Number of licenses consumed by the resource.</p>
    #[serde(rename = "ConsumedLicenses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_licenses: Option<i64>,
    /// <p>Resource type of the resource consuming a license.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

/// <p>Details about a consumption configuration.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ConsumptionConfiguration {
    /// <p>Details about a borrow configuration.</p>
    #[serde(rename = "BorrowConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub borrow_configuration: Option<BorrowConfiguration>,
    /// <p>Details about a provisional configuration.</p>
    #[serde(rename = "ProvisionalConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisional_configuration: Option<ProvisionalConfiguration>,
    /// <p>Renewal frequency.</p>
    #[serde(rename = "RenewType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renew_type: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateGrantRequest {
    /// <p>Allowed operations for the grant.</p>
    #[serde(rename = "AllowedOperations")]
    pub allowed_operations: Vec<String>,
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "ClientToken")]
    pub client_token: String,
    /// <p>Grant name.</p>
    #[serde(rename = "GrantName")]
    pub grant_name: String,
    /// <p>Home Region of the grant.</p>
    #[serde(rename = "HomeRegion")]
    pub home_region: String,
    /// <p>Amazon Resource Name (ARN) of the license.</p>
    #[serde(rename = "LicenseArn")]
    pub license_arn: String,
    /// <p>The grant principals.</p>
    #[serde(rename = "Principals")]
    pub principals: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateGrantResponse {
    /// <p>Grant ARN.</p>
    #[serde(rename = "GrantArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_arn: Option<String>,
    /// <p>Grant status.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>Grant version.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateGrantVersionRequest {
    /// <p>Allowed operations for the grant.</p>
    #[serde(rename = "AllowedOperations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_operations: Option<Vec<String>>,
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "ClientToken")]
    pub client_token: String,
    /// <p>Amazon Resource Name (ARN) of the grant.</p>
    #[serde(rename = "GrantArn")]
    pub grant_arn: String,
    /// <p>Grant name.</p>
    #[serde(rename = "GrantName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_name: Option<String>,
    /// <p>Current version of the grant.</p>
    #[serde(rename = "SourceVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_version: Option<String>,
    /// <p>Grant status.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateGrantVersionResponse {
    /// <p>Grant ARN.</p>
    #[serde(rename = "GrantArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_arn: Option<String>,
    /// <p>Grant status.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>New version of the grant.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateLicenseConfigurationRequest {
    /// <p>Description of the license configuration.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>When true, disassociates a resource when software is uninstalled.</p>
    #[serde(rename = "DisassociateWhenNotFound")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disassociate_when_not_found: Option<bool>,
    /// <p>Number of licenses managed by the license configuration.</p>
    #[serde(rename = "LicenseCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_count: Option<i64>,
    /// <p>Indicates whether hard or soft license enforcement is used. Exceeding a hard limit blocks the launch of new instances.</p>
    #[serde(rename = "LicenseCountHardLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_count_hard_limit: Option<bool>,
    /// <p>Dimension used to track the license inventory.</p>
    #[serde(rename = "LicenseCountingType")]
    pub license_counting_type: String,
    /// <p>License rules. The syntax is #name=value (for example, #allowedTenancy=EC2-DedicatedHost). The available rules vary by dimension, as follows.</p> <ul> <li> <p> <code>Cores</code> dimension: <code>allowedTenancy</code> | <code>licenseAffinityToHost</code> | <code>maximumCores</code> | <code>minimumCores</code> </p> </li> <li> <p> <code>Instances</code> dimension: <code>allowedTenancy</code> | <code>maximumCores</code> | <code>minimumCores</code> | <code>maximumSockets</code> | <code>minimumSockets</code> | <code>maximumVcpus</code> | <code>minimumVcpus</code> </p> </li> <li> <p> <code>Sockets</code> dimension: <code>allowedTenancy</code> | <code>licenseAffinityToHost</code> | <code>maximumSockets</code> | <code>minimumSockets</code> </p> </li> <li> <p> <code>vCPUs</code> dimension: <code>allowedTenancy</code> | <code>honorVcpuOptimization</code> | <code>maximumVcpus</code> | <code>minimumVcpus</code> </p> </li> </ul> <p>The unit for <code>licenseAffinityToHost</code> is days and the range is 1 to 180. The possible values for <code>allowedTenancy</code> are <code>EC2-Default</code>, <code>EC2-DedicatedHost</code>, and <code>EC2-DedicatedInstance</code>. The possible values for <code>honorVcpuOptimization</code> are <code>True</code> and <code>False</code>.</p>
    #[serde(rename = "LicenseRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_rules: Option<Vec<String>>,
    /// <p>Name of the license configuration.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>Product information.</p>
    #[serde(rename = "ProductInformationList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_information_list: Option<Vec<ProductInformation>>,
    /// <p>Tags to add to the license configuration.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateLicenseConfigurationResponse {
    /// <p>Amazon Resource Name (ARN) of the license configuration.</p>
    #[serde(rename = "LicenseConfigurationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_configuration_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateLicenseRequest {
    /// <p>License beneficiary.</p>
    #[serde(rename = "Beneficiary")]
    pub beneficiary: String,
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "ClientToken")]
    pub client_token: String,
    /// <p>Configuration for consumption of the license. Choose a provisional configuration for workloads running with continuous connectivity. Choose a borrow configuration for workloads with offline usage.</p>
    #[serde(rename = "ConsumptionConfiguration")]
    pub consumption_configuration: ConsumptionConfiguration,
    /// <p>License entitlements.</p>
    #[serde(rename = "Entitlements")]
    pub entitlements: Vec<Entitlement>,
    /// <p>Home Region for the license.</p>
    #[serde(rename = "HomeRegion")]
    pub home_region: String,
    /// <p>License issuer.</p>
    #[serde(rename = "Issuer")]
    pub issuer: Issuer,
    /// <p>Information about the license.</p>
    #[serde(rename = "LicenseMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_metadata: Option<Vec<Metadata>>,
    /// <p>License name.</p>
    #[serde(rename = "LicenseName")]
    pub license_name: String,
    /// <p>Product name.</p>
    #[serde(rename = "ProductName")]
    pub product_name: String,
    /// <p>Product SKU.</p>
    #[serde(rename = "ProductSKU")]
    pub product_sku: String,
    /// <p>Date and time range during which the license is valid, in ISO8601-UTC format.</p>
    #[serde(rename = "Validity")]
    pub validity: DatetimeRange,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateLicenseResponse {
    /// <p>Amazon Resource Name (ARN) of the license.</p>
    #[serde(rename = "LicenseArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_arn: Option<String>,
    /// <p>License status.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>License version.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateLicenseVersionRequest {
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "ClientToken")]
    pub client_token: String,
    /// <p>Configuration for consumption of the license. Choose a provisional configuration for workloads running with continuous connectivity. Choose a borrow configuration for workloads with offline usage.</p>
    #[serde(rename = "ConsumptionConfiguration")]
    pub consumption_configuration: ConsumptionConfiguration,
    /// <p>License entitlements.</p>
    #[serde(rename = "Entitlements")]
    pub entitlements: Vec<Entitlement>,
    /// <p>Home Region of the license.</p>
    #[serde(rename = "HomeRegion")]
    pub home_region: String,
    /// <p>License issuer.</p>
    #[serde(rename = "Issuer")]
    pub issuer: Issuer,
    /// <p>Amazon Resource Name (ARN) of the license.</p>
    #[serde(rename = "LicenseArn")]
    pub license_arn: String,
    /// <p>Information about the license.</p>
    #[serde(rename = "LicenseMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_metadata: Option<Vec<Metadata>>,
    /// <p>License name.</p>
    #[serde(rename = "LicenseName")]
    pub license_name: String,
    /// <p>Product name.</p>
    #[serde(rename = "ProductName")]
    pub product_name: String,
    /// <p>Current version of the license.</p>
    #[serde(rename = "SourceVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_version: Option<String>,
    /// <p>License status.</p>
    #[serde(rename = "Status")]
    pub status: String,
    /// <p>Date and time range during which the license is valid, in ISO8601-UTC format.</p>
    #[serde(rename = "Validity")]
    pub validity: DatetimeRange,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateLicenseVersionResponse {
    /// <p>License ARN.</p>
    #[serde(rename = "LicenseArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_arn: Option<String>,
    /// <p>License status.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>New version of the license.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateTokenRequest {
    /// <p>Idempotency token, valid for 10 minutes.</p>
    #[serde(rename = "ClientToken")]
    pub client_token: String,
    /// <p>Token expiration, in days, counted from token creation. The default is 365 days.</p>
    #[serde(rename = "ExpirationInDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_in_days: Option<i64>,
    /// <p>Amazon Resource Name (ARN) of the license. The ARN is mapped to the aud claim of the JWT token.</p>
    #[serde(rename = "LicenseArn")]
    pub license_arn: String,
    /// <p>Amazon Resource Name (ARN) of the IAM roles to embed in the token. License Manager does not check whether the roles are in use.</p>
    #[serde(rename = "RoleArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arns: Option<Vec<String>>,
    /// <p>Data specified by the caller to be included in the JWT token. The data is mapped to the amr claim of the JWT token.</p>
    #[serde(rename = "TokenProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_properties: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateTokenResponse {
    /// <p>Refresh token, encoded as a JWT token.</p>
    #[serde(rename = "Token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// <p>Token ID.</p>
    #[serde(rename = "TokenId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_id: Option<String>,
    /// <p>Token type.</p>
    #[serde(rename = "TokenType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_type: Option<String>,
}

/// <p>Describes a time range, in ISO8601-UTC format.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DatetimeRange {
    /// <p>Start of the time range.</p>
    #[serde(rename = "Begin")]
    pub begin: String,
    /// <p>End of the time range.</p>
    #[serde(rename = "End")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteGrantRequest {
    /// <p>Amazon Resource Name (ARN) of the grant.</p>
    #[serde(rename = "GrantArn")]
    pub grant_arn: String,
    /// <p>Current version of the grant.</p>
    #[serde(rename = "Version")]
    pub version: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteGrantResponse {
    /// <p>Grant ARN.</p>
    #[serde(rename = "GrantArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_arn: Option<String>,
    /// <p>Grant status.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>Grant version.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteLicenseConfigurationRequest {
    /// <p>ID of the license configuration.</p>
    #[serde(rename = "LicenseConfigurationArn")]
    pub license_configuration_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteLicenseConfigurationResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteLicenseRequest {
    /// <p>Amazon Resource Name (ARN) of the license.</p>
    #[serde(rename = "LicenseArn")]
    pub license_arn: String,
    /// <p>Current version of the license.</p>
    #[serde(rename = "SourceVersion")]
    pub source_version: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteLicenseResponse {
    /// <p>Date on which the license is deleted.</p>
    #[serde(rename = "DeletionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_date: Option<String>,
    /// <p>License status.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteTokenRequest {
    /// <p>Token ID.</p>
    #[serde(rename = "TokenId")]
    pub token_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteTokenResponse {}

/// <p>Describes a resource entitled for use with a license.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Entitlement {
    /// <p>Indicates whether check-ins are allowed.</p>
    #[serde(rename = "AllowCheckIn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_check_in: Option<bool>,
    /// <p>Maximum entitlement count. Use if the unit is not None.</p>
    #[serde(rename = "MaxCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_count: Option<i64>,
    /// <p>Entitlement name.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>Indicates whether overages are allowed.</p>
    #[serde(rename = "Overage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overage: Option<bool>,
    /// <p>Entitlement unit.</p>
    #[serde(rename = "Unit")]
    pub unit: String,
    /// <p>Entitlement resource. Use only if the unit is None.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>Data associated with an entitlement resource.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct EntitlementData {
    /// <p>Entitlement data name.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>Entitlement data unit.</p>
    #[serde(rename = "Unit")]
    pub unit: String,
    /// <p>Entitlement data value.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>Usage associated with an entitlement resource.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EntitlementUsage {
    /// <p>Resource usage consumed.</p>
    #[serde(rename = "ConsumedValue")]
    pub consumed_value: String,
    /// <p>Maximum entitlement usage count.</p>
    #[serde(rename = "MaxCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_count: Option<String>,
    /// <p>Entitlement usage name.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>Entitlement usage unit.</p>
    #[serde(rename = "Unit")]
    pub unit: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ExtendLicenseConsumptionRequest {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request. Provides an error response if you do not have the required permissions.</p>
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// <p>License consumption token.</p>
    #[serde(rename = "LicenseConsumptionToken")]
    pub license_consumption_token: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ExtendLicenseConsumptionResponse {
    /// <p>Date and time at which the license consumption expires.</p>
    #[serde(rename = "Expiration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<String>,
    /// <p>License consumption token.</p>
    #[serde(rename = "LicenseConsumptionToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_consumption_token: Option<String>,
}

/// <p>A filter name and value pair that is used to return more specific results from a describe operation. Filters can be used to match a set of resources by specific criteria, such as tags, attributes, or IDs.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Filter {
    /// <p>Name of the filter. Filter names are case-sensitive.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Filter values. Filter values are case-sensitive.</p>
    #[serde(rename = "Values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetAccessTokenRequest {
    /// <p>Refresh token, encoded as a JWT token.</p>
    #[serde(rename = "Token")]
    pub token: String,
    /// <p>Token properties to validate against those present in the JWT token.</p>
    #[serde(rename = "TokenProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_properties: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetAccessTokenResponse {
    /// <p>Temporary access token.</p>
    #[serde(rename = "AccessToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetGrantRequest {
    /// <p>Amazon Resource Name (ARN) of the grant.</p>
    #[serde(rename = "GrantArn")]
    pub grant_arn: String,
    /// <p>Grant version.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetGrantResponse {
    /// <p>Grant details.</p>
    #[serde(rename = "Grant")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant: Option<Grant>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetLicenseConfigurationRequest {
    /// <p>Amazon Resource Name (ARN) of the license configuration.</p>
    #[serde(rename = "LicenseConfigurationArn")]
    pub license_configuration_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetLicenseConfigurationResponse {
    /// <p>Automated discovery information.</p>
    #[serde(rename = "AutomatedDiscoveryInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automated_discovery_information: Option<AutomatedDiscoveryInformation>,
    /// <p>Summaries of the licenses consumed by resources.</p>
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
    /// <p>When true, disassociates a resource when software is uninstalled.</p>
    #[serde(rename = "DisassociateWhenNotFound")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disassociate_when_not_found: Option<bool>,
    /// <p>Amazon Resource Name (ARN) of the license configuration.</p>
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
    /// <p>Dimension on which the licenses are counted.</p>
    #[serde(rename = "LicenseCountingType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_counting_type: Option<String>,
    /// <p>License rules.</p>
    #[serde(rename = "LicenseRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_rules: Option<Vec<String>>,
    /// <p>Summaries of the managed resources.</p>
    #[serde(rename = "ManagedResourceSummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_resource_summary_list: Option<Vec<ManagedResourceSummary>>,
    /// <p>Name of the license configuration.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Account ID of the owner of the license configuration.</p>
    #[serde(rename = "OwnerAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_account_id: Option<String>,
    /// <p>Product information.</p>
    #[serde(rename = "ProductInformationList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_information_list: Option<Vec<ProductInformation>>,
    /// <p>License configuration status.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>Tags for the license configuration.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetLicenseRequest {
    /// <p>Amazon Resource Name (ARN) of the license.</p>
    #[serde(rename = "LicenseArn")]
    pub license_arn: String,
    /// <p>License version.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetLicenseResponse {
    /// <p>License details.</p>
    #[serde(rename = "License")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<License>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetLicenseUsageRequest {
    /// <p>Amazon Resource Name (ARN) of the license.</p>
    #[serde(rename = "LicenseArn")]
    pub license_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetLicenseUsageResponse {
    /// <p>License usage details.</p>
    #[serde(rename = "LicenseUsage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_usage: Option<LicenseUsage>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetServiceSettingsRequest {}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetServiceSettingsResponse {
    /// <p>Indicates whether cross-account discovery is enabled.</p>
    #[serde(rename = "EnableCrossAccountsDiscovery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_cross_accounts_discovery: Option<bool>,
    /// <p>Amazon Resource Name (ARN) of the AWS resource share. The License Manager master account will provide member accounts with access to this share.</p>
    #[serde(rename = "LicenseManagerResourceShareArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_manager_resource_share_arn: Option<String>,
    /// <p>Indicates whether AWS Organizations is integrated with License Manager for cross-account discovery.</p>
    #[serde(rename = "OrganizationConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_configuration: Option<OrganizationConfiguration>,
    /// <p>Regional S3 bucket path for storing reports, license trail event data, discovery data, and so on.</p>
    #[serde(rename = "S3BucketArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_arn: Option<String>,
    /// <p>SNS topic configured to receive notifications from License Manager.</p>
    #[serde(rename = "SnsTopicArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arn: Option<String>,
}

/// <p>Describes a grant.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Grant {
    /// <p>Amazon Resource Name (ARN) of the grant.</p>
    #[serde(rename = "GrantArn")]
    pub grant_arn: String,
    /// <p>Grant name.</p>
    #[serde(rename = "GrantName")]
    pub grant_name: String,
    /// <p>Grant status.</p>
    #[serde(rename = "GrantStatus")]
    pub grant_status: String,
    /// <p>Granted operations.</p>
    #[serde(rename = "GrantedOperations")]
    pub granted_operations: Vec<String>,
    /// <p>The grantee principal ARN.</p>
    #[serde(rename = "GranteePrincipalArn")]
    pub grantee_principal_arn: String,
    /// <p>Home Region of the grant.</p>
    #[serde(rename = "HomeRegion")]
    pub home_region: String,
    /// <p>License ARN.</p>
    #[serde(rename = "LicenseArn")]
    pub license_arn: String,
    /// <p>Parent ARN.</p>
    #[serde(rename = "ParentArn")]
    pub parent_arn: String,
    /// <p>Grant status reason.</p>
    #[serde(rename = "StatusReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    /// <p>Grant version.</p>
    #[serde(rename = "Version")]
    pub version: String,
}

/// <p>Describes a license that is granted to a grantee.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GrantedLicense {
    /// <p>Granted license beneficiary.</p>
    #[serde(rename = "Beneficiary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary: Option<String>,
    /// <p>Configuration for consumption of the license.</p>
    #[serde(rename = "ConsumptionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumption_configuration: Option<ConsumptionConfiguration>,
    /// <p>Creation time of the granted license.</p>
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// <p>License entitlements.</p>
    #[serde(rename = "Entitlements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlements: Option<Vec<Entitlement>>,
    /// <p>Home Region of the granted license.</p>
    #[serde(rename = "HomeRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_region: Option<String>,
    /// <p>Granted license issuer.</p>
    #[serde(rename = "Issuer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<IssuerDetails>,
    /// <p>Amazon Resource Name (ARN) of the license.</p>
    #[serde(rename = "LicenseArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_arn: Option<String>,
    /// <p>Granted license metadata.</p>
    #[serde(rename = "LicenseMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_metadata: Option<Vec<Metadata>>,
    /// <p>License name.</p>
    #[serde(rename = "LicenseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_name: Option<String>,
    /// <p>Product name.</p>
    #[serde(rename = "ProductName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    /// <p>Product SKU.</p>
    #[serde(rename = "ProductSKU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_sku: Option<String>,
    /// <p>Granted license received metadata.</p>
    #[serde(rename = "ReceivedMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_metadata: Option<ReceivedMetadata>,
    /// <p>Granted license status.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>Date and time range during which the granted license is valid, in ISO8601-UTC format.</p>
    #[serde(rename = "Validity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity: Option<DatetimeRange>,
    /// <p>Version of the granted license.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>An inventory filter.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct InventoryFilter {
    /// <p>Condition of the filter.</p>
    #[serde(rename = "Condition")]
    pub condition: String,
    /// <p>Name of the filter.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>Value of the filter.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>Details about the issuer of a license.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Issuer {
    /// <p>Issuer name.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>Asymmetric CMK from AWS Key Management Service. The CMK must have a key usage of sign and verify, and support the RSASSA-PSS SHA-256 signing algorithm.</p>
    #[serde(rename = "SignKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sign_key: Option<String>,
}

/// <p>Details associated with the issuer of a license.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct IssuerDetails {
    /// <p>Issuer key fingerprint.</p>
    #[serde(rename = "KeyFingerprint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_fingerprint: Option<String>,
    /// <p>Issuer name.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Asymmetric CMK from AWS Key Management Service. The CMK must have a key usage of sign and verify, and support the RSASSA-PSS SHA-256 signing algorithm.</p>
    #[serde(rename = "SignKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sign_key: Option<String>,
}

/// <p>Software license that is managed in AWS License Manager.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct License {
    /// <p>License beneficiary.</p>
    #[serde(rename = "Beneficiary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary: Option<String>,
    /// <p>Configuration for consumption of the license.</p>
    #[serde(rename = "ConsumptionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumption_configuration: Option<ConsumptionConfiguration>,
    /// <p>License creation time.</p>
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// <p>License entitlements.</p>
    #[serde(rename = "Entitlements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlements: Option<Vec<Entitlement>>,
    /// <p>Home Region of the license.</p>
    #[serde(rename = "HomeRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_region: Option<String>,
    /// <p>License issuer.</p>
    #[serde(rename = "Issuer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<IssuerDetails>,
    /// <p>Amazon Resource Name (ARN) of the license.</p>
    #[serde(rename = "LicenseArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_arn: Option<String>,
    /// <p>License metadata.</p>
    #[serde(rename = "LicenseMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_metadata: Option<Vec<Metadata>>,
    /// <p>License name.</p>
    #[serde(rename = "LicenseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_name: Option<String>,
    /// <p>Product name.</p>
    #[serde(rename = "ProductName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    /// <p>Product SKU.</p>
    #[serde(rename = "ProductSKU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_sku: Option<String>,
    /// <p>License status.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>Date and time range during which the license is valid, in ISO8601-UTC format.</p>
    #[serde(rename = "Validity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity: Option<DatetimeRange>,
    /// <p>License version.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>A license configuration is an abstraction of a customer license agreement that can be consumed and enforced by License Manager. Components include specifications for the license type (licensing by instance, socket, CPU, or vCPU), allowed tenancy (shared tenancy, Dedicated Instance, Dedicated Host, or all of these), host affinity (how long a VM must be associated with a host), and the number of licenses purchased and used.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LicenseConfiguration {
    /// <p>Automated discovery information.</p>
    #[serde(rename = "AutomatedDiscoveryInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automated_discovery_information: Option<AutomatedDiscoveryInformation>,
    /// <p>Summaries for licenses consumed by various resources.</p>
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
    /// <p>When true, disassociates a resource when software is uninstalled.</p>
    #[serde(rename = "DisassociateWhenNotFound")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disassociate_when_not_found: Option<bool>,
    /// <p>Amazon Resource Name (ARN) of the license configuration.</p>
    #[serde(rename = "LicenseConfigurationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_configuration_arn: Option<String>,
    /// <p>Unique ID of the license configuration.</p>
    #[serde(rename = "LicenseConfigurationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_configuration_id: Option<String>,
    /// <p>Number of licenses managed by the license configuration.</p>
    #[serde(rename = "LicenseCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_count: Option<i64>,
    /// <p>Number of available licenses as a hard limit.</p>
    #[serde(rename = "LicenseCountHardLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_count_hard_limit: Option<bool>,
    /// <p>Dimension to use to track the license inventory.</p>
    #[serde(rename = "LicenseCountingType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_counting_type: Option<String>,
    /// <p>License rules.</p>
    #[serde(rename = "LicenseRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_rules: Option<Vec<String>>,
    /// <p>Summaries for managed resources.</p>
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
    /// <p>Product information.</p>
    #[serde(rename = "ProductInformationList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_information_list: Option<Vec<ProductInformation>>,
    /// <p>Status of the license configuration.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Describes an association with a license configuration.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LicenseConfigurationAssociation {
    /// <p>Scope of AMI associations. The possible value is <code>cross-account</code>.</p>
    #[serde(rename = "AmiAssociationScope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ami_association_scope: Option<String>,
    /// <p>Time when the license configuration was associated with the resource.</p>
    #[serde(rename = "AssociationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_time: Option<f64>,
    /// <p>Amazon Resource Name (ARN) of the resource.</p>
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

/// <p>Details about the usage of a resource associated with a license configuration.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LicenseConfigurationUsage {
    /// <p>Time when the license configuration was initially associated with the resource.</p>
    #[serde(rename = "AssociationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_time: Option<f64>,
    /// <p>Number of licenses consumed by the resource.</p>
    #[serde(rename = "ConsumedLicenses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_licenses: Option<i64>,
    /// <p>Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "ResourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    /// <p>ID of the account that owns the resource.</p>
    #[serde(rename = "ResourceOwnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_owner_id: Option<String>,
    /// <p>Status of the resource.</p>
    #[serde(rename = "ResourceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_status: Option<String>,
    /// <p>Type of resource.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

/// <p>Describes the failure of a license operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LicenseOperationFailure {
    /// <p>Error message.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>Failure time.</p>
    #[serde(rename = "FailureTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_time: Option<f64>,
    /// <p>Reserved.</p>
    #[serde(rename = "MetadataList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_list: Option<Vec<Metadata>>,
    /// <p>Name of the operation.</p>
    #[serde(rename = "OperationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
    /// <p>The requester is "License Manager Automated Discovery".</p>
    #[serde(rename = "OperationRequestedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_requested_by: Option<String>,
    /// <p>Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "ResourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    /// <p>ID of the AWS account that owns the resource.</p>
    #[serde(rename = "ResourceOwnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_owner_id: Option<String>,
    /// <p>Resource type.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

/// <p>Details for associating a license configuration with a resource.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct LicenseSpecification {
    /// <p>Scope of AMI associations. The possible value is <code>cross-account</code>.</p>
    #[serde(rename = "AmiAssociationScope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ami_association_scope: Option<String>,
    /// <p>Amazon Resource Name (ARN) of the license configuration.</p>
    #[serde(rename = "LicenseConfigurationArn")]
    pub license_configuration_arn: String,
}

/// <p>Describes the entitlement usage associated with a license.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LicenseUsage {
    /// <p>License entitlement usages.</p>
    #[serde(rename = "EntitlementUsages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlement_usages: Option<Vec<EntitlementUsage>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListAssociationsForLicenseConfigurationRequest {
    /// <p>Amazon Resource Name (ARN) of a license configuration.</p>
    #[serde(rename = "LicenseConfigurationArn")]
    pub license_configuration_arn: String,
    /// <p>Maximum number of results to return in a single call.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListAssociationsForLicenseConfigurationResponse {
    /// <p>Information about the associations for the license configuration.</p>
    #[serde(rename = "LicenseConfigurationAssociations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_configuration_associations: Option<Vec<LicenseConfigurationAssociation>>,
    /// <p>Token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListDistributedGrantsRequest {
    /// <p><p>Filters to scope the results. The following filters are supported:</p> <ul> <li> <p> <code>LicenseARN</code> </p> </li> <li> <p> <code>Status</code> </p> </li> <li> <p> <code>PrincipalARN</code> </p> </li> <li> <p> <code>ParentARN</code> </p> </li> </ul></p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>Amazon Resource Names (ARNs) of the grants.</p>
    #[serde(rename = "GrantArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_arns: Option<Vec<String>>,
    /// <p>Maximum number of results to return in a single call.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListDistributedGrantsResponse {
    /// <p>Distributed grant details.</p>
    #[serde(rename = "Grants")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grants: Option<Vec<Grant>>,
    /// <p>Token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListFailuresForLicenseConfigurationOperationsRequest {
    /// <p>Amazon Resource Name of the license configuration.</p>
    #[serde(rename = "LicenseConfigurationArn")]
    pub license_configuration_arn: String,
    /// <p>Maximum number of results to return in a single call.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListFailuresForLicenseConfigurationOperationsResponse {
    /// <p>License configuration operations that failed.</p>
    #[serde(rename = "LicenseOperationFailureList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_operation_failure_list: Option<Vec<LicenseOperationFailure>>,
    /// <p>Token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListLicenseConfigurationsRequest {
    /// <p><p>Filters to scope the results. The following filters and logical operators are supported:</p> <ul> <li> <p> <code>licenseCountingType</code> - The dimension on which licenses are counted. Possible values are <code>vCPU</code> | <code>Instance</code> | <code>Core</code> | <code>Socket</code>. Logical operators are <code>EQUALS</code> | <code>NOT<em>EQUALS</code>.</p> </li> <li> <p> <code>enforceLicenseCount</code> - A Boolean value that indicates whether hard license enforcement is used. Logical operators are <code>EQUALS</code> | <code>NOT</em>EQUALS</code>.</p> </li> <li> <p> <code>usagelimitExceeded</code> - A Boolean value that indicates whether the available licenses have been exceeded. Logical operators are <code>EQUALS</code> | <code>NOT_EQUALS</code>.</p> </li> </ul></p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>Amazon Resource Names (ARN) of the license configurations.</p>
    #[serde(rename = "LicenseConfigurationArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_configuration_arns: Option<Vec<String>>,
    /// <p>Maximum number of results to return in a single call.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListLicenseConfigurationsResponse {
    /// <p>Information about the license configurations.</p>
    #[serde(rename = "LicenseConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_configurations: Option<Vec<LicenseConfiguration>>,
    /// <p>Token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListLicenseSpecificationsForResourceRequest {
    /// <p>Maximum number of results to return in a single call.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Amazon Resource Name (ARN) of a resource that has an associated license configuration.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListLicenseVersionsRequest {
    /// <p>Amazon Resource Name (ARN) of the license.</p>
    #[serde(rename = "LicenseArn")]
    pub license_arn: String,
    /// <p>Maximum number of results to return in a single call.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListLicenseVersionsResponse {
    /// <p>License details.</p>
    #[serde(rename = "Licenses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub licenses: Option<Vec<License>>,
    /// <p>Token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListLicensesRequest {
    /// <p><p>Filters to scope the results. The following filters are supported:</p> <ul> <li> <p> <code>Beneficiary</code> </p> </li> <li> <p> <code>ProductSKU</code> </p> </li> <li> <p> <code>KeyFingerprint</code> </p> </li> <li> <p> <code>Status</code> </p> </li> </ul></p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>Amazon Resource Names (ARNs) of the licenses.</p>
    #[serde(rename = "LicenseArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_arns: Option<Vec<String>>,
    /// <p>Maximum number of results to return in a single call.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListLicensesResponse {
    /// <p>License details.</p>
    #[serde(rename = "Licenses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub licenses: Option<Vec<License>>,
    /// <p>Token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListReceivedGrantsRequest {
    /// <p><p>Filters to scope the results. The following filters are supported:</p> <ul> <li> <p> <code>LicenseARN</code> </p> </li> <li> <p> <code>Status</code> </p> </li> </ul></p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>Amazon Resource Names (ARNs) of the grants.</p>
    #[serde(rename = "GrantArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_arns: Option<Vec<String>>,
    /// <p>Maximum number of results to return in a single call.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListReceivedGrantsResponse {
    /// <p>Received grant details.</p>
    #[serde(rename = "Grants")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grants: Option<Vec<Grant>>,
    /// <p>Token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListReceivedLicensesRequest {
    /// <p><p>Filters to scope the results. The following filters are supported:</p> <ul> <li> <p> <code>ProductSKU</code> </p> </li> <li> <p> <code>Status</code> </p> </li> <li> <p> <code>KeyFingerprint</code> </p> </li> <li> <p> <code>Issuer</code> </p> </li> </ul></p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>Amazon Resource Names (ARNs) of the licenses.</p>
    #[serde(rename = "LicenseArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_arns: Option<Vec<String>>,
    /// <p>Maximum number of results to return in a single call.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListReceivedLicensesResponse {
    /// <p>Received license details.</p>
    #[serde(rename = "Licenses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub licenses: Option<Vec<GrantedLicense>>,
    /// <p>Token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListResourceInventoryRequest {
    /// <p><p>Filters to scope the results. The following filters and logical operators are supported:</p> <ul> <li> <p> <code>account<em>id</code> - The ID of the AWS account that owns the resource. Logical operators are <code>EQUALS</code> | <code>NOT</em>EQUALS</code>.</p> </li> <li> <p> <code>application<em>name</code> - The name of the application. Logical operators are <code>EQUALS</code> | <code>BEGINS</em>WITH</code>.</p> </li> <li> <p> <code>license<em>included</code> - The type of license included. Logical operators are <code>EQUALS</code> | <code>NOT</em>EQUALS</code>. Possible values are <code>sql-server-enterprise</code> | <code>sql-server-standard</code> | <code>sql-server-web</code> | <code>windows-server-datacenter</code>.</p> </li> <li> <p> <code>platform</code> - The platform of the resource. Logical operators are <code>EQUALS</code> | <code>BEGINS<em>WITH</code>.</p> </li> <li> <p> <code>resource</em>id</code> - The ID of the resource. Logical operators are <code>EQUALS</code> | <code>NOT<em>EQUALS</code>.</p> </li> <li> <p> <code>tag:&lt;key&gt;</code> - The key/value combination of a tag assigned to the resource. Logical operators are <code>EQUALS</code> (single account) or <code>EQUALS</code> | <code>NOT</em>EQUALS</code> (cross account).</p> </li> </ul></p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<InventoryFilter>>,
    /// <p>Maximum number of results to return in a single call.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListResourceInventoryResponse {
    /// <p>Token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the resources.</p>
    #[serde(rename = "ResourceInventoryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_inventory_list: Option<Vec<ResourceInventory>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>Amazon Resource Name (ARN) of the license configuration.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>Information about the tags.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTokensRequest {
    /// <p><p>Filters to scope the results. The following filter is supported:</p> <ul> <li> <p> <code>licenseArns</code> </p> </li> </ul></p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>Maximum number of results to return in a single call.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Token IDs.</p>
    #[serde(rename = "TokenIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_ids: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTokensResponse {
    /// <p>Token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Received token details.</p>
    #[serde(rename = "Tokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokens: Option<Vec<TokenData>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListUsageForLicenseConfigurationRequest {
    /// <p><p>Filters to scope the results. The following filters and logical operators are supported:</p> <ul> <li> <p> <code>resourceArn</code> - The ARN of the license configuration resource. Logical operators are <code>EQUALS</code> | <code>NOT<em>EQUALS</code>.</p> </li> <li> <p> <code>resourceType</code> - The resource type (<code>EC2</em>INSTANCE</code> | <code>EC2<em>HOST</code> | <code>EC2</em>AMI</code> | <code>SYSTEMS<em>MANAGER</em>MANAGED<em>INSTANCE</code>). Logical operators are <code>EQUALS</code> | <code>NOT</em>EQUALS</code>.</p> </li> <li> <p> <code>resourceAccount</code> - The ID of the account that owns the resource. Logical operators are <code>EQUALS</code> | <code>NOT_EQUALS</code>.</p> </li> </ul></p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>Amazon Resource Name (ARN) of the license configuration.</p>
    #[serde(rename = "LicenseConfigurationArn")]
    pub license_configuration_arn: String,
    /// <p>Maximum number of results to return in a single call.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListUsageForLicenseConfigurationResponse {
    /// <p>Information about the license configurations.</p>
    #[serde(rename = "LicenseConfigurationUsageList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_configuration_usage_list: Option<Vec<LicenseConfigurationUsage>>,
    /// <p>Token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Summary information about a managed resource.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ManagedResourceSummary {
    /// <p>Number of resources associated with licenses.</p>
    #[serde(rename = "AssociationCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_count: Option<i64>,
    /// <p>Type of resource associated with a license.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

/// <p>Describes key/value pairs.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Metadata {
    /// <p>The key name.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The value.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>Configuration information for AWS Organizations.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct OrganizationConfiguration {
    /// <p>Enables AWS Organization integration.</p>
    #[serde(rename = "EnableIntegration")]
    pub enable_integration: bool,
}

/// <p>Describes product information for a license configuration.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ProductInformation {
    /// <p><p>Product information filters.</p> <p>The following filters and logical operators are supported when the resource type is <code>SSM<em>MANAGED</code>:</p> <ul> <li> <p> <code>Application Name</code> - The name of the application. Logical operator is <code>EQUALS</code>.</p> </li> <li> <p> <code>Application Publisher</code> - The publisher of the application. Logical operator is <code>EQUALS</code>.</p> </li> <li> <p> <code>Application Version</code> - The version of the application. Logical operator is <code>EQUALS</code>.</p> </li> <li> <p> <code>Platform Name</code> - The name of the platform. Logical operator is <code>EQUALS</code>.</p> </li> <li> <p> <code>Platform Type</code> - The platform type. Logical operator is <code>EQUALS</code>.</p> </li> <li> <p> <code>License Included</code> - The type of license included. Logical operators are <code>EQUALS</code> and <code>NOT</em>EQUALS</code>. Possible values are: <code>sql-server-enterprise</code> | <code>sql-server-standard</code> | <code>sql-server-web</code> | <code>windows-server-datacenter</code>.</p> </li> </ul> <p>The following filters and logical operators are supported when the resource type is <code>RDS</code>:</p> <ul> <li> <p> <code>Engine Edition</code> - The edition of the database engine. Logical operator is <code>EQUALS</code>. Possible values are: <code>oracle-ee</code> | <code>oracle-se</code> | <code>oracle-se1</code> | <code>oracle-se2</code>.</p> </li> <li> <p> <code>License Pack</code> - The license pack. Logical operator is <code>EQUALS</code>. Possible values are: <code>data guard</code> | <code>diagnostic pack sqlt</code> | <code>tuning pack sqlt</code> | <code>ols</code> | <code>olap</code>.</p> </li> </ul></p>
    #[serde(rename = "ProductInformationFilterList")]
    pub product_information_filter_list: Vec<ProductInformationFilter>,
    /// <p>Resource type. The possible values are <code>SSM_MANAGED</code> | <code>RDS</code>.</p>
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
}

/// <p>Describes product information filters.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ProductInformationFilter {
    /// <p>Logical operator.</p>
    #[serde(rename = "ProductInformationFilterComparator")]
    pub product_information_filter_comparator: String,
    /// <p>Filter name.</p>
    #[serde(rename = "ProductInformationFilterName")]
    pub product_information_filter_name: String,
    /// <p>Filter value.</p>
    #[serde(rename = "ProductInformationFilterValue")]
    pub product_information_filter_value: Vec<String>,
}

/// <p>Details about a provisional configuration.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ProvisionalConfiguration {
    /// <p>Maximum time for the provisional configuration, in minutes.</p>
    #[serde(rename = "MaxTimeToLiveInMinutes")]
    pub max_time_to_live_in_minutes: i64,
}

/// <p>Metadata associated with received licenses and grants.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ReceivedMetadata {
    /// <p>Allowed operations.</p>
    #[serde(rename = "AllowedOperations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_operations: Option<Vec<String>>,
    /// <p>Received status.</p>
    #[serde(rename = "ReceivedStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_status: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RejectGrantRequest {
    /// <p>Amazon Resource Name (ARN) of the grant.</p>
    #[serde(rename = "GrantArn")]
    pub grant_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RejectGrantResponse {
    /// <p>Grant ARN.</p>
    #[serde(rename = "GrantArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_arn: Option<String>,
    /// <p>Grant status.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>Grant version.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>Details about a resource.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResourceInventory {
    /// <p>Platform of the resource.</p>
    #[serde(rename = "Platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// <p>Platform version of the resource in the inventory.</p>
    #[serde(rename = "PlatformVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<String>,
    /// <p>Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "ResourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    /// <p>ID of the resource.</p>
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// <p>ID of the account that owns the resource.</p>
    #[serde(rename = "ResourceOwningAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_owning_account_id: Option<String>,
    /// <p>Type of resource.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

/// <p>Details about a tag for a license configuration.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Tag {
    /// <p>Tag key.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>Tag value.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>Amazon Resource Name (ARN) of the license configuration.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>One or more tags.</p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

/// <p>Describes a token.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TokenData {
    /// <p>Token expiration time, in ISO8601-UTC format.</p>
    #[serde(rename = "ExpirationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    /// <p>Amazon Resource Name (ARN) of the license.</p>
    #[serde(rename = "LicenseArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_arn: Option<String>,
    /// <p>Amazon Resource Names (ARN) of the roles included in the token.</p>
    #[serde(rename = "RoleArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arns: Option<Vec<String>>,
    /// <p>Token status. The possible values are <code>AVAILABLE</code> and <code>DELETED</code>.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>Token ID.</p>
    #[serde(rename = "TokenId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_id: Option<String>,
    /// <p>Data specified by the caller.</p>
    #[serde(rename = "TokenProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_properties: Option<Vec<String>>,
    /// <p>Type of token generated. The supported value is <code>REFRESH_TOKEN</code>.</p>
    #[serde(rename = "TokenType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_type: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>Amazon Resource Name (ARN) of the license configuration.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>Keys identifying the tags to remove.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateLicenseConfigurationRequest {
    /// <p>New description of the license configuration.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>When true, disassociates a resource when software is uninstalled.</p>
    #[serde(rename = "DisassociateWhenNotFound")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disassociate_when_not_found: Option<bool>,
    /// <p>Amazon Resource Name (ARN) of the license configuration.</p>
    #[serde(rename = "LicenseConfigurationArn")]
    pub license_configuration_arn: String,
    /// <p>New status of the license configuration.</p>
    #[serde(rename = "LicenseConfigurationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_configuration_status: Option<String>,
    /// <p>New number of licenses managed by the license configuration.</p>
    #[serde(rename = "LicenseCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_count: Option<i64>,
    /// <p>New hard limit of the number of available licenses.</p>
    #[serde(rename = "LicenseCountHardLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_count_hard_limit: Option<bool>,
    /// <p>New license rule. The only rule that you can add after you create a license configuration is licenseAffinityToHost.</p>
    #[serde(rename = "LicenseRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_rules: Option<Vec<String>>,
    /// <p>New name of the license configuration.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>New product information.</p>
    #[serde(rename = "ProductInformationList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_information_list: Option<Vec<ProductInformation>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateLicenseConfigurationResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateLicenseSpecificationsForResourceRequest {
    /// <p>ARNs of the license configurations to add.</p>
    #[serde(rename = "AddLicenseSpecifications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_license_specifications: Option<Vec<LicenseSpecification>>,
    /// <p>ARNs of the license configurations to remove.</p>
    #[serde(rename = "RemoveLicenseSpecifications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_license_specifications: Option<Vec<LicenseSpecification>>,
    /// <p>Amazon Resource Name (ARN) of the AWS resource.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateLicenseSpecificationsForResourceResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateServiceSettingsRequest {
    /// <p>Activates cross-account discovery.</p>
    #[serde(rename = "EnableCrossAccountsDiscovery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_cross_accounts_discovery: Option<bool>,
    /// <p>Enables integration with AWS Organizations for cross-account discovery.</p>
    #[serde(rename = "OrganizationConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_configuration: Option<OrganizationConfiguration>,
    /// <p>Amazon Resource Name (ARN) of the Amazon S3 bucket where the License Manager information is stored.</p>
    #[serde(rename = "S3BucketArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_arn: Option<String>,
    /// <p>Amazon Resource Name (ARN) of the Amazon SNS topic used for License Manager alerts.</p>
    #[serde(rename = "SnsTopicArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arn: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateServiceSettingsResponse {}

/// Errors returned by AcceptGrant
#[derive(Debug, PartialEq)]
pub enum AcceptGrantError {
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

impl AcceptGrantError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AcceptGrantError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(AcceptGrantError::AccessDenied(err.msg))
                }
                "AuthorizationException" => {
                    return RusotoError::Service(AcceptGrantError::Authorization(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(AcceptGrantError::InvalidParameterValue(err.msg))
                }
                "RateLimitExceededException" => {
                    return RusotoError::Service(AcceptGrantError::RateLimitExceeded(err.msg))
                }
                "ResourceLimitExceededException" => {
                    return RusotoError::Service(AcceptGrantError::ResourceLimitExceeded(err.msg))
                }
                "ServerInternalException" => {
                    return RusotoError::Service(AcceptGrantError::ServerInternal(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AcceptGrantError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AcceptGrantError::AccessDenied(ref cause) => write!(f, "{}", cause),
            AcceptGrantError::Authorization(ref cause) => write!(f, "{}", cause),
            AcceptGrantError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            AcceptGrantError::RateLimitExceeded(ref cause) => write!(f, "{}", cause),
            AcceptGrantError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
            AcceptGrantError::ServerInternal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AcceptGrantError {}
/// Errors returned by CheckInLicense
#[derive(Debug, PartialEq)]
pub enum CheckInLicenseError {
    /// <p>Access to resource denied.</p>
    AccessDenied(String),
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    Authorization(String),
    /// <p>There was a conflict processing the request. Try your request again.</p>
    Conflict(String),
    /// <p>Too many requests have been submitted. Try again after a brief wait.</p>
    RateLimitExceeded(String),
    /// <p>The resource cannot be found.</p>
    ResourceNotFound(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternal(String),
}

impl CheckInLicenseError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CheckInLicenseError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CheckInLicenseError::AccessDenied(err.msg))
                }
                "AuthorizationException" => {
                    return RusotoError::Service(CheckInLicenseError::Authorization(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CheckInLicenseError::Conflict(err.msg))
                }
                "RateLimitExceededException" => {
                    return RusotoError::Service(CheckInLicenseError::RateLimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CheckInLicenseError::ResourceNotFound(err.msg))
                }
                "ServerInternalException" => {
                    return RusotoError::Service(CheckInLicenseError::ServerInternal(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CheckInLicenseError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CheckInLicenseError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CheckInLicenseError::Authorization(ref cause) => write!(f, "{}", cause),
            CheckInLicenseError::Conflict(ref cause) => write!(f, "{}", cause),
            CheckInLicenseError::RateLimitExceeded(ref cause) => write!(f, "{}", cause),
            CheckInLicenseError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CheckInLicenseError::ServerInternal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CheckInLicenseError {}
/// Errors returned by CheckoutBorrowLicense
#[derive(Debug, PartialEq)]
pub enum CheckoutBorrowLicenseError {
    /// <p>Access to resource denied.</p>
    AccessDenied(String),
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    Authorization(String),
    /// <p>The entitlement is not allowed.</p>
    EntitlementNotAllowed(String),
    /// <p>There are no entitlements found for this license, or the entitlement maximum count is reached.</p>
    NoEntitlementsAllowed(String),
    /// <p>Too many requests have been submitted. Try again after a brief wait.</p>
    RateLimitExceeded(String),
    /// <p>This is not the correct Region for the resource. Try again.</p>
    Redirect(String),
    /// <p>The resource cannot be found.</p>
    ResourceNotFound(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternal(String),
    /// <p>The digital signature method is unsupported. Try your request again.</p>
    UnsupportedDigitalSignatureMethod(String),
}

impl CheckoutBorrowLicenseError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CheckoutBorrowLicenseError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CheckoutBorrowLicenseError::AccessDenied(err.msg))
                }
                "AuthorizationException" => {
                    return RusotoError::Service(CheckoutBorrowLicenseError::Authorization(err.msg))
                }
                "EntitlementNotAllowedException" => {
                    return RusotoError::Service(CheckoutBorrowLicenseError::EntitlementNotAllowed(
                        err.msg,
                    ))
                }
                "NoEntitlementsAllowedException" => {
                    return RusotoError::Service(CheckoutBorrowLicenseError::NoEntitlementsAllowed(
                        err.msg,
                    ))
                }
                "RateLimitExceededException" => {
                    return RusotoError::Service(CheckoutBorrowLicenseError::RateLimitExceeded(
                        err.msg,
                    ))
                }
                "RedirectException" => {
                    return RusotoError::Service(CheckoutBorrowLicenseError::Redirect(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CheckoutBorrowLicenseError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServerInternalException" => {
                    return RusotoError::Service(CheckoutBorrowLicenseError::ServerInternal(
                        err.msg,
                    ))
                }
                "UnsupportedDigitalSignatureMethodException" => {
                    return RusotoError::Service(
                        CheckoutBorrowLicenseError::UnsupportedDigitalSignatureMethod(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CheckoutBorrowLicenseError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CheckoutBorrowLicenseError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CheckoutBorrowLicenseError::Authorization(ref cause) => write!(f, "{}", cause),
            CheckoutBorrowLicenseError::EntitlementNotAllowed(ref cause) => write!(f, "{}", cause),
            CheckoutBorrowLicenseError::NoEntitlementsAllowed(ref cause) => write!(f, "{}", cause),
            CheckoutBorrowLicenseError::RateLimitExceeded(ref cause) => write!(f, "{}", cause),
            CheckoutBorrowLicenseError::Redirect(ref cause) => write!(f, "{}", cause),
            CheckoutBorrowLicenseError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CheckoutBorrowLicenseError::ServerInternal(ref cause) => write!(f, "{}", cause),
            CheckoutBorrowLicenseError::UnsupportedDigitalSignatureMethod(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CheckoutBorrowLicenseError {}
/// Errors returned by CheckoutLicense
#[derive(Debug, PartialEq)]
pub enum CheckoutLicenseError {
    /// <p>Access to resource denied.</p>
    AccessDenied(String),
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    Authorization(String),
    /// <p>There are no entitlements found for this license, or the entitlement maximum count is reached.</p>
    NoEntitlementsAllowed(String),
    /// <p>Too many requests have been submitted. Try again after a brief wait.</p>
    RateLimitExceeded(String),
    /// <p>This is not the correct Region for the resource. Try again.</p>
    Redirect(String),
    /// <p>The resource cannot be found.</p>
    ResourceNotFound(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternal(String),
    /// <p>The digital signature method is unsupported. Try your request again.</p>
    UnsupportedDigitalSignatureMethod(String),
}

impl CheckoutLicenseError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CheckoutLicenseError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CheckoutLicenseError::AccessDenied(err.msg))
                }
                "AuthorizationException" => {
                    return RusotoError::Service(CheckoutLicenseError::Authorization(err.msg))
                }
                "NoEntitlementsAllowedException" => {
                    return RusotoError::Service(CheckoutLicenseError::NoEntitlementsAllowed(
                        err.msg,
                    ))
                }
                "RateLimitExceededException" => {
                    return RusotoError::Service(CheckoutLicenseError::RateLimitExceeded(err.msg))
                }
                "RedirectException" => {
                    return RusotoError::Service(CheckoutLicenseError::Redirect(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CheckoutLicenseError::ResourceNotFound(err.msg))
                }
                "ServerInternalException" => {
                    return RusotoError::Service(CheckoutLicenseError::ServerInternal(err.msg))
                }
                "UnsupportedDigitalSignatureMethodException" => {
                    return RusotoError::Service(
                        CheckoutLicenseError::UnsupportedDigitalSignatureMethod(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CheckoutLicenseError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CheckoutLicenseError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CheckoutLicenseError::Authorization(ref cause) => write!(f, "{}", cause),
            CheckoutLicenseError::NoEntitlementsAllowed(ref cause) => write!(f, "{}", cause),
            CheckoutLicenseError::RateLimitExceeded(ref cause) => write!(f, "{}", cause),
            CheckoutLicenseError::Redirect(ref cause) => write!(f, "{}", cause),
            CheckoutLicenseError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CheckoutLicenseError::ServerInternal(ref cause) => write!(f, "{}", cause),
            CheckoutLicenseError::UnsupportedDigitalSignatureMethod(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CheckoutLicenseError {}
/// Errors returned by CreateGrant
#[derive(Debug, PartialEq)]
pub enum CreateGrantError {
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

impl CreateGrantError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateGrantError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateGrantError::AccessDenied(err.msg))
                }
                "AuthorizationException" => {
                    return RusotoError::Service(CreateGrantError::Authorization(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(CreateGrantError::InvalidParameterValue(err.msg))
                }
                "RateLimitExceededException" => {
                    return RusotoError::Service(CreateGrantError::RateLimitExceeded(err.msg))
                }
                "ResourceLimitExceededException" => {
                    return RusotoError::Service(CreateGrantError::ResourceLimitExceeded(err.msg))
                }
                "ServerInternalException" => {
                    return RusotoError::Service(CreateGrantError::ServerInternal(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateGrantError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateGrantError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateGrantError::Authorization(ref cause) => write!(f, "{}", cause),
            CreateGrantError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            CreateGrantError::RateLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateGrantError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateGrantError::ServerInternal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateGrantError {}
/// Errors returned by CreateGrantVersion
#[derive(Debug, PartialEq)]
pub enum CreateGrantVersionError {
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

impl CreateGrantVersionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateGrantVersionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateGrantVersionError::AccessDenied(err.msg))
                }
                "AuthorizationException" => {
                    return RusotoError::Service(CreateGrantVersionError::Authorization(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(CreateGrantVersionError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "RateLimitExceededException" => {
                    return RusotoError::Service(CreateGrantVersionError::RateLimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceLimitExceededException" => {
                    return RusotoError::Service(CreateGrantVersionError::ResourceLimitExceeded(
                        err.msg,
                    ))
                }
                "ServerInternalException" => {
                    return RusotoError::Service(CreateGrantVersionError::ServerInternal(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateGrantVersionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateGrantVersionError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateGrantVersionError::Authorization(ref cause) => write!(f, "{}", cause),
            CreateGrantVersionError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            CreateGrantVersionError::RateLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateGrantVersionError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateGrantVersionError::ServerInternal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateGrantVersionError {}
/// Errors returned by CreateLicense
#[derive(Debug, PartialEq)]
pub enum CreateLicenseError {
    /// <p>Access to resource denied.</p>
    AccessDenied(String),
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    Authorization(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValue(String),
    /// <p>Too many requests have been submitted. Try again after a brief wait.</p>
    RateLimitExceeded(String),
    /// <p>This is not the correct Region for the resource. Try again.</p>
    Redirect(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternal(String),
}

impl CreateLicenseError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateLicenseError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateLicenseError::AccessDenied(err.msg))
                }
                "AuthorizationException" => {
                    return RusotoError::Service(CreateLicenseError::Authorization(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(CreateLicenseError::InvalidParameterValue(err.msg))
                }
                "RateLimitExceededException" => {
                    return RusotoError::Service(CreateLicenseError::RateLimitExceeded(err.msg))
                }
                "RedirectException" => {
                    return RusotoError::Service(CreateLicenseError::Redirect(err.msg))
                }
                "ServerInternalException" => {
                    return RusotoError::Service(CreateLicenseError::ServerInternal(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateLicenseError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateLicenseError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateLicenseError::Authorization(ref cause) => write!(f, "{}", cause),
            CreateLicenseError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            CreateLicenseError::RateLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateLicenseError::Redirect(ref cause) => write!(f, "{}", cause),
            CreateLicenseError::ServerInternal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateLicenseError {}
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
            #[allow(clippy::single_match)]
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateLicenseConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateLicenseConfigurationError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateLicenseConfigurationError::Authorization(ref cause) => write!(f, "{}", cause),
            CreateLicenseConfigurationError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateLicenseConfigurationError::RateLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateLicenseConfigurationError::ResourceLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateLicenseConfigurationError::ServerInternal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateLicenseConfigurationError {}
/// Errors returned by CreateLicenseVersion
#[derive(Debug, PartialEq)]
pub enum CreateLicenseVersionError {
    /// <p>Access to resource denied.</p>
    AccessDenied(String),
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    Authorization(String),
    /// <p>There was a conflict processing the request. Try your request again.</p>
    Conflict(String),
    /// <p>Too many requests have been submitted. Try again after a brief wait.</p>
    RateLimitExceeded(String),
    /// <p>This is not the correct Region for the resource. Try again.</p>
    Redirect(String),
    /// <p>The resource cannot be found.</p>
    ResourceNotFound(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternal(String),
}

impl CreateLicenseVersionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateLicenseVersionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateLicenseVersionError::AccessDenied(err.msg))
                }
                "AuthorizationException" => {
                    return RusotoError::Service(CreateLicenseVersionError::Authorization(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateLicenseVersionError::Conflict(err.msg))
                }
                "RateLimitExceededException" => {
                    return RusotoError::Service(CreateLicenseVersionError::RateLimitExceeded(
                        err.msg,
                    ))
                }
                "RedirectException" => {
                    return RusotoError::Service(CreateLicenseVersionError::Redirect(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateLicenseVersionError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServerInternalException" => {
                    return RusotoError::Service(CreateLicenseVersionError::ServerInternal(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateLicenseVersionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateLicenseVersionError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateLicenseVersionError::Authorization(ref cause) => write!(f, "{}", cause),
            CreateLicenseVersionError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateLicenseVersionError::RateLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateLicenseVersionError::Redirect(ref cause) => write!(f, "{}", cause),
            CreateLicenseVersionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateLicenseVersionError::ServerInternal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateLicenseVersionError {}
/// Errors returned by CreateToken
#[derive(Debug, PartialEq)]
pub enum CreateTokenError {
    /// <p>Access to resource denied.</p>
    AccessDenied(String),
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    Authorization(String),
    /// <p>Too many requests have been submitted. Try again after a brief wait.</p>
    RateLimitExceeded(String),
    /// <p>This is not the correct Region for the resource. Try again.</p>
    Redirect(String),
    /// <p>Your resource limits have been exceeded.</p>
    ResourceLimitExceeded(String),
    /// <p>The resource cannot be found.</p>
    ResourceNotFound(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternal(String),
}

impl CreateTokenError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateTokenError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateTokenError::AccessDenied(err.msg))
                }
                "AuthorizationException" => {
                    return RusotoError::Service(CreateTokenError::Authorization(err.msg))
                }
                "RateLimitExceededException" => {
                    return RusotoError::Service(CreateTokenError::RateLimitExceeded(err.msg))
                }
                "RedirectException" => {
                    return RusotoError::Service(CreateTokenError::Redirect(err.msg))
                }
                "ResourceLimitExceededException" => {
                    return RusotoError::Service(CreateTokenError::ResourceLimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateTokenError::ResourceNotFound(err.msg))
                }
                "ServerInternalException" => {
                    return RusotoError::Service(CreateTokenError::ServerInternal(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateTokenError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateTokenError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateTokenError::Authorization(ref cause) => write!(f, "{}", cause),
            CreateTokenError::RateLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateTokenError::Redirect(ref cause) => write!(f, "{}", cause),
            CreateTokenError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateTokenError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateTokenError::ServerInternal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateTokenError {}
/// Errors returned by DeleteGrant
#[derive(Debug, PartialEq)]
pub enum DeleteGrantError {
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

impl DeleteGrantError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteGrantError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteGrantError::AccessDenied(err.msg))
                }
                "AuthorizationException" => {
                    return RusotoError::Service(DeleteGrantError::Authorization(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(DeleteGrantError::InvalidParameterValue(err.msg))
                }
                "RateLimitExceededException" => {
                    return RusotoError::Service(DeleteGrantError::RateLimitExceeded(err.msg))
                }
                "ResourceLimitExceededException" => {
                    return RusotoError::Service(DeleteGrantError::ResourceLimitExceeded(err.msg))
                }
                "ServerInternalException" => {
                    return RusotoError::Service(DeleteGrantError::ServerInternal(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteGrantError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteGrantError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteGrantError::Authorization(ref cause) => write!(f, "{}", cause),
            DeleteGrantError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            DeleteGrantError::RateLimitExceeded(ref cause) => write!(f, "{}", cause),
            DeleteGrantError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
            DeleteGrantError::ServerInternal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteGrantError {}
/// Errors returned by DeleteLicense
#[derive(Debug, PartialEq)]
pub enum DeleteLicenseError {
    /// <p>Access to resource denied.</p>
    AccessDenied(String),
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    Authorization(String),
    /// <p>There was a conflict processing the request. Try your request again.</p>
    Conflict(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValue(String),
    /// <p>Too many requests have been submitted. Try again after a brief wait.</p>
    RateLimitExceeded(String),
    /// <p>This is not the correct Region for the resource. Try again.</p>
    Redirect(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternal(String),
}

impl DeleteLicenseError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteLicenseError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteLicenseError::AccessDenied(err.msg))
                }
                "AuthorizationException" => {
                    return RusotoError::Service(DeleteLicenseError::Authorization(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DeleteLicenseError::Conflict(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(DeleteLicenseError::InvalidParameterValue(err.msg))
                }
                "RateLimitExceededException" => {
                    return RusotoError::Service(DeleteLicenseError::RateLimitExceeded(err.msg))
                }
                "RedirectException" => {
                    return RusotoError::Service(DeleteLicenseError::Redirect(err.msg))
                }
                "ServerInternalException" => {
                    return RusotoError::Service(DeleteLicenseError::ServerInternal(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteLicenseError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteLicenseError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteLicenseError::Authorization(ref cause) => write!(f, "{}", cause),
            DeleteLicenseError::Conflict(ref cause) => write!(f, "{}", cause),
            DeleteLicenseError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            DeleteLicenseError::RateLimitExceeded(ref cause) => write!(f, "{}", cause),
            DeleteLicenseError::Redirect(ref cause) => write!(f, "{}", cause),
            DeleteLicenseError::ServerInternal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteLicenseError {}
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
            #[allow(clippy::single_match)]
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteLicenseConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteLicenseConfigurationError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteLicenseConfigurationError::Authorization(ref cause) => write!(f, "{}", cause),
            DeleteLicenseConfigurationError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteLicenseConfigurationError::RateLimitExceeded(ref cause) => write!(f, "{}", cause),
            DeleteLicenseConfigurationError::ServerInternal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteLicenseConfigurationError {}
/// Errors returned by DeleteToken
#[derive(Debug, PartialEq)]
pub enum DeleteTokenError {
    /// <p>Access to resource denied.</p>
    AccessDenied(String),
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    Authorization(String),
    /// <p>Too many requests have been submitted. Try again after a brief wait.</p>
    RateLimitExceeded(String),
    /// <p>This is not the correct Region for the resource. Try again.</p>
    Redirect(String),
    /// <p>The resource cannot be found.</p>
    ResourceNotFound(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternal(String),
}

impl DeleteTokenError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteTokenError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteTokenError::AccessDenied(err.msg))
                }
                "AuthorizationException" => {
                    return RusotoError::Service(DeleteTokenError::Authorization(err.msg))
                }
                "RateLimitExceededException" => {
                    return RusotoError::Service(DeleteTokenError::RateLimitExceeded(err.msg))
                }
                "RedirectException" => {
                    return RusotoError::Service(DeleteTokenError::Redirect(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteTokenError::ResourceNotFound(err.msg))
                }
                "ServerInternalException" => {
                    return RusotoError::Service(DeleteTokenError::ServerInternal(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteTokenError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteTokenError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteTokenError::Authorization(ref cause) => write!(f, "{}", cause),
            DeleteTokenError::RateLimitExceeded(ref cause) => write!(f, "{}", cause),
            DeleteTokenError::Redirect(ref cause) => write!(f, "{}", cause),
            DeleteTokenError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteTokenError::ServerInternal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteTokenError {}
/// Errors returned by ExtendLicenseConsumption
#[derive(Debug, PartialEq)]
pub enum ExtendLicenseConsumptionError {
    /// <p>Access to resource denied.</p>
    AccessDenied(String),
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    Authorization(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValue(String),
    /// <p>Too many requests have been submitted. Try again after a brief wait.</p>
    RateLimitExceeded(String),
    /// <p>The resource cannot be found.</p>
    ResourceNotFound(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternal(String),
}

impl ExtendLicenseConsumptionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ExtendLicenseConsumptionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ExtendLicenseConsumptionError::AccessDenied(
                        err.msg,
                    ))
                }
                "AuthorizationException" => {
                    return RusotoError::Service(ExtendLicenseConsumptionError::Authorization(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        ExtendLicenseConsumptionError::InvalidParameterValue(err.msg),
                    )
                }
                "RateLimitExceededException" => {
                    return RusotoError::Service(ExtendLicenseConsumptionError::RateLimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ExtendLicenseConsumptionError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServerInternalException" => {
                    return RusotoError::Service(ExtendLicenseConsumptionError::ServerInternal(
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
impl fmt::Display for ExtendLicenseConsumptionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ExtendLicenseConsumptionError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ExtendLicenseConsumptionError::Authorization(ref cause) => write!(f, "{}", cause),
            ExtendLicenseConsumptionError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            ExtendLicenseConsumptionError::RateLimitExceeded(ref cause) => write!(f, "{}", cause),
            ExtendLicenseConsumptionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ExtendLicenseConsumptionError::ServerInternal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ExtendLicenseConsumptionError {}
/// Errors returned by GetAccessToken
#[derive(Debug, PartialEq)]
pub enum GetAccessTokenError {
    /// <p>Access to resource denied.</p>
    AccessDenied(String),
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    Authorization(String),
    /// <p>Too many requests have been submitted. Try again after a brief wait.</p>
    RateLimitExceeded(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternal(String),
}

impl GetAccessTokenError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetAccessTokenError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetAccessTokenError::AccessDenied(err.msg))
                }
                "AuthorizationException" => {
                    return RusotoError::Service(GetAccessTokenError::Authorization(err.msg))
                }
                "RateLimitExceededException" => {
                    return RusotoError::Service(GetAccessTokenError::RateLimitExceeded(err.msg))
                }
                "ServerInternalException" => {
                    return RusotoError::Service(GetAccessTokenError::ServerInternal(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetAccessTokenError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetAccessTokenError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetAccessTokenError::Authorization(ref cause) => write!(f, "{}", cause),
            GetAccessTokenError::RateLimitExceeded(ref cause) => write!(f, "{}", cause),
            GetAccessTokenError::ServerInternal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetAccessTokenError {}
/// Errors returned by GetGrant
#[derive(Debug, PartialEq)]
pub enum GetGrantError {
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

impl GetGrantError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetGrantError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetGrantError::AccessDenied(err.msg))
                }
                "AuthorizationException" => {
                    return RusotoError::Service(GetGrantError::Authorization(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(GetGrantError::InvalidParameterValue(err.msg))
                }
                "RateLimitExceededException" => {
                    return RusotoError::Service(GetGrantError::RateLimitExceeded(err.msg))
                }
                "ResourceLimitExceededException" => {
                    return RusotoError::Service(GetGrantError::ResourceLimitExceeded(err.msg))
                }
                "ServerInternalException" => {
                    return RusotoError::Service(GetGrantError::ServerInternal(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetGrantError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetGrantError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetGrantError::Authorization(ref cause) => write!(f, "{}", cause),
            GetGrantError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            GetGrantError::RateLimitExceeded(ref cause) => write!(f, "{}", cause),
            GetGrantError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
            GetGrantError::ServerInternal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetGrantError {}
/// Errors returned by GetLicense
#[derive(Debug, PartialEq)]
pub enum GetLicenseError {
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

impl GetLicenseError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetLicenseError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetLicenseError::AccessDenied(err.msg))
                }
                "AuthorizationException" => {
                    return RusotoError::Service(GetLicenseError::Authorization(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(GetLicenseError::InvalidParameterValue(err.msg))
                }
                "RateLimitExceededException" => {
                    return RusotoError::Service(GetLicenseError::RateLimitExceeded(err.msg))
                }
                "ServerInternalException" => {
                    return RusotoError::Service(GetLicenseError::ServerInternal(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetLicenseError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetLicenseError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetLicenseError::Authorization(ref cause) => write!(f, "{}", cause),
            GetLicenseError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            GetLicenseError::RateLimitExceeded(ref cause) => write!(f, "{}", cause),
            GetLicenseError::ServerInternal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetLicenseError {}
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
            #[allow(clippy::single_match)]
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetLicenseConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetLicenseConfigurationError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetLicenseConfigurationError::Authorization(ref cause) => write!(f, "{}", cause),
            GetLicenseConfigurationError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            GetLicenseConfigurationError::RateLimitExceeded(ref cause) => write!(f, "{}", cause),
            GetLicenseConfigurationError::ServerInternal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetLicenseConfigurationError {}
/// Errors returned by GetLicenseUsage
#[derive(Debug, PartialEq)]
pub enum GetLicenseUsageError {
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

impl GetLicenseUsageError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetLicenseUsageError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetLicenseUsageError::AccessDenied(err.msg))
                }
                "AuthorizationException" => {
                    return RusotoError::Service(GetLicenseUsageError::Authorization(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(GetLicenseUsageError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "RateLimitExceededException" => {
                    return RusotoError::Service(GetLicenseUsageError::RateLimitExceeded(err.msg))
                }
                "ServerInternalException" => {
                    return RusotoError::Service(GetLicenseUsageError::ServerInternal(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetLicenseUsageError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetLicenseUsageError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetLicenseUsageError::Authorization(ref cause) => write!(f, "{}", cause),
            GetLicenseUsageError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            GetLicenseUsageError::RateLimitExceeded(ref cause) => write!(f, "{}", cause),
            GetLicenseUsageError::ServerInternal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetLicenseUsageError {}
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
            #[allow(clippy::single_match)]
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetServiceSettingsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetServiceSettingsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetServiceSettingsError::Authorization(ref cause) => write!(f, "{}", cause),
            GetServiceSettingsError::RateLimitExceeded(ref cause) => write!(f, "{}", cause),
            GetServiceSettingsError::ServerInternal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetServiceSettingsError {}
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
            #[allow(clippy::single_match)]
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListAssociationsForLicenseConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListAssociationsForLicenseConfigurationError::AccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            ListAssociationsForLicenseConfigurationError::Authorization(ref cause) => {
                write!(f, "{}", cause)
            }
            ListAssociationsForLicenseConfigurationError::FilterLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            ListAssociationsForLicenseConfigurationError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            ListAssociationsForLicenseConfigurationError::RateLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            ListAssociationsForLicenseConfigurationError::ServerInternal(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListAssociationsForLicenseConfigurationError {}
/// Errors returned by ListDistributedGrants
#[derive(Debug, PartialEq)]
pub enum ListDistributedGrantsError {
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

impl ListDistributedGrantsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDistributedGrantsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListDistributedGrantsError::AccessDenied(err.msg))
                }
                "AuthorizationException" => {
                    return RusotoError::Service(ListDistributedGrantsError::Authorization(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(ListDistributedGrantsError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "RateLimitExceededException" => {
                    return RusotoError::Service(ListDistributedGrantsError::RateLimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceLimitExceededException" => {
                    return RusotoError::Service(ListDistributedGrantsError::ResourceLimitExceeded(
                        err.msg,
                    ))
                }
                "ServerInternalException" => {
                    return RusotoError::Service(ListDistributedGrantsError::ServerInternal(
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
impl fmt::Display for ListDistributedGrantsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDistributedGrantsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListDistributedGrantsError::Authorization(ref cause) => write!(f, "{}", cause),
            ListDistributedGrantsError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            ListDistributedGrantsError::RateLimitExceeded(ref cause) => write!(f, "{}", cause),
            ListDistributedGrantsError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
            ListDistributedGrantsError::ServerInternal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListDistributedGrantsError {}
/// Errors returned by ListFailuresForLicenseConfigurationOperations
#[derive(Debug, PartialEq)]
pub enum ListFailuresForLicenseConfigurationOperationsError {
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

impl ListFailuresForLicenseConfigurationOperationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListFailuresForLicenseConfigurationOperationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        ListFailuresForLicenseConfigurationOperationsError::AccessDenied(err.msg),
                    )
                }
                "AuthorizationException" => {
                    return RusotoError::Service(
                        ListFailuresForLicenseConfigurationOperationsError::Authorization(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        ListFailuresForLicenseConfigurationOperationsError::InvalidParameterValue(
                            err.msg,
                        ),
                    )
                }
                "RateLimitExceededException" => {
                    return RusotoError::Service(
                        ListFailuresForLicenseConfigurationOperationsError::RateLimitExceeded(
                            err.msg,
                        ),
                    )
                }
                "ServerInternalException" => {
                    return RusotoError::Service(
                        ListFailuresForLicenseConfigurationOperationsError::ServerInternal(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListFailuresForLicenseConfigurationOperationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListFailuresForLicenseConfigurationOperationsError::AccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            ListFailuresForLicenseConfigurationOperationsError::Authorization(ref cause) => {
                write!(f, "{}", cause)
            }
            ListFailuresForLicenseConfigurationOperationsError::InvalidParameterValue(
                ref cause,
            ) => write!(f, "{}", cause),
            ListFailuresForLicenseConfigurationOperationsError::RateLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            ListFailuresForLicenseConfigurationOperationsError::ServerInternal(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListFailuresForLicenseConfigurationOperationsError {}
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
            #[allow(clippy::single_match)]
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListLicenseConfigurationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListLicenseConfigurationsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListLicenseConfigurationsError::Authorization(ref cause) => write!(f, "{}", cause),
            ListLicenseConfigurationsError::FilterLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            ListLicenseConfigurationsError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            ListLicenseConfigurationsError::RateLimitExceeded(ref cause) => write!(f, "{}", cause),
            ListLicenseConfigurationsError::ServerInternal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListLicenseConfigurationsError {}
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
            #[allow(clippy::single_match)]
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListLicenseSpecificationsForResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListLicenseSpecificationsForResourceError::AccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            ListLicenseSpecificationsForResourceError::Authorization(ref cause) => {
                write!(f, "{}", cause)
            }
            ListLicenseSpecificationsForResourceError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            ListLicenseSpecificationsForResourceError::RateLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            ListLicenseSpecificationsForResourceError::ServerInternal(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListLicenseSpecificationsForResourceError {}
/// Errors returned by ListLicenseVersions
#[derive(Debug, PartialEq)]
pub enum ListLicenseVersionsError {
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

impl ListLicenseVersionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListLicenseVersionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListLicenseVersionsError::AccessDenied(err.msg))
                }
                "AuthorizationException" => {
                    return RusotoError::Service(ListLicenseVersionsError::Authorization(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(ListLicenseVersionsError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "RateLimitExceededException" => {
                    return RusotoError::Service(ListLicenseVersionsError::RateLimitExceeded(
                        err.msg,
                    ))
                }
                "ServerInternalException" => {
                    return RusotoError::Service(ListLicenseVersionsError::ServerInternal(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListLicenseVersionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListLicenseVersionsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListLicenseVersionsError::Authorization(ref cause) => write!(f, "{}", cause),
            ListLicenseVersionsError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            ListLicenseVersionsError::RateLimitExceeded(ref cause) => write!(f, "{}", cause),
            ListLicenseVersionsError::ServerInternal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListLicenseVersionsError {}
/// Errors returned by ListLicenses
#[derive(Debug, PartialEq)]
pub enum ListLicensesError {
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

impl ListLicensesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListLicensesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListLicensesError::AccessDenied(err.msg))
                }
                "AuthorizationException" => {
                    return RusotoError::Service(ListLicensesError::Authorization(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(ListLicensesError::InvalidParameterValue(err.msg))
                }
                "RateLimitExceededException" => {
                    return RusotoError::Service(ListLicensesError::RateLimitExceeded(err.msg))
                }
                "ServerInternalException" => {
                    return RusotoError::Service(ListLicensesError::ServerInternal(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListLicensesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListLicensesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListLicensesError::Authorization(ref cause) => write!(f, "{}", cause),
            ListLicensesError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            ListLicensesError::RateLimitExceeded(ref cause) => write!(f, "{}", cause),
            ListLicensesError::ServerInternal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListLicensesError {}
/// Errors returned by ListReceivedGrants
#[derive(Debug, PartialEq)]
pub enum ListReceivedGrantsError {
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

impl ListReceivedGrantsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListReceivedGrantsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListReceivedGrantsError::AccessDenied(err.msg))
                }
                "AuthorizationException" => {
                    return RusotoError::Service(ListReceivedGrantsError::Authorization(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(ListReceivedGrantsError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "RateLimitExceededException" => {
                    return RusotoError::Service(ListReceivedGrantsError::RateLimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceLimitExceededException" => {
                    return RusotoError::Service(ListReceivedGrantsError::ResourceLimitExceeded(
                        err.msg,
                    ))
                }
                "ServerInternalException" => {
                    return RusotoError::Service(ListReceivedGrantsError::ServerInternal(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListReceivedGrantsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListReceivedGrantsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListReceivedGrantsError::Authorization(ref cause) => write!(f, "{}", cause),
            ListReceivedGrantsError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            ListReceivedGrantsError::RateLimitExceeded(ref cause) => write!(f, "{}", cause),
            ListReceivedGrantsError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
            ListReceivedGrantsError::ServerInternal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListReceivedGrantsError {}
/// Errors returned by ListReceivedLicenses
#[derive(Debug, PartialEq)]
pub enum ListReceivedLicensesError {
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

impl ListReceivedLicensesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListReceivedLicensesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListReceivedLicensesError::AccessDenied(err.msg))
                }
                "AuthorizationException" => {
                    return RusotoError::Service(ListReceivedLicensesError::Authorization(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(ListReceivedLicensesError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "RateLimitExceededException" => {
                    return RusotoError::Service(ListReceivedLicensesError::RateLimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceLimitExceededException" => {
                    return RusotoError::Service(ListReceivedLicensesError::ResourceLimitExceeded(
                        err.msg,
                    ))
                }
                "ServerInternalException" => {
                    return RusotoError::Service(ListReceivedLicensesError::ServerInternal(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListReceivedLicensesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListReceivedLicensesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListReceivedLicensesError::Authorization(ref cause) => write!(f, "{}", cause),
            ListReceivedLicensesError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            ListReceivedLicensesError::RateLimitExceeded(ref cause) => write!(f, "{}", cause),
            ListReceivedLicensesError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
            ListReceivedLicensesError::ServerInternal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListReceivedLicensesError {}
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
            #[allow(clippy::single_match)]
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListResourceInventoryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListResourceInventoryError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListResourceInventoryError::Authorization(ref cause) => write!(f, "{}", cause),
            ListResourceInventoryError::FailedDependency(ref cause) => write!(f, "{}", cause),
            ListResourceInventoryError::FilterLimitExceeded(ref cause) => write!(f, "{}", cause),
            ListResourceInventoryError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            ListResourceInventoryError::RateLimitExceeded(ref cause) => write!(f, "{}", cause),
            ListResourceInventoryError::ServerInternal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListResourceInventoryError {}
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
            #[allow(clippy::single_match)]
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListTagsForResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTagsForResourceError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::Authorization(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::RateLimitExceeded(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::ServerInternal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by ListTokens
#[derive(Debug, PartialEq)]
pub enum ListTokensError {
    /// <p>Access to resource denied.</p>
    AccessDenied(String),
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    Authorization(String),
    /// <p>Too many requests have been submitted. Try again after a brief wait.</p>
    RateLimitExceeded(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternal(String),
}

impl ListTokensError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTokensError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListTokensError::AccessDenied(err.msg))
                }
                "AuthorizationException" => {
                    return RusotoError::Service(ListTokensError::Authorization(err.msg))
                }
                "RateLimitExceededException" => {
                    return RusotoError::Service(ListTokensError::RateLimitExceeded(err.msg))
                }
                "ServerInternalException" => {
                    return RusotoError::Service(ListTokensError::ServerInternal(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListTokensError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTokensError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListTokensError::Authorization(ref cause) => write!(f, "{}", cause),
            ListTokensError::RateLimitExceeded(ref cause) => write!(f, "{}", cause),
            ListTokensError::ServerInternal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTokensError {}
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
            #[allow(clippy::single_match)]
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListUsageForLicenseConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListUsageForLicenseConfigurationError::AccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            ListUsageForLicenseConfigurationError::Authorization(ref cause) => {
                write!(f, "{}", cause)
            }
            ListUsageForLicenseConfigurationError::FilterLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            ListUsageForLicenseConfigurationError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            ListUsageForLicenseConfigurationError::RateLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            ListUsageForLicenseConfigurationError::ServerInternal(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListUsageForLicenseConfigurationError {}
/// Errors returned by RejectGrant
#[derive(Debug, PartialEq)]
pub enum RejectGrantError {
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

impl RejectGrantError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RejectGrantError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(RejectGrantError::AccessDenied(err.msg))
                }
                "AuthorizationException" => {
                    return RusotoError::Service(RejectGrantError::Authorization(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(RejectGrantError::InvalidParameterValue(err.msg))
                }
                "RateLimitExceededException" => {
                    return RusotoError::Service(RejectGrantError::RateLimitExceeded(err.msg))
                }
                "ResourceLimitExceededException" => {
                    return RusotoError::Service(RejectGrantError::ResourceLimitExceeded(err.msg))
                }
                "ServerInternalException" => {
                    return RusotoError::Service(RejectGrantError::ServerInternal(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RejectGrantError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RejectGrantError::AccessDenied(ref cause) => write!(f, "{}", cause),
            RejectGrantError::Authorization(ref cause) => write!(f, "{}", cause),
            RejectGrantError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            RejectGrantError::RateLimitExceeded(ref cause) => write!(f, "{}", cause),
            RejectGrantError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
            RejectGrantError::ServerInternal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RejectGrantError {}
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
            #[allow(clippy::single_match)]
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for TagResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TagResourceError::AccessDenied(ref cause) => write!(f, "{}", cause),
            TagResourceError::Authorization(ref cause) => write!(f, "{}", cause),
            TagResourceError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            TagResourceError::RateLimitExceeded(ref cause) => write!(f, "{}", cause),
            TagResourceError::ServerInternal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
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
            #[allow(clippy::single_match)]
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UntagResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UntagResourceError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UntagResourceError::Authorization(ref cause) => write!(f, "{}", cause),
            UntagResourceError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            UntagResourceError::RateLimitExceeded(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ServerInternal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
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
    /// <p>Your resource limits have been exceeded.</p>
    ResourceLimitExceeded(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternal(String),
}

impl UpdateLicenseConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateLicenseConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
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
                "ResourceLimitExceededException" => {
                    return RusotoError::Service(
                        UpdateLicenseConfigurationError::ResourceLimitExceeded(err.msg),
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateLicenseConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateLicenseConfigurationError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateLicenseConfigurationError::Authorization(ref cause) => write!(f, "{}", cause),
            UpdateLicenseConfigurationError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateLicenseConfigurationError::RateLimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateLicenseConfigurationError::ResourceLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateLicenseConfigurationError::ServerInternal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateLicenseConfigurationError {}
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
            #[allow(clippy::single_match)]
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateLicenseSpecificationsForResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateLicenseSpecificationsForResourceError::AccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateLicenseSpecificationsForResourceError::Authorization(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateLicenseSpecificationsForResourceError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateLicenseSpecificationsForResourceError::InvalidResourceState(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateLicenseSpecificationsForResourceError::LicenseUsage(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateLicenseSpecificationsForResourceError::RateLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateLicenseSpecificationsForResourceError::ServerInternal(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateLicenseSpecificationsForResourceError {}
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
            #[allow(clippy::single_match)]
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateServiceSettingsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateServiceSettingsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateServiceSettingsError::Authorization(ref cause) => write!(f, "{}", cause),
            UpdateServiceSettingsError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            UpdateServiceSettingsError::RateLimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateServiceSettingsError::ServerInternal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateServiceSettingsError {}
/// Trait representing the capabilities of the AWS License Manager API. AWS License Manager clients implement this trait.
#[async_trait]
pub trait LicenseManager {
    /// <p>Accepts the specified grant.</p>
    async fn accept_grant(
        &self,
        input: AcceptGrantRequest,
    ) -> Result<AcceptGrantResponse, RusotoError<AcceptGrantError>>;

    /// <p>Checks in the specified license. Check in a license when it is no longer in use.</p>
    async fn check_in_license(
        &self,
        input: CheckInLicenseRequest,
    ) -> Result<CheckInLicenseResponse, RusotoError<CheckInLicenseError>>;

    /// <p>Checks out the specified license for offline use.</p>
    async fn checkout_borrow_license(
        &self,
        input: CheckoutBorrowLicenseRequest,
    ) -> Result<CheckoutBorrowLicenseResponse, RusotoError<CheckoutBorrowLicenseError>>;

    /// <p>Checks out the specified license.</p>
    async fn checkout_license(
        &self,
        input: CheckoutLicenseRequest,
    ) -> Result<CheckoutLicenseResponse, RusotoError<CheckoutLicenseError>>;

    /// <p>Creates a grant for the specified license. A grant shares the use of license entitlements with specific AWS accounts.</p>
    async fn create_grant(
        &self,
        input: CreateGrantRequest,
    ) -> Result<CreateGrantResponse, RusotoError<CreateGrantError>>;

    /// <p>Creates a new version of the specified grant.</p>
    async fn create_grant_version(
        &self,
        input: CreateGrantVersionRequest,
    ) -> Result<CreateGrantVersionResponse, RusotoError<CreateGrantVersionError>>;

    /// <p>Creates a license.</p>
    async fn create_license(
        &self,
        input: CreateLicenseRequest,
    ) -> Result<CreateLicenseResponse, RusotoError<CreateLicenseError>>;

    /// <p>Creates a license configuration.</p> <p>A license configuration is an abstraction of a customer license agreement that can be consumed and enforced by License Manager. Components include specifications for the license type (licensing by instance, socket, CPU, or vCPU), allowed tenancy (shared tenancy, Dedicated Instance, Dedicated Host, or all of these), license affinity to host (how long a license must be associated with a host), and the number of licenses purchased and used.</p>
    async fn create_license_configuration(
        &self,
        input: CreateLicenseConfigurationRequest,
    ) -> Result<CreateLicenseConfigurationResponse, RusotoError<CreateLicenseConfigurationError>>;

    /// <p>Creates a new version of the specified license.</p>
    async fn create_license_version(
        &self,
        input: CreateLicenseVersionRequest,
    ) -> Result<CreateLicenseVersionResponse, RusotoError<CreateLicenseVersionError>>;

    /// <p>Creates a long-lived token.</p> <p>A refresh token is a JWT token used to get an access token. With an access token, you can call AssumeRoleWithWebIdentity to get role credentials that you can use to call License Manager to manage the specified license.</p>
    async fn create_token(
        &self,
        input: CreateTokenRequest,
    ) -> Result<CreateTokenResponse, RusotoError<CreateTokenError>>;

    /// <p>Deletes the specified grant.</p>
    async fn delete_grant(
        &self,
        input: DeleteGrantRequest,
    ) -> Result<DeleteGrantResponse, RusotoError<DeleteGrantError>>;

    /// <p>Deletes the specified license.</p>
    async fn delete_license(
        &self,
        input: DeleteLicenseRequest,
    ) -> Result<DeleteLicenseResponse, RusotoError<DeleteLicenseError>>;

    /// <p>Deletes the specified license configuration.</p> <p>You cannot delete a license configuration that is in use.</p>
    async fn delete_license_configuration(
        &self,
        input: DeleteLicenseConfigurationRequest,
    ) -> Result<DeleteLicenseConfigurationResponse, RusotoError<DeleteLicenseConfigurationError>>;

    /// <p>Deletes the specified token. Must be called in the license home Region.</p>
    async fn delete_token(
        &self,
        input: DeleteTokenRequest,
    ) -> Result<DeleteTokenResponse, RusotoError<DeleteTokenError>>;

    /// <p>Extends the expiration date for license consumption.</p>
    async fn extend_license_consumption(
        &self,
        input: ExtendLicenseConsumptionRequest,
    ) -> Result<ExtendLicenseConsumptionResponse, RusotoError<ExtendLicenseConsumptionError>>;

    /// <p>Gets a temporary access token to use with AssumeRoleWithWebIdentity. Access tokens are valid for one hour.</p>
    async fn get_access_token(
        &self,
        input: GetAccessTokenRequest,
    ) -> Result<GetAccessTokenResponse, RusotoError<GetAccessTokenError>>;

    /// <p>Gets detailed information about the specified grant.</p>
    async fn get_grant(
        &self,
        input: GetGrantRequest,
    ) -> Result<GetGrantResponse, RusotoError<GetGrantError>>;

    /// <p>Gets detailed information about the specified license.</p>
    async fn get_license(
        &self,
        input: GetLicenseRequest,
    ) -> Result<GetLicenseResponse, RusotoError<GetLicenseError>>;

    /// <p>Gets detailed information about the specified license configuration.</p>
    async fn get_license_configuration(
        &self,
        input: GetLicenseConfigurationRequest,
    ) -> Result<GetLicenseConfigurationResponse, RusotoError<GetLicenseConfigurationError>>;

    /// <p>Gets detailed information about the usage of the specified license.</p>
    async fn get_license_usage(
        &self,
        input: GetLicenseUsageRequest,
    ) -> Result<GetLicenseUsageResponse, RusotoError<GetLicenseUsageError>>;

    /// <p>Gets the License Manager settings for the current Region.</p>
    async fn get_service_settings(
        &self,
    ) -> Result<GetServiceSettingsResponse, RusotoError<GetServiceSettingsError>>;

    /// <p>Lists the resource associations for the specified license configuration.</p> <p>Resource associations need not consume licenses from a license configuration. For example, an AMI or a stopped instance might not consume a license (depending on the license rules).</p>
    async fn list_associations_for_license_configuration(
        &self,
        input: ListAssociationsForLicenseConfigurationRequest,
    ) -> Result<
        ListAssociationsForLicenseConfigurationResponse,
        RusotoError<ListAssociationsForLicenseConfigurationError>,
    >;

    /// <p>Lists the grants distributed for the specified license.</p>
    async fn list_distributed_grants(
        &self,
        input: ListDistributedGrantsRequest,
    ) -> Result<ListDistributedGrantsResponse, RusotoError<ListDistributedGrantsError>>;

    /// <p>Lists the license configuration operations that failed.</p>
    async fn list_failures_for_license_configuration_operations(
        &self,
        input: ListFailuresForLicenseConfigurationOperationsRequest,
    ) -> Result<
        ListFailuresForLicenseConfigurationOperationsResponse,
        RusotoError<ListFailuresForLicenseConfigurationOperationsError>,
    >;

    /// <p>Lists the license configurations for your account.</p>
    async fn list_license_configurations(
        &self,
        input: ListLicenseConfigurationsRequest,
    ) -> Result<ListLicenseConfigurationsResponse, RusotoError<ListLicenseConfigurationsError>>;

    /// <p>Describes the license configurations for the specified resource.</p>
    async fn list_license_specifications_for_resource(
        &self,
        input: ListLicenseSpecificationsForResourceRequest,
    ) -> Result<
        ListLicenseSpecificationsForResourceResponse,
        RusotoError<ListLicenseSpecificationsForResourceError>,
    >;

    /// <p>Lists all versions of the specified license.</p>
    async fn list_license_versions(
        &self,
        input: ListLicenseVersionsRequest,
    ) -> Result<ListLicenseVersionsResponse, RusotoError<ListLicenseVersionsError>>;

    /// <p>Lists the licenses for your account.</p>
    async fn list_licenses(
        &self,
        input: ListLicensesRequest,
    ) -> Result<ListLicensesResponse, RusotoError<ListLicensesError>>;

    /// <p>Lists grants that are received but not accepted.</p>
    async fn list_received_grants(
        &self,
        input: ListReceivedGrantsRequest,
    ) -> Result<ListReceivedGrantsResponse, RusotoError<ListReceivedGrantsError>>;

    /// <p>Lists received licenses.</p>
    async fn list_received_licenses(
        &self,
        input: ListReceivedLicensesRequest,
    ) -> Result<ListReceivedLicensesResponse, RusotoError<ListReceivedLicensesError>>;

    /// <p>Lists resources managed using Systems Manager inventory.</p>
    async fn list_resource_inventory(
        &self,
        input: ListResourceInventoryRequest,
    ) -> Result<ListResourceInventoryResponse, RusotoError<ListResourceInventoryError>>;

    /// <p>Lists the tags for the specified license configuration.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>Lists your tokens.</p>
    async fn list_tokens(
        &self,
        input: ListTokensRequest,
    ) -> Result<ListTokensResponse, RusotoError<ListTokensError>>;

    /// <p>Lists all license usage records for a license configuration, displaying license consumption details by resource at a selected point in time. Use this action to audit the current license consumption for any license inventory and configuration.</p>
    async fn list_usage_for_license_configuration(
        &self,
        input: ListUsageForLicenseConfigurationRequest,
    ) -> Result<
        ListUsageForLicenseConfigurationResponse,
        RusotoError<ListUsageForLicenseConfigurationError>,
    >;

    /// <p>Rejects the specified grant.</p>
    async fn reject_grant(
        &self,
        input: RejectGrantRequest,
    ) -> Result<RejectGrantResponse, RusotoError<RejectGrantError>>;

    /// <p>Adds the specified tags to the specified license configuration.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>>;

    /// <p>Removes the specified tags from the specified license configuration.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>>;

    /// <p>Modifies the attributes of an existing license configuration.</p>
    async fn update_license_configuration(
        &self,
        input: UpdateLicenseConfigurationRequest,
    ) -> Result<UpdateLicenseConfigurationResponse, RusotoError<UpdateLicenseConfigurationError>>;

    /// <p>Adds or removes the specified license configurations for the specified AWS resource.</p> <p>You can update the license specifications of AMIs, instances, and hosts. You cannot update the license specifications for launch templates and AWS CloudFormation templates, as they send license configurations to the operation that creates the resource.</p>
    async fn update_license_specifications_for_resource(
        &self,
        input: UpdateLicenseSpecificationsForResourceRequest,
    ) -> Result<
        UpdateLicenseSpecificationsForResourceResponse,
        RusotoError<UpdateLicenseSpecificationsForResourceError>,
    >;

    /// <p>Updates License Manager settings for the current Region.</p>
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

    pub fn new_with_client(client: Client, region: region::Region) -> LicenseManagerClient {
        LicenseManagerClient { client, region }
    }
}

#[async_trait]
impl LicenseManager for LicenseManagerClient {
    /// <p>Accepts the specified grant.</p>
    async fn accept_grant(
        &self,
        input: AcceptGrantRequest,
    ) -> Result<AcceptGrantResponse, RusotoError<AcceptGrantError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSLicenseManager.AcceptGrant");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, AcceptGrantError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<AcceptGrantResponse, _>()
    }

    /// <p>Checks in the specified license. Check in a license when it is no longer in use.</p>
    async fn check_in_license(
        &self,
        input: CheckInLicenseRequest,
    ) -> Result<CheckInLicenseResponse, RusotoError<CheckInLicenseError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSLicenseManager.CheckInLicense");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CheckInLicenseError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CheckInLicenseResponse, _>()
    }

    /// <p>Checks out the specified license for offline use.</p>
    async fn checkout_borrow_license(
        &self,
        input: CheckoutBorrowLicenseRequest,
    ) -> Result<CheckoutBorrowLicenseResponse, RusotoError<CheckoutBorrowLicenseError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSLicenseManager.CheckoutBorrowLicense");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CheckoutBorrowLicenseError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<CheckoutBorrowLicenseResponse, _>()
    }

    /// <p>Checks out the specified license.</p>
    async fn checkout_license(
        &self,
        input: CheckoutLicenseRequest,
    ) -> Result<CheckoutLicenseResponse, RusotoError<CheckoutLicenseError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSLicenseManager.CheckoutLicense");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CheckoutLicenseError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CheckoutLicenseResponse, _>()
    }

    /// <p>Creates a grant for the specified license. A grant shares the use of license entitlements with specific AWS accounts.</p>
    async fn create_grant(
        &self,
        input: CreateGrantRequest,
    ) -> Result<CreateGrantResponse, RusotoError<CreateGrantError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSLicenseManager.CreateGrant");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateGrantError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateGrantResponse, _>()
    }

    /// <p>Creates a new version of the specified grant.</p>
    async fn create_grant_version(
        &self,
        input: CreateGrantVersionRequest,
    ) -> Result<CreateGrantVersionResponse, RusotoError<CreateGrantVersionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSLicenseManager.CreateGrantVersion");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateGrantVersionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateGrantVersionResponse, _>()
    }

    /// <p>Creates a license.</p>
    async fn create_license(
        &self,
        input: CreateLicenseRequest,
    ) -> Result<CreateLicenseResponse, RusotoError<CreateLicenseError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSLicenseManager.CreateLicense");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateLicenseError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateLicenseResponse, _>()
    }

    /// <p>Creates a license configuration.</p> <p>A license configuration is an abstraction of a customer license agreement that can be consumed and enforced by License Manager. Components include specifications for the license type (licensing by instance, socket, CPU, or vCPU), allowed tenancy (shared tenancy, Dedicated Instance, Dedicated Host, or all of these), license affinity to host (how long a license must be associated with a host), and the number of licenses purchased and used.</p>
    async fn create_license_configuration(
        &self,
        input: CreateLicenseConfigurationRequest,
    ) -> Result<CreateLicenseConfigurationResponse, RusotoError<CreateLicenseConfigurationError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSLicenseManager.CreateLicenseConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateLicenseConfigurationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<CreateLicenseConfigurationResponse, _>()
    }

    /// <p>Creates a new version of the specified license.</p>
    async fn create_license_version(
        &self,
        input: CreateLicenseVersionRequest,
    ) -> Result<CreateLicenseVersionResponse, RusotoError<CreateLicenseVersionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSLicenseManager.CreateLicenseVersion");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateLicenseVersionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<CreateLicenseVersionResponse, _>()
    }

    /// <p>Creates a long-lived token.</p> <p>A refresh token is a JWT token used to get an access token. With an access token, you can call AssumeRoleWithWebIdentity to get role credentials that you can use to call License Manager to manage the specified license.</p>
    async fn create_token(
        &self,
        input: CreateTokenRequest,
    ) -> Result<CreateTokenResponse, RusotoError<CreateTokenError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSLicenseManager.CreateToken");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateTokenError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateTokenResponse, _>()
    }

    /// <p>Deletes the specified grant.</p>
    async fn delete_grant(
        &self,
        input: DeleteGrantRequest,
    ) -> Result<DeleteGrantResponse, RusotoError<DeleteGrantError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSLicenseManager.DeleteGrant");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteGrantError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteGrantResponse, _>()
    }

    /// <p>Deletes the specified license.</p>
    async fn delete_license(
        &self,
        input: DeleteLicenseRequest,
    ) -> Result<DeleteLicenseResponse, RusotoError<DeleteLicenseError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSLicenseManager.DeleteLicense");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteLicenseError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteLicenseResponse, _>()
    }

    /// <p>Deletes the specified license configuration.</p> <p>You cannot delete a license configuration that is in use.</p>
    async fn delete_license_configuration(
        &self,
        input: DeleteLicenseConfigurationRequest,
    ) -> Result<DeleteLicenseConfigurationResponse, RusotoError<DeleteLicenseConfigurationError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSLicenseManager.DeleteLicenseConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteLicenseConfigurationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DeleteLicenseConfigurationResponse, _>()
    }

    /// <p>Deletes the specified token. Must be called in the license home Region.</p>
    async fn delete_token(
        &self,
        input: DeleteTokenRequest,
    ) -> Result<DeleteTokenResponse, RusotoError<DeleteTokenError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSLicenseManager.DeleteToken");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteTokenError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteTokenResponse, _>()
    }

    /// <p>Extends the expiration date for license consumption.</p>
    async fn extend_license_consumption(
        &self,
        input: ExtendLicenseConsumptionRequest,
    ) -> Result<ExtendLicenseConsumptionResponse, RusotoError<ExtendLicenseConsumptionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSLicenseManager.ExtendLicenseConsumption");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ExtendLicenseConsumptionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ExtendLicenseConsumptionResponse, _>()
    }

    /// <p>Gets a temporary access token to use with AssumeRoleWithWebIdentity. Access tokens are valid for one hour.</p>
    async fn get_access_token(
        &self,
        input: GetAccessTokenRequest,
    ) -> Result<GetAccessTokenResponse, RusotoError<GetAccessTokenError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSLicenseManager.GetAccessToken");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetAccessTokenError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetAccessTokenResponse, _>()
    }

    /// <p>Gets detailed information about the specified grant.</p>
    async fn get_grant(
        &self,
        input: GetGrantRequest,
    ) -> Result<GetGrantResponse, RusotoError<GetGrantError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSLicenseManager.GetGrant");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetGrantError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetGrantResponse, _>()
    }

    /// <p>Gets detailed information about the specified license.</p>
    async fn get_license(
        &self,
        input: GetLicenseRequest,
    ) -> Result<GetLicenseResponse, RusotoError<GetLicenseError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSLicenseManager.GetLicense");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetLicenseError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetLicenseResponse, _>()
    }

    /// <p>Gets detailed information about the specified license configuration.</p>
    async fn get_license_configuration(
        &self,
        input: GetLicenseConfigurationRequest,
    ) -> Result<GetLicenseConfigurationResponse, RusotoError<GetLicenseConfigurationError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSLicenseManager.GetLicenseConfiguration");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetLicenseConfigurationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<GetLicenseConfigurationResponse, _>()
    }

    /// <p>Gets detailed information about the usage of the specified license.</p>
    async fn get_license_usage(
        &self,
        input: GetLicenseUsageRequest,
    ) -> Result<GetLicenseUsageResponse, RusotoError<GetLicenseUsageError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSLicenseManager.GetLicenseUsage");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetLicenseUsageError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetLicenseUsageResponse, _>()
    }

    /// <p>Gets the License Manager settings for the current Region.</p>
    async fn get_service_settings(
        &self,
    ) -> Result<GetServiceSettingsResponse, RusotoError<GetServiceSettingsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSLicenseManager.GetServiceSettings");
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let response = self
            .sign_and_dispatch(request, GetServiceSettingsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetServiceSettingsResponse, _>()
    }

    /// <p>Lists the resource associations for the specified license configuration.</p> <p>Resource associations need not consume licenses from a license configuration. For example, an AMI or a stopped instance might not consume a license (depending on the license rules).</p>
    async fn list_associations_for_license_configuration(
        &self,
        input: ListAssociationsForLicenseConfigurationRequest,
    ) -> Result<
        ListAssociationsForLicenseConfigurationResponse,
        RusotoError<ListAssociationsForLicenseConfigurationError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSLicenseManager.ListAssociationsForLicenseConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                ListAssociationsForLicenseConfigurationError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListAssociationsForLicenseConfigurationResponse, _>()
    }

    /// <p>Lists the grants distributed for the specified license.</p>
    async fn list_distributed_grants(
        &self,
        input: ListDistributedGrantsRequest,
    ) -> Result<ListDistributedGrantsResponse, RusotoError<ListDistributedGrantsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSLicenseManager.ListDistributedGrants");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListDistributedGrantsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListDistributedGrantsResponse, _>()
    }

    /// <p>Lists the license configuration operations that failed.</p>
    async fn list_failures_for_license_configuration_operations(
        &self,
        input: ListFailuresForLicenseConfigurationOperationsRequest,
    ) -> Result<
        ListFailuresForLicenseConfigurationOperationsResponse,
        RusotoError<ListFailuresForLicenseConfigurationOperationsError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSLicenseManager.ListFailuresForLicenseConfigurationOperations",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                ListFailuresForLicenseConfigurationOperationsError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListFailuresForLicenseConfigurationOperationsResponse, _>()
    }

    /// <p>Lists the license configurations for your account.</p>
    async fn list_license_configurations(
        &self,
        input: ListLicenseConfigurationsRequest,
    ) -> Result<ListLicenseConfigurationsResponse, RusotoError<ListLicenseConfigurationsError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSLicenseManager.ListLicenseConfigurations",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListLicenseConfigurationsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListLicenseConfigurationsResponse, _>()
    }

    /// <p>Describes the license configurations for the specified resource.</p>
    async fn list_license_specifications_for_resource(
        &self,
        input: ListLicenseSpecificationsForResourceRequest,
    ) -> Result<
        ListLicenseSpecificationsForResourceResponse,
        RusotoError<ListLicenseSpecificationsForResourceError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSLicenseManager.ListLicenseSpecificationsForResource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                ListLicenseSpecificationsForResourceError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListLicenseSpecificationsForResourceResponse, _>()
    }

    /// <p>Lists all versions of the specified license.</p>
    async fn list_license_versions(
        &self,
        input: ListLicenseVersionsRequest,
    ) -> Result<ListLicenseVersionsResponse, RusotoError<ListLicenseVersionsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSLicenseManager.ListLicenseVersions");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListLicenseVersionsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListLicenseVersionsResponse, _>()
    }

    /// <p>Lists the licenses for your account.</p>
    async fn list_licenses(
        &self,
        input: ListLicensesRequest,
    ) -> Result<ListLicensesResponse, RusotoError<ListLicensesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSLicenseManager.ListLicenses");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListLicensesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListLicensesResponse, _>()
    }

    /// <p>Lists grants that are received but not accepted.</p>
    async fn list_received_grants(
        &self,
        input: ListReceivedGrantsRequest,
    ) -> Result<ListReceivedGrantsResponse, RusotoError<ListReceivedGrantsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSLicenseManager.ListReceivedGrants");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListReceivedGrantsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListReceivedGrantsResponse, _>()
    }

    /// <p>Lists received licenses.</p>
    async fn list_received_licenses(
        &self,
        input: ListReceivedLicensesRequest,
    ) -> Result<ListReceivedLicensesResponse, RusotoError<ListReceivedLicensesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSLicenseManager.ListReceivedLicenses");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListReceivedLicensesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListReceivedLicensesResponse, _>()
    }

    /// <p>Lists resources managed using Systems Manager inventory.</p>
    async fn list_resource_inventory(
        &self,
        input: ListResourceInventoryRequest,
    ) -> Result<ListResourceInventoryResponse, RusotoError<ListResourceInventoryError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSLicenseManager.ListResourceInventory");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListResourceInventoryError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListResourceInventoryResponse, _>()
    }

    /// <p>Lists the tags for the specified license configuration.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSLicenseManager.ListTagsForResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListTagsForResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListTagsForResourceResponse, _>()
    }

    /// <p>Lists your tokens.</p>
    async fn list_tokens(
        &self,
        input: ListTokensRequest,
    ) -> Result<ListTokensResponse, RusotoError<ListTokensError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSLicenseManager.ListTokens");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListTokensError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListTokensResponse, _>()
    }

    /// <p>Lists all license usage records for a license configuration, displaying license consumption details by resource at a selected point in time. Use this action to audit the current license consumption for any license inventory and configuration.</p>
    async fn list_usage_for_license_configuration(
        &self,
        input: ListUsageForLicenseConfigurationRequest,
    ) -> Result<
        ListUsageForLicenseConfigurationResponse,
        RusotoError<ListUsageForLicenseConfigurationError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSLicenseManager.ListUsageForLicenseConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                ListUsageForLicenseConfigurationError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListUsageForLicenseConfigurationResponse, _>()
    }

    /// <p>Rejects the specified grant.</p>
    async fn reject_grant(
        &self,
        input: RejectGrantRequest,
    ) -> Result<RejectGrantResponse, RusotoError<RejectGrantError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSLicenseManager.RejectGrant");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, RejectGrantError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<RejectGrantResponse, _>()
    }

    /// <p>Adds the specified tags to the specified license configuration.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSLicenseManager.TagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, TagResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<TagResourceResponse, _>()
    }

    /// <p>Removes the specified tags from the specified license configuration.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSLicenseManager.UntagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UntagResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UntagResourceResponse, _>()
    }

    /// <p>Modifies the attributes of an existing license configuration.</p>
    async fn update_license_configuration(
        &self,
        input: UpdateLicenseConfigurationRequest,
    ) -> Result<UpdateLicenseConfigurationResponse, RusotoError<UpdateLicenseConfigurationError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSLicenseManager.UpdateLicenseConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateLicenseConfigurationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<UpdateLicenseConfigurationResponse, _>()
    }

    /// <p>Adds or removes the specified license configurations for the specified AWS resource.</p> <p>You can update the license specifications of AMIs, instances, and hosts. You cannot update the license specifications for launch templates and AWS CloudFormation templates, as they send license configurations to the operation that creates the resource.</p>
    async fn update_license_specifications_for_resource(
        &self,
        input: UpdateLicenseSpecificationsForResourceRequest,
    ) -> Result<
        UpdateLicenseSpecificationsForResourceResponse,
        RusotoError<UpdateLicenseSpecificationsForResourceError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSLicenseManager.UpdateLicenseSpecificationsForResource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                UpdateLicenseSpecificationsForResourceError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<UpdateLicenseSpecificationsForResourceResponse, _>()
    }

    /// <p>Updates License Manager settings for the current Region.</p>
    async fn update_service_settings(
        &self,
        input: UpdateServiceSettingsRequest,
    ) -> Result<UpdateServiceSettingsResponse, RusotoError<UpdateServiceSettingsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSLicenseManager.UpdateServiceSettings");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateServiceSettingsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<UpdateServiceSettingsResponse, _>()
    }
}
