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

impl AcmClient {
    fn new_signed_request(&self, http_method: &str, request_uri: &str) -> SignedRequest {
        let mut request = SignedRequest::new(http_method, "acm", &self.region, request_uri);

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

use serde_json;
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AddTagsToCertificateRequest {
    /// <p>String that contains the ARN of the ACM certificate to which the tag is to be applied. This must be of the form:</p> <p> <code>arn:aws:acm:region:123456789012:certificate/12345678-1234-1234-1234-123456789012</code> </p> <p>For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>. </p>
    #[serde(rename = "CertificateArn")]
    pub certificate_arn: String,
    /// <p>The key-value pair that defines the tag. The tag value is optional.</p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

/// <p>Contains metadata about an ACM certificate. This structure is returned in the response to a <a>DescribeCertificate</a> request. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CertificateDetail {
    /// <p>The Amazon Resource Name (ARN) of the certificate. For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a> in the <i>AWS General Reference</i>.</p>
    #[serde(rename = "CertificateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the ACM PCA private certificate authority (CA) that issued the certificate. This has the following format: </p> <p> <code>arn:aws:acm-pca:region:account:certificate-authority/12345678-1234-1234-1234-123456789012</code> </p>
    #[serde(rename = "CertificateAuthorityArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_authority_arn: Option<String>,
    /// <p>The time at which the certificate was requested. This value exists only when the certificate type is <code>AMAZON_ISSUED</code>. </p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The fully qualified domain name for the certificate, such as www.example.com or example.com.</p>
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// <p>Contains information about the initial validation of each domain name that occurs as a result of the <a>RequestCertificate</a> request. This field exists only when the certificate type is <code>AMAZON_ISSUED</code>. </p>
    #[serde(rename = "DomainValidationOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_validation_options: Option<Vec<DomainValidation>>,
    /// <p>Contains a list of Extended Key Usage X.509 v3 extension objects. Each object specifies a purpose for which the certificate public key can be used and consists of a name and an object identifier (OID). </p>
    #[serde(rename = "ExtendedKeyUsages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_key_usages: Option<Vec<ExtendedKeyUsage>>,
    /// <p>The reason the certificate request failed. This value exists only when the certificate status is <code>FAILED</code>. For more information, see <a href="https://docs.aws.amazon.com/acm/latest/userguide/troubleshooting.html#troubleshooting-failed">Certificate Request Failed</a> in the <i>AWS Certificate Manager User Guide</i>. </p>
    #[serde(rename = "FailureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<FailureReason>,
    /// <p>The date and time at which the certificate was imported. This value exists only when the certificate type is <code>IMPORTED</code>. </p>
    #[serde(rename = "ImportedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imported_at: Option<f64>,
    /// <p>A list of ARNs for the AWS resources that are using the certificate. A certificate can be used by multiple AWS resources. </p>
    #[serde(rename = "InUseBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_use_by: Option<Vec<String>>,
    /// <p>The time at which the certificate was issued. This value exists only when the certificate type is <code>AMAZON_ISSUED</code>. </p>
    #[serde(rename = "IssuedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_at: Option<f64>,
    /// <p>The name of the certificate authority that issued and signed the certificate.</p>
    #[serde(rename = "Issuer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    /// <p>The algorithm that was used to generate the public-private key pair.</p>
    #[serde(rename = "KeyAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_algorithm: Option<KeyAlgorithm>,
    /// <p>A list of Key Usage X.509 v3 extension objects. Each object is a string value that identifies the purpose of the public key contained in the certificate. Possible extension values include DIGITAL_SIGNATURE, KEY_ENCHIPHERMENT, NON_REPUDIATION, and more.</p>
    #[serde(rename = "KeyUsages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_usages: Option<Vec<KeyUsage>>,
    /// <p>The time after which the certificate is not valid.</p>
    #[serde(rename = "NotAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_after: Option<f64>,
    /// <p>The time before which the certificate is not valid.</p>
    #[serde(rename = "NotBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_before: Option<f64>,
    /// <p>Value that specifies whether to add the certificate to a transparency log. Certificate transparency makes it possible to detect SSL certificates that have been mistakenly or maliciously issued. A browser might respond to certificate that has not been logged by showing an error message. The logs are cryptographically secure. </p>
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<CertificateOptions>,
    /// <p>Specifies whether the certificate is eligible for renewal. At this time, only exported private certificates can be renewed with the <a>RenewCertificate</a> command.</p>
    #[serde(rename = "RenewalEligibility")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_eligibility: Option<RenewalEligibility>,
    /// <p>Contains information about the status of ACM's <a href="https://docs.aws.amazon.com/acm/latest/userguide/acm-renewal.html">managed renewal</a> for the certificate. This field exists only when the certificate type is <code>AMAZON_ISSUED</code>.</p>
    #[serde(rename = "RenewalSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_summary: Option<RenewalSummary>,
    /// <p>The reason the certificate was revoked. This value exists only when the certificate status is <code>REVOKED</code>. </p>
    #[serde(rename = "RevocationReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revocation_reason: Option<RevocationReason>,
    /// <p>The time at which the certificate was revoked. This value exists only when the certificate status is <code>REVOKED</code>. </p>
    #[serde(rename = "RevokedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoked_at: Option<f64>,
    /// <p>The serial number of the certificate.</p>
    #[serde(rename = "Serial")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial: Option<String>,
    /// <p>The algorithm that was used to sign the certificate.</p>
    #[serde(rename = "SignatureAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_algorithm: Option<String>,
    /// <p>The status of the certificate.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CertificateStatus>,
    /// <p>The name of the entity that is associated with the public key contained in the certificate.</p>
    #[serde(rename = "Subject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// <p>One or more domain names (subject alternative names) included in the certificate. This list contains the domain names that are bound to the public key that is contained in the certificate. The subject alternative names include the canonical domain name (CN) of the certificate and additional domain names that can be used to connect to the website. </p>
    #[serde(rename = "SubjectAlternativeNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_alternative_names: Option<Vec<String>>,
    /// <p>The source of the certificate. For certificates provided by ACM, this value is <code>AMAZON_ISSUED</code>. For certificates that you imported with <a>ImportCertificate</a>, this value is <code>IMPORTED</code>. ACM does not provide <a href="https://docs.aws.amazon.com/acm/latest/userguide/acm-renewal.html">managed renewal</a> for imported certificates. For more information about the differences between certificates that you import and those that ACM provides, see <a href="https://docs.aws.amazon.com/acm/latest/userguide/import-certificate.html">Importing Certificates</a> in the <i>AWS Certificate Manager User Guide</i>. </p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<CertificateType>,
}

/// <p>Structure that contains options for your certificate. Currently, you can use this only to specify whether to opt in to or out of certificate transparency logging. Some browsers require that public certificates issued for your domain be recorded in a log. Certificates that are not logged typically generate a browser error. Transparency makes it possible for you to detect SSL/TLS certificates that have been mistakenly or maliciously issued for your domain. For general information, see <a href="https://docs.aws.amazon.com/acm/latest/userguide/acm-concepts.html#concept-transparency">Certificate Transparency Logging</a>. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CertificateOptions {
    /// <p>You can opt out of certificate transparency logging by specifying the <code>DISABLED</code> option. Opt in by specifying <code>ENABLED</code>. </p>
    #[serde(rename = "CertificateTransparencyLoggingPreference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_transparency_logging_preference:
        Option<CertificateTransparencyLoggingPreference>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownCertificateStatus {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum CertificateStatus {
    Expired,
    Failed,
    Inactive,
    Issued,
    PendingValidation,
    Revoked,
    ValidationTimedOut,
    #[doc(hidden)]
    UnknownVariant(UnknownCertificateStatus),
}

impl Default for CertificateStatus {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for CertificateStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for CertificateStatus {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for CertificateStatus {
    fn into(self) -> String {
        match self {
            CertificateStatus::Expired => "EXPIRED".to_string(),
            CertificateStatus::Failed => "FAILED".to_string(),
            CertificateStatus::Inactive => "INACTIVE".to_string(),
            CertificateStatus::Issued => "ISSUED".to_string(),
            CertificateStatus::PendingValidation => "PENDING_VALIDATION".to_string(),
            CertificateStatus::Revoked => "REVOKED".to_string(),
            CertificateStatus::ValidationTimedOut => "VALIDATION_TIMED_OUT".to_string(),
            CertificateStatus::UnknownVariant(UnknownCertificateStatus { name: original }) => {
                original
            }
        }
    }
}

impl<'a> Into<&'a str> for &'a CertificateStatus {
    fn into(self) -> &'a str {
        match self {
            CertificateStatus::Expired => &"EXPIRED",
            CertificateStatus::Failed => &"FAILED",
            CertificateStatus::Inactive => &"INACTIVE",
            CertificateStatus::Issued => &"ISSUED",
            CertificateStatus::PendingValidation => &"PENDING_VALIDATION",
            CertificateStatus::Revoked => &"REVOKED",
            CertificateStatus::ValidationTimedOut => &"VALIDATION_TIMED_OUT",
            CertificateStatus::UnknownVariant(UnknownCertificateStatus { name: original }) => {
                original
            }
        }
    }
}

impl From<&str> for CertificateStatus {
    fn from(name: &str) -> Self {
        match name {
            "EXPIRED" => CertificateStatus::Expired,
            "FAILED" => CertificateStatus::Failed,
            "INACTIVE" => CertificateStatus::Inactive,
            "ISSUED" => CertificateStatus::Issued,
            "PENDING_VALIDATION" => CertificateStatus::PendingValidation,
            "REVOKED" => CertificateStatus::Revoked,
            "VALIDATION_TIMED_OUT" => CertificateStatus::ValidationTimedOut,
            _ => CertificateStatus::UnknownVariant(UnknownCertificateStatus {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for CertificateStatus {
    fn from(name: String) -> Self {
        match &*name {
            "EXPIRED" => CertificateStatus::Expired,
            "FAILED" => CertificateStatus::Failed,
            "INACTIVE" => CertificateStatus::Inactive,
            "ISSUED" => CertificateStatus::Issued,
            "PENDING_VALIDATION" => CertificateStatus::PendingValidation,
            "REVOKED" => CertificateStatus::Revoked,
            "VALIDATION_TIMED_OUT" => CertificateStatus::ValidationTimedOut,
            _ => CertificateStatus::UnknownVariant(UnknownCertificateStatus { name }),
        }
    }
}

impl ::std::str::FromStr for CertificateStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for CertificateStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for CertificateStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>This structure is returned in the response object of <a>ListCertificates</a> action. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CertificateSummary {
    /// <p>Amazon Resource Name (ARN) of the certificate. This is of the form:</p> <p> <code>arn:aws:acm:region:123456789012:certificate/12345678-1234-1234-1234-123456789012</code> </p> <p>For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>. </p>
    #[serde(rename = "CertificateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    /// <p>Fully qualified domain name (FQDN), such as www.example.com or example.com, for the certificate.</p>
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownCertificateTransparencyLoggingPreference {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum CertificateTransparencyLoggingPreference {
    Disabled,
    Enabled,
    #[doc(hidden)]
    UnknownVariant(UnknownCertificateTransparencyLoggingPreference),
}

impl Default for CertificateTransparencyLoggingPreference {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for CertificateTransparencyLoggingPreference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for CertificateTransparencyLoggingPreference {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for CertificateTransparencyLoggingPreference {
    fn into(self) -> String {
        match self {
            CertificateTransparencyLoggingPreference::Disabled => "DISABLED".to_string(),
            CertificateTransparencyLoggingPreference::Enabled => "ENABLED".to_string(),
            CertificateTransparencyLoggingPreference::UnknownVariant(
                UnknownCertificateTransparencyLoggingPreference { name: original },
            ) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a CertificateTransparencyLoggingPreference {
    fn into(self) -> &'a str {
        match self {
            CertificateTransparencyLoggingPreference::Disabled => &"DISABLED",
            CertificateTransparencyLoggingPreference::Enabled => &"ENABLED",
            CertificateTransparencyLoggingPreference::UnknownVariant(
                UnknownCertificateTransparencyLoggingPreference { name: original },
            ) => original,
        }
    }
}

impl From<&str> for CertificateTransparencyLoggingPreference {
    fn from(name: &str) -> Self {
        match name {
            "DISABLED" => CertificateTransparencyLoggingPreference::Disabled,
            "ENABLED" => CertificateTransparencyLoggingPreference::Enabled,
            _ => CertificateTransparencyLoggingPreference::UnknownVariant(
                UnknownCertificateTransparencyLoggingPreference {
                    name: name.to_owned(),
                },
            ),
        }
    }
}

impl From<String> for CertificateTransparencyLoggingPreference {
    fn from(name: String) -> Self {
        match &*name {
            "DISABLED" => CertificateTransparencyLoggingPreference::Disabled,
            "ENABLED" => CertificateTransparencyLoggingPreference::Enabled,
            _ => CertificateTransparencyLoggingPreference::UnknownVariant(
                UnknownCertificateTransparencyLoggingPreference { name },
            ),
        }
    }
}

impl ::std::str::FromStr for CertificateTransparencyLoggingPreference {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for CertificateTransparencyLoggingPreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for CertificateTransparencyLoggingPreference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownCertificateType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum CertificateType {
    AmazonIssued,
    Imported,
    Private,
    #[doc(hidden)]
    UnknownVariant(UnknownCertificateType),
}

impl Default for CertificateType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for CertificateType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for CertificateType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for CertificateType {
    fn into(self) -> String {
        match self {
            CertificateType::AmazonIssued => "AMAZON_ISSUED".to_string(),
            CertificateType::Imported => "IMPORTED".to_string(),
            CertificateType::Private => "PRIVATE".to_string(),
            CertificateType::UnknownVariant(UnknownCertificateType { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a CertificateType {
    fn into(self) -> &'a str {
        match self {
            CertificateType::AmazonIssued => &"AMAZON_ISSUED",
            CertificateType::Imported => &"IMPORTED",
            CertificateType::Private => &"PRIVATE",
            CertificateType::UnknownVariant(UnknownCertificateType { name: original }) => original,
        }
    }
}

impl From<&str> for CertificateType {
    fn from(name: &str) -> Self {
        match name {
            "AMAZON_ISSUED" => CertificateType::AmazonIssued,
            "IMPORTED" => CertificateType::Imported,
            "PRIVATE" => CertificateType::Private,
            _ => CertificateType::UnknownVariant(UnknownCertificateType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for CertificateType {
    fn from(name: String) -> Self {
        match &*name {
            "AMAZON_ISSUED" => CertificateType::AmazonIssued,
            "IMPORTED" => CertificateType::Imported,
            "PRIVATE" => CertificateType::Private,
            _ => CertificateType::UnknownVariant(UnknownCertificateType { name }),
        }
    }
}

impl ::std::str::FromStr for CertificateType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for CertificateType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for CertificateType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteCertificateRequest {
    /// <p>String that contains the ARN of the ACM certificate to be deleted. This must be of the form:</p> <p> <code>arn:aws:acm:region:123456789012:certificate/12345678-1234-1234-1234-123456789012</code> </p> <p>For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>.</p>
    #[serde(rename = "CertificateArn")]
    pub certificate_arn: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeCertificateRequest {
    /// <p>The Amazon Resource Name (ARN) of the ACM certificate. The ARN must have the following form:</p> <p> <code>arn:aws:acm:region:123456789012:certificate/12345678-1234-1234-1234-123456789012</code> </p> <p>For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>.</p>
    #[serde(rename = "CertificateArn")]
    pub certificate_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeCertificateResponse {
    /// <p>Metadata about an ACM certificate.</p>
    #[serde(rename = "Certificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<CertificateDetail>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownDomainStatus {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum DomainStatus {
    Failed,
    PendingValidation,
    Success,
    #[doc(hidden)]
    UnknownVariant(UnknownDomainStatus),
}

impl Default for DomainStatus {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for DomainStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for DomainStatus {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for DomainStatus {
    fn into(self) -> String {
        match self {
            DomainStatus::Failed => "FAILED".to_string(),
            DomainStatus::PendingValidation => "PENDING_VALIDATION".to_string(),
            DomainStatus::Success => "SUCCESS".to_string(),
            DomainStatus::UnknownVariant(UnknownDomainStatus { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a DomainStatus {
    fn into(self) -> &'a str {
        match self {
            DomainStatus::Failed => &"FAILED",
            DomainStatus::PendingValidation => &"PENDING_VALIDATION",
            DomainStatus::Success => &"SUCCESS",
            DomainStatus::UnknownVariant(UnknownDomainStatus { name: original }) => original,
        }
    }
}

impl From<&str> for DomainStatus {
    fn from(name: &str) -> Self {
        match name {
            "FAILED" => DomainStatus::Failed,
            "PENDING_VALIDATION" => DomainStatus::PendingValidation,
            "SUCCESS" => DomainStatus::Success,
            _ => DomainStatus::UnknownVariant(UnknownDomainStatus {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for DomainStatus {
    fn from(name: String) -> Self {
        match &*name {
            "FAILED" => DomainStatus::Failed,
            "PENDING_VALIDATION" => DomainStatus::PendingValidation,
            "SUCCESS" => DomainStatus::Success,
            _ => DomainStatus::UnknownVariant(UnknownDomainStatus { name }),
        }
    }
}

impl ::std::str::FromStr for DomainStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for DomainStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for DomainStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Contains information about the validation of each domain name in the certificate.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DomainValidation {
    /// <p>A fully qualified domain name (FQDN) in the certificate. For example, <code>www.example.com</code> or <code>example.com</code>. </p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// <p>Contains the CNAME record that you add to your DNS database for domain validation. For more information, see <a href="https://docs.aws.amazon.com/acm/latest/userguide/gs-acm-validate-dns.html">Use DNS to Validate Domain Ownership</a>.</p> <p>Note: The CNAME information that you need does not include the name of your domain. If you include&#x2028; your domain name in the DNS database CNAME record, validation fails.&#x2028; For example, if the name is "_a79865eb4cd1a6ab990a45779b4e0b96.yourdomain.com", only "_a79865eb4cd1a6ab990a45779b4e0b96" must be used.</p>
    #[serde(rename = "ResourceRecord")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_record: Option<ResourceRecord>,
    /// <p>The domain name that ACM used to send domain validation emails.</p>
    #[serde(rename = "ValidationDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_domain: Option<String>,
    /// <p>A list of email addresses that ACM used to send domain validation emails.</p>
    #[serde(rename = "ValidationEmails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_emails: Option<Vec<String>>,
    /// <p>Specifies the domain validation method.</p>
    #[serde(rename = "ValidationMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_method: Option<ValidationMethod>,
    /// <p><p>The validation status of the domain name. This can be one of the following values:</p> <ul> <li> <p> <code>PENDING_VALIDATION</code> </p> </li> <li> <p> <code/>SUCCESS</p> </li> <li> <p> <code/>FAILED</p> </li> </ul></p>
    #[serde(rename = "ValidationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_status: Option<DomainStatus>,
}

/// <p>Contains information about the domain names that you want ACM to use to send you emails that enable you to validate domain ownership.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DomainValidationOption {
    /// <p>A fully qualified domain name (FQDN) in the certificate request.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// <p><p>The domain name that you want ACM to use to send you validation emails. This domain name is the suffix of the email addresses that you want ACM to use. This must be the same as the <code>DomainName</code> value or a superdomain of the <code>DomainName</code> value. For example, if you request a certificate for <code>testing.example.com</code>, you can specify <code>example.com</code> for this value. In that case, ACM sends domain validation emails to the following five addresses:</p> <ul> <li> <p>admin@example.com</p> </li> <li> <p>administrator@example.com</p> </li> <li> <p>hostmaster@example.com</p> </li> <li> <p>postmaster@example.com</p> </li> <li> <p>webmaster@example.com</p> </li> </ul></p>
    #[serde(rename = "ValidationDomain")]
    pub validation_domain: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ExportCertificateRequest {
    /// <p>An Amazon Resource Name (ARN) of the issued certificate. This must be of the form:</p> <p> <code>arn:aws:acm:region:account:certificate/12345678-1234-1234-1234-123456789012</code> </p>
    #[serde(rename = "CertificateArn")]
    pub certificate_arn: String,
    /// <p>Passphrase to associate with the encrypted exported private key. If you want to later decrypt the private key, you must have the passphrase. You can use the following OpenSSL command to decrypt a private key: </p> <p> <code>openssl rsa -in encrypted_key.pem -out decrypted_key.pem</code> </p>
    #[serde(rename = "Passphrase")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub passphrase: bytes::Bytes,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ExportCertificateResponse {
    /// <p>The base64 PEM-encoded certificate.</p>
    #[serde(rename = "Certificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    /// <p>The base64 PEM-encoded certificate chain. This does not include the certificate that you are exporting.</p>
    #[serde(rename = "CertificateChain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_chain: Option<String>,
    /// <p>The encrypted private key associated with the public key in the certificate. The key is output in PKCS #8 format and is base64 PEM-encoded. </p>
    #[serde(rename = "PrivateKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
}

/// <p>The Extended Key Usage X.509 v3 extension defines one or more purposes for which the public key can be used. This is in addition to or in place of the basic purposes specified by the Key Usage extension. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ExtendedKeyUsage {
    /// <p>The name of an Extended Key Usage value.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<ExtendedKeyUsageName>,
    /// <p><p>An object identifier (OID) for the extension value. OIDs are strings of numbers separated by periods. The following OIDs are defined in RFC 3280 and RFC 5280. </p> <ul> <li> <p> <code>1.3.6.1.5.5.7.3.1 (TLS<em>WEB</em>SERVER<em>AUTHENTICATION)</code> </p> </li> <li> <p> <code>1.3.6.1.5.5.7.3.2 (TLS</em>WEB<em>CLIENT</em>AUTHENTICATION)</code> </p> </li> <li> <p> <code>1.3.6.1.5.5.7.3.3 (CODE<em>SIGNING)</code> </p> </li> <li> <p> <code>1.3.6.1.5.5.7.3.4 (EMAIL</em>PROTECTION)</code> </p> </li> <li> <p> <code>1.3.6.1.5.5.7.3.8 (TIME<em>STAMPING)</code> </p> </li> <li> <p> <code>1.3.6.1.5.5.7.3.9 (OCSP</em>SIGNING)</code> </p> </li> <li> <p> <code>1.3.6.1.5.5.7.3.5 (IPSEC<em>END</em>SYSTEM)</code> </p> </li> <li> <p> <code>1.3.6.1.5.5.7.3.6 (IPSEC<em>TUNNEL)</code> </p> </li> <li> <p> <code>1.3.6.1.5.5.7.3.7 (IPSEC</em>USER)</code> </p> </li> </ul></p>
    #[serde(rename = "OID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oid: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownExtendedKeyUsageName {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ExtendedKeyUsageName {
    Any,
    CodeSigning,
    Custom,
    EmailProtection,
    IpsecEndSystem,
    IpsecTunnel,
    IpsecUser,
    None,
    OcspSigning,
    TimeStamping,
    TlsWebClientAuthentication,
    TlsWebServerAuthentication,
    #[doc(hidden)]
    UnknownVariant(UnknownExtendedKeyUsageName),
}

impl Default for ExtendedKeyUsageName {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for ExtendedKeyUsageName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for ExtendedKeyUsageName {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for ExtendedKeyUsageName {
    fn into(self) -> String {
        match self {
            ExtendedKeyUsageName::Any => "ANY".to_string(),
            ExtendedKeyUsageName::CodeSigning => "CODE_SIGNING".to_string(),
            ExtendedKeyUsageName::Custom => "CUSTOM".to_string(),
            ExtendedKeyUsageName::EmailProtection => "EMAIL_PROTECTION".to_string(),
            ExtendedKeyUsageName::IpsecEndSystem => "IPSEC_END_SYSTEM".to_string(),
            ExtendedKeyUsageName::IpsecTunnel => "IPSEC_TUNNEL".to_string(),
            ExtendedKeyUsageName::IpsecUser => "IPSEC_USER".to_string(),
            ExtendedKeyUsageName::None => "NONE".to_string(),
            ExtendedKeyUsageName::OcspSigning => "OCSP_SIGNING".to_string(),
            ExtendedKeyUsageName::TimeStamping => "TIME_STAMPING".to_string(),
            ExtendedKeyUsageName::TlsWebClientAuthentication => {
                "TLS_WEB_CLIENT_AUTHENTICATION".to_string()
            }
            ExtendedKeyUsageName::TlsWebServerAuthentication => {
                "TLS_WEB_SERVER_AUTHENTICATION".to_string()
            }
            ExtendedKeyUsageName::UnknownVariant(UnknownExtendedKeyUsageName {
                name: original,
            }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a ExtendedKeyUsageName {
    fn into(self) -> &'a str {
        match self {
            ExtendedKeyUsageName::Any => &"ANY",
            ExtendedKeyUsageName::CodeSigning => &"CODE_SIGNING",
            ExtendedKeyUsageName::Custom => &"CUSTOM",
            ExtendedKeyUsageName::EmailProtection => &"EMAIL_PROTECTION",
            ExtendedKeyUsageName::IpsecEndSystem => &"IPSEC_END_SYSTEM",
            ExtendedKeyUsageName::IpsecTunnel => &"IPSEC_TUNNEL",
            ExtendedKeyUsageName::IpsecUser => &"IPSEC_USER",
            ExtendedKeyUsageName::None => &"NONE",
            ExtendedKeyUsageName::OcspSigning => &"OCSP_SIGNING",
            ExtendedKeyUsageName::TimeStamping => &"TIME_STAMPING",
            ExtendedKeyUsageName::TlsWebClientAuthentication => &"TLS_WEB_CLIENT_AUTHENTICATION",
            ExtendedKeyUsageName::TlsWebServerAuthentication => &"TLS_WEB_SERVER_AUTHENTICATION",
            ExtendedKeyUsageName::UnknownVariant(UnknownExtendedKeyUsageName {
                name: original,
            }) => original,
        }
    }
}

impl From<&str> for ExtendedKeyUsageName {
    fn from(name: &str) -> Self {
        match name {
            "ANY" => ExtendedKeyUsageName::Any,
            "CODE_SIGNING" => ExtendedKeyUsageName::CodeSigning,
            "CUSTOM" => ExtendedKeyUsageName::Custom,
            "EMAIL_PROTECTION" => ExtendedKeyUsageName::EmailProtection,
            "IPSEC_END_SYSTEM" => ExtendedKeyUsageName::IpsecEndSystem,
            "IPSEC_TUNNEL" => ExtendedKeyUsageName::IpsecTunnel,
            "IPSEC_USER" => ExtendedKeyUsageName::IpsecUser,
            "NONE" => ExtendedKeyUsageName::None,
            "OCSP_SIGNING" => ExtendedKeyUsageName::OcspSigning,
            "TIME_STAMPING" => ExtendedKeyUsageName::TimeStamping,
            "TLS_WEB_CLIENT_AUTHENTICATION" => ExtendedKeyUsageName::TlsWebClientAuthentication,
            "TLS_WEB_SERVER_AUTHENTICATION" => ExtendedKeyUsageName::TlsWebServerAuthentication,
            _ => ExtendedKeyUsageName::UnknownVariant(UnknownExtendedKeyUsageName {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for ExtendedKeyUsageName {
    fn from(name: String) -> Self {
        match &*name {
            "ANY" => ExtendedKeyUsageName::Any,
            "CODE_SIGNING" => ExtendedKeyUsageName::CodeSigning,
            "CUSTOM" => ExtendedKeyUsageName::Custom,
            "EMAIL_PROTECTION" => ExtendedKeyUsageName::EmailProtection,
            "IPSEC_END_SYSTEM" => ExtendedKeyUsageName::IpsecEndSystem,
            "IPSEC_TUNNEL" => ExtendedKeyUsageName::IpsecTunnel,
            "IPSEC_USER" => ExtendedKeyUsageName::IpsecUser,
            "NONE" => ExtendedKeyUsageName::None,
            "OCSP_SIGNING" => ExtendedKeyUsageName::OcspSigning,
            "TIME_STAMPING" => ExtendedKeyUsageName::TimeStamping,
            "TLS_WEB_CLIENT_AUTHENTICATION" => ExtendedKeyUsageName::TlsWebClientAuthentication,
            "TLS_WEB_SERVER_AUTHENTICATION" => ExtendedKeyUsageName::TlsWebServerAuthentication,
            _ => ExtendedKeyUsageName::UnknownVariant(UnknownExtendedKeyUsageName { name }),
        }
    }
}

impl ::std::str::FromStr for ExtendedKeyUsageName {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for ExtendedKeyUsageName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ExtendedKeyUsageName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownFailureReason {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum FailureReason {
    AdditionalVerificationRequired,
    CaaError,
    DomainNotAllowed,
    DomainValidationDenied,
    InvalidPublicDomain,
    NoAvailableContacts,
    Other,
    PcaAccessDenied,
    PcaInvalidArgs,
    PcaInvalidArn,
    PcaInvalidDuration,
    PcaInvalidState,
    PcaLimitExceeded,
    PcaNameConstraintsValidation,
    PcaRequestFailed,
    PcaResourceNotFound,
    SlrNotFound,
    #[doc(hidden)]
    UnknownVariant(UnknownFailureReason),
}

impl Default for FailureReason {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for FailureReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for FailureReason {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for FailureReason {
    fn into(self) -> String {
        match self {
            FailureReason::AdditionalVerificationRequired => {
                "ADDITIONAL_VERIFICATION_REQUIRED".to_string()
            }
            FailureReason::CaaError => "CAA_ERROR".to_string(),
            FailureReason::DomainNotAllowed => "DOMAIN_NOT_ALLOWED".to_string(),
            FailureReason::DomainValidationDenied => "DOMAIN_VALIDATION_DENIED".to_string(),
            FailureReason::InvalidPublicDomain => "INVALID_PUBLIC_DOMAIN".to_string(),
            FailureReason::NoAvailableContacts => "NO_AVAILABLE_CONTACTS".to_string(),
            FailureReason::Other => "OTHER".to_string(),
            FailureReason::PcaAccessDenied => "PCA_ACCESS_DENIED".to_string(),
            FailureReason::PcaInvalidArgs => "PCA_INVALID_ARGS".to_string(),
            FailureReason::PcaInvalidArn => "PCA_INVALID_ARN".to_string(),
            FailureReason::PcaInvalidDuration => "PCA_INVALID_DURATION".to_string(),
            FailureReason::PcaInvalidState => "PCA_INVALID_STATE".to_string(),
            FailureReason::PcaLimitExceeded => "PCA_LIMIT_EXCEEDED".to_string(),
            FailureReason::PcaNameConstraintsValidation => {
                "PCA_NAME_CONSTRAINTS_VALIDATION".to_string()
            }
            FailureReason::PcaRequestFailed => "PCA_REQUEST_FAILED".to_string(),
            FailureReason::PcaResourceNotFound => "PCA_RESOURCE_NOT_FOUND".to_string(),
            FailureReason::SlrNotFound => "SLR_NOT_FOUND".to_string(),
            FailureReason::UnknownVariant(UnknownFailureReason { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a FailureReason {
    fn into(self) -> &'a str {
        match self {
            FailureReason::AdditionalVerificationRequired => &"ADDITIONAL_VERIFICATION_REQUIRED",
            FailureReason::CaaError => &"CAA_ERROR",
            FailureReason::DomainNotAllowed => &"DOMAIN_NOT_ALLOWED",
            FailureReason::DomainValidationDenied => &"DOMAIN_VALIDATION_DENIED",
            FailureReason::InvalidPublicDomain => &"INVALID_PUBLIC_DOMAIN",
            FailureReason::NoAvailableContacts => &"NO_AVAILABLE_CONTACTS",
            FailureReason::Other => &"OTHER",
            FailureReason::PcaAccessDenied => &"PCA_ACCESS_DENIED",
            FailureReason::PcaInvalidArgs => &"PCA_INVALID_ARGS",
            FailureReason::PcaInvalidArn => &"PCA_INVALID_ARN",
            FailureReason::PcaInvalidDuration => &"PCA_INVALID_DURATION",
            FailureReason::PcaInvalidState => &"PCA_INVALID_STATE",
            FailureReason::PcaLimitExceeded => &"PCA_LIMIT_EXCEEDED",
            FailureReason::PcaNameConstraintsValidation => &"PCA_NAME_CONSTRAINTS_VALIDATION",
            FailureReason::PcaRequestFailed => &"PCA_REQUEST_FAILED",
            FailureReason::PcaResourceNotFound => &"PCA_RESOURCE_NOT_FOUND",
            FailureReason::SlrNotFound => &"SLR_NOT_FOUND",
            FailureReason::UnknownVariant(UnknownFailureReason { name: original }) => original,
        }
    }
}

impl From<&str> for FailureReason {
    fn from(name: &str) -> Self {
        match name {
            "ADDITIONAL_VERIFICATION_REQUIRED" => FailureReason::AdditionalVerificationRequired,
            "CAA_ERROR" => FailureReason::CaaError,
            "DOMAIN_NOT_ALLOWED" => FailureReason::DomainNotAllowed,
            "DOMAIN_VALIDATION_DENIED" => FailureReason::DomainValidationDenied,
            "INVALID_PUBLIC_DOMAIN" => FailureReason::InvalidPublicDomain,
            "NO_AVAILABLE_CONTACTS" => FailureReason::NoAvailableContacts,
            "OTHER" => FailureReason::Other,
            "PCA_ACCESS_DENIED" => FailureReason::PcaAccessDenied,
            "PCA_INVALID_ARGS" => FailureReason::PcaInvalidArgs,
            "PCA_INVALID_ARN" => FailureReason::PcaInvalidArn,
            "PCA_INVALID_DURATION" => FailureReason::PcaInvalidDuration,
            "PCA_INVALID_STATE" => FailureReason::PcaInvalidState,
            "PCA_LIMIT_EXCEEDED" => FailureReason::PcaLimitExceeded,
            "PCA_NAME_CONSTRAINTS_VALIDATION" => FailureReason::PcaNameConstraintsValidation,
            "PCA_REQUEST_FAILED" => FailureReason::PcaRequestFailed,
            "PCA_RESOURCE_NOT_FOUND" => FailureReason::PcaResourceNotFound,
            "SLR_NOT_FOUND" => FailureReason::SlrNotFound,
            _ => FailureReason::UnknownVariant(UnknownFailureReason {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for FailureReason {
    fn from(name: String) -> Self {
        match &*name {
            "ADDITIONAL_VERIFICATION_REQUIRED" => FailureReason::AdditionalVerificationRequired,
            "CAA_ERROR" => FailureReason::CaaError,
            "DOMAIN_NOT_ALLOWED" => FailureReason::DomainNotAllowed,
            "DOMAIN_VALIDATION_DENIED" => FailureReason::DomainValidationDenied,
            "INVALID_PUBLIC_DOMAIN" => FailureReason::InvalidPublicDomain,
            "NO_AVAILABLE_CONTACTS" => FailureReason::NoAvailableContacts,
            "OTHER" => FailureReason::Other,
            "PCA_ACCESS_DENIED" => FailureReason::PcaAccessDenied,
            "PCA_INVALID_ARGS" => FailureReason::PcaInvalidArgs,
            "PCA_INVALID_ARN" => FailureReason::PcaInvalidArn,
            "PCA_INVALID_DURATION" => FailureReason::PcaInvalidDuration,
            "PCA_INVALID_STATE" => FailureReason::PcaInvalidState,
            "PCA_LIMIT_EXCEEDED" => FailureReason::PcaLimitExceeded,
            "PCA_NAME_CONSTRAINTS_VALIDATION" => FailureReason::PcaNameConstraintsValidation,
            "PCA_REQUEST_FAILED" => FailureReason::PcaRequestFailed,
            "PCA_RESOURCE_NOT_FOUND" => FailureReason::PcaResourceNotFound,
            "SLR_NOT_FOUND" => FailureReason::SlrNotFound,
            _ => FailureReason::UnknownVariant(UnknownFailureReason { name }),
        }
    }
}

impl ::std::str::FromStr for FailureReason {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for FailureReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for FailureReason {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>This structure can be used in the <a>ListCertificates</a> action to filter the output of the certificate list. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Filters {
    /// <p>Specify one or more <a>ExtendedKeyUsage</a> extension values.</p>
    #[serde(rename = "extendedKeyUsage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_key_usage: Option<Vec<ExtendedKeyUsageName>>,
    /// <p>Specify one or more algorithms that can be used to generate key pairs.</p> <p>Default filtering returns only <code>RSA_2048</code> certificates. To return other certificate types, provide the desired type signatures in a comma-separated list. For example, <code>"keyTypes": ["RSA_2048,RSA_4096"]</code> returns both <code>RSA_2048</code> and <code>RSA_4096</code> certificates.</p>
    #[serde(rename = "keyTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_types: Option<Vec<KeyAlgorithm>>,
    /// <p>Specify one or more <a>KeyUsage</a> extension values.</p>
    #[serde(rename = "keyUsage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_usage: Option<Vec<KeyUsageName>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetCertificateRequest {
    /// <p>String that contains a certificate ARN in the following format:</p> <p> <code>arn:aws:acm:region:123456789012:certificate/12345678-1234-1234-1234-123456789012</code> </p> <p>For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>.</p>
    #[serde(rename = "CertificateArn")]
    pub certificate_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetCertificateResponse {
    /// <p>The ACM-issued certificate corresponding to the ARN specified as input.</p>
    #[serde(rename = "Certificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    /// <p>Certificates forming the requested certificate's chain of trust. The chain consists of the certificate of the issuing CA and the intermediate certificates of any other subordinate CAs. </p>
    #[serde(rename = "CertificateChain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_chain: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ImportCertificateRequest {
    /// <p>The certificate to import.</p>
    #[serde(rename = "Certificate")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub certificate: bytes::Bytes,
    /// <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Name (ARN)</a> of an imported certificate to replace. To import a new certificate, omit this field. </p>
    #[serde(rename = "CertificateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    /// <p>The PEM encoded certificate chain.</p>
    #[serde(rename = "CertificateChain")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_chain: Option<bytes::Bytes>,
    /// <p>The private key that matches the public key in the certificate.</p>
    #[serde(rename = "PrivateKey")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub private_key: bytes::Bytes,
    /// <p>One or more resource tags to associate with the imported certificate. </p> <p>Note: You cannot apply tags when reimporting a certificate.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ImportCertificateResponse {
    /// <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Name (ARN)</a> of the imported certificate.</p>
    #[serde(rename = "CertificateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownKeyAlgorithm {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum KeyAlgorithm {
    EcPrime256V1,
    EcSecp384R1,
    EcSecp521R1,
    Rsa1024,
    Rsa2048,
    Rsa4096,
    #[doc(hidden)]
    UnknownVariant(UnknownKeyAlgorithm),
}

impl Default for KeyAlgorithm {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for KeyAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for KeyAlgorithm {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for KeyAlgorithm {
    fn into(self) -> String {
        match self {
            KeyAlgorithm::EcPrime256V1 => "EC_prime256v1".to_string(),
            KeyAlgorithm::EcSecp384R1 => "EC_secp384r1".to_string(),
            KeyAlgorithm::EcSecp521R1 => "EC_secp521r1".to_string(),
            KeyAlgorithm::Rsa1024 => "RSA_1024".to_string(),
            KeyAlgorithm::Rsa2048 => "RSA_2048".to_string(),
            KeyAlgorithm::Rsa4096 => "RSA_4096".to_string(),
            KeyAlgorithm::UnknownVariant(UnknownKeyAlgorithm { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a KeyAlgorithm {
    fn into(self) -> &'a str {
        match self {
            KeyAlgorithm::EcPrime256V1 => &"EC_prime256v1",
            KeyAlgorithm::EcSecp384R1 => &"EC_secp384r1",
            KeyAlgorithm::EcSecp521R1 => &"EC_secp521r1",
            KeyAlgorithm::Rsa1024 => &"RSA_1024",
            KeyAlgorithm::Rsa2048 => &"RSA_2048",
            KeyAlgorithm::Rsa4096 => &"RSA_4096",
            KeyAlgorithm::UnknownVariant(UnknownKeyAlgorithm { name: original }) => original,
        }
    }
}

impl From<&str> for KeyAlgorithm {
    fn from(name: &str) -> Self {
        match name {
            "EC_prime256v1" => KeyAlgorithm::EcPrime256V1,
            "EC_secp384r1" => KeyAlgorithm::EcSecp384R1,
            "EC_secp521r1" => KeyAlgorithm::EcSecp521R1,
            "RSA_1024" => KeyAlgorithm::Rsa1024,
            "RSA_2048" => KeyAlgorithm::Rsa2048,
            "RSA_4096" => KeyAlgorithm::Rsa4096,
            _ => KeyAlgorithm::UnknownVariant(UnknownKeyAlgorithm {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for KeyAlgorithm {
    fn from(name: String) -> Self {
        match &*name {
            "EC_prime256v1" => KeyAlgorithm::EcPrime256V1,
            "EC_secp384r1" => KeyAlgorithm::EcSecp384R1,
            "EC_secp521r1" => KeyAlgorithm::EcSecp521R1,
            "RSA_1024" => KeyAlgorithm::Rsa1024,
            "RSA_2048" => KeyAlgorithm::Rsa2048,
            "RSA_4096" => KeyAlgorithm::Rsa4096,
            _ => KeyAlgorithm::UnknownVariant(UnknownKeyAlgorithm { name }),
        }
    }
}

impl ::std::str::FromStr for KeyAlgorithm {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for KeyAlgorithm {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for KeyAlgorithm {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>The Key Usage X.509 v3 extension defines the purpose of the public key contained in the certificate.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct KeyUsage {
    /// <p>A string value that contains a Key Usage extension name.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<KeyUsageName>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownKeyUsageName {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum KeyUsageName {
    Any,
    CertificateSigning,
    CrlSigning,
    Custom,
    DataEncipherment,
    DecipherOnly,
    DigitalSignature,
    EncipherOnly,
    KeyAgreement,
    KeyEncipherment,
    NonRepudiation,
    #[doc(hidden)]
    UnknownVariant(UnknownKeyUsageName),
}

impl Default for KeyUsageName {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for KeyUsageName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for KeyUsageName {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for KeyUsageName {
    fn into(self) -> String {
        match self {
            KeyUsageName::Any => "ANY".to_string(),
            KeyUsageName::CertificateSigning => "CERTIFICATE_SIGNING".to_string(),
            KeyUsageName::CrlSigning => "CRL_SIGNING".to_string(),
            KeyUsageName::Custom => "CUSTOM".to_string(),
            KeyUsageName::DataEncipherment => "DATA_ENCIPHERMENT".to_string(),
            KeyUsageName::DecipherOnly => "DECIPHER_ONLY".to_string(),
            KeyUsageName::DigitalSignature => "DIGITAL_SIGNATURE".to_string(),
            KeyUsageName::EncipherOnly => "ENCIPHER_ONLY".to_string(),
            KeyUsageName::KeyAgreement => "KEY_AGREEMENT".to_string(),
            KeyUsageName::KeyEncipherment => "KEY_ENCIPHERMENT".to_string(),
            KeyUsageName::NonRepudiation => "NON_REPUDIATION".to_string(),
            KeyUsageName::UnknownVariant(UnknownKeyUsageName { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a KeyUsageName {
    fn into(self) -> &'a str {
        match self {
            KeyUsageName::Any => &"ANY",
            KeyUsageName::CertificateSigning => &"CERTIFICATE_SIGNING",
            KeyUsageName::CrlSigning => &"CRL_SIGNING",
            KeyUsageName::Custom => &"CUSTOM",
            KeyUsageName::DataEncipherment => &"DATA_ENCIPHERMENT",
            KeyUsageName::DecipherOnly => &"DECIPHER_ONLY",
            KeyUsageName::DigitalSignature => &"DIGITAL_SIGNATURE",
            KeyUsageName::EncipherOnly => &"ENCIPHER_ONLY",
            KeyUsageName::KeyAgreement => &"KEY_AGREEMENT",
            KeyUsageName::KeyEncipherment => &"KEY_ENCIPHERMENT",
            KeyUsageName::NonRepudiation => &"NON_REPUDIATION",
            KeyUsageName::UnknownVariant(UnknownKeyUsageName { name: original }) => original,
        }
    }
}

impl From<&str> for KeyUsageName {
    fn from(name: &str) -> Self {
        match name {
            "ANY" => KeyUsageName::Any,
            "CERTIFICATE_SIGNING" => KeyUsageName::CertificateSigning,
            "CRL_SIGNING" => KeyUsageName::CrlSigning,
            "CUSTOM" => KeyUsageName::Custom,
            "DATA_ENCIPHERMENT" => KeyUsageName::DataEncipherment,
            "DECIPHER_ONLY" => KeyUsageName::DecipherOnly,
            "DIGITAL_SIGNATURE" => KeyUsageName::DigitalSignature,
            "ENCIPHER_ONLY" => KeyUsageName::EncipherOnly,
            "KEY_AGREEMENT" => KeyUsageName::KeyAgreement,
            "KEY_ENCIPHERMENT" => KeyUsageName::KeyEncipherment,
            "NON_REPUDIATION" => KeyUsageName::NonRepudiation,
            _ => KeyUsageName::UnknownVariant(UnknownKeyUsageName {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for KeyUsageName {
    fn from(name: String) -> Self {
        match &*name {
            "ANY" => KeyUsageName::Any,
            "CERTIFICATE_SIGNING" => KeyUsageName::CertificateSigning,
            "CRL_SIGNING" => KeyUsageName::CrlSigning,
            "CUSTOM" => KeyUsageName::Custom,
            "DATA_ENCIPHERMENT" => KeyUsageName::DataEncipherment,
            "DECIPHER_ONLY" => KeyUsageName::DecipherOnly,
            "DIGITAL_SIGNATURE" => KeyUsageName::DigitalSignature,
            "ENCIPHER_ONLY" => KeyUsageName::EncipherOnly,
            "KEY_AGREEMENT" => KeyUsageName::KeyAgreement,
            "KEY_ENCIPHERMENT" => KeyUsageName::KeyEncipherment,
            "NON_REPUDIATION" => KeyUsageName::NonRepudiation,
            _ => KeyUsageName::UnknownVariant(UnknownKeyUsageName { name }),
        }
    }
}

impl ::std::str::FromStr for KeyUsageName {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for KeyUsageName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for KeyUsageName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListCertificatesRequest {
    /// <p>Filter the certificate list by status value.</p>
    #[serde(rename = "CertificateStatuses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_statuses: Option<Vec<CertificateStatus>>,
    /// <p>Filter the certificate list. For more information, see the <a>Filters</a> structure.</p>
    #[serde(rename = "Includes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub includes: Option<Filters>,
    /// <p>Use this parameter when paginating results to specify the maximum number of items to return in the response. If additional items exist beyond the number you specify, the <code>NextToken</code> element is sent in the response. Use this <code>NextToken</code> value in a subsequent request to retrieve additional items.</p>
    #[serde(rename = "MaxItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i64>,
    /// <p>Use this parameter only when paginating results and only in a subsequent request after you receive a response with truncated results. Set it to the value of <code>NextToken</code> from the response you just received.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListCertificatesResponse {
    /// <p>A list of ACM certificates.</p>
    #[serde(rename = "CertificateSummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_summary_list: Option<Vec<CertificateSummary>>,
    /// <p>When the list is truncated, this value is present and contains the value to use for the <code>NextToken</code> parameter in a subsequent pagination request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForCertificateRequest {
    /// <p>String that contains the ARN of the ACM certificate for which you want to list the tags. This must have the following form:</p> <p> <code>arn:aws:acm:region:123456789012:certificate/12345678-1234-1234-1234-123456789012</code> </p> <p>For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>. </p>
    #[serde(rename = "CertificateArn")]
    pub certificate_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForCertificateResponse {
    /// <p>The key-value pairs that define the applied tags.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownRecordType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum RecordType {
    Cname,
    #[doc(hidden)]
    UnknownVariant(UnknownRecordType),
}

impl Default for RecordType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for RecordType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for RecordType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for RecordType {
    fn into(self) -> String {
        match self {
            RecordType::Cname => "CNAME".to_string(),
            RecordType::UnknownVariant(UnknownRecordType { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a RecordType {
    fn into(self) -> &'a str {
        match self {
            RecordType::Cname => &"CNAME",
            RecordType::UnknownVariant(UnknownRecordType { name: original }) => original,
        }
    }
}

impl From<&str> for RecordType {
    fn from(name: &str) -> Self {
        match name {
            "CNAME" => RecordType::Cname,
            _ => RecordType::UnknownVariant(UnknownRecordType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for RecordType {
    fn from(name: String) -> Self {
        match &*name {
            "CNAME" => RecordType::Cname,
            _ => RecordType::UnknownVariant(UnknownRecordType { name }),
        }
    }
}

impl ::std::str::FromStr for RecordType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for RecordType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for RecordType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RemoveTagsFromCertificateRequest {
    /// <p>String that contains the ARN of the ACM Certificate with one or more tags that you want to remove. This must be of the form:</p> <p> <code>arn:aws:acm:region:123456789012:certificate/12345678-1234-1234-1234-123456789012</code> </p> <p>For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>. </p>
    #[serde(rename = "CertificateArn")]
    pub certificate_arn: String,
    /// <p>The key-value pair that defines the tag to remove.</p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RenewCertificateRequest {
    /// <p>String that contains the ARN of the ACM certificate to be renewed. This must be of the form:</p> <p> <code>arn:aws:acm:region:123456789012:certificate/12345678-1234-1234-1234-123456789012</code> </p> <p>For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>.</p>
    #[serde(rename = "CertificateArn")]
    pub certificate_arn: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownRenewalEligibility {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum RenewalEligibility {
    Eligible,
    Ineligible,
    #[doc(hidden)]
    UnknownVariant(UnknownRenewalEligibility),
}

impl Default for RenewalEligibility {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for RenewalEligibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for RenewalEligibility {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for RenewalEligibility {
    fn into(self) -> String {
        match self {
            RenewalEligibility::Eligible => "ELIGIBLE".to_string(),
            RenewalEligibility::Ineligible => "INELIGIBLE".to_string(),
            RenewalEligibility::UnknownVariant(UnknownRenewalEligibility { name: original }) => {
                original
            }
        }
    }
}

impl<'a> Into<&'a str> for &'a RenewalEligibility {
    fn into(self) -> &'a str {
        match self {
            RenewalEligibility::Eligible => &"ELIGIBLE",
            RenewalEligibility::Ineligible => &"INELIGIBLE",
            RenewalEligibility::UnknownVariant(UnknownRenewalEligibility { name: original }) => {
                original
            }
        }
    }
}

impl From<&str> for RenewalEligibility {
    fn from(name: &str) -> Self {
        match name {
            "ELIGIBLE" => RenewalEligibility::Eligible,
            "INELIGIBLE" => RenewalEligibility::Ineligible,
            _ => RenewalEligibility::UnknownVariant(UnknownRenewalEligibility {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for RenewalEligibility {
    fn from(name: String) -> Self {
        match &*name {
            "ELIGIBLE" => RenewalEligibility::Eligible,
            "INELIGIBLE" => RenewalEligibility::Ineligible,
            _ => RenewalEligibility::UnknownVariant(UnknownRenewalEligibility { name }),
        }
    }
}

impl ::std::str::FromStr for RenewalEligibility {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for RenewalEligibility {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for RenewalEligibility {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownRenewalStatus {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum RenewalStatus {
    Failed,
    PendingAutoRenewal,
    PendingValidation,
    Success,
    #[doc(hidden)]
    UnknownVariant(UnknownRenewalStatus),
}

impl Default for RenewalStatus {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for RenewalStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for RenewalStatus {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for RenewalStatus {
    fn into(self) -> String {
        match self {
            RenewalStatus::Failed => "FAILED".to_string(),
            RenewalStatus::PendingAutoRenewal => "PENDING_AUTO_RENEWAL".to_string(),
            RenewalStatus::PendingValidation => "PENDING_VALIDATION".to_string(),
            RenewalStatus::Success => "SUCCESS".to_string(),
            RenewalStatus::UnknownVariant(UnknownRenewalStatus { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a RenewalStatus {
    fn into(self) -> &'a str {
        match self {
            RenewalStatus::Failed => &"FAILED",
            RenewalStatus::PendingAutoRenewal => &"PENDING_AUTO_RENEWAL",
            RenewalStatus::PendingValidation => &"PENDING_VALIDATION",
            RenewalStatus::Success => &"SUCCESS",
            RenewalStatus::UnknownVariant(UnknownRenewalStatus { name: original }) => original,
        }
    }
}

impl From<&str> for RenewalStatus {
    fn from(name: &str) -> Self {
        match name {
            "FAILED" => RenewalStatus::Failed,
            "PENDING_AUTO_RENEWAL" => RenewalStatus::PendingAutoRenewal,
            "PENDING_VALIDATION" => RenewalStatus::PendingValidation,
            "SUCCESS" => RenewalStatus::Success,
            _ => RenewalStatus::UnknownVariant(UnknownRenewalStatus {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for RenewalStatus {
    fn from(name: String) -> Self {
        match &*name {
            "FAILED" => RenewalStatus::Failed,
            "PENDING_AUTO_RENEWAL" => RenewalStatus::PendingAutoRenewal,
            "PENDING_VALIDATION" => RenewalStatus::PendingValidation,
            "SUCCESS" => RenewalStatus::Success,
            _ => RenewalStatus::UnknownVariant(UnknownRenewalStatus { name }),
        }
    }
}

impl ::std::str::FromStr for RenewalStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for RenewalStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for RenewalStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Contains information about the status of ACM's <a href="https://docs.aws.amazon.com/acm/latest/userguide/acm-renewal.html">managed renewal</a> for the certificate. This structure exists only when the certificate type is <code>AMAZON_ISSUED</code>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RenewalSummary {
    /// <p>Contains information about the validation of each domain name in the certificate, as it pertains to ACM's <a href="https://docs.aws.amazon.com/acm/latest/userguide/acm-renewal.html">managed renewal</a>. This is different from the initial validation that occurs as a result of the <a>RequestCertificate</a> request. This field exists only when the certificate type is <code>AMAZON_ISSUED</code>.</p>
    #[serde(rename = "DomainValidationOptions")]
    pub domain_validation_options: Vec<DomainValidation>,
    /// <p>The status of ACM's <a href="https://docs.aws.amazon.com/acm/latest/userguide/acm-renewal.html">managed renewal</a> of the certificate.</p>
    #[serde(rename = "RenewalStatus")]
    pub renewal_status: RenewalStatus,
    /// <p>The reason that a renewal request was unsuccessful.</p>
    #[serde(rename = "RenewalStatusReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_status_reason: Option<FailureReason>,
    /// <p>The time at which the renewal summary was last updated.</p>
    #[serde(rename = "UpdatedAt")]
    pub updated_at: f64,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RequestCertificateRequest {
    /// <p>The Amazon Resource Name (ARN) of the private certificate authority (CA) that will be used to issue the certificate. If you do not provide an ARN and you are trying to request a private certificate, ACM will attempt to issue a public certificate. For more information about private CAs, see the <a href="https://docs.aws.amazon.com/acm-pca/latest/userguide/PcaWelcome.html">AWS Certificate Manager Private Certificate Authority (PCA)</a> user guide. The ARN must have the following form: </p> <p> <code>arn:aws:acm-pca:region:account:certificate-authority/12345678-1234-1234-1234-123456789012</code> </p>
    #[serde(rename = "CertificateAuthorityArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_authority_arn: Option<String>,
    /// <p> Fully qualified domain name (FQDN), such as www.example.com, that you want to secure with an ACM certificate. Use an asterisk (*) to create a wildcard certificate that protects several sites in the same domain. For example, *.example.com protects www.example.com, site.example.com, and images.example.com. </p> <p> The first domain name you enter cannot exceed 64 octets, including periods. Each subsequent Subject Alternative Name (SAN), however, can be up to 253 octets in length. </p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// <p>The domain name that you want ACM to use to send you emails so that you can validate domain ownership.</p>
    #[serde(rename = "DomainValidationOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_validation_options: Option<Vec<DomainValidationOption>>,
    /// <p>Customer chosen string that can be used to distinguish between calls to <code>RequestCertificate</code>. Idempotency tokens time out after one hour. Therefore, if you call <code>RequestCertificate</code> multiple times with the same idempotency token within one hour, ACM recognizes that you are requesting only one certificate and will issue only one. If you change the idempotency token for each call, ACM recognizes that you are requesting multiple certificates.</p>
    #[serde(rename = "IdempotencyToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_token: Option<String>,
    /// <p>Currently, you can use this parameter to specify whether to add the certificate to a certificate transparency log. Certificate transparency makes it possible to detect SSL/TLS certificates that have been mistakenly or maliciously issued. Certificates that have not been logged typically produce an error message in a browser. For more information, see <a href="https://docs.aws.amazon.com/acm/latest/userguide/acm-bestpractices.html#best-practices-transparency">Opting Out of Certificate Transparency Logging</a>.</p>
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<CertificateOptions>,
    /// <p><p>Additional FQDNs to be included in the Subject Alternative Name extension of the ACM certificate. For example, add the name www.example.net to a certificate for which the <code>DomainName</code> field is www.example.com if users can reach your site by using either name. The maximum number of domain names that you can add to an ACM certificate is 100. However, the initial quota is 10 domain names. If you need more than 10 names, you must request a quota increase. For more information, see <a href="https://docs.aws.amazon.com/acm/latest/userguide/acm-limits.html">Quotas</a>.</p> <p> The maximum length of a SAN DNS name is 253 octets. The name is made up of multiple labels separated by periods. No label can be longer than 63 octets. Consider the following examples: </p> <ul> <li> <p> <code>(63 octets).(63 octets).(63 octets).(61 octets)</code> is legal because the total length is 253 octets (63+1+63+1+63+1+61) and no label exceeds 63 octets.</p> </li> <li> <p> <code>(64 octets).(63 octets).(63 octets).(61 octets)</code> is not legal because the total length exceeds 253 octets (64+1+63+1+63+1+61) and the first label exceeds 63 octets.</p> </li> <li> <p> <code>(63 octets).(63 octets).(63 octets).(62 octets)</code> is not legal because the total length of the DNS name (63+1+63+1+63+1+62) exceeds 253 octets.</p> </li> </ul></p>
    #[serde(rename = "SubjectAlternativeNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_alternative_names: Option<Vec<String>>,
    /// <p>One or more resource tags to associate with the certificate.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The method you want to use if you are requesting a public certificate to validate that you own or control domain. You can <a href="https://docs.aws.amazon.com/acm/latest/userguide/gs-acm-validate-dns.html">validate with DNS</a> or <a href="https://docs.aws.amazon.com/acm/latest/userguide/gs-acm-validate-email.html">validate with email</a>. We recommend that you use DNS validation. </p>
    #[serde(rename = "ValidationMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_method: Option<ValidationMethod>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RequestCertificateResponse {
    /// <p>String that contains the ARN of the issued certificate. This must be of the form:</p> <p> <code>arn:aws:acm:us-east-1:123456789012:certificate/12345678-1234-1234-1234-123456789012</code> </p>
    #[serde(rename = "CertificateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ResendValidationEmailRequest {
    /// <p>String that contains the ARN of the requested certificate. The certificate ARN is generated and returned by the <a>RequestCertificate</a> action as soon as the request is made. By default, using this parameter causes email to be sent to all top-level domains you specified in the certificate request. The ARN must be of the form: </p> <p> <code>arn:aws:acm:us-east-1:123456789012:certificate/12345678-1234-1234-1234-123456789012</code> </p>
    #[serde(rename = "CertificateArn")]
    pub certificate_arn: String,
    /// <p>The fully qualified domain name (FQDN) of the certificate that needs to be validated.</p>
    #[serde(rename = "Domain")]
    pub domain: String,
    /// <p><p>The base validation domain that will act as the suffix of the email addresses that are used to send the emails. This must be the same as the <code>Domain</code> value or a superdomain of the <code>Domain</code> value. For example, if you requested a certificate for <code>site.subdomain.example.com</code> and specify a <b>ValidationDomain</b> of <code>subdomain.example.com</code>, ACM sends email to the domain registrant, technical contact, and administrative contact in WHOIS and the following five addresses:</p> <ul> <li> <p>admin@subdomain.example.com</p> </li> <li> <p>administrator@subdomain.example.com</p> </li> <li> <p>hostmaster@subdomain.example.com</p> </li> <li> <p>postmaster@subdomain.example.com</p> </li> <li> <p>webmaster@subdomain.example.com</p> </li> </ul></p>
    #[serde(rename = "ValidationDomain")]
    pub validation_domain: String,
}

/// <p>Contains a DNS record value that you can use to can use to validate ownership or control of a domain. This is used by the <a>DescribeCertificate</a> action. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResourceRecord {
    /// <p>The name of the DNS record to create in your domain. This is supplied by ACM.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The type of DNS record. Currently this can be <code>CNAME</code>.</p>
    #[serde(rename = "Type")]
    pub type_: RecordType,
    /// <p>The value of the CNAME record to add to your DNS database. This is supplied by ACM.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownRevocationReason {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum RevocationReason {
    AffiliationChanged,
    AACompromise,
    CaCompromise,
    CertificateHold,
    CessationOfOperation,
    KeyCompromise,
    PrivilegeWithdrawn,
    RemoveFromCrl,
    Superceded,
    Unspecified,
    #[doc(hidden)]
    UnknownVariant(UnknownRevocationReason),
}

impl Default for RevocationReason {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for RevocationReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for RevocationReason {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for RevocationReason {
    fn into(self) -> String {
        match self {
            RevocationReason::AffiliationChanged => "AFFILIATION_CHANGED".to_string(),
            RevocationReason::AACompromise => "A_A_COMPROMISE".to_string(),
            RevocationReason::CaCompromise => "CA_COMPROMISE".to_string(),
            RevocationReason::CertificateHold => "CERTIFICATE_HOLD".to_string(),
            RevocationReason::CessationOfOperation => "CESSATION_OF_OPERATION".to_string(),
            RevocationReason::KeyCompromise => "KEY_COMPROMISE".to_string(),
            RevocationReason::PrivilegeWithdrawn => "PRIVILEGE_WITHDRAWN".to_string(),
            RevocationReason::RemoveFromCrl => "REMOVE_FROM_CRL".to_string(),
            RevocationReason::Superceded => "SUPERCEDED".to_string(),
            RevocationReason::Unspecified => "UNSPECIFIED".to_string(),
            RevocationReason::UnknownVariant(UnknownRevocationReason { name: original }) => {
                original
            }
        }
    }
}

impl<'a> Into<&'a str> for &'a RevocationReason {
    fn into(self) -> &'a str {
        match self {
            RevocationReason::AffiliationChanged => &"AFFILIATION_CHANGED",
            RevocationReason::AACompromise => &"A_A_COMPROMISE",
            RevocationReason::CaCompromise => &"CA_COMPROMISE",
            RevocationReason::CertificateHold => &"CERTIFICATE_HOLD",
            RevocationReason::CessationOfOperation => &"CESSATION_OF_OPERATION",
            RevocationReason::KeyCompromise => &"KEY_COMPROMISE",
            RevocationReason::PrivilegeWithdrawn => &"PRIVILEGE_WITHDRAWN",
            RevocationReason::RemoveFromCrl => &"REMOVE_FROM_CRL",
            RevocationReason::Superceded => &"SUPERCEDED",
            RevocationReason::Unspecified => &"UNSPECIFIED",
            RevocationReason::UnknownVariant(UnknownRevocationReason { name: original }) => {
                original
            }
        }
    }
}

impl From<&str> for RevocationReason {
    fn from(name: &str) -> Self {
        match name {
            "AFFILIATION_CHANGED" => RevocationReason::AffiliationChanged,
            "A_A_COMPROMISE" => RevocationReason::AACompromise,
            "CA_COMPROMISE" => RevocationReason::CaCompromise,
            "CERTIFICATE_HOLD" => RevocationReason::CertificateHold,
            "CESSATION_OF_OPERATION" => RevocationReason::CessationOfOperation,
            "KEY_COMPROMISE" => RevocationReason::KeyCompromise,
            "PRIVILEGE_WITHDRAWN" => RevocationReason::PrivilegeWithdrawn,
            "REMOVE_FROM_CRL" => RevocationReason::RemoveFromCrl,
            "SUPERCEDED" => RevocationReason::Superceded,
            "UNSPECIFIED" => RevocationReason::Unspecified,
            _ => RevocationReason::UnknownVariant(UnknownRevocationReason {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for RevocationReason {
    fn from(name: String) -> Self {
        match &*name {
            "AFFILIATION_CHANGED" => RevocationReason::AffiliationChanged,
            "A_A_COMPROMISE" => RevocationReason::AACompromise,
            "CA_COMPROMISE" => RevocationReason::CaCompromise,
            "CERTIFICATE_HOLD" => RevocationReason::CertificateHold,
            "CESSATION_OF_OPERATION" => RevocationReason::CessationOfOperation,
            "KEY_COMPROMISE" => RevocationReason::KeyCompromise,
            "PRIVILEGE_WITHDRAWN" => RevocationReason::PrivilegeWithdrawn,
            "REMOVE_FROM_CRL" => RevocationReason::RemoveFromCrl,
            "SUPERCEDED" => RevocationReason::Superceded,
            "UNSPECIFIED" => RevocationReason::Unspecified,
            _ => RevocationReason::UnknownVariant(UnknownRevocationReason { name }),
        }
    }
}

impl ::std::str::FromStr for RevocationReason {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for RevocationReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for RevocationReason {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>A key-value pair that identifies or specifies metadata about an ACM resource.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Tag {
    /// <p>The key of the tag.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The value of the tag.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateCertificateOptionsRequest {
    /// <p>ARN of the requested certificate to update. This must be of the form:</p> <p> <code>arn:aws:acm:us-east-1:<i>account</i>:certificate/<i>12345678-1234-1234-1234-123456789012</i> </code> </p>
    #[serde(rename = "CertificateArn")]
    pub certificate_arn: String,
    /// <p>Use to update the options for your certificate. Currently, you can specify whether to add your certificate to a transparency log. Certificate transparency makes it possible to detect SSL/TLS certificates that have been mistakenly or maliciously issued. Certificates that have not been logged typically produce an error message in a browser. </p>
    #[serde(rename = "Options")]
    pub options: CertificateOptions,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownValidationMethod {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ValidationMethod {
    Dns,
    Email,
    #[doc(hidden)]
    UnknownVariant(UnknownValidationMethod),
}

impl Default for ValidationMethod {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for ValidationMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for ValidationMethod {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for ValidationMethod {
    fn into(self) -> String {
        match self {
            ValidationMethod::Dns => "DNS".to_string(),
            ValidationMethod::Email => "EMAIL".to_string(),
            ValidationMethod::UnknownVariant(UnknownValidationMethod { name: original }) => {
                original
            }
        }
    }
}

impl<'a> Into<&'a str> for &'a ValidationMethod {
    fn into(self) -> &'a str {
        match self {
            ValidationMethod::Dns => &"DNS",
            ValidationMethod::Email => &"EMAIL",
            ValidationMethod::UnknownVariant(UnknownValidationMethod { name: original }) => {
                original
            }
        }
    }
}

impl From<&str> for ValidationMethod {
    fn from(name: &str) -> Self {
        match name {
            "DNS" => ValidationMethod::Dns,
            "EMAIL" => ValidationMethod::Email,
            _ => ValidationMethod::UnknownVariant(UnknownValidationMethod {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for ValidationMethod {
    fn from(name: String) -> Self {
        match &*name {
            "DNS" => ValidationMethod::Dns,
            "EMAIL" => ValidationMethod::Email,
            _ => ValidationMethod::UnknownVariant(UnknownValidationMethod { name }),
        }
    }
}

impl ::std::str::FromStr for ValidationMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for ValidationMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ValidationMethod {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// Errors returned by AddTagsToCertificate
#[derive(Debug, PartialEq)]
pub enum AddTagsToCertificateError {
    /// <p>The requested Amazon Resource Name (ARN) does not refer to an existing resource.</p>
    InvalidArn(String),
    /// <p>An input parameter was invalid.</p>
    InvalidParameter(String),
    /// <p>One or both of the values that make up the key-value pair is not valid. For example, you cannot specify a tag value that begins with <code>aws:</code>.</p>
    InvalidTag(String),
    /// <p>The specified certificate cannot be found in the caller's account or the caller's account cannot be found.</p>
    ResourceNotFound(String),
    /// <p>A specified tag did not comply with an existing tag policy and was rejected.</p>
    TagPolicy(String),
    /// <p>The request contains too many tags. Try the request again with fewer tags.</p>
    TooManyTags(String),
}

impl AddTagsToCertificateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AddTagsToCertificateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArnException" => {
                    return RusotoError::Service(AddTagsToCertificateError::InvalidArn(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AddTagsToCertificateError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidTagException" => {
                    return RusotoError::Service(AddTagsToCertificateError::InvalidTag(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AddTagsToCertificateError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "TagPolicyException" => {
                    return RusotoError::Service(AddTagsToCertificateError::TagPolicy(err.msg))
                }
                "TooManyTagsException" => {
                    return RusotoError::Service(AddTagsToCertificateError::TooManyTags(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AddTagsToCertificateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AddTagsToCertificateError::InvalidArn(ref cause) => write!(f, "{}", cause),
            AddTagsToCertificateError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            AddTagsToCertificateError::InvalidTag(ref cause) => write!(f, "{}", cause),
            AddTagsToCertificateError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            AddTagsToCertificateError::TagPolicy(ref cause) => write!(f, "{}", cause),
            AddTagsToCertificateError::TooManyTags(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AddTagsToCertificateError {}
/// Errors returned by DeleteCertificate
#[derive(Debug, PartialEq)]
pub enum DeleteCertificateError {
    /// <p>The requested Amazon Resource Name (ARN) does not refer to an existing resource.</p>
    InvalidArn(String),
    /// <p>The certificate is in use by another AWS service in the caller's account. Remove the association and try again.</p>
    ResourceInUse(String),
    /// <p>The specified certificate cannot be found in the caller's account or the caller's account cannot be found.</p>
    ResourceNotFound(String),
}

impl DeleteCertificateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteCertificateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArnException" => {
                    return RusotoError::Service(DeleteCertificateError::InvalidArn(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteCertificateError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteCertificateError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteCertificateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteCertificateError::InvalidArn(ref cause) => write!(f, "{}", cause),
            DeleteCertificateError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            DeleteCertificateError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteCertificateError {}
/// Errors returned by DescribeCertificate
#[derive(Debug, PartialEq)]
pub enum DescribeCertificateError {
    /// <p>The requested Amazon Resource Name (ARN) does not refer to an existing resource.</p>
    InvalidArn(String),
    /// <p>The specified certificate cannot be found in the caller's account or the caller's account cannot be found.</p>
    ResourceNotFound(String),
}

impl DescribeCertificateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeCertificateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArnException" => {
                    return RusotoError::Service(DescribeCertificateError::InvalidArn(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeCertificateError::ResourceNotFound(
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
impl fmt::Display for DescribeCertificateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeCertificateError::InvalidArn(ref cause) => write!(f, "{}", cause),
            DescribeCertificateError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeCertificateError {}
/// Errors returned by ExportCertificate
#[derive(Debug, PartialEq)]
pub enum ExportCertificateError {
    /// <p>The requested Amazon Resource Name (ARN) does not refer to an existing resource.</p>
    InvalidArn(String),
    /// <p>The certificate request is in process and the certificate in your account has not yet been issued.</p>
    RequestInProgress(String),
    /// <p>The specified certificate cannot be found in the caller's account or the caller's account cannot be found.</p>
    ResourceNotFound(String),
}

impl ExportCertificateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ExportCertificateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArnException" => {
                    return RusotoError::Service(ExportCertificateError::InvalidArn(err.msg))
                }
                "RequestInProgressException" => {
                    return RusotoError::Service(ExportCertificateError::RequestInProgress(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ExportCertificateError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ExportCertificateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ExportCertificateError::InvalidArn(ref cause) => write!(f, "{}", cause),
            ExportCertificateError::RequestInProgress(ref cause) => write!(f, "{}", cause),
            ExportCertificateError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ExportCertificateError {}
/// Errors returned by GetCertificate
#[derive(Debug, PartialEq)]
pub enum GetCertificateError {
    /// <p>The requested Amazon Resource Name (ARN) does not refer to an existing resource.</p>
    InvalidArn(String),
    /// <p>The certificate request is in process and the certificate in your account has not yet been issued.</p>
    RequestInProgress(String),
    /// <p>The specified certificate cannot be found in the caller's account or the caller's account cannot be found.</p>
    ResourceNotFound(String),
}

impl GetCertificateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetCertificateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArnException" => {
                    return RusotoError::Service(GetCertificateError::InvalidArn(err.msg))
                }
                "RequestInProgressException" => {
                    return RusotoError::Service(GetCertificateError::RequestInProgress(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetCertificateError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetCertificateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetCertificateError::InvalidArn(ref cause) => write!(f, "{}", cause),
            GetCertificateError::RequestInProgress(ref cause) => write!(f, "{}", cause),
            GetCertificateError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetCertificateError {}
/// Errors returned by ImportCertificate
#[derive(Debug, PartialEq)]
pub enum ImportCertificateError {
    /// <p>An input parameter was invalid.</p>
    InvalidParameter(String),
    /// <p>One or both of the values that make up the key-value pair is not valid. For example, you cannot specify a tag value that begins with <code>aws:</code>.</p>
    InvalidTag(String),
    /// <p>An ACM quota has been exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified certificate cannot be found in the caller's account or the caller's account cannot be found.</p>
    ResourceNotFound(String),
    /// <p>A specified tag did not comply with an existing tag policy and was rejected.</p>
    TagPolicy(String),
    /// <p>The request contains too many tags. Try the request again with fewer tags.</p>
    TooManyTags(String),
}

impl ImportCertificateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ImportCertificateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(ImportCertificateError::InvalidParameter(err.msg))
                }
                "InvalidTagException" => {
                    return RusotoError::Service(ImportCertificateError::InvalidTag(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ImportCertificateError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ImportCertificateError::ResourceNotFound(err.msg))
                }
                "TagPolicyException" => {
                    return RusotoError::Service(ImportCertificateError::TagPolicy(err.msg))
                }
                "TooManyTagsException" => {
                    return RusotoError::Service(ImportCertificateError::TooManyTags(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ImportCertificateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ImportCertificateError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ImportCertificateError::InvalidTag(ref cause) => write!(f, "{}", cause),
            ImportCertificateError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            ImportCertificateError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ImportCertificateError::TagPolicy(ref cause) => write!(f, "{}", cause),
            ImportCertificateError::TooManyTags(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ImportCertificateError {}
/// Errors returned by ListCertificates
#[derive(Debug, PartialEq)]
pub enum ListCertificatesError {
    /// <p>One or more of of request parameters specified is not valid.</p>
    InvalidArgs(String),
}

impl ListCertificatesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListCertificatesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArgsException" => {
                    return RusotoError::Service(ListCertificatesError::InvalidArgs(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListCertificatesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListCertificatesError::InvalidArgs(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListCertificatesError {}
/// Errors returned by ListTagsForCertificate
#[derive(Debug, PartialEq)]
pub enum ListTagsForCertificateError {
    /// <p>The requested Amazon Resource Name (ARN) does not refer to an existing resource.</p>
    InvalidArn(String),
    /// <p>The specified certificate cannot be found in the caller's account or the caller's account cannot be found.</p>
    ResourceNotFound(String),
}

impl ListTagsForCertificateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForCertificateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArnException" => {
                    return RusotoError::Service(ListTagsForCertificateError::InvalidArn(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListTagsForCertificateError::ResourceNotFound(
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
impl fmt::Display for ListTagsForCertificateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTagsForCertificateError::InvalidArn(ref cause) => write!(f, "{}", cause),
            ListTagsForCertificateError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForCertificateError {}
/// Errors returned by RemoveTagsFromCertificate
#[derive(Debug, PartialEq)]
pub enum RemoveTagsFromCertificateError {
    /// <p>The requested Amazon Resource Name (ARN) does not refer to an existing resource.</p>
    InvalidArn(String),
    /// <p>An input parameter was invalid.</p>
    InvalidParameter(String),
    /// <p>One or both of the values that make up the key-value pair is not valid. For example, you cannot specify a tag value that begins with <code>aws:</code>.</p>
    InvalidTag(String),
    /// <p>The specified certificate cannot be found in the caller's account or the caller's account cannot be found.</p>
    ResourceNotFound(String),
    /// <p>A specified tag did not comply with an existing tag policy and was rejected.</p>
    TagPolicy(String),
}

impl RemoveTagsFromCertificateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RemoveTagsFromCertificateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArnException" => {
                    return RusotoError::Service(RemoveTagsFromCertificateError::InvalidArn(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(RemoveTagsFromCertificateError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidTagException" => {
                    return RusotoError::Service(RemoveTagsFromCertificateError::InvalidTag(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(RemoveTagsFromCertificateError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "TagPolicyException" => {
                    return RusotoError::Service(RemoveTagsFromCertificateError::TagPolicy(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RemoveTagsFromCertificateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RemoveTagsFromCertificateError::InvalidArn(ref cause) => write!(f, "{}", cause),
            RemoveTagsFromCertificateError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            RemoveTagsFromCertificateError::InvalidTag(ref cause) => write!(f, "{}", cause),
            RemoveTagsFromCertificateError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            RemoveTagsFromCertificateError::TagPolicy(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RemoveTagsFromCertificateError {}
/// Errors returned by RenewCertificate
#[derive(Debug, PartialEq)]
pub enum RenewCertificateError {
    /// <p>The requested Amazon Resource Name (ARN) does not refer to an existing resource.</p>
    InvalidArn(String),
    /// <p>The specified certificate cannot be found in the caller's account or the caller's account cannot be found.</p>
    ResourceNotFound(String),
}

impl RenewCertificateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RenewCertificateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArnException" => {
                    return RusotoError::Service(RenewCertificateError::InvalidArn(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(RenewCertificateError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RenewCertificateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RenewCertificateError::InvalidArn(ref cause) => write!(f, "{}", cause),
            RenewCertificateError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RenewCertificateError {}
/// Errors returned by RequestCertificate
#[derive(Debug, PartialEq)]
pub enum RequestCertificateError {
    /// <p>The requested Amazon Resource Name (ARN) does not refer to an existing resource.</p>
    InvalidArn(String),
    /// <p>One or more values in the <a>DomainValidationOption</a> structure is incorrect.</p>
    InvalidDomainValidationOptions(String),
    /// <p>An input parameter was invalid.</p>
    InvalidParameter(String),
    /// <p>One or both of the values that make up the key-value pair is not valid. For example, you cannot specify a tag value that begins with <code>aws:</code>.</p>
    InvalidTag(String),
    /// <p>An ACM quota has been exceeded.</p>
    LimitExceeded(String),
    /// <p>A specified tag did not comply with an existing tag policy and was rejected.</p>
    TagPolicy(String),
    /// <p>The request contains too many tags. Try the request again with fewer tags.</p>
    TooManyTags(String),
}

impl RequestCertificateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RequestCertificateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArnException" => {
                    return RusotoError::Service(RequestCertificateError::InvalidArn(err.msg))
                }
                "InvalidDomainValidationOptionsException" => {
                    return RusotoError::Service(
                        RequestCertificateError::InvalidDomainValidationOptions(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(RequestCertificateError::InvalidParameter(err.msg))
                }
                "InvalidTagException" => {
                    return RusotoError::Service(RequestCertificateError::InvalidTag(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(RequestCertificateError::LimitExceeded(err.msg))
                }
                "TagPolicyException" => {
                    return RusotoError::Service(RequestCertificateError::TagPolicy(err.msg))
                }
                "TooManyTagsException" => {
                    return RusotoError::Service(RequestCertificateError::TooManyTags(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RequestCertificateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RequestCertificateError::InvalidArn(ref cause) => write!(f, "{}", cause),
            RequestCertificateError::InvalidDomainValidationOptions(ref cause) => {
                write!(f, "{}", cause)
            }
            RequestCertificateError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            RequestCertificateError::InvalidTag(ref cause) => write!(f, "{}", cause),
            RequestCertificateError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            RequestCertificateError::TagPolicy(ref cause) => write!(f, "{}", cause),
            RequestCertificateError::TooManyTags(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RequestCertificateError {}
/// Errors returned by ResendValidationEmail
#[derive(Debug, PartialEq)]
pub enum ResendValidationEmailError {
    /// <p>The requested Amazon Resource Name (ARN) does not refer to an existing resource.</p>
    InvalidArn(String),
    /// <p>One or more values in the <a>DomainValidationOption</a> structure is incorrect.</p>
    InvalidDomainValidationOptions(String),
    /// <p>Processing has reached an invalid state.</p>
    InvalidState(String),
    /// <p>The specified certificate cannot be found in the caller's account or the caller's account cannot be found.</p>
    ResourceNotFound(String),
}

impl ResendValidationEmailError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ResendValidationEmailError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArnException" => {
                    return RusotoError::Service(ResendValidationEmailError::InvalidArn(err.msg))
                }
                "InvalidDomainValidationOptionsException" => {
                    return RusotoError::Service(
                        ResendValidationEmailError::InvalidDomainValidationOptions(err.msg),
                    )
                }
                "InvalidStateException" => {
                    return RusotoError::Service(ResendValidationEmailError::InvalidState(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ResendValidationEmailError::ResourceNotFound(
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
impl fmt::Display for ResendValidationEmailError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ResendValidationEmailError::InvalidArn(ref cause) => write!(f, "{}", cause),
            ResendValidationEmailError::InvalidDomainValidationOptions(ref cause) => {
                write!(f, "{}", cause)
            }
            ResendValidationEmailError::InvalidState(ref cause) => write!(f, "{}", cause),
            ResendValidationEmailError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ResendValidationEmailError {}
/// Errors returned by UpdateCertificateOptions
#[derive(Debug, PartialEq)]
pub enum UpdateCertificateOptionsError {
    /// <p>The requested Amazon Resource Name (ARN) does not refer to an existing resource.</p>
    InvalidArn(String),
    /// <p>Processing has reached an invalid state.</p>
    InvalidState(String),
    /// <p>An ACM quota has been exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified certificate cannot be found in the caller's account or the caller's account cannot be found.</p>
    ResourceNotFound(String),
}

impl UpdateCertificateOptionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateCertificateOptionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArnException" => {
                    return RusotoError::Service(UpdateCertificateOptionsError::InvalidArn(err.msg))
                }
                "InvalidStateException" => {
                    return RusotoError::Service(UpdateCertificateOptionsError::InvalidState(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateCertificateOptionsError::LimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateCertificateOptionsError::ResourceNotFound(
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
impl fmt::Display for UpdateCertificateOptionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateCertificateOptionsError::InvalidArn(ref cause) => write!(f, "{}", cause),
            UpdateCertificateOptionsError::InvalidState(ref cause) => write!(f, "{}", cause),
            UpdateCertificateOptionsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateCertificateOptionsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateCertificateOptionsError {}
/// Trait representing the capabilities of the ACM API. ACM clients implement this trait.
#[async_trait]
pub trait Acm {
    /// <p>Adds one or more tags to an ACM certificate. Tags are labels that you can use to identify and organize your AWS resources. Each tag consists of a <code>key</code> and an optional <code>value</code>. You specify the certificate on input by its Amazon Resource Name (ARN). You specify the tag by using a key-value pair. </p> <p>You can apply a tag to just one certificate if you want to identify a specific characteristic of that certificate, or you can apply the same tag to multiple certificates if you want to filter for a common relationship among those certificates. Similarly, you can apply the same tag to multiple resources if you want to specify a relationship among those resources. For example, you can add the same tag to an ACM certificate and an Elastic Load Balancing load balancer to indicate that they are both used by the same website. For more information, see <a href="https://docs.aws.amazon.com/acm/latest/userguide/tags.html">Tagging ACM certificates</a>. </p> <p>To remove one or more tags, use the <a>RemoveTagsFromCertificate</a> action. To view all of the tags that have been applied to the certificate, use the <a>ListTagsForCertificate</a> action. </p>
    async fn add_tags_to_certificate(
        &self,
        input: AddTagsToCertificateRequest,
    ) -> Result<(), RusotoError<AddTagsToCertificateError>>;

    /// <p><p>Deletes a certificate and its associated private key. If this action succeeds, the certificate no longer appears in the list that can be displayed by calling the <a>ListCertificates</a> action or be retrieved by calling the <a>GetCertificate</a> action. The certificate will not be available for use by AWS services integrated with ACM. </p> <note> <p>You cannot delete an ACM certificate that is being used by another AWS service. To delete a certificate that is in use, the certificate association must first be removed.</p> </note></p>
    async fn delete_certificate(
        &self,
        input: DeleteCertificateRequest,
    ) -> Result<(), RusotoError<DeleteCertificateError>>;

    /// <p>Returns detailed metadata about the specified ACM certificate.</p>
    async fn describe_certificate(
        &self,
        input: DescribeCertificateRequest,
    ) -> Result<DescribeCertificateResponse, RusotoError<DescribeCertificateError>>;

    /// <p>Exports a private certificate issued by a private certificate authority (CA) for use anywhere. The exported file contains the certificate, the certificate chain, and the encrypted private 2048-bit RSA key associated with the public key that is embedded in the certificate. For security, you must assign a passphrase for the private key when exporting it. </p> <p>For information about exporting and formatting a certificate using the ACM console or CLI, see <a href="https://docs.aws.amazon.com/acm/latest/userguide/gs-acm-export-private.html">Export a Private Certificate</a>.</p>
    async fn export_certificate(
        &self,
        input: ExportCertificateRequest,
    ) -> Result<ExportCertificateResponse, RusotoError<ExportCertificateError>>;

    /// <p>Retrieves an Amazon-issued certificate and its certificate chain. The chain consists of the certificate of the issuing CA and the intermediate certificates of any other subordinate CAs. All of the certificates are base64 encoded. You can use <a href="https://wiki.openssl.org/index.php/Command_Line_Utilities">OpenSSL</a> to decode the certificates and inspect individual fields.</p>
    async fn get_certificate(
        &self,
        input: GetCertificateRequest,
    ) -> Result<GetCertificateResponse, RusotoError<GetCertificateError>>;

    /// <p>Imports a certificate into AWS Certificate Manager (ACM) to use with services that are integrated with ACM. Note that <a href="https://docs.aws.amazon.com/acm/latest/userguide/acm-services.html">integrated services</a> allow only certificate types and keys they support to be associated with their resources. Further, their support differs depending on whether the certificate is imported into IAM or into ACM. For more information, see the documentation for each service. For more information about importing certificates into ACM, see <a href="https://docs.aws.amazon.com/acm/latest/userguide/import-certificate.html">Importing Certificates</a> in the <i>AWS Certificate Manager User Guide</i>. </p> <note> <p>ACM does not provide <a href="https://docs.aws.amazon.com/acm/latest/userguide/acm-renewal.html">managed renewal</a> for certificates that you import.</p> </note> <p>Note the following guidelines when importing third party certificates:</p> <ul> <li> <p>You must enter the private key that matches the certificate you are importing.</p> </li> <li> <p>The private key must be unencrypted. You cannot import a private key that is protected by a password or a passphrase.</p> </li> <li> <p>If the certificate you are importing is not self-signed, you must enter its certificate chain.</p> </li> <li> <p>If a certificate chain is included, the issuer must be the subject of one of the certificates in the chain.</p> </li> <li> <p>The certificate, private key, and certificate chain must be PEM-encoded.</p> </li> <li> <p>The current time must be between the <code>Not Before</code> and <code>Not After</code> certificate fields.</p> </li> <li> <p>The <code>Issuer</code> field must not be empty.</p> </li> <li> <p>The OCSP authority URL, if present, must not exceed 1000 characters.</p> </li> <li> <p>To import a new certificate, omit the <code>CertificateArn</code> argument. Include this argument only when you want to replace a previously imported certifica</p> </li> <li> <p>When you import a certificate by using the CLI, you must specify the certificate, the certificate chain, and the private key by their file names preceded by <code>file://</code>. For example, you can specify a certificate saved in the <code>C:\temp</code> folder as <code>file://C:\temp\certificate_to_import.pem</code>. If you are making an HTTP or HTTPS Query request, include these arguments as BLOBs. </p> </li> <li> <p>When you import a certificate by using an SDK, you must specify the certificate, the certificate chain, and the private key files in the manner required by the programming language you're using. </p> </li> <li> <p>The cryptographic algorithm of an imported certificate must match the algorithm of the signing CA. For example, if the signing CA key type is RSA, then the certificate key type must also be RSA.</p> </li> </ul> <p>This operation returns the <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Name (ARN)</a> of the imported certificate.</p>
    async fn import_certificate(
        &self,
        input: ImportCertificateRequest,
    ) -> Result<ImportCertificateResponse, RusotoError<ImportCertificateError>>;

    /// <p>Retrieves a list of certificate ARNs and domain names. You can request that only certificates that match a specific status be listed. You can also filter by specific attributes of the certificate. Default filtering returns only <code>RSA_2048</code> certificates. For more information, see <a>Filters</a>.</p>
    async fn list_certificates(
        &self,
        input: ListCertificatesRequest,
    ) -> Result<ListCertificatesResponse, RusotoError<ListCertificatesError>>;

    /// <p>Lists the tags that have been applied to the ACM certificate. Use the certificate's Amazon Resource Name (ARN) to specify the certificate. To add a tag to an ACM certificate, use the <a>AddTagsToCertificate</a> action. To delete a tag, use the <a>RemoveTagsFromCertificate</a> action. </p>
    async fn list_tags_for_certificate(
        &self,
        input: ListTagsForCertificateRequest,
    ) -> Result<ListTagsForCertificateResponse, RusotoError<ListTagsForCertificateError>>;

    /// <p>Remove one or more tags from an ACM certificate. A tag consists of a key-value pair. If you do not specify the value portion of the tag when calling this function, the tag will be removed regardless of value. If you specify a value, the tag is removed only if it is associated with the specified value. </p> <p>To add tags to a certificate, use the <a>AddTagsToCertificate</a> action. To view all of the tags that have been applied to a specific ACM certificate, use the <a>ListTagsForCertificate</a> action. </p>
    async fn remove_tags_from_certificate(
        &self,
        input: RemoveTagsFromCertificateRequest,
    ) -> Result<(), RusotoError<RemoveTagsFromCertificateError>>;

    /// <p>Renews an eligable ACM certificate. At this time, only exported private certificates can be renewed with this operation. In order to renew your ACM PCA certificates with ACM, you must first <a href="https://docs.aws.amazon.com/acm-pca/latest/userguide/PcaPermissions.html">grant the ACM service principal permission to do so</a>. For more information, see <a href="https://docs.aws.amazon.com/acm/latest/userguide/manual-renewal.html">Testing Managed Renewal</a> in the ACM User Guide.</p>
    async fn renew_certificate(
        &self,
        input: RenewCertificateRequest,
    ) -> Result<(), RusotoError<RenewCertificateError>>;

    /// <p>Requests an ACM certificate for use with other AWS services. To request an ACM certificate, you must specify a fully qualified domain name (FQDN) in the <code>DomainName</code> parameter. You can also specify additional FQDNs in the <code>SubjectAlternativeNames</code> parameter. </p> <p>If you are requesting a private certificate, domain validation is not required. If you are requesting a public certificate, each domain name that you specify must be validated to verify that you own or control the domain. You can use <a href="https://docs.aws.amazon.com/acm/latest/userguide/gs-acm-validate-dns.html">DNS validation</a> or <a href="https://docs.aws.amazon.com/acm/latest/userguide/gs-acm-validate-email.html">email validation</a>. We recommend that you use DNS validation. ACM issues public certificates after receiving approval from the domain owner. </p>
    async fn request_certificate(
        &self,
        input: RequestCertificateRequest,
    ) -> Result<RequestCertificateResponse, RusotoError<RequestCertificateError>>;

    /// <p>Resends the email that requests domain ownership validation. The domain owner or an authorized representative must approve the ACM certificate before it can be issued. The certificate can be approved by clicking a link in the mail to navigate to the Amazon certificate approval website and then clicking <b>I Approve</b>. However, the validation email can be blocked by spam filters. Therefore, if you do not receive the original mail, you can request that the mail be resent within 72 hours of requesting the ACM certificate. If more than 72 hours have elapsed since your original request or since your last attempt to resend validation mail, you must request a new certificate. For more information about setting up your contact email addresses, see <a href="https://docs.aws.amazon.com/acm/latest/userguide/setup-email.html">Configure Email for your Domain</a>. </p>
    async fn resend_validation_email(
        &self,
        input: ResendValidationEmailRequest,
    ) -> Result<(), RusotoError<ResendValidationEmailError>>;

    /// <p>Updates a certificate. Currently, you can use this function to specify whether to opt in to or out of recording your certificate in a certificate transparency log. For more information, see <a href="https://docs.aws.amazon.com/acm/latest/userguide/acm-bestpractices.html#best-practices-transparency"> Opting Out of Certificate Transparency Logging</a>. </p>
    async fn update_certificate_options(
        &self,
        input: UpdateCertificateOptionsRequest,
    ) -> Result<(), RusotoError<UpdateCertificateOptionsError>>;
}
/// A client for the ACM API.
#[derive(Clone)]
pub struct AcmClient {
    client: Client,
    region: region::Region,
}

impl AcmClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> AcmClient {
        AcmClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> AcmClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        AcmClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> AcmClient {
        AcmClient { client, region }
    }
}

#[async_trait]
impl Acm for AcmClient {
    /// <p>Adds one or more tags to an ACM certificate. Tags are labels that you can use to identify and organize your AWS resources. Each tag consists of a <code>key</code> and an optional <code>value</code>. You specify the certificate on input by its Amazon Resource Name (ARN). You specify the tag by using a key-value pair. </p> <p>You can apply a tag to just one certificate if you want to identify a specific characteristic of that certificate, or you can apply the same tag to multiple certificates if you want to filter for a common relationship among those certificates. Similarly, you can apply the same tag to multiple resources if you want to specify a relationship among those resources. For example, you can add the same tag to an ACM certificate and an Elastic Load Balancing load balancer to indicate that they are both used by the same website. For more information, see <a href="https://docs.aws.amazon.com/acm/latest/userguide/tags.html">Tagging ACM certificates</a>. </p> <p>To remove one or more tags, use the <a>RemoveTagsFromCertificate</a> action. To view all of the tags that have been applied to the certificate, use the <a>ListTagsForCertificate</a> action. </p>
    async fn add_tags_to_certificate(
        &self,
        input: AddTagsToCertificateRequest,
    ) -> Result<(), RusotoError<AddTagsToCertificateError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "CertificateManager.AddTagsToCertificate");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, AddTagsToCertificateError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p><p>Deletes a certificate and its associated private key. If this action succeeds, the certificate no longer appears in the list that can be displayed by calling the <a>ListCertificates</a> action or be retrieved by calling the <a>GetCertificate</a> action. The certificate will not be available for use by AWS services integrated with ACM. </p> <note> <p>You cannot delete an ACM certificate that is being used by another AWS service. To delete a certificate that is in use, the certificate association must first be removed.</p> </note></p>
    async fn delete_certificate(
        &self,
        input: DeleteCertificateRequest,
    ) -> Result<(), RusotoError<DeleteCertificateError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "CertificateManager.DeleteCertificate");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteCertificateError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Returns detailed metadata about the specified ACM certificate.</p>
    async fn describe_certificate(
        &self,
        input: DescribeCertificateRequest,
    ) -> Result<DescribeCertificateResponse, RusotoError<DescribeCertificateError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "CertificateManager.DescribeCertificate");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeCertificateError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeCertificateResponse, _>()
    }

    /// <p>Exports a private certificate issued by a private certificate authority (CA) for use anywhere. The exported file contains the certificate, the certificate chain, and the encrypted private 2048-bit RSA key associated with the public key that is embedded in the certificate. For security, you must assign a passphrase for the private key when exporting it. </p> <p>For information about exporting and formatting a certificate using the ACM console or CLI, see <a href="https://docs.aws.amazon.com/acm/latest/userguide/gs-acm-export-private.html">Export a Private Certificate</a>.</p>
    async fn export_certificate(
        &self,
        input: ExportCertificateRequest,
    ) -> Result<ExportCertificateResponse, RusotoError<ExportCertificateError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "CertificateManager.ExportCertificate");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ExportCertificateError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ExportCertificateResponse, _>()
    }

    /// <p>Retrieves an Amazon-issued certificate and its certificate chain. The chain consists of the certificate of the issuing CA and the intermediate certificates of any other subordinate CAs. All of the certificates are base64 encoded. You can use <a href="https://wiki.openssl.org/index.php/Command_Line_Utilities">OpenSSL</a> to decode the certificates and inspect individual fields.</p>
    async fn get_certificate(
        &self,
        input: GetCertificateRequest,
    ) -> Result<GetCertificateResponse, RusotoError<GetCertificateError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "CertificateManager.GetCertificate");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetCertificateError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetCertificateResponse, _>()
    }

    /// <p>Imports a certificate into AWS Certificate Manager (ACM) to use with services that are integrated with ACM. Note that <a href="https://docs.aws.amazon.com/acm/latest/userguide/acm-services.html">integrated services</a> allow only certificate types and keys they support to be associated with their resources. Further, their support differs depending on whether the certificate is imported into IAM or into ACM. For more information, see the documentation for each service. For more information about importing certificates into ACM, see <a href="https://docs.aws.amazon.com/acm/latest/userguide/import-certificate.html">Importing Certificates</a> in the <i>AWS Certificate Manager User Guide</i>. </p> <note> <p>ACM does not provide <a href="https://docs.aws.amazon.com/acm/latest/userguide/acm-renewal.html">managed renewal</a> for certificates that you import.</p> </note> <p>Note the following guidelines when importing third party certificates:</p> <ul> <li> <p>You must enter the private key that matches the certificate you are importing.</p> </li> <li> <p>The private key must be unencrypted. You cannot import a private key that is protected by a password or a passphrase.</p> </li> <li> <p>If the certificate you are importing is not self-signed, you must enter its certificate chain.</p> </li> <li> <p>If a certificate chain is included, the issuer must be the subject of one of the certificates in the chain.</p> </li> <li> <p>The certificate, private key, and certificate chain must be PEM-encoded.</p> </li> <li> <p>The current time must be between the <code>Not Before</code> and <code>Not After</code> certificate fields.</p> </li> <li> <p>The <code>Issuer</code> field must not be empty.</p> </li> <li> <p>The OCSP authority URL, if present, must not exceed 1000 characters.</p> </li> <li> <p>To import a new certificate, omit the <code>CertificateArn</code> argument. Include this argument only when you want to replace a previously imported certifica</p> </li> <li> <p>When you import a certificate by using the CLI, you must specify the certificate, the certificate chain, and the private key by their file names preceded by <code>file://</code>. For example, you can specify a certificate saved in the <code>C:\temp</code> folder as <code>file://C:\temp\certificate_to_import.pem</code>. If you are making an HTTP or HTTPS Query request, include these arguments as BLOBs. </p> </li> <li> <p>When you import a certificate by using an SDK, you must specify the certificate, the certificate chain, and the private key files in the manner required by the programming language you're using. </p> </li> <li> <p>The cryptographic algorithm of an imported certificate must match the algorithm of the signing CA. For example, if the signing CA key type is RSA, then the certificate key type must also be RSA.</p> </li> </ul> <p>This operation returns the <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Name (ARN)</a> of the imported certificate.</p>
    async fn import_certificate(
        &self,
        input: ImportCertificateRequest,
    ) -> Result<ImportCertificateResponse, RusotoError<ImportCertificateError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "CertificateManager.ImportCertificate");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ImportCertificateError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ImportCertificateResponse, _>()
    }

    /// <p>Retrieves a list of certificate ARNs and domain names. You can request that only certificates that match a specific status be listed. You can also filter by specific attributes of the certificate. Default filtering returns only <code>RSA_2048</code> certificates. For more information, see <a>Filters</a>.</p>
    async fn list_certificates(
        &self,
        input: ListCertificatesRequest,
    ) -> Result<ListCertificatesResponse, RusotoError<ListCertificatesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "CertificateManager.ListCertificates");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListCertificatesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListCertificatesResponse, _>()
    }

    /// <p>Lists the tags that have been applied to the ACM certificate. Use the certificate's Amazon Resource Name (ARN) to specify the certificate. To add a tag to an ACM certificate, use the <a>AddTagsToCertificate</a> action. To delete a tag, use the <a>RemoveTagsFromCertificate</a> action. </p>
    async fn list_tags_for_certificate(
        &self,
        input: ListTagsForCertificateRequest,
    ) -> Result<ListTagsForCertificateResponse, RusotoError<ListTagsForCertificateError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "CertificateManager.ListTagsForCertificate");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListTagsForCertificateError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListTagsForCertificateResponse, _>()
    }

    /// <p>Remove one or more tags from an ACM certificate. A tag consists of a key-value pair. If you do not specify the value portion of the tag when calling this function, the tag will be removed regardless of value. If you specify a value, the tag is removed only if it is associated with the specified value. </p> <p>To add tags to a certificate, use the <a>AddTagsToCertificate</a> action. To view all of the tags that have been applied to a specific ACM certificate, use the <a>ListTagsForCertificate</a> action. </p>
    async fn remove_tags_from_certificate(
        &self,
        input: RemoveTagsFromCertificateRequest,
    ) -> Result<(), RusotoError<RemoveTagsFromCertificateError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "CertificateManager.RemoveTagsFromCertificate",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, RemoveTagsFromCertificateError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Renews an eligable ACM certificate. At this time, only exported private certificates can be renewed with this operation. In order to renew your ACM PCA certificates with ACM, you must first <a href="https://docs.aws.amazon.com/acm-pca/latest/userguide/PcaPermissions.html">grant the ACM service principal permission to do so</a>. For more information, see <a href="https://docs.aws.amazon.com/acm/latest/userguide/manual-renewal.html">Testing Managed Renewal</a> in the ACM User Guide.</p>
    async fn renew_certificate(
        &self,
        input: RenewCertificateRequest,
    ) -> Result<(), RusotoError<RenewCertificateError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "CertificateManager.RenewCertificate");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, RenewCertificateError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Requests an ACM certificate for use with other AWS services. To request an ACM certificate, you must specify a fully qualified domain name (FQDN) in the <code>DomainName</code> parameter. You can also specify additional FQDNs in the <code>SubjectAlternativeNames</code> parameter. </p> <p>If you are requesting a private certificate, domain validation is not required. If you are requesting a public certificate, each domain name that you specify must be validated to verify that you own or control the domain. You can use <a href="https://docs.aws.amazon.com/acm/latest/userguide/gs-acm-validate-dns.html">DNS validation</a> or <a href="https://docs.aws.amazon.com/acm/latest/userguide/gs-acm-validate-email.html">email validation</a>. We recommend that you use DNS validation. ACM issues public certificates after receiving approval from the domain owner. </p>
    async fn request_certificate(
        &self,
        input: RequestCertificateRequest,
    ) -> Result<RequestCertificateResponse, RusotoError<RequestCertificateError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "CertificateManager.RequestCertificate");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, RequestCertificateError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<RequestCertificateResponse, _>()
    }

    /// <p>Resends the email that requests domain ownership validation. The domain owner or an authorized representative must approve the ACM certificate before it can be issued. The certificate can be approved by clicking a link in the mail to navigate to the Amazon certificate approval website and then clicking <b>I Approve</b>. However, the validation email can be blocked by spam filters. Therefore, if you do not receive the original mail, you can request that the mail be resent within 72 hours of requesting the ACM certificate. If more than 72 hours have elapsed since your original request or since your last attempt to resend validation mail, you must request a new certificate. For more information about setting up your contact email addresses, see <a href="https://docs.aws.amazon.com/acm/latest/userguide/setup-email.html">Configure Email for your Domain</a>. </p>
    async fn resend_validation_email(
        &self,
        input: ResendValidationEmailRequest,
    ) -> Result<(), RusotoError<ResendValidationEmailError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "CertificateManager.ResendValidationEmail");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ResendValidationEmailError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Updates a certificate. Currently, you can use this function to specify whether to opt in to or out of recording your certificate in a certificate transparency log. For more information, see <a href="https://docs.aws.amazon.com/acm/latest/userguide/acm-bestpractices.html#best-practices-transparency"> Opting Out of Certificate Transparency Logging</a>. </p>
    async fn update_certificate_options(
        &self,
        input: UpdateCertificateOptionsRequest,
    ) -> Result<(), RusotoError<UpdateCertificateOptionsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "CertificateManager.UpdateCertificateOptions",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateCertificateOptionsError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }
}
