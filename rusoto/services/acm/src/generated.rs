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
use rusoto_core::reactor::{CredentialsProvider, RequestDispatcher};
use rusoto_core::region;
use rusoto_core::request::DispatchSignedRequest;
use rusoto_core::{ClientInner, RusotoFuture};

use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};
use rusoto_core::request::HttpDispatchError;

use hyper::StatusCode;
use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_str;
use serde_json::Value as SerdeJsonValue;
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AddTagsToCertificateRequest {
    /// <p>String that contains the ARN of the ACM certificate to which the tag is to be applied. This must be of the form:</p> <p> <code>arn:aws:acm:region:123456789012:certificate/12345678-1234-1234-1234-123456789012</code> </p> <p>For more information about ARNs, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>. </p>
    #[serde(rename = "CertificateArn")]
    pub certificate_arn: String,
    /// <p>The key-value pair that defines the tag. The tag value is optional.</p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

/// <p>Contains metadata about an ACM certificate. This structure is returned in the response to a <a>DescribeCertificate</a> request. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CertificateDetail {
    /// <p>The Amazon Resource Name (ARN) of the certificate. For more information about ARNs, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a> in the <i>AWS General Reference</i>.</p>
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
    /// <p>The reason the certificate request failed. This value exists only when the certificate status is <code>FAILED</code>. For more information, see <a href="http://docs.aws.amazon.com/http:/docs.aws.amazon.comacm/latest/userguide/troubleshooting.html#troubleshooting-failed">Certificate Request Failed</a> in the <i>AWS Certificate Manager User Guide</i>. </p>
    #[serde(rename = "FailureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
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
    pub key_algorithm: Option<String>,
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
    /// <p>Specifies whether the certificate is eligible for renewal.</p>
    #[serde(rename = "RenewalEligibility")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_eligibility: Option<String>,
    /// <p>Contains information about the status of ACM's <a href="http://docs.aws.amazon.com/http:/docs.aws.amazon.comacm/latest/userguide/acm-renewal.html">managed renewal</a> for the certificate. This field exists only when the certificate type is <code>AMAZON_ISSUED</code>.</p>
    #[serde(rename = "RenewalSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_summary: Option<RenewalSummary>,
    /// <p>The reason the certificate was revoked. This value exists only when the certificate status is <code>REVOKED</code>. </p>
    #[serde(rename = "RevocationReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revocation_reason: Option<String>,
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
    pub status: Option<String>,
    /// <p>The name of the entity that is associated with the public key contained in the certificate.</p>
    #[serde(rename = "Subject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// <p>One or more domain names (subject alternative names) included in the certificate. This list contains the domain names that are bound to the public key that is contained in the certificate. The subject alternative names include the canonical domain name (CN) of the certificate and additional domain names that can be used to connect to the website. </p>
    #[serde(rename = "SubjectAlternativeNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_alternative_names: Option<Vec<String>>,
    /// <p>The source of the certificate. For certificates provided by ACM, this value is <code>AMAZON_ISSUED</code>. For certificates that you imported with <a>ImportCertificate</a>, this value is <code>IMPORTED</code>. ACM does not provide <a href="http://docs.aws.amazon.com/http:/docs.aws.amazon.comacm/latest/userguide/acm-renewal.html">managed renewal</a> for imported certificates. For more information about the differences between certificates that you import and those that ACM provides, see <a href="http://docs.aws.amazon.com/http:/docs.aws.amazon.comacm/latest/userguide/import-certificate.html">Importing Certificates</a> in the <i>AWS Certificate Manager User Guide</i>. </p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Structure that contains options for your certificate. Currently, you can use this only to specify whether to opt in to or out of certificate transparency logging. Some browsers require that public certificates issued for your domain be recorded in a log. Certificates that are not logged typically generate a browser error. Transparency makes it possible for you to detect SSL/TLS certificates that have been mistakenly or maliciously issued for your domain. For general information, see <a href="http://docs.aws.amazon.com/http:/docs.aws.amazon.comacm/latest/userguide/acm-concepts.html#concept-transparency">Certificate Transparency Logging</a>. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CertificateOptions {
    /// <p>You can opt out of certificate transparency logging by specifying the <code>DISABLED</code> option. Opt in by specifying <code>ENABLED</code>. </p>
    #[serde(rename = "CertificateTransparencyLoggingPreference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_transparency_logging_preference: Option<String>,
}

/// <p>This structure is returned in the response object of <a>ListCertificates</a> action. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CertificateSummary {
    /// <p>Amazon Resource Name (ARN) of the certificate. This is of the form:</p> <p> <code>arn:aws:acm:region:123456789012:certificate/12345678-1234-1234-1234-123456789012</code> </p> <p>For more information about ARNs, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>. </p>
    #[serde(rename = "CertificateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    /// <p>Fully qualified domain name (FQDN), such as www.example.com or example.com, for the certificate.</p>
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteCertificateRequest {
    /// <p>String that contains the ARN of the ACM certificate to be deleted. This must be of the form:</p> <p> <code>arn:aws:acm:region:123456789012:certificate/12345678-1234-1234-1234-123456789012</code> </p> <p>For more information about ARNs, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>.</p>
    #[serde(rename = "CertificateArn")]
    pub certificate_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeCertificateRequest {
    /// <p>The Amazon Resource Name (ARN) of the ACM certificate. The ARN must have the following form:</p> <p> <code>arn:aws:acm:region:123456789012:certificate/12345678-1234-1234-1234-123456789012</code> </p> <p>For more information about ARNs, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>.</p>
    #[serde(rename = "CertificateArn")]
    pub certificate_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeCertificateResponse {
    /// <p>Metadata about an ACM certificate.</p>
    #[serde(rename = "Certificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<CertificateDetail>,
}

/// <p>Contains information about the validation of each domain name in the certificate.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DomainValidation {
    /// <p>A fully qualified domain name (FQDN) in the certificate. For example, <code>www.example.com</code> or <code>example.com</code>. </p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// <p>Contains the CNAME record that you add to your DNS database for domain validation. For more information, see <a href="http://docs.aws.amazon.com/http:/docs.aws.amazon.comacm/latest/userguide/gs-acm-validate-dns.html">Use DNS to Validate Domain Ownership</a>.</p>
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
    pub validation_method: Option<String>,
    /// <p><p>The validation status of the domain name. This can be one of the following values:</p> <ul> <li> <p> <code>PENDING_VALIDATION</code> </p> </li> <li> <p> <code/>SUCCESS</p> </li> <li> <p> <code/>FAILED</p> </li> </ul></p>
    #[serde(rename = "ValidationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_status: Option<String>,
}

/// <p>Contains information about the domain names that you want ACM to use to send you emails that enable you to validate domain ownership.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DomainValidationOption {
    /// <p>A fully qualified domain name (FQDN) in the certificate request.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// <p><p>The domain name that you want ACM to use to send you validation emails. This domain name is the suffix of the email addresses that you want ACM to use. This must be the same as the <code>DomainName</code> value or a superdomain of the <code>DomainName</code> value. For example, if you request a certificate for <code>testing.example.com</code>, you can specify <code>example.com</code> for this value. In that case, ACM sends domain validation emails to the following five addresses:</p> <ul> <li> <p>admin@example.com</p> </li> <li> <p>administrator@example.com</p> </li> <li> <p>hostmaster@example.com</p> </li> <li> <p>postmaster@example.com</p> </li> <li> <p>webmaster@example.com</p> </li> </ul></p>
    #[serde(rename = "ValidationDomain")]
    pub validation_domain: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
    pub passphrase: Vec<u8>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ExportCertificateResponse {
    /// <p>The base64 PEM-encoded certificate.</p>
    #[serde(rename = "Certificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    /// <p>The base64 PEM-encoded certificate chain. This does not include the certificate that you are exporting.</p>
    #[serde(rename = "CertificateChain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_chain: Option<String>,
    /// <p>The PEM-encoded private key associated with the public key in the certificate.</p>
    #[serde(rename = "PrivateKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
}

/// <p>The Extended Key Usage X.509 v3 extension defines one or more purposes for which the public key can be used. This is in addition to or in place of the basic purposes specified by the Key Usage extension. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ExtendedKeyUsage {
    /// <p>The name of an Extended Key Usage value.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p><p>An object identifier (OID) for the extension value. OIDs are strings of numbers separated by periods. The following OIDs are defined in RFC 3280 and RFC 5280. </p> <ul> <li> <p> <code>1.3.6.1.5.5.7.3.1 (TLS<em>WEB</em>SERVER<em>AUTHENTICATION)</code> </p> </li> <li> <p> <code>1.3.6.1.5.5.7.3.2 (TLS</em>WEB<em>CLIENT</em>AUTHENTICATION)</code> </p> </li> <li> <p> <code>1.3.6.1.5.5.7.3.3 (CODE<em>SIGNING)</code> </p> </li> <li> <p> <code>1.3.6.1.5.5.7.3.4 (EMAIL</em>PROTECTION)</code> </p> </li> <li> <p> <code>1.3.6.1.5.5.7.3.8 (TIME<em>STAMPING)</code> </p> </li> <li> <p> <code>1.3.6.1.5.5.7.3.9 (OCSP</em>SIGNING)</code> </p> </li> <li> <p> <code>1.3.6.1.5.5.7.3.5 (IPSEC<em>END</em>SYSTEM)</code> </p> </li> <li> <p> <code>1.3.6.1.5.5.7.3.6 (IPSEC<em>TUNNEL)</code> </p> </li> <li> <p> <code>1.3.6.1.5.5.7.3.7 (IPSEC</em>USER)</code> </p> </li> </ul></p>
    #[serde(rename = "OID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oid: Option<String>,
}

/// <p>This structure can be used in the <a>ListCertificates</a> action to filter the output of the certificate list. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct Filters {
    /// <p>Specify one or more <a>ExtendedKeyUsage</a> extension values.</p>
    #[serde(rename = "extendedKeyUsage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_key_usage: Option<Vec<String>>,
    /// <p>Specify one or more algorithms that can be used to generate key pairs.</p>
    #[serde(rename = "keyTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_types: Option<Vec<String>>,
    /// <p>Specify one or more <a>KeyUsage</a> extension values.</p>
    #[serde(rename = "keyUsage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_usage: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetCertificateRequest {
    /// <p>String that contains a certificate ARN in the following format:</p> <p> <code>arn:aws:acm:region:123456789012:certificate/12345678-1234-1234-1234-123456789012</code> </p> <p>For more information about ARNs, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>.</p>
    #[serde(rename = "CertificateArn")]
    pub certificate_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetCertificateResponse {
    /// <p>String that contains the ACM certificate represented by the ARN specified at input.</p>
    #[serde(rename = "Certificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    /// <p>The certificate chain that contains the root certificate issued by the certificate authority (CA).</p>
    #[serde(rename = "CertificateChain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_chain: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ImportCertificateRequest {
    /// <p>The certificate to import.</p>
    #[serde(rename = "Certificate")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub certificate: Vec<u8>,
    /// <p>The <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Name (ARN)</a> of an imported certificate to replace. To import a new certificate, omit this field. </p>
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
    pub certificate_chain: Option<Vec<u8>>,
    /// <p>The private key that matches the public key in the certificate.</p>
    #[serde(rename = "PrivateKey")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub private_key: Vec<u8>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ImportCertificateResponse {
    /// <p>The <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Name (ARN)</a> of the imported certificate.</p>
    #[serde(rename = "CertificateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
}

/// <p>The Key Usage X.509 v3 extension defines the purpose of the public key contained in the certificate.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct KeyUsage {
    /// <p>A string value that contains a Key Usage extension name.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListCertificatesRequest {
    /// <p>Filter the certificate list by status value.</p>
    #[serde(rename = "CertificateStatuses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_statuses: Option<Vec<String>>,
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListTagsForCertificateRequest {
    /// <p>String that contains the ARN of the ACM certificate for which you want to list the tags. This must have the following form:</p> <p> <code>arn:aws:acm:region:123456789012:certificate/12345678-1234-1234-1234-123456789012</code> </p> <p>For more information about ARNs, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>. </p>
    #[serde(rename = "CertificateArn")]
    pub certificate_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListTagsForCertificateResponse {
    /// <p>The key-value pairs that define the applied tags.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RemoveTagsFromCertificateRequest {
    /// <p>String that contains the ARN of the ACM Certificate with one or more tags that you want to remove. This must be of the form:</p> <p> <code>arn:aws:acm:region:123456789012:certificate/12345678-1234-1234-1234-123456789012</code> </p> <p>For more information about ARNs, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>. </p>
    #[serde(rename = "CertificateArn")]
    pub certificate_arn: String,
    /// <p>The key-value pair that defines the tag to remove.</p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

/// <p>Contains information about the status of ACM's <a href="http://docs.aws.amazon.com/http:/docs.aws.amazon.comacm/latest/userguide/acm-renewal.html">managed renewal</a> for the certificate. This structure exists only when the certificate type is <code>AMAZON_ISSUED</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct RenewalSummary {
    /// <p>Contains information about the validation of each domain name in the certificate, as it pertains to ACM's <a href="http://docs.aws.amazon.com/http:/docs.aws.amazon.comacm/latest/userguide/acm-renewal.html">managed renewal</a>. This is different from the initial validation that occurs as a result of the <a>RequestCertificate</a> request. This field exists only when the certificate type is <code>AMAZON_ISSUED</code>.</p>
    #[serde(rename = "DomainValidationOptions")]
    pub domain_validation_options: Vec<DomainValidation>,
    /// <p>The status of ACM's <a href="http://docs.aws.amazon.com/http:/docs.aws.amazon.comacm/latest/userguide/acm-renewal.html">managed renewal</a> of the certificate.</p>
    #[serde(rename = "RenewalStatus")]
    pub renewal_status: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RequestCertificateRequest {
    /// <p>The Amazon Resource Name (ARN) of the private certificate authority (CA) that will be used to issue the certificate. For more information about private CAs, see the <a href="http://docs.aws.amazon.com/http:/docs.aws.amazon.comacm-pca/latest/userguide/PcaWelcome.html">AWS Certificate Manager Private Certificate Authority (PCA)</a> user guide. The ARN must have the following form: </p> <p> <code>arn:aws:acm-pca:region:account:certificate-authority/12345678-1234-1234-1234-123456789012</code> </p>
    #[serde(rename = "CertificateAuthorityArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_authority_arn: Option<String>,
    /// <p> Fully qualified domain name (FQDN), such as www.example.com, of the site that you want to secure with an ACM Certificate. Use an asterisk (*) to create a wildcard certificate that protects several sites in the same domain. For example, *.example.com protects www.example.com, site.example.com, and images.example.com. </p> <p> The first domain name you enter cannot exceed 63 octets, including periods. Each subsequent Subject Alternative Name (SAN), however, can be up to 253 octets in length. </p>
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
    /// <p>Currently, you can use this parameter to specify whether to add the certificate to a certificate transparency log. Certificate transparency makes it possible to detect SSL/TLS certificates that have been mistakenly or maliciously issued. Certificates that have not been logged typically produce an error message in a browser. For more information, see <a href="http://docs.aws.amazon.com/http:/docs.aws.amazon.comacm/latest/userguide/acm-bestpractices.html#best-practices-transparency">Opting Out of Certificate Transparency Logging</a>.</p>
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<CertificateOptions>,
    /// <p><p>Additional FQDNs to be included in the Subject Alternative Name extension of the ACM certificate. For example, add the name www.example.net to a certificate for which the <code>DomainName</code> field is www.example.com if users can reach your site by using either name. The maximum number of domain names that you can add to an ACM certificate is 100. However, the initial limit is 10 domain names. If you need more than 10 names, you must request a limit increase. For more information, see <a href="http://docs.aws.amazon.com/http:/docs.aws.amazon.comacm/latest/userguide/acm-limits.html">Limits</a>.</p> <p> The maximum length of a SAN DNS name is 253 octets. The name is made up of multiple labels separated by periods. No label can be longer than 63 octets. Consider the following examples: </p> <ul> <li> <p> <code>(63 octets).(63 octets).(63 octets).(61 octets)</code> is legal because the total length is 253 octets (63+1+63+1+63+1+61) and no label exceeds 63 octets.</p> </li> <li> <p> <code>(64 octets).(63 octets).(63 octets).(61 octets)</code> is not legal because the total length exceeds 253 octets (64+1+63+1+63+1+61) and the first label exceeds 63 octets.</p> </li> <li> <p> <code>(63 octets).(63 octets).(63 octets).(62 octets)</code> is not legal because the total length of the DNS name (63+1+63+1+63+1+62) exceeds 253 octets.</p> </li> </ul></p>
    #[serde(rename = "SubjectAlternativeNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_alternative_names: Option<Vec<String>>,
    /// <p>The method you want to use to validate that you own or control domain. You can <a href="http://docs.aws.amazon.com/http:/docs.aws.amazon.comacm/latest/userguide/gs-acm-validate-dns.html">validate with DNS</a> or <a href="http://docs.aws.amazon.com/http:/docs.aws.amazon.comacm/latest/userguide/gs-acm-validate-email.html">validate with email</a>. We recommend that you use DNS validation. </p>
    #[serde(rename = "ValidationMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_method: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct RequestCertificateResponse {
    /// <p>String that contains the ARN of the issued certificate. This must be of the form:</p> <p> <code>arn:aws:acm:us-east-1:123456789012:certificate/12345678-1234-1234-1234-123456789012</code> </p>
    #[serde(rename = "CertificateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ResourceRecord {
    /// <p>The name of the DNS record to create in your domain. This is supplied by ACM.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The type of DNS record. Currently this can be <code>CNAME</code>.</p>
    #[serde(rename = "Type")]
    pub type_: String,
    /// <p>The value of the CNAME record to add to your DNS database. This is supplied by ACM.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

/// <p>A key-value pair that identifies or specifies metadata about an ACM resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>The key of the tag.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The value of the tag.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateCertificateOptionsRequest {
    /// <p>ARN of the requested certificate to update. This must be of the form:</p> <p> <code>arn:aws:acm:us-east-1:<i>account</i>:certificate/<i>12345678-1234-1234-1234-123456789012</i> </code> </p>
    #[serde(rename = "CertificateArn")]
    pub certificate_arn: String,
    /// <p>Use to update the options for your certificate. Currently, you can specify whether to add your certificate to a transparency log. Certificate transparency makes it possible to detect SSL/TLS certificates that have been mistakenly or maliciously issued. Certificates that have not been logged typically produce an error message in a browser. </p>
    #[serde(rename = "Options")]
    pub options: CertificateOptions,
}

/// Errors returned by AddTagsToCertificate
#[derive(Debug, PartialEq)]
pub enum AddTagsToCertificateError {
    /// <p>The requested Amazon Resource Name (ARN) does not refer to an existing resource.</p>
    InvalidArn(String),
    /// <p>One or both of the values that make up the key-value pair is not valid. For example, you cannot specify a tag value that begins with <code>aws:</code>.</p>
    InvalidTag(String),
    /// <p>The specified certificate cannot be found in the caller's account or the caller's account cannot be found.</p>
    ResourceNotFound(String),
    /// <p>The request contains too many tags. Try the request again with fewer tags.</p>
    TooManyTags(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl AddTagsToCertificateError {
    pub fn from_body(body: &str) -> AddTagsToCertificateError {
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
                    "InvalidArnException" => {
                        AddTagsToCertificateError::InvalidArn(String::from(error_message))
                    }
                    "InvalidTagException" => {
                        AddTagsToCertificateError::InvalidTag(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        AddTagsToCertificateError::ResourceNotFound(String::from(error_message))
                    }
                    "TooManyTagsException" => {
                        AddTagsToCertificateError::TooManyTags(String::from(error_message))
                    }
                    "ValidationException" => {
                        AddTagsToCertificateError::Validation(error_message.to_string())
                    }
                    _ => AddTagsToCertificateError::Unknown(String::from(body)),
                }
            }
            Err(_) => AddTagsToCertificateError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AddTagsToCertificateError {
    fn from(err: serde_json::error::Error) -> AddTagsToCertificateError {
        AddTagsToCertificateError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AddTagsToCertificateError {
    fn from(err: CredentialsError) -> AddTagsToCertificateError {
        AddTagsToCertificateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AddTagsToCertificateError {
    fn from(err: HttpDispatchError) -> AddTagsToCertificateError {
        AddTagsToCertificateError::HttpDispatch(err)
    }
}
impl From<io::Error> for AddTagsToCertificateError {
    fn from(err: io::Error) -> AddTagsToCertificateError {
        AddTagsToCertificateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AddTagsToCertificateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AddTagsToCertificateError {
    fn description(&self) -> &str {
        match *self {
            AddTagsToCertificateError::InvalidArn(ref cause) => cause,
            AddTagsToCertificateError::InvalidTag(ref cause) => cause,
            AddTagsToCertificateError::ResourceNotFound(ref cause) => cause,
            AddTagsToCertificateError::TooManyTags(ref cause) => cause,
            AddTagsToCertificateError::Validation(ref cause) => cause,
            AddTagsToCertificateError::Credentials(ref err) => err.description(),
            AddTagsToCertificateError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AddTagsToCertificateError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteCertificate
#[derive(Debug, PartialEq)]
pub enum DeleteCertificateError {
    /// <p>The requested Amazon Resource Name (ARN) does not refer to an existing resource.</p>
    InvalidArn(String),
    /// <p>The certificate is in use by another AWS service in the caller's account. Remove the association and try again.</p>
    ResourceInUse(String),
    /// <p>The specified certificate cannot be found in the caller's account or the caller's account cannot be found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteCertificateError {
    pub fn from_body(body: &str) -> DeleteCertificateError {
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
                    "InvalidArnException" => {
                        DeleteCertificateError::InvalidArn(String::from(error_message))
                    }
                    "ResourceInUseException" => {
                        DeleteCertificateError::ResourceInUse(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteCertificateError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteCertificateError::Validation(error_message.to_string())
                    }
                    _ => DeleteCertificateError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteCertificateError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteCertificateError {
    fn from(err: serde_json::error::Error) -> DeleteCertificateError {
        DeleteCertificateError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteCertificateError {
    fn from(err: CredentialsError) -> DeleteCertificateError {
        DeleteCertificateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteCertificateError {
    fn from(err: HttpDispatchError) -> DeleteCertificateError {
        DeleteCertificateError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteCertificateError {
    fn from(err: io::Error) -> DeleteCertificateError {
        DeleteCertificateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteCertificateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteCertificateError {
    fn description(&self) -> &str {
        match *self {
            DeleteCertificateError::InvalidArn(ref cause) => cause,
            DeleteCertificateError::ResourceInUse(ref cause) => cause,
            DeleteCertificateError::ResourceNotFound(ref cause) => cause,
            DeleteCertificateError::Validation(ref cause) => cause,
            DeleteCertificateError::Credentials(ref err) => err.description(),
            DeleteCertificateError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteCertificateError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeCertificate
#[derive(Debug, PartialEq)]
pub enum DescribeCertificateError {
    /// <p>The requested Amazon Resource Name (ARN) does not refer to an existing resource.</p>
    InvalidArn(String),
    /// <p>The specified certificate cannot be found in the caller's account or the caller's account cannot be found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeCertificateError {
    pub fn from_body(body: &str) -> DescribeCertificateError {
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
                    "InvalidArnException" => {
                        DescribeCertificateError::InvalidArn(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DescribeCertificateError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeCertificateError::Validation(error_message.to_string())
                    }
                    _ => DescribeCertificateError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeCertificateError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeCertificateError {
    fn from(err: serde_json::error::Error) -> DescribeCertificateError {
        DescribeCertificateError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeCertificateError {
    fn from(err: CredentialsError) -> DescribeCertificateError {
        DescribeCertificateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeCertificateError {
    fn from(err: HttpDispatchError) -> DescribeCertificateError {
        DescribeCertificateError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeCertificateError {
    fn from(err: io::Error) -> DescribeCertificateError {
        DescribeCertificateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeCertificateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeCertificateError {
    fn description(&self) -> &str {
        match *self {
            DescribeCertificateError::InvalidArn(ref cause) => cause,
            DescribeCertificateError::ResourceNotFound(ref cause) => cause,
            DescribeCertificateError::Validation(ref cause) => cause,
            DescribeCertificateError::Credentials(ref err) => err.description(),
            DescribeCertificateError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeCertificateError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ExportCertificate
#[derive(Debug, PartialEq)]
pub enum ExportCertificateError {
    /// <p>The requested Amazon Resource Name (ARN) does not refer to an existing resource.</p>
    InvalidArn(String),
    /// <p>The certificate request is in process and the certificate in your account has not yet been issued.</p>
    RequestInProgress(String),
    /// <p>The specified certificate cannot be found in the caller's account or the caller's account cannot be found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ExportCertificateError {
    pub fn from_body(body: &str) -> ExportCertificateError {
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
                    "InvalidArnException" => {
                        ExportCertificateError::InvalidArn(String::from(error_message))
                    }
                    "RequestInProgressException" => {
                        ExportCertificateError::RequestInProgress(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ExportCertificateError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        ExportCertificateError::Validation(error_message.to_string())
                    }
                    _ => ExportCertificateError::Unknown(String::from(body)),
                }
            }
            Err(_) => ExportCertificateError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ExportCertificateError {
    fn from(err: serde_json::error::Error) -> ExportCertificateError {
        ExportCertificateError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ExportCertificateError {
    fn from(err: CredentialsError) -> ExportCertificateError {
        ExportCertificateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ExportCertificateError {
    fn from(err: HttpDispatchError) -> ExportCertificateError {
        ExportCertificateError::HttpDispatch(err)
    }
}
impl From<io::Error> for ExportCertificateError {
    fn from(err: io::Error) -> ExportCertificateError {
        ExportCertificateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ExportCertificateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ExportCertificateError {
    fn description(&self) -> &str {
        match *self {
            ExportCertificateError::InvalidArn(ref cause) => cause,
            ExportCertificateError::RequestInProgress(ref cause) => cause,
            ExportCertificateError::ResourceNotFound(ref cause) => cause,
            ExportCertificateError::Validation(ref cause) => cause,
            ExportCertificateError::Credentials(ref err) => err.description(),
            ExportCertificateError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ExportCertificateError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetCertificate
#[derive(Debug, PartialEq)]
pub enum GetCertificateError {
    /// <p>The requested Amazon Resource Name (ARN) does not refer to an existing resource.</p>
    InvalidArn(String),
    /// <p>The certificate request is in process and the certificate in your account has not yet been issued.</p>
    RequestInProgress(String),
    /// <p>The specified certificate cannot be found in the caller's account or the caller's account cannot be found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetCertificateError {
    pub fn from_body(body: &str) -> GetCertificateError {
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
                    "InvalidArnException" => {
                        GetCertificateError::InvalidArn(String::from(error_message))
                    }
                    "RequestInProgressException" => {
                        GetCertificateError::RequestInProgress(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        GetCertificateError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetCertificateError::Validation(error_message.to_string())
                    }
                    _ => GetCertificateError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetCertificateError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetCertificateError {
    fn from(err: serde_json::error::Error) -> GetCertificateError {
        GetCertificateError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetCertificateError {
    fn from(err: CredentialsError) -> GetCertificateError {
        GetCertificateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetCertificateError {
    fn from(err: HttpDispatchError) -> GetCertificateError {
        GetCertificateError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetCertificateError {
    fn from(err: io::Error) -> GetCertificateError {
        GetCertificateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetCertificateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetCertificateError {
    fn description(&self) -> &str {
        match *self {
            GetCertificateError::InvalidArn(ref cause) => cause,
            GetCertificateError::RequestInProgress(ref cause) => cause,
            GetCertificateError::ResourceNotFound(ref cause) => cause,
            GetCertificateError::Validation(ref cause) => cause,
            GetCertificateError::Credentials(ref err) => err.description(),
            GetCertificateError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetCertificateError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ImportCertificate
#[derive(Debug, PartialEq)]
pub enum ImportCertificateError {
    /// <p>An ACM limit has been exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified certificate cannot be found in the caller's account or the caller's account cannot be found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ImportCertificateError {
    pub fn from_body(body: &str) -> ImportCertificateError {
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
                    "LimitExceededException" => {
                        ImportCertificateError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ImportCertificateError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        ImportCertificateError::Validation(error_message.to_string())
                    }
                    _ => ImportCertificateError::Unknown(String::from(body)),
                }
            }
            Err(_) => ImportCertificateError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ImportCertificateError {
    fn from(err: serde_json::error::Error) -> ImportCertificateError {
        ImportCertificateError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ImportCertificateError {
    fn from(err: CredentialsError) -> ImportCertificateError {
        ImportCertificateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ImportCertificateError {
    fn from(err: HttpDispatchError) -> ImportCertificateError {
        ImportCertificateError::HttpDispatch(err)
    }
}
impl From<io::Error> for ImportCertificateError {
    fn from(err: io::Error) -> ImportCertificateError {
        ImportCertificateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ImportCertificateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ImportCertificateError {
    fn description(&self) -> &str {
        match *self {
            ImportCertificateError::LimitExceeded(ref cause) => cause,
            ImportCertificateError::ResourceNotFound(ref cause) => cause,
            ImportCertificateError::Validation(ref cause) => cause,
            ImportCertificateError::Credentials(ref err) => err.description(),
            ImportCertificateError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ImportCertificateError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListCertificates
#[derive(Debug, PartialEq)]
pub enum ListCertificatesError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListCertificatesError {
    pub fn from_body(body: &str) -> ListCertificatesError {
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
                    "ValidationException" => {
                        ListCertificatesError::Validation(error_message.to_string())
                    }
                    _ => ListCertificatesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListCertificatesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListCertificatesError {
    fn from(err: serde_json::error::Error) -> ListCertificatesError {
        ListCertificatesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListCertificatesError {
    fn from(err: CredentialsError) -> ListCertificatesError {
        ListCertificatesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListCertificatesError {
    fn from(err: HttpDispatchError) -> ListCertificatesError {
        ListCertificatesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListCertificatesError {
    fn from(err: io::Error) -> ListCertificatesError {
        ListCertificatesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListCertificatesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListCertificatesError {
    fn description(&self) -> &str {
        match *self {
            ListCertificatesError::Validation(ref cause) => cause,
            ListCertificatesError::Credentials(ref err) => err.description(),
            ListCertificatesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListCertificatesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTagsForCertificate
#[derive(Debug, PartialEq)]
pub enum ListTagsForCertificateError {
    /// <p>The requested Amazon Resource Name (ARN) does not refer to an existing resource.</p>
    InvalidArn(String),
    /// <p>The specified certificate cannot be found in the caller's account or the caller's account cannot be found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListTagsForCertificateError {
    pub fn from_body(body: &str) -> ListTagsForCertificateError {
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
                    "InvalidArnException" => {
                        ListTagsForCertificateError::InvalidArn(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ListTagsForCertificateError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListTagsForCertificateError::Validation(error_message.to_string())
                    }
                    _ => ListTagsForCertificateError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListTagsForCertificateError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListTagsForCertificateError {
    fn from(err: serde_json::error::Error) -> ListTagsForCertificateError {
        ListTagsForCertificateError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListTagsForCertificateError {
    fn from(err: CredentialsError) -> ListTagsForCertificateError {
        ListTagsForCertificateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListTagsForCertificateError {
    fn from(err: HttpDispatchError) -> ListTagsForCertificateError {
        ListTagsForCertificateError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListTagsForCertificateError {
    fn from(err: io::Error) -> ListTagsForCertificateError {
        ListTagsForCertificateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListTagsForCertificateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTagsForCertificateError {
    fn description(&self) -> &str {
        match *self {
            ListTagsForCertificateError::InvalidArn(ref cause) => cause,
            ListTagsForCertificateError::ResourceNotFound(ref cause) => cause,
            ListTagsForCertificateError::Validation(ref cause) => cause,
            ListTagsForCertificateError::Credentials(ref err) => err.description(),
            ListTagsForCertificateError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListTagsForCertificateError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RemoveTagsFromCertificate
#[derive(Debug, PartialEq)]
pub enum RemoveTagsFromCertificateError {
    /// <p>The requested Amazon Resource Name (ARN) does not refer to an existing resource.</p>
    InvalidArn(String),
    /// <p>One or both of the values that make up the key-value pair is not valid. For example, you cannot specify a tag value that begins with <code>aws:</code>.</p>
    InvalidTag(String),
    /// <p>The specified certificate cannot be found in the caller's account or the caller's account cannot be found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RemoveTagsFromCertificateError {
    pub fn from_body(body: &str) -> RemoveTagsFromCertificateError {
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
                    "InvalidArnException" => {
                        RemoveTagsFromCertificateError::InvalidArn(String::from(error_message))
                    }
                    "InvalidTagException" => {
                        RemoveTagsFromCertificateError::InvalidTag(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        RemoveTagsFromCertificateError::ResourceNotFound(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        RemoveTagsFromCertificateError::Validation(error_message.to_string())
                    }
                    _ => RemoveTagsFromCertificateError::Unknown(String::from(body)),
                }
            }
            Err(_) => RemoveTagsFromCertificateError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RemoveTagsFromCertificateError {
    fn from(err: serde_json::error::Error) -> RemoveTagsFromCertificateError {
        RemoveTagsFromCertificateError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RemoveTagsFromCertificateError {
    fn from(err: CredentialsError) -> RemoveTagsFromCertificateError {
        RemoveTagsFromCertificateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RemoveTagsFromCertificateError {
    fn from(err: HttpDispatchError) -> RemoveTagsFromCertificateError {
        RemoveTagsFromCertificateError::HttpDispatch(err)
    }
}
impl From<io::Error> for RemoveTagsFromCertificateError {
    fn from(err: io::Error) -> RemoveTagsFromCertificateError {
        RemoveTagsFromCertificateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RemoveTagsFromCertificateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RemoveTagsFromCertificateError {
    fn description(&self) -> &str {
        match *self {
            RemoveTagsFromCertificateError::InvalidArn(ref cause) => cause,
            RemoveTagsFromCertificateError::InvalidTag(ref cause) => cause,
            RemoveTagsFromCertificateError::ResourceNotFound(ref cause) => cause,
            RemoveTagsFromCertificateError::Validation(ref cause) => cause,
            RemoveTagsFromCertificateError::Credentials(ref err) => err.description(),
            RemoveTagsFromCertificateError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RemoveTagsFromCertificateError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RequestCertificate
#[derive(Debug, PartialEq)]
pub enum RequestCertificateError {
    /// <p>The requested Amazon Resource Name (ARN) does not refer to an existing resource.</p>
    InvalidArn(String),
    /// <p>One or more values in the <a>DomainValidationOption</a> structure is incorrect.</p>
    InvalidDomainValidationOptions(String),
    /// <p>An ACM limit has been exceeded.</p>
    LimitExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RequestCertificateError {
    pub fn from_body(body: &str) -> RequestCertificateError {
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
                    "InvalidArnException" => {
                        RequestCertificateError::InvalidArn(String::from(error_message))
                    }
                    "InvalidDomainValidationOptionsException" => {
                        RequestCertificateError::InvalidDomainValidationOptions(String::from(
                            error_message,
                        ))
                    }
                    "LimitExceededException" => {
                        RequestCertificateError::LimitExceeded(String::from(error_message))
                    }
                    "ValidationException" => {
                        RequestCertificateError::Validation(error_message.to_string())
                    }
                    _ => RequestCertificateError::Unknown(String::from(body)),
                }
            }
            Err(_) => RequestCertificateError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RequestCertificateError {
    fn from(err: serde_json::error::Error) -> RequestCertificateError {
        RequestCertificateError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RequestCertificateError {
    fn from(err: CredentialsError) -> RequestCertificateError {
        RequestCertificateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RequestCertificateError {
    fn from(err: HttpDispatchError) -> RequestCertificateError {
        RequestCertificateError::HttpDispatch(err)
    }
}
impl From<io::Error> for RequestCertificateError {
    fn from(err: io::Error) -> RequestCertificateError {
        RequestCertificateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RequestCertificateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RequestCertificateError {
    fn description(&self) -> &str {
        match *self {
            RequestCertificateError::InvalidArn(ref cause) => cause,
            RequestCertificateError::InvalidDomainValidationOptions(ref cause) => cause,
            RequestCertificateError::LimitExceeded(ref cause) => cause,
            RequestCertificateError::Validation(ref cause) => cause,
            RequestCertificateError::Credentials(ref err) => err.description(),
            RequestCertificateError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RequestCertificateError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ResendValidationEmailError {
    pub fn from_body(body: &str) -> ResendValidationEmailError {
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
                    "InvalidArnException" => {
                        ResendValidationEmailError::InvalidArn(String::from(error_message))
                    }
                    "InvalidDomainValidationOptionsException" => {
                        ResendValidationEmailError::InvalidDomainValidationOptions(String::from(
                            error_message,
                        ))
                    }
                    "InvalidStateException" => {
                        ResendValidationEmailError::InvalidState(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ResendValidationEmailError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        ResendValidationEmailError::Validation(error_message.to_string())
                    }
                    _ => ResendValidationEmailError::Unknown(String::from(body)),
                }
            }
            Err(_) => ResendValidationEmailError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ResendValidationEmailError {
    fn from(err: serde_json::error::Error) -> ResendValidationEmailError {
        ResendValidationEmailError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ResendValidationEmailError {
    fn from(err: CredentialsError) -> ResendValidationEmailError {
        ResendValidationEmailError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ResendValidationEmailError {
    fn from(err: HttpDispatchError) -> ResendValidationEmailError {
        ResendValidationEmailError::HttpDispatch(err)
    }
}
impl From<io::Error> for ResendValidationEmailError {
    fn from(err: io::Error) -> ResendValidationEmailError {
        ResendValidationEmailError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ResendValidationEmailError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ResendValidationEmailError {
    fn description(&self) -> &str {
        match *self {
            ResendValidationEmailError::InvalidArn(ref cause) => cause,
            ResendValidationEmailError::InvalidDomainValidationOptions(ref cause) => cause,
            ResendValidationEmailError::InvalidState(ref cause) => cause,
            ResendValidationEmailError::ResourceNotFound(ref cause) => cause,
            ResendValidationEmailError::Validation(ref cause) => cause,
            ResendValidationEmailError::Credentials(ref err) => err.description(),
            ResendValidationEmailError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ResendValidationEmailError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateCertificateOptions
#[derive(Debug, PartialEq)]
pub enum UpdateCertificateOptionsError {
    /// <p>The requested Amazon Resource Name (ARN) does not refer to an existing resource.</p>
    InvalidArn(String),
    /// <p>Processing has reached an invalid state.</p>
    InvalidState(String),
    /// <p>An ACM limit has been exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified certificate cannot be found in the caller's account or the caller's account cannot be found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateCertificateOptionsError {
    pub fn from_body(body: &str) -> UpdateCertificateOptionsError {
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
                    "InvalidArnException" => {
                        UpdateCertificateOptionsError::InvalidArn(String::from(error_message))
                    }
                    "InvalidStateException" => {
                        UpdateCertificateOptionsError::InvalidState(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        UpdateCertificateOptionsError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        UpdateCertificateOptionsError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateCertificateOptionsError::Validation(error_message.to_string())
                    }
                    _ => UpdateCertificateOptionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateCertificateOptionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateCertificateOptionsError {
    fn from(err: serde_json::error::Error) -> UpdateCertificateOptionsError {
        UpdateCertificateOptionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateCertificateOptionsError {
    fn from(err: CredentialsError) -> UpdateCertificateOptionsError {
        UpdateCertificateOptionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateCertificateOptionsError {
    fn from(err: HttpDispatchError) -> UpdateCertificateOptionsError {
        UpdateCertificateOptionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateCertificateOptionsError {
    fn from(err: io::Error) -> UpdateCertificateOptionsError {
        UpdateCertificateOptionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateCertificateOptionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateCertificateOptionsError {
    fn description(&self) -> &str {
        match *self {
            UpdateCertificateOptionsError::InvalidArn(ref cause) => cause,
            UpdateCertificateOptionsError::InvalidState(ref cause) => cause,
            UpdateCertificateOptionsError::LimitExceeded(ref cause) => cause,
            UpdateCertificateOptionsError::ResourceNotFound(ref cause) => cause,
            UpdateCertificateOptionsError::Validation(ref cause) => cause,
            UpdateCertificateOptionsError::Credentials(ref err) => err.description(),
            UpdateCertificateOptionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateCertificateOptionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the ACM API. ACM clients implement this trait.
pub trait Acm {
    /// <p>Adds one or more tags to an ACM certificate. Tags are labels that you can use to identify and organize your AWS resources. Each tag consists of a <code>key</code> and an optional <code>value</code>. You specify the certificate on input by its Amazon Resource Name (ARN). You specify the tag by using a key-value pair. </p> <p>You can apply a tag to just one certificate if you want to identify a specific characteristic of that certificate, or you can apply the same tag to multiple certificates if you want to filter for a common relationship among those certificates. Similarly, you can apply the same tag to multiple resources if you want to specify a relationship among those resources. For example, you can add the same tag to an ACM certificate and an Elastic Load Balancing load balancer to indicate that they are both used by the same website. For more information, see <a href="http://docs.aws.amazon.com/http:/docs.aws.amazon.comacm/latest/userguide/tags.html">Tagging ACM certificates</a>. </p> <p>To remove one or more tags, use the <a>RemoveTagsFromCertificate</a> action. To view all of the tags that have been applied to the certificate, use the <a>ListTagsForCertificate</a> action. </p>
    fn add_tags_to_certificate(
        &self,
        input: AddTagsToCertificateRequest,
    ) -> RusotoFuture<(), AddTagsToCertificateError>;

    /// <p><p>Deletes a certificate and its associated private key. If this action succeeds, the certificate no longer appears in the list that can be displayed by calling the <a>ListCertificates</a> action or be retrieved by calling the <a>GetCertificate</a> action. The certificate will not be available for use by AWS services integrated with ACM. </p> <note> <p>You cannot delete an ACM certificate that is being used by another AWS service. To delete a certificate that is in use, the certificate association must first be removed.</p> </note></p>
    fn delete_certificate(
        &self,
        input: DeleteCertificateRequest,
    ) -> RusotoFuture<(), DeleteCertificateError>;

    /// <p>Returns detailed metadata about the specified ACM certificate.</p>
    fn describe_certificate(
        &self,
        input: DescribeCertificateRequest,
    ) -> RusotoFuture<DescribeCertificateResponse, DescribeCertificateError>;

    /// <p>Exports a certificate for use anywhere. You can export the certificate, the certificate chain, and the encrypted private key associated with the public key embedded in the certificate. You must store the private key securely. The private key is a 2048 bit RSA key. You must provide a passphrase for the private key when exporting it. You can use the following OpenSSL command to decrypt it later. Provide the passphrase when prompted. </p> <p> <code>openssl rsa -in encrypted_key.pem -out decrypted_key.pem</code> </p>
    fn export_certificate(
        &self,
        input: ExportCertificateRequest,
    ) -> RusotoFuture<ExportCertificateResponse, ExportCertificateError>;

    /// <p>Retrieves a certificate specified by an ARN and its certificate chain . The chain is an ordered list of certificates that contains the end entity certificate, intermediate certificates of subordinate CAs, and the root certificate in that order. The certificate and certificate chain are base64 encoded. If you want to decode the certificate to see the individual fields, you can use OpenSSL.</p>
    fn get_certificate(
        &self,
        input: GetCertificateRequest,
    ) -> RusotoFuture<GetCertificateResponse, GetCertificateError>;

    /// <p>Imports a certificate into AWS Certificate Manager (ACM) to use with services that are integrated with ACM. Note that <a href="http://docs.aws.amazon.com/http:/docs.aws.amazon.comacm/latest/userguide/acm-services.html">integrated services</a> allow only certificate types and keys they support to be associated with their resources. Further, their support differs depending on whether the certificate is imported into IAM or into ACM. For more information, see the documentation for each service. For more information about importing certificates into ACM, see <a href="http://docs.aws.amazon.com/http:/docs.aws.amazon.comacm/latest/userguide/import-certificate.html">Importing Certificates</a> in the <i>AWS Certificate Manager User Guide</i>. </p> <note> <p>ACM does not provide <a href="http://docs.aws.amazon.com/http:/docs.aws.amazon.comacm/latest/userguide/acm-renewal.html">managed renewal</a> for certificates that you import.</p> </note> <p>Note the following guidelines when importing third party certificates:</p> <ul> <li> <p>You must enter the private key that matches the certificate you are importing.</p> </li> <li> <p>The private key must be unencrypted. You cannot import a private key that is protected by a password or a passphrase.</p> </li> <li> <p>If the certificate you are importing is not self-signed, you must enter its certificate chain.</p> </li> <li> <p>If a certificate chain is included, the issuer must be the subject of one of the certificates in the chain.</p> </li> <li> <p>The certificate, private key, and certificate chain must be PEM-encoded.</p> </li> <li> <p>The current time must be between the <code>Not Before</code> and <code>Not After</code> certificate fields.</p> </li> <li> <p>The <code>Issuer</code> field must not be empty.</p> </li> <li> <p>The OCSP authority URL, if present, must not exceed 1000 characters.</p> </li> <li> <p>To import a new certificate, omit the <code>CertificateArn</code> argument. Include this argument only when you want to replace a previously imported certificate.</p> </li> <li> <p>When you import a certificate by using the CLI or one of the SDKs, you must specify the certificate, the certificate chain, and the private key by their file names preceded by <code>file://</code>. For example, you can specify a certificate saved in the <code>C:\temp</code> folder as <code>file://C:\temp\certificate_to_import.pem</code>. If you are making an HTTP or HTTPS Query request, include these arguments as BLOBs. </p> </li> </ul> <p>This operation returns the <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Name (ARN)</a> of the imported certificate.</p>
    fn import_certificate(
        &self,
        input: ImportCertificateRequest,
    ) -> RusotoFuture<ImportCertificateResponse, ImportCertificateError>;

    /// <p>Retrieves a list of certificate ARNs and domain names. You can request that only certificates that match a specific status be listed. You can also filter by specific attributes of the certificate. </p>
    fn list_certificates(
        &self,
        input: ListCertificatesRequest,
    ) -> RusotoFuture<ListCertificatesResponse, ListCertificatesError>;

    /// <p>Lists the tags that have been applied to the ACM certificate. Use the certificate's Amazon Resource Name (ARN) to specify the certificate. To add a tag to an ACM certificate, use the <a>AddTagsToCertificate</a> action. To delete a tag, use the <a>RemoveTagsFromCertificate</a> action. </p>
    fn list_tags_for_certificate(
        &self,
        input: ListTagsForCertificateRequest,
    ) -> RusotoFuture<ListTagsForCertificateResponse, ListTagsForCertificateError>;

    /// <p>Remove one or more tags from an ACM certificate. A tag consists of a key-value pair. If you do not specify the value portion of the tag when calling this function, the tag will be removed regardless of value. If you specify a value, the tag is removed only if it is associated with the specified value. </p> <p>To add tags to a certificate, use the <a>AddTagsToCertificate</a> action. To view all of the tags that have been applied to a specific ACM certificate, use the <a>ListTagsForCertificate</a> action. </p>
    fn remove_tags_from_certificate(
        &self,
        input: RemoveTagsFromCertificateRequest,
    ) -> RusotoFuture<(), RemoveTagsFromCertificateError>;

    /// <p>Requests an ACM certificate for use with other AWS services. To request an ACM certificate, you must specify the fully qualified domain name (FQDN) for your site in the <code>DomainName</code> parameter. You can also specify additional FQDNs in the <code>SubjectAlternativeNames</code> parameter. </p> <p>Each domain name that you specify must be validated to verify that you own or control the domain. You can use <a href="http://docs.aws.amazon.com/http:/docs.aws.amazon.comacm/latest/userguide/gs-acm-validate-dns.html">DNS validation</a> or <a href="http://docs.aws.amazon.com/http:/docs.aws.amazon.comacm/latest/userguide/gs-acm-validate-email.html">email validation</a>. We recommend that you use DNS validation. </p> <p>If you choose email validation, email is sent to the domain owner to request approval to issue the certificate. Email is sent to three registered contact addresses in the WHOIS database and to five common system administration addresses formed from the <code>DomainName</code> you enter or the optional <code>ValidationDomain</code> parameter. For more information, see <a href="http://docs.aws.amazon.com/http:/docs.aws.amazon.comacm/latest/userguide/gs-acm-validate-email.html">Validate with Email</a>. </p> <p>After receiving approval from the domain owner, the ACM certificate is issued.</p>
    fn request_certificate(
        &self,
        input: RequestCertificateRequest,
    ) -> RusotoFuture<RequestCertificateResponse, RequestCertificateError>;

    /// <p>Resends the email that requests domain ownership validation. The domain owner or an authorized representative must approve the ACM certificate before it can be issued. The certificate can be approved by clicking a link in the mail to navigate to the Amazon certificate approval website and then clicking <b>I Approve</b>. However, the validation email can be blocked by spam filters. Therefore, if you do not receive the original mail, you can request that the mail be resent within 72 hours of requesting the ACM certificate. If more than 72 hours have elapsed since your original request or since your last attempt to resend validation mail, you must request a new certificate. For more information about setting up your contact email addresses, see <a href="http://docs.aws.amazon.com/http:/docs.aws.amazon.comacm/latest/userguide/setup-email.html">Configure Email for your Domain</a>. </p>
    fn resend_validation_email(
        &self,
        input: ResendValidationEmailRequest,
    ) -> RusotoFuture<(), ResendValidationEmailError>;

    /// <p>Updates a certificate. Currently, you can use this function to specify whether to opt in to or out of recording your certificate in a certificate transparency log. For more information, see <a href="http://docs.aws.amazon.com/http:/docs.aws.amazon.comacm/latest/userguide/acm-bestpractices.html#best-practices-transparency"> Opting Out of Certificate Transparency Logging</a>. </p>
    fn update_certificate_options(
        &self,
        input: UpdateCertificateOptionsRequest,
    ) -> RusotoFuture<(), UpdateCertificateOptionsError>;
}
/// A client for the ACM API.
pub struct AcmClient<P = CredentialsProvider, D = RequestDispatcher>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    inner: ClientInner<P, D>,
    region: region::Region,
}

impl AcmClient {
    /// Creates a simple client backed by an implicit event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    ///
    /// See the `rusoto_core::reactor` module for more details.
    pub fn simple(region: region::Region) -> AcmClient {
        AcmClient::new(
            RequestDispatcher::default(),
            CredentialsProvider::default(),
            region,
        )
    }
}

impl<P, D> AcmClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        AcmClient {
            inner: ClientInner::new(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl<P, D> Acm for AcmClient<P, D>
where
    P: ProvideAwsCredentials + 'static,
    D: DispatchSignedRequest + 'static,
{
    /// <p>Adds one or more tags to an ACM certificate. Tags are labels that you can use to identify and organize your AWS resources. Each tag consists of a <code>key</code> and an optional <code>value</code>. You specify the certificate on input by its Amazon Resource Name (ARN). You specify the tag by using a key-value pair. </p> <p>You can apply a tag to just one certificate if you want to identify a specific characteristic of that certificate, or you can apply the same tag to multiple certificates if you want to filter for a common relationship among those certificates. Similarly, you can apply the same tag to multiple resources if you want to specify a relationship among those resources. For example, you can add the same tag to an ACM certificate and an Elastic Load Balancing load balancer to indicate that they are both used by the same website. For more information, see <a href="http://docs.aws.amazon.com/http:/docs.aws.amazon.comacm/latest/userguide/tags.html">Tagging ACM certificates</a>. </p> <p>To remove one or more tags, use the <a>RemoveTagsFromCertificate</a> action. To view all of the tags that have been applied to the certificate, use the <a>ListTagsForCertificate</a> action. </p>
    fn add_tags_to_certificate(
        &self,
        input: AddTagsToCertificateRequest,
    ) -> RusotoFuture<(), AddTagsToCertificateError> {
        let mut request = SignedRequest::new("POST", "acm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CertificateManager.AddTagsToCertificate");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(AddTagsToCertificateError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Deletes a certificate and its associated private key. If this action succeeds, the certificate no longer appears in the list that can be displayed by calling the <a>ListCertificates</a> action or be retrieved by calling the <a>GetCertificate</a> action. The certificate will not be available for use by AWS services integrated with ACM. </p> <note> <p>You cannot delete an ACM certificate that is being used by another AWS service. To delete a certificate that is in use, the certificate association must first be removed.</p> </note></p>
    fn delete_certificate(
        &self,
        input: DeleteCertificateRequest,
    ) -> RusotoFuture<(), DeleteCertificateError> {
        let mut request = SignedRequest::new("POST", "acm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CertificateManager.DeleteCertificate");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteCertificateError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns detailed metadata about the specified ACM certificate.</p>
    fn describe_certificate(
        &self,
        input: DescribeCertificateRequest,
    ) -> RusotoFuture<DescribeCertificateResponse, DescribeCertificateError> {
        let mut request = SignedRequest::new("POST", "acm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CertificateManager.DescribeCertificate");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeCertificateResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeCertificateError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Exports a certificate for use anywhere. You can export the certificate, the certificate chain, and the encrypted private key associated with the public key embedded in the certificate. You must store the private key securely. The private key is a 2048 bit RSA key. You must provide a passphrase for the private key when exporting it. You can use the following OpenSSL command to decrypt it later. Provide the passphrase when prompted. </p> <p> <code>openssl rsa -in encrypted_key.pem -out decrypted_key.pem</code> </p>
    fn export_certificate(
        &self,
        input: ExportCertificateRequest,
    ) -> RusotoFuture<ExportCertificateResponse, ExportCertificateError> {
        let mut request = SignedRequest::new("POST", "acm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CertificateManager.ExportCertificate");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ExportCertificateResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ExportCertificateError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Retrieves a certificate specified by an ARN and its certificate chain . The chain is an ordered list of certificates that contains the end entity certificate, intermediate certificates of subordinate CAs, and the root certificate in that order. The certificate and certificate chain are base64 encoded. If you want to decode the certificate to see the individual fields, you can use OpenSSL.</p>
    fn get_certificate(
        &self,
        input: GetCertificateRequest,
    ) -> RusotoFuture<GetCertificateResponse, GetCertificateError> {
        let mut request = SignedRequest::new("POST", "acm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CertificateManager.GetCertificate");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetCertificateResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetCertificateError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Imports a certificate into AWS Certificate Manager (ACM) to use with services that are integrated with ACM. Note that <a href="http://docs.aws.amazon.com/http:/docs.aws.amazon.comacm/latest/userguide/acm-services.html">integrated services</a> allow only certificate types and keys they support to be associated with their resources. Further, their support differs depending on whether the certificate is imported into IAM or into ACM. For more information, see the documentation for each service. For more information about importing certificates into ACM, see <a href="http://docs.aws.amazon.com/http:/docs.aws.amazon.comacm/latest/userguide/import-certificate.html">Importing Certificates</a> in the <i>AWS Certificate Manager User Guide</i>. </p> <note> <p>ACM does not provide <a href="http://docs.aws.amazon.com/http:/docs.aws.amazon.comacm/latest/userguide/acm-renewal.html">managed renewal</a> for certificates that you import.</p> </note> <p>Note the following guidelines when importing third party certificates:</p> <ul> <li> <p>You must enter the private key that matches the certificate you are importing.</p> </li> <li> <p>The private key must be unencrypted. You cannot import a private key that is protected by a password or a passphrase.</p> </li> <li> <p>If the certificate you are importing is not self-signed, you must enter its certificate chain.</p> </li> <li> <p>If a certificate chain is included, the issuer must be the subject of one of the certificates in the chain.</p> </li> <li> <p>The certificate, private key, and certificate chain must be PEM-encoded.</p> </li> <li> <p>The current time must be between the <code>Not Before</code> and <code>Not After</code> certificate fields.</p> </li> <li> <p>The <code>Issuer</code> field must not be empty.</p> </li> <li> <p>The OCSP authority URL, if present, must not exceed 1000 characters.</p> </li> <li> <p>To import a new certificate, omit the <code>CertificateArn</code> argument. Include this argument only when you want to replace a previously imported certificate.</p> </li> <li> <p>When you import a certificate by using the CLI or one of the SDKs, you must specify the certificate, the certificate chain, and the private key by their file names preceded by <code>file://</code>. For example, you can specify a certificate saved in the <code>C:\temp</code> folder as <code>file://C:\temp\certificate_to_import.pem</code>. If you are making an HTTP or HTTPS Query request, include these arguments as BLOBs. </p> </li> </ul> <p>This operation returns the <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Name (ARN)</a> of the imported certificate.</p>
    fn import_certificate(
        &self,
        input: ImportCertificateRequest,
    ) -> RusotoFuture<ImportCertificateResponse, ImportCertificateError> {
        let mut request = SignedRequest::new("POST", "acm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CertificateManager.ImportCertificate");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ImportCertificateResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ImportCertificateError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Retrieves a list of certificate ARNs and domain names. You can request that only certificates that match a specific status be listed. You can also filter by specific attributes of the certificate. </p>
    fn list_certificates(
        &self,
        input: ListCertificatesRequest,
    ) -> RusotoFuture<ListCertificatesResponse, ListCertificatesError> {
        let mut request = SignedRequest::new("POST", "acm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CertificateManager.ListCertificates");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListCertificatesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListCertificatesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists the tags that have been applied to the ACM certificate. Use the certificate's Amazon Resource Name (ARN) to specify the certificate. To add a tag to an ACM certificate, use the <a>AddTagsToCertificate</a> action. To delete a tag, use the <a>RemoveTagsFromCertificate</a> action. </p>
    fn list_tags_for_certificate(
        &self,
        input: ListTagsForCertificateRequest,
    ) -> RusotoFuture<ListTagsForCertificateResponse, ListTagsForCertificateError> {
        let mut request = SignedRequest::new("POST", "acm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CertificateManager.ListTagsForCertificate");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListTagsForCertificateResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListTagsForCertificateError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Remove one or more tags from an ACM certificate. A tag consists of a key-value pair. If you do not specify the value portion of the tag when calling this function, the tag will be removed regardless of value. If you specify a value, the tag is removed only if it is associated with the specified value. </p> <p>To add tags to a certificate, use the <a>AddTagsToCertificate</a> action. To view all of the tags that have been applied to a specific ACM certificate, use the <a>ListTagsForCertificate</a> action. </p>
    fn remove_tags_from_certificate(
        &self,
        input: RemoveTagsFromCertificateRequest,
    ) -> RusotoFuture<(), RemoveTagsFromCertificateError> {
        let mut request = SignedRequest::new("POST", "acm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CertificateManager.RemoveTagsFromCertificate",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(RemoveTagsFromCertificateError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Requests an ACM certificate for use with other AWS services. To request an ACM certificate, you must specify the fully qualified domain name (FQDN) for your site in the <code>DomainName</code> parameter. You can also specify additional FQDNs in the <code>SubjectAlternativeNames</code> parameter. </p> <p>Each domain name that you specify must be validated to verify that you own or control the domain. You can use <a href="http://docs.aws.amazon.com/http:/docs.aws.amazon.comacm/latest/userguide/gs-acm-validate-dns.html">DNS validation</a> or <a href="http://docs.aws.amazon.com/http:/docs.aws.amazon.comacm/latest/userguide/gs-acm-validate-email.html">email validation</a>. We recommend that you use DNS validation. </p> <p>If you choose email validation, email is sent to the domain owner to request approval to issue the certificate. Email is sent to three registered contact addresses in the WHOIS database and to five common system administration addresses formed from the <code>DomainName</code> you enter or the optional <code>ValidationDomain</code> parameter. For more information, see <a href="http://docs.aws.amazon.com/http:/docs.aws.amazon.comacm/latest/userguide/gs-acm-validate-email.html">Validate with Email</a>. </p> <p>After receiving approval from the domain owner, the ACM certificate is issued.</p>
    fn request_certificate(
        &self,
        input: RequestCertificateRequest,
    ) -> RusotoFuture<RequestCertificateResponse, RequestCertificateError> {
        let mut request = SignedRequest::new("POST", "acm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CertificateManager.RequestCertificate");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<RequestCertificateResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(RequestCertificateError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Resends the email that requests domain ownership validation. The domain owner or an authorized representative must approve the ACM certificate before it can be issued. The certificate can be approved by clicking a link in the mail to navigate to the Amazon certificate approval website and then clicking <b>I Approve</b>. However, the validation email can be blocked by spam filters. Therefore, if you do not receive the original mail, you can request that the mail be resent within 72 hours of requesting the ACM certificate. If more than 72 hours have elapsed since your original request or since your last attempt to resend validation mail, you must request a new certificate. For more information about setting up your contact email addresses, see <a href="http://docs.aws.amazon.com/http:/docs.aws.amazon.comacm/latest/userguide/setup-email.html">Configure Email for your Domain</a>. </p>
    fn resend_validation_email(
        &self,
        input: ResendValidationEmailRequest,
    ) -> RusotoFuture<(), ResendValidationEmailError> {
        let mut request = SignedRequest::new("POST", "acm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CertificateManager.ResendValidationEmail");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ResendValidationEmailError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Updates a certificate. Currently, you can use this function to specify whether to opt in to or out of recording your certificate in a certificate transparency log. For more information, see <a href="http://docs.aws.amazon.com/http:/docs.aws.amazon.comacm/latest/userguide/acm-bestpractices.html#best-practices-transparency"> Opting Out of Certificate Transparency Logging</a>. </p>
    fn update_certificate_options(
        &self,
        input: UpdateCertificateOptionsRequest,
    ) -> RusotoFuture<(), UpdateCertificateOptionsError> {
        let mut request = SignedRequest::new("POST", "acm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CertificateManager.UpdateCertificateOptions",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateCertificateOptionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }
}

#[cfg(test)]
mod protocol_tests {}
