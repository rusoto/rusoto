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
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoFuture};

use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};
use rusoto_core::request::HttpDispatchError;

use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_slice;
use serde_json::Value as SerdeJsonValue;
/// <p>Contains information about the certificate subject. The certificate can be one issued by your private certificate authority (CA) or it can be your private CA certificate. The <b>Subject</b> field in the certificate identifies the entity that owns or controls the public key in the certificate. The entity can be a user, computer, device, or service. The <b>Subject</b> must contain an X.500 distinguished name (DN). A DN is a sequence of relative distinguished names (RDNs). The RDNs are separated by commas in the certificate. The DN must be unique for each entity, but your private CA can issue more than one certificate with the same DN to the same entity. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ASN1Subject {
    /// <p>Fully qualified domain name (FQDN) associated with the certificate subject.</p>
    #[serde(rename = "CommonName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_name: Option<String>,
    /// <p>Two-digit code that specifies the country in which the certificate subject located.</p>
    #[serde(rename = "Country")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// <p>Disambiguating information for the certificate subject.</p>
    #[serde(rename = "DistinguishedNameQualifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distinguished_name_qualifier: Option<String>,
    /// <p>Typically a qualifier appended to the name of an individual. Examples include Jr. for junior, Sr. for senior, and III for third.</p>
    #[serde(rename = "GenerationQualifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_qualifier: Option<String>,
    /// <p>First name.</p>
    #[serde(rename = "GivenName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    /// <p>Concatenation that typically contains the first letter of the <b>GivenName</b>, the first letter of the middle name if one exists, and the first letter of the <b>SurName</b>.</p>
    #[serde(rename = "Initials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initials: Option<String>,
    /// <p>The locality (such as a city or town) in which the certificate subject is located.</p>
    #[serde(rename = "Locality")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locality: Option<String>,
    /// <p>Legal name of the organization with which the certificate subject is affiliated. </p>
    #[serde(rename = "Organization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    /// <p>A subdivision or unit of the organization (such as sales or finance) with which the certificate subject is affiliated.</p>
    #[serde(rename = "OrganizationalUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizational_unit: Option<String>,
    /// <p>Typically a shortened version of a longer <b>GivenName</b>. For example, Jonathan is often shortened to John. Elizabeth is often shortened to Beth, Liz, or Eliza.</p>
    #[serde(rename = "Pseudonym")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pseudonym: Option<String>,
    /// <p>The certificate serial number.</p>
    #[serde(rename = "SerialNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    /// <p>State in which the subject of the certificate is located.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>Family name. In the US and the UK, for example, the surname of an individual is ordered last. In Asian cultures the surname is typically ordered first.</p>
    #[serde(rename = "Surname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surname: Option<String>,
    /// <p>A title such as Mr. or Ms., which is pre-pended to the name to refer formally to the certificate subject.</p>
    #[serde(rename = "Title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// <p>Contains information about your private certificate authority (CA). Your private CA can issue and revoke X.509 digital certificates. Digital certificates verify that the entity named in the certificate <b>Subject</b> field owns or controls the public key contained in the <b>Subject Public Key Info</b> field. Call the <a>CreateCertificateAuthority</a> operation to create your private CA. You must then call the <a>GetCertificateAuthorityCertificate</a> operation to retrieve a private CA certificate signing request (CSR). Take the CSR to your on-premises CA and sign it with the root CA certificate or a subordinate certificate. Call the <a>ImportCertificateAuthorityCertificate</a> operation to import the signed certificate into AWS Certificate Manager (ACM). </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CertificateAuthority {
    /// <p>Amazon Resource Name (ARN) for your private certificate authority (CA). The format is <code> <i>12345678-1234-1234-1234-123456789012</i> </code>.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>Your private CA configuration.</p>
    #[serde(rename = "CertificateAuthorityConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_authority_configuration: Option<CertificateAuthorityConfiguration>,
    /// <p>Date and time at which your private CA was created.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>Reason the request to create your private CA failed.</p>
    #[serde(rename = "FailureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>Date and time at which your private CA was last updated.</p>
    #[serde(rename = "LastStateChangeAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_state_change_at: Option<f64>,
    /// <p>Date and time after which your private CA certificate is not valid.</p>
    #[serde(rename = "NotAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_after: Option<f64>,
    /// <p>Date and time before which your private CA certificate is not valid.</p>
    #[serde(rename = "NotBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_before: Option<f64>,
    /// <p>The period during which a deleted CA can be restored. For more information, see the <code>PermanentDeletionTimeInDays</code> parameter of the <a>DeleteCertificateAuthorityRequest</a> operation. </p>
    #[serde(rename = "RestorableUntil")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restorable_until: Option<f64>,
    /// <p>Information about the certificate revocation list (CRL) created and maintained by your private CA. </p>
    #[serde(rename = "RevocationConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revocation_configuration: Option<RevocationConfiguration>,
    /// <p>Serial number of your private CA.</p>
    #[serde(rename = "Serial")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial: Option<String>,
    /// <p>Status of your private CA.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>Type of your private CA.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Contains configuration information for your private certificate authority (CA). This includes information about the class of public key algorithm and the key pair that your private CA creates when it issues a certificate, the signature algorithm it uses used when issuing certificates, and its X.500 distinguished name. You must specify this information when you call the <a>CreateCertificateAuthority</a> operation. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CertificateAuthorityConfiguration {
    /// <p>Type of the public key algorithm and size, in bits, of the key pair that your key pair creates when it issues a certificate.</p>
    #[serde(rename = "KeyAlgorithm")]
    pub key_algorithm: String,
    /// <p>Name of the algorithm your private CA uses to sign certificate requests.</p>
    #[serde(rename = "SigningAlgorithm")]
    pub signing_algorithm: String,
    /// <p>Structure that contains X.500 distinguished name information for your private CA.</p>
    #[serde(rename = "Subject")]
    pub subject: ASN1Subject,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateCertificateAuthorityAuditReportRequest {
    /// <p>Format in which to create the report. This can be either <b>JSON</b> or <b>CSV</b>.</p>
    #[serde(rename = "AuditReportResponseFormat")]
    pub audit_report_response_format: String,
    /// <p>Amazon Resource Name (ARN) of the CA to be audited. This is of the form:</p> <p> <code>arn:aws:acm-pca:<i>region</i>:<i>account</i>:certificate-authority/<i>12345678-1234-1234-1234-123456789012</i> </code>.</p>
    #[serde(rename = "CertificateAuthorityArn")]
    pub certificate_authority_arn: String,
    /// <p>Name of the S3 bucket that will contain the audit report.</p>
    #[serde(rename = "S3BucketName")]
    pub s3_bucket_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateCertificateAuthorityAuditReportResponse {
    /// <p>An alphanumeric string that contains a report identifier.</p>
    #[serde(rename = "AuditReportId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_report_id: Option<String>,
    /// <p>The <b>key</b> that uniquely identifies the report file in your S3 bucket.</p>
    #[serde(rename = "S3Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateCertificateAuthorityRequest {
    /// <p>Name and bit size of the private key algorithm, the name of the signing algorithm, and X.500 certificate subject information.</p>
    #[serde(rename = "CertificateAuthorityConfiguration")]
    pub certificate_authority_configuration: CertificateAuthorityConfiguration,
    /// <p>The type of the certificate authority. Currently, this must be <b>SUBORDINATE</b>.</p>
    #[serde(rename = "CertificateAuthorityType")]
    pub certificate_authority_type: String,
    /// <p>Alphanumeric string that can be used to distinguish between calls to <b>CreateCertificateAuthority</b>. Idempotency tokens time out after five minutes. Therefore, if you call <b>CreateCertificateAuthority</b> multiple times with the same idempotency token within a five minute period, ACM PCA recognizes that you are requesting only one certificate. As a result, ACM PCA issues only one. If you change the idempotency token for each call, however, ACM PCA recognizes that you are requesting multiple certificates.</p>
    #[serde(rename = "IdempotencyToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_token: Option<String>,
    /// <p>Contains a Boolean value that you can use to enable a certification revocation list (CRL) for the CA, the name of the S3 bucket to which ACM PCA will write the CRL, and an optional CNAME alias that you can use to hide the name of your bucket in the <b>CRL Distribution Points</b> extension of your CA certificate. For more information, see the <a>CrlConfiguration</a> structure. </p>
    #[serde(rename = "RevocationConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revocation_configuration: Option<RevocationConfiguration>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateCertificateAuthorityResponse {
    /// <p>If successful, the Amazon Resource Name (ARN) of the certificate authority (CA). This is of the form: </p> <p> <code>arn:aws:acm-pca:<i>region</i>:<i>account</i>:certificate-authority/<i>12345678-1234-1234-1234-123456789012</i> </code>. </p>
    #[serde(rename = "CertificateAuthorityArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_authority_arn: Option<String>,
}

/// <p>Contains configuration information for a certificate revocation list (CRL). Your private certificate authority (CA) creates base CRLs. Delta CRLs are not supported. You can enable CRLs for your new or an existing private CA by setting the <b>Enabled</b> parameter to <code>true</code>. Your private CA writes CRLs to an S3 bucket that you specify in the <b>S3BucketName</b> parameter. You can hide the name of your bucket by specifying a value for the <b>CustomCname</b> parameter. Your private CA copies the CNAME or the S3 bucket name to the <b>CRL Distribution Points</b> extension of each certificate it issues. Your S3 bucket policy must give write permission to ACM PCA. </p> <p>Your private CA uses the value in the <b>ExpirationInDays</b> parameter to calculate the <b>nextUpdate</b> field in the CRL. The CRL is refreshed at 1/2 the age of next update or when a certificate is revoked. When a certificate is revoked, it is recorded in the next CRL that is generated and in the next audit report. Only time valid certificates are listed in the CRL. Expired certificates are not included. </p> <p>CRLs contain the following fields:</p> <ul> <li> <p> <b>Version</b>: The current version number defined in RFC 5280 is V2. The integer value is 0x1. </p> </li> <li> <p> <b>Signature Algorithm</b>: The name of the algorithm used to sign the CRL.</p> </li> <li> <p> <b>Issuer</b>: The X.500 distinguished name of your private CA that issued the CRL.</p> </li> <li> <p> <b>Last Update</b>: The issue date and time of this CRL.</p> </li> <li> <p> <b>Next Update</b>: The day and time by which the next CRL will be issued.</p> </li> <li> <p> <b>Revoked Certificates</b>: List of revoked certificates. Each list item contains the following information.</p> <ul> <li> <p> <b>Serial Number</b>: The serial number, in hexadecimal format, of the revoked certificate.</p> </li> <li> <p> <b>Revocation Date</b>: Date and time the certificate was revoked.</p> </li> <li> <p> <b>CRL Entry Extensions</b>: Optional extensions for the CRL entry.</p> <ul> <li> <p> <b>X509v3 CRL Reason Code</b>: Reason the certificate was revoked.</p> </li> </ul> </li> </ul> </li> <li> <p> <b>CRL Extensions</b>: Optional extensions for the CRL.</p> <ul> <li> <p> <b>X509v3 Authority Key Identifier</b>: Identifies the public key associated with the private key used to sign the certificate.</p> </li> <li> <p> <b>X509v3 CRL Number:</b>: Decimal sequence number for the CRL.</p> </li> </ul> </li> <li> <p> <b>Signature Algorithm</b>: Algorithm used by your private CA to sign the CRL.</p> </li> <li> <p> <b>Signature Value</b>: Signature computed over the CRL.</p> </li> </ul> <p>Certificate revocation lists created by ACM PCA are DER-encoded. You can use the following OpenSSL command to list a CRL.</p> <p> <code>openssl crl -inform DER -text -in <i>crl_path</i> -noout</code> </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CrlConfiguration {
    /// <p>Name inserted into the certificate <b>CRL Distribution Points</b> extension that enables the use of an alias for the CRL distribution point. Use this value if you don't want the name of your S3 bucket to be public.</p>
    #[serde(rename = "CustomCname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_cname: Option<String>,
    /// <p>Boolean value that specifies whether certificate revocation lists (CRLs) are enabled. You can use this value to enable certificate revocation for a new CA when you call the <a>CreateCertificateAuthority</a> operation or for an existing CA when you call the <a>UpdateCertificateAuthority</a> operation. </p>
    #[serde(rename = "Enabled")]
    pub enabled: bool,
    /// <p>Number of days until a certificate expires.</p>
    #[serde(rename = "ExpirationInDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_in_days: Option<i64>,
    /// <p>Name of the S3 bucket that contains the CRL. If you do not provide a value for the <b>CustomCname</b> argument, the name of your S3 bucket is placed into the <b>CRL Distribution Points</b> extension of the issued certificate. You can change the name of your bucket by calling the <a>UpdateCertificateAuthority</a> operation. You must specify a bucket policy that allows ACM PCA to write the CRL to your bucket.</p>
    #[serde(rename = "S3BucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteCertificateAuthorityRequest {
    /// <p>The Amazon Resource Name (ARN) that was returned when you called <a>CreateCertificateAuthority</a>. This must have the following form: </p> <p> <code>arn:aws:acm-pca:<i>region</i>:<i>account</i>:certificate-authority/<i>12345678-1234-1234-1234-123456789012</i> </code>. </p>
    #[serde(rename = "CertificateAuthorityArn")]
    pub certificate_authority_arn: String,
    /// <p>The number of days to make a CA restorable after it has been deleted. This can be anywhere from 7 to 30 days, with 30 being the default.</p>
    #[serde(rename = "PermanentDeletionTimeInDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permanent_deletion_time_in_days: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeCertificateAuthorityAuditReportRequest {
    /// <p>The report ID returned by calling the <a>CreateCertificateAuthorityAuditReport</a> operation.</p>
    #[serde(rename = "AuditReportId")]
    pub audit_report_id: String,
    /// <p>The Amazon Resource Name (ARN) of the private CA. This must be of the form:</p> <p> <code>arn:aws:acm-pca:<i>region</i>:<i>account</i>:certificate-authority/<i>12345678-1234-1234-1234-123456789012</i> </code>. </p>
    #[serde(rename = "CertificateAuthorityArn")]
    pub certificate_authority_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeCertificateAuthorityAuditReportResponse {
    /// <p>Specifies whether report creation is in progress, has succeeded, or has failed.</p>
    #[serde(rename = "AuditReportStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_report_status: Option<String>,
    /// <p>The date and time at which the report was created.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>Name of the S3 bucket that contains the report.</p>
    #[serde(rename = "S3BucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_name: Option<String>,
    /// <p>S3 <b>key</b> that uniquely identifies the report file in your S3 bucket.</p>
    #[serde(rename = "S3Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeCertificateAuthorityRequest {
    /// <p>The Amazon Resource Name (ARN) that was returned when you called <a>CreateCertificateAuthority</a>. This must be of the form: </p> <p> <code>arn:aws:acm-pca:<i>region</i>:<i>account</i>:certificate-authority/<i>12345678-1234-1234-1234-123456789012</i> </code>. </p>
    #[serde(rename = "CertificateAuthorityArn")]
    pub certificate_authority_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeCertificateAuthorityResponse {
    /// <p>A <a>CertificateAuthority</a> structure that contains information about your private CA.</p>
    #[serde(rename = "CertificateAuthority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_authority: Option<CertificateAuthority>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetCertificateAuthorityCertificateRequest {
    /// <p>The Amazon Resource Name (ARN) of your private CA. This is of the form:</p> <p> <code>arn:aws:acm-pca:<i>region</i>:<i>account</i>:certificate-authority/<i>12345678-1234-1234-1234-123456789012</i> </code>. </p>
    #[serde(rename = "CertificateAuthorityArn")]
    pub certificate_authority_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetCertificateAuthorityCertificateResponse {
    /// <p>Base64-encoded certificate authority (CA) certificate.</p>
    #[serde(rename = "Certificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    /// <p>Base64-encoded certificate chain that includes any intermediate certificates and chains up to root on-premises certificate that you used to sign your private CA certificate. The chain does not include your private CA certificate. </p>
    #[serde(rename = "CertificateChain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_chain: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetCertificateAuthorityCsrRequest {
    /// <p>The Amazon Resource Name (ARN) that was returned when you called the <a>CreateCertificateAuthority</a> operation. This must be of the form: </p> <p> <code>arn:aws:acm-pca:<i>region</i>:<i>account</i>:certificate-authority/<i>12345678-1234-1234-1234-123456789012</i> </code> </p>
    #[serde(rename = "CertificateAuthorityArn")]
    pub certificate_authority_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetCertificateAuthorityCsrResponse {
    /// <p>The base64 PEM-encoded certificate signing request (CSR) for your private CA certificate.</p>
    #[serde(rename = "Csr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csr: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetCertificateRequest {
    /// <p>The ARN of the issued certificate. The ARN contains the certificate serial number and must be in the following form: </p> <p> <code>arn:aws:acm-pca:<i>region</i>:<i>account</i>:certificate-authority/<i>12345678-1234-1234-1234-123456789012</i>/certificate/<i>286535153982981100925020015808220737245</i> </code> </p>
    #[serde(rename = "CertificateArn")]
    pub certificate_arn: String,
    /// <p>The Amazon Resource Name (ARN) that was returned when you called <a>CreateCertificateAuthority</a>. This must be of the form: </p> <p> <code>arn:aws:acm-pca:<i>region</i>:<i>account</i>:certificate-authority/<i>12345678-1234-1234-1234-123456789012</i> </code>. </p>
    #[serde(rename = "CertificateAuthorityArn")]
    pub certificate_authority_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetCertificateResponse {
    /// <p>The base64 PEM-encoded certificate specified by the <code>CertificateArn</code> parameter.</p>
    #[serde(rename = "Certificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    /// <p>The base64 PEM-encoded certificate chain that chains up to the on-premises root CA certificate that you used to sign your private CA certificate. </p>
    #[serde(rename = "CertificateChain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_chain: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ImportCertificateAuthorityCertificateRequest {
    /// <p>The PEM-encoded certificate for your private CA. This must be signed by using your on-premises CA.</p>
    #[serde(rename = "Certificate")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub certificate: Vec<u8>,
    /// <p>The Amazon Resource Name (ARN) that was returned when you called <a>CreateCertificateAuthority</a>. This must be of the form: </p> <p> <code>arn:aws:acm-pca:<i>region</i>:<i>account</i>:certificate-authority/<i>12345678-1234-1234-1234-123456789012</i> </code> </p>
    #[serde(rename = "CertificateAuthorityArn")]
    pub certificate_authority_arn: String,
    /// <p>A PEM-encoded file that contains all of your certificates, other than the certificate you're importing, chaining up to your root CA. Your on-premises root certificate is the last in the chain, and each certificate in the chain signs the one preceding. </p>
    #[serde(rename = "CertificateChain")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub certificate_chain: Vec<u8>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct IssueCertificateRequest {
    /// <p>The Amazon Resource Name (ARN) that was returned when you called <a>CreateCertificateAuthority</a>. This must be of the form:</p> <p> <code>arn:aws:acm-pca:<i>region</i>:<i>account</i>:certificate-authority/<i>12345678-1234-1234-1234-123456789012</i> </code> </p>
    #[serde(rename = "CertificateAuthorityArn")]
    pub certificate_authority_arn: String,
    /// <p>The certificate signing request (CSR) for the certificate you want to issue. You can use the following OpenSSL command to create the CSR and a 2048 bit RSA private key. </p> <p> <code>openssl req -new -newkey rsa:2048 -days 365 -keyout private/test_cert_priv_key.pem -out csr/test_cert_.csr</code> </p> <p>If you have a configuration file, you can use the following OpenSSL command. The <code>usr_cert</code> block in the configuration file contains your X509 version 3 extensions. </p> <p> <code>openssl req -new -config openssl_rsa.cnf -extensions usr_cert -newkey rsa:2048 -days -365 -keyout private/test_cert_priv_key.pem -out csr/test_cert_.csr</code> </p>
    #[serde(rename = "Csr")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub csr: Vec<u8>,
    /// <p>Custom string that can be used to distinguish between calls to the <b>IssueCertificate</b> operation. Idempotency tokens time out after one hour. Therefore, if you call <b>IssueCertificate</b> multiple times with the same idempotency token within 5 minutes, ACM PCA recognizes that you are requesting only one certificate and will issue only one. If you change the idempotency token for each call, PCA recognizes that you are requesting multiple certificates.</p>
    #[serde(rename = "IdempotencyToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_token: Option<String>,
    /// <p>The name of the algorithm that will be used to sign the certificate to be issued.</p>
    #[serde(rename = "SigningAlgorithm")]
    pub signing_algorithm: String,
    /// <p>The type of the validity period.</p>
    #[serde(rename = "Validity")]
    pub validity: Validity,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct IssueCertificateResponse {
    /// <p>The Amazon Resource Name (ARN) of the issued certificate and the certificate serial number. This is of the form:</p> <p> <code>arn:aws:acm-pca:<i>region</i>:<i>account</i>:certificate-authority/<i>12345678-1234-1234-1234-123456789012</i>/certificate/<i>286535153982981100925020015808220737245</i> </code> </p>
    #[serde(rename = "CertificateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListCertificateAuthoritiesRequest {
    /// <p>Use this parameter when paginating results to specify the maximum number of items to return in the response on each page. If additional items exist beyond the number you specify, the <code>NextToken</code> element is sent in the response. Use this <code>NextToken</code> value in a subsequent request to retrieve additional items.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Use this parameter when paginating results in a subsequent request after you receive a response with truncated results. Set it to the value of the <code>NextToken</code> parameter from the response you just received.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListCertificateAuthoritiesResponse {
    /// <p>Summary information about each certificate authority you have created.</p>
    #[serde(rename = "CertificateAuthorities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_authorities: Option<Vec<CertificateAuthority>>,
    /// <p>When the list is truncated, this value is present and should be used for the <code>NextToken</code> parameter in a subsequent pagination request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListTagsRequest {
    /// <p>The Amazon Resource Name (ARN) that was returned when you called the <a>CreateCertificateAuthority</a> operation. This must be of the form: </p> <p> <code>arn:aws:acm-pca:<i>region</i>:<i>account</i>:certificate-authority/<i>12345678-1234-1234-1234-123456789012</i> </code> </p>
    #[serde(rename = "CertificateAuthorityArn")]
    pub certificate_authority_arn: String,
    /// <p>Use this parameter when paginating results to specify the maximum number of items to return in the response. If additional items exist beyond the number you specify, the <b>NextToken</b> element is sent in the response. Use this <b>NextToken</b> value in a subsequent request to retrieve additional items.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Use this parameter when paginating results in a subsequent request after you receive a response with truncated results. Set it to the value of <b>NextToken</b> from the response you just received.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListTagsResponse {
    /// <p>When the list is truncated, this value is present and should be used for the <b>NextToken</b> parameter in a subsequent pagination request. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The tags associated with your private CA.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RestoreCertificateAuthorityRequest {
    /// <p>The Amazon Resource Name (ARN) that was returned when you called the <a>CreateCertificateAuthority</a> operation. This must be of the form: </p> <p> <code>arn:aws:acm-pca:<i>region</i>:<i>account</i>:certificate-authority/<i>12345678-1234-1234-1234-123456789012</i> </code> </p>
    #[serde(rename = "CertificateAuthorityArn")]
    pub certificate_authority_arn: String,
}

/// <p>Certificate revocation information used by the <a>CreateCertificateAuthority</a> and <a>UpdateCertificateAuthority</a> operations. Your private certificate authority (CA) can create and maintain a certificate revocation list (CRL). A CRL contains information about certificates revoked by your CA. For more information, see <a>RevokeCertificate</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RevocationConfiguration {
    /// <p>Configuration of the certificate revocation list (CRL), if any, maintained by your private CA.</p>
    #[serde(rename = "CrlConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crl_configuration: Option<CrlConfiguration>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RevokeCertificateRequest {
    /// <p>Amazon Resource Name (ARN) of the private CA that issued the certificate to be revoked. This must be of the form:</p> <p> <code>arn:aws:acm-pca:<i>region</i>:<i>account</i>:certificate-authority/<i>12345678-1234-1234-1234-123456789012</i> </code> </p>
    #[serde(rename = "CertificateAuthorityArn")]
    pub certificate_authority_arn: String,
    /// <p>Serial number of the certificate to be revoked. This must be in hexadecimal format. You can retrieve the serial number by calling <a>GetCertificate</a> with the Amazon Resource Name (ARN) of the certificate you want and the ARN of your private CA. The <b>GetCertificate</b> operation retrieves the certificate in the PEM format. You can use the following OpenSSL command to list the certificate in text format and copy the hexadecimal serial number. </p> <p> <code>openssl x509 -in <i>file_path</i> -text -noout</code> </p> <p>You can also copy the serial number from the console or use the <a href="https://docs.aws.amazon.com/acm/latest/APIReference/API_DescribeCertificate.html">DescribeCertificate</a> operation in the <i>AWS Certificate Manager API Reference</i>. </p>
    #[serde(rename = "CertificateSerial")]
    pub certificate_serial: String,
    /// <p>Specifies why you revoked the certificate.</p>
    #[serde(rename = "RevocationReason")]
    pub revocation_reason: String,
}

/// <p>Tags are labels that you can use to identify and organize your private CAs. Each tag consists of a key and an optional value. You can associate up to 50 tags with a private CA. To add one or more tags to a private CA, call the <a>TagCertificateAuthority</a> operation. To remove a tag, call the <a>UntagCertificateAuthority</a> operation. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>Key (name) of the tag.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>Value of the tag.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TagCertificateAuthorityRequest {
    /// <p>The Amazon Resource Name (ARN) that was returned when you called <a>CreateCertificateAuthority</a>. This must be of the form: </p> <p> <code>arn:aws:acm-pca:<i>region</i>:<i>account</i>:certificate-authority/<i>12345678-1234-1234-1234-123456789012</i> </code> </p>
    #[serde(rename = "CertificateAuthorityArn")]
    pub certificate_authority_arn: String,
    /// <p>List of tags to be associated with the CA.</p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UntagCertificateAuthorityRequest {
    /// <p>The Amazon Resource Name (ARN) that was returned when you called <a>CreateCertificateAuthority</a>. This must be of the form: </p> <p> <code>arn:aws:acm-pca:<i>region</i>:<i>account</i>:certificate-authority/<i>12345678-1234-1234-1234-123456789012</i> </code> </p>
    #[serde(rename = "CertificateAuthorityArn")]
    pub certificate_authority_arn: String,
    /// <p>List of tags to be removed from the CA.</p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateCertificateAuthorityRequest {
    /// <p>Amazon Resource Name (ARN) of the private CA that issued the certificate to be revoked. This must be of the form:</p> <p> <code>arn:aws:acm-pca:<i>region</i>:<i>account</i>:certificate-authority/<i>12345678-1234-1234-1234-123456789012</i> </code> </p>
    #[serde(rename = "CertificateAuthorityArn")]
    pub certificate_authority_arn: String,
    /// <p>Revocation information for your private CA.</p>
    #[serde(rename = "RevocationConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revocation_configuration: Option<RevocationConfiguration>,
    /// <p>Status of your private CA.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Length of time for which the certificate issued by your private certificate authority (CA), or by the private CA itself, is valid in days, months, or years. You can issue a certificate by calling the <a>IssueCertificate</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct Validity {
    /// <p>Specifies whether the <code>Value</code> parameter represents days, months, or years.</p>
    #[serde(rename = "Type")]
    pub type_: String,
    /// <p>Time period.</p>
    #[serde(rename = "Value")]
    pub value: i64,
}

/// Errors returned by CreateCertificateAuthority
#[derive(Debug, PartialEq)]
pub enum CreateCertificateAuthorityError {
    /// <p>One or more of the specified arguments was not valid.</p>
    InvalidArgs(String),
    /// <p>The S3 bucket policy is not valid. The policy must give ACM PCA rights to read from and write to the bucket and find the bucket location.</p>
    InvalidPolicy(String),
    /// <p>An ACM PCA limit has been exceeded. See the exception message returned to determine the limit that was exceeded.</p>
    LimitExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateCertificateAuthorityError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateCertificateAuthorityError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidArgsException" => {
                    return CreateCertificateAuthorityError::InvalidArgs(String::from(error_message))
                }
                "InvalidPolicyException" => {
                    return CreateCertificateAuthorityError::InvalidPolicy(String::from(
                        error_message,
                    ))
                }
                "LimitExceededException" => {
                    return CreateCertificateAuthorityError::LimitExceeded(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return CreateCertificateAuthorityError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return CreateCertificateAuthorityError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateCertificateAuthorityError {
    fn from(err: serde_json::error::Error) -> CreateCertificateAuthorityError {
        CreateCertificateAuthorityError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateCertificateAuthorityError {
    fn from(err: CredentialsError) -> CreateCertificateAuthorityError {
        CreateCertificateAuthorityError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateCertificateAuthorityError {
    fn from(err: HttpDispatchError) -> CreateCertificateAuthorityError {
        CreateCertificateAuthorityError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateCertificateAuthorityError {
    fn from(err: io::Error) -> CreateCertificateAuthorityError {
        CreateCertificateAuthorityError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateCertificateAuthorityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateCertificateAuthorityError {
    fn description(&self) -> &str {
        match *self {
            CreateCertificateAuthorityError::InvalidArgs(ref cause) => cause,
            CreateCertificateAuthorityError::InvalidPolicy(ref cause) => cause,
            CreateCertificateAuthorityError::LimitExceeded(ref cause) => cause,
            CreateCertificateAuthorityError::Validation(ref cause) => cause,
            CreateCertificateAuthorityError::Credentials(ref err) => err.description(),
            CreateCertificateAuthorityError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateCertificateAuthorityError::ParseError(ref cause) => cause,
            CreateCertificateAuthorityError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateCertificateAuthorityAuditReport
#[derive(Debug, PartialEq)]
pub enum CreateCertificateAuthorityAuditReportError {
    /// <p>One or more of the specified arguments was not valid.</p>
    InvalidArgs(String),
    /// <p>The requested Amazon Resource Name (ARN) does not refer to an existing resource.</p>
    InvalidArn(String),
    /// <p>The private CA is in a state during which a report cannot be generated.</p>
    InvalidState(String),
    /// <p>The request has failed for an unspecified reason.</p>
    RequestFailed(String),
    /// <p>Your request is already in progress.</p>
    RequestInProgress(String),
    /// <p>A resource such as a private CA, S3 bucket, certificate, or audit report cannot be found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateCertificateAuthorityAuditReportError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateCertificateAuthorityAuditReportError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidArgsException" => {
                    return CreateCertificateAuthorityAuditReportError::InvalidArgs(String::from(
                        error_message,
                    ))
                }
                "InvalidArnException" => {
                    return CreateCertificateAuthorityAuditReportError::InvalidArn(String::from(
                        error_message,
                    ))
                }
                "InvalidStateException" => {
                    return CreateCertificateAuthorityAuditReportError::InvalidState(String::from(
                        error_message,
                    ))
                }
                "RequestFailedException" => {
                    return CreateCertificateAuthorityAuditReportError::RequestFailed(String::from(
                        error_message,
                    ))
                }
                "RequestInProgressException" => {
                    return CreateCertificateAuthorityAuditReportError::RequestInProgress(
                        String::from(error_message),
                    )
                }
                "ResourceNotFoundException" => {
                    return CreateCertificateAuthorityAuditReportError::ResourceNotFound(
                        String::from(error_message),
                    )
                }
                "ValidationException" => {
                    return CreateCertificateAuthorityAuditReportError::Validation(
                        error_message.to_string(),
                    )
                }
                _ => {}
            }
        }
        return CreateCertificateAuthorityAuditReportError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateCertificateAuthorityAuditReportError {
    fn from(err: serde_json::error::Error) -> CreateCertificateAuthorityAuditReportError {
        CreateCertificateAuthorityAuditReportError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateCertificateAuthorityAuditReportError {
    fn from(err: CredentialsError) -> CreateCertificateAuthorityAuditReportError {
        CreateCertificateAuthorityAuditReportError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateCertificateAuthorityAuditReportError {
    fn from(err: HttpDispatchError) -> CreateCertificateAuthorityAuditReportError {
        CreateCertificateAuthorityAuditReportError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateCertificateAuthorityAuditReportError {
    fn from(err: io::Error) -> CreateCertificateAuthorityAuditReportError {
        CreateCertificateAuthorityAuditReportError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateCertificateAuthorityAuditReportError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateCertificateAuthorityAuditReportError {
    fn description(&self) -> &str {
        match *self {
            CreateCertificateAuthorityAuditReportError::InvalidArgs(ref cause) => cause,
            CreateCertificateAuthorityAuditReportError::InvalidArn(ref cause) => cause,
            CreateCertificateAuthorityAuditReportError::InvalidState(ref cause) => cause,
            CreateCertificateAuthorityAuditReportError::RequestFailed(ref cause) => cause,
            CreateCertificateAuthorityAuditReportError::RequestInProgress(ref cause) => cause,
            CreateCertificateAuthorityAuditReportError::ResourceNotFound(ref cause) => cause,
            CreateCertificateAuthorityAuditReportError::Validation(ref cause) => cause,
            CreateCertificateAuthorityAuditReportError::Credentials(ref err) => err.description(),
            CreateCertificateAuthorityAuditReportError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateCertificateAuthorityAuditReportError::ParseError(ref cause) => cause,
            CreateCertificateAuthorityAuditReportError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteCertificateAuthority
#[derive(Debug, PartialEq)]
pub enum DeleteCertificateAuthorityError {
    /// <p>A previous update to your private CA is still ongoing.</p>
    ConcurrentModification(String),
    /// <p>The requested Amazon Resource Name (ARN) does not refer to an existing resource.</p>
    InvalidArn(String),
    /// <p>The private CA is in a state during which a report cannot be generated.</p>
    InvalidState(String),
    /// <p>A resource such as a private CA, S3 bucket, certificate, or audit report cannot be found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteCertificateAuthorityError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteCertificateAuthorityError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ConcurrentModificationException" => {
                    return DeleteCertificateAuthorityError::ConcurrentModification(String::from(
                        error_message,
                    ))
                }
                "InvalidArnException" => {
                    return DeleteCertificateAuthorityError::InvalidArn(String::from(error_message))
                }
                "InvalidStateException" => {
                    return DeleteCertificateAuthorityError::InvalidState(String::from(
                        error_message,
                    ))
                }
                "ResourceNotFoundException" => {
                    return DeleteCertificateAuthorityError::ResourceNotFound(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return DeleteCertificateAuthorityError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteCertificateAuthorityError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteCertificateAuthorityError {
    fn from(err: serde_json::error::Error) -> DeleteCertificateAuthorityError {
        DeleteCertificateAuthorityError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteCertificateAuthorityError {
    fn from(err: CredentialsError) -> DeleteCertificateAuthorityError {
        DeleteCertificateAuthorityError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteCertificateAuthorityError {
    fn from(err: HttpDispatchError) -> DeleteCertificateAuthorityError {
        DeleteCertificateAuthorityError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteCertificateAuthorityError {
    fn from(err: io::Error) -> DeleteCertificateAuthorityError {
        DeleteCertificateAuthorityError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteCertificateAuthorityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteCertificateAuthorityError {
    fn description(&self) -> &str {
        match *self {
            DeleteCertificateAuthorityError::ConcurrentModification(ref cause) => cause,
            DeleteCertificateAuthorityError::InvalidArn(ref cause) => cause,
            DeleteCertificateAuthorityError::InvalidState(ref cause) => cause,
            DeleteCertificateAuthorityError::ResourceNotFound(ref cause) => cause,
            DeleteCertificateAuthorityError::Validation(ref cause) => cause,
            DeleteCertificateAuthorityError::Credentials(ref err) => err.description(),
            DeleteCertificateAuthorityError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteCertificateAuthorityError::ParseError(ref cause) => cause,
            DeleteCertificateAuthorityError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeCertificateAuthority
#[derive(Debug, PartialEq)]
pub enum DescribeCertificateAuthorityError {
    /// <p>The requested Amazon Resource Name (ARN) does not refer to an existing resource.</p>
    InvalidArn(String),
    /// <p>A resource such as a private CA, S3 bucket, certificate, or audit report cannot be found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DescribeCertificateAuthorityError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeCertificateAuthorityError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidArnException" => {
                    return DescribeCertificateAuthorityError::InvalidArn(String::from(
                        error_message,
                    ))
                }
                "ResourceNotFoundException" => {
                    return DescribeCertificateAuthorityError::ResourceNotFound(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return DescribeCertificateAuthorityError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DescribeCertificateAuthorityError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeCertificateAuthorityError {
    fn from(err: serde_json::error::Error) -> DescribeCertificateAuthorityError {
        DescribeCertificateAuthorityError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeCertificateAuthorityError {
    fn from(err: CredentialsError) -> DescribeCertificateAuthorityError {
        DescribeCertificateAuthorityError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeCertificateAuthorityError {
    fn from(err: HttpDispatchError) -> DescribeCertificateAuthorityError {
        DescribeCertificateAuthorityError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeCertificateAuthorityError {
    fn from(err: io::Error) -> DescribeCertificateAuthorityError {
        DescribeCertificateAuthorityError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeCertificateAuthorityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeCertificateAuthorityError {
    fn description(&self) -> &str {
        match *self {
            DescribeCertificateAuthorityError::InvalidArn(ref cause) => cause,
            DescribeCertificateAuthorityError::ResourceNotFound(ref cause) => cause,
            DescribeCertificateAuthorityError::Validation(ref cause) => cause,
            DescribeCertificateAuthorityError::Credentials(ref err) => err.description(),
            DescribeCertificateAuthorityError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeCertificateAuthorityError::ParseError(ref cause) => cause,
            DescribeCertificateAuthorityError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeCertificateAuthorityAuditReport
#[derive(Debug, PartialEq)]
pub enum DescribeCertificateAuthorityAuditReportError {
    /// <p>One or more of the specified arguments was not valid.</p>
    InvalidArgs(String),
    /// <p>The requested Amazon Resource Name (ARN) does not refer to an existing resource.</p>
    InvalidArn(String),
    /// <p>A resource such as a private CA, S3 bucket, certificate, or audit report cannot be found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DescribeCertificateAuthorityAuditReportError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> DescribeCertificateAuthorityAuditReportError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidArgsException" => {
                    return DescribeCertificateAuthorityAuditReportError::InvalidArgs(String::from(
                        error_message,
                    ))
                }
                "InvalidArnException" => {
                    return DescribeCertificateAuthorityAuditReportError::InvalidArn(String::from(
                        error_message,
                    ))
                }
                "ResourceNotFoundException" => {
                    return DescribeCertificateAuthorityAuditReportError::ResourceNotFound(
                        String::from(error_message),
                    )
                }
                "ValidationException" => {
                    return DescribeCertificateAuthorityAuditReportError::Validation(
                        error_message.to_string(),
                    )
                }
                _ => {}
            }
        }
        return DescribeCertificateAuthorityAuditReportError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeCertificateAuthorityAuditReportError {
    fn from(err: serde_json::error::Error) -> DescribeCertificateAuthorityAuditReportError {
        DescribeCertificateAuthorityAuditReportError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeCertificateAuthorityAuditReportError {
    fn from(err: CredentialsError) -> DescribeCertificateAuthorityAuditReportError {
        DescribeCertificateAuthorityAuditReportError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeCertificateAuthorityAuditReportError {
    fn from(err: HttpDispatchError) -> DescribeCertificateAuthorityAuditReportError {
        DescribeCertificateAuthorityAuditReportError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeCertificateAuthorityAuditReportError {
    fn from(err: io::Error) -> DescribeCertificateAuthorityAuditReportError {
        DescribeCertificateAuthorityAuditReportError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeCertificateAuthorityAuditReportError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeCertificateAuthorityAuditReportError {
    fn description(&self) -> &str {
        match *self {
            DescribeCertificateAuthorityAuditReportError::InvalidArgs(ref cause) => cause,
            DescribeCertificateAuthorityAuditReportError::InvalidArn(ref cause) => cause,
            DescribeCertificateAuthorityAuditReportError::ResourceNotFound(ref cause) => cause,
            DescribeCertificateAuthorityAuditReportError::Validation(ref cause) => cause,
            DescribeCertificateAuthorityAuditReportError::Credentials(ref err) => err.description(),
            DescribeCertificateAuthorityAuditReportError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeCertificateAuthorityAuditReportError::ParseError(ref cause) => cause,
            DescribeCertificateAuthorityAuditReportError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetCertificate
#[derive(Debug, PartialEq)]
pub enum GetCertificateError {
    /// <p>The requested Amazon Resource Name (ARN) does not refer to an existing resource.</p>
    InvalidArn(String),
    /// <p>The private CA is in a state during which a report cannot be generated.</p>
    InvalidState(String),
    /// <p>The request has failed for an unspecified reason.</p>
    RequestFailed(String),
    /// <p>Your request is already in progress.</p>
    RequestInProgress(String),
    /// <p>A resource such as a private CA, S3 bucket, certificate, or audit report cannot be found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetCertificateError {
    pub fn from_response(res: BufferedHttpResponse) -> GetCertificateError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidArnException" => {
                    return GetCertificateError::InvalidArn(String::from(error_message))
                }
                "InvalidStateException" => {
                    return GetCertificateError::InvalidState(String::from(error_message))
                }
                "RequestFailedException" => {
                    return GetCertificateError::RequestFailed(String::from(error_message))
                }
                "RequestInProgressException" => {
                    return GetCertificateError::RequestInProgress(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return GetCertificateError::ResourceNotFound(String::from(error_message))
                }
                "ValidationException" => {
                    return GetCertificateError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetCertificateError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetCertificateError {
    fn from(err: serde_json::error::Error) -> GetCertificateError {
        GetCertificateError::ParseError(err.description().to_string())
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
            GetCertificateError::InvalidState(ref cause) => cause,
            GetCertificateError::RequestFailed(ref cause) => cause,
            GetCertificateError::RequestInProgress(ref cause) => cause,
            GetCertificateError::ResourceNotFound(ref cause) => cause,
            GetCertificateError::Validation(ref cause) => cause,
            GetCertificateError::Credentials(ref err) => err.description(),
            GetCertificateError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetCertificateError::ParseError(ref cause) => cause,
            GetCertificateError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetCertificateAuthorityCertificate
#[derive(Debug, PartialEq)]
pub enum GetCertificateAuthorityCertificateError {
    /// <p>The requested Amazon Resource Name (ARN) does not refer to an existing resource.</p>
    InvalidArn(String),
    /// <p>The private CA is in a state during which a report cannot be generated.</p>
    InvalidState(String),
    /// <p>A resource such as a private CA, S3 bucket, certificate, or audit report cannot be found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetCertificateAuthorityCertificateError {
    pub fn from_response(res: BufferedHttpResponse) -> GetCertificateAuthorityCertificateError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidArnException" => {
                    return GetCertificateAuthorityCertificateError::InvalidArn(String::from(
                        error_message,
                    ))
                }
                "InvalidStateException" => {
                    return GetCertificateAuthorityCertificateError::InvalidState(String::from(
                        error_message,
                    ))
                }
                "ResourceNotFoundException" => {
                    return GetCertificateAuthorityCertificateError::ResourceNotFound(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return GetCertificateAuthorityCertificateError::Validation(
                        error_message.to_string(),
                    )
                }
                _ => {}
            }
        }
        return GetCertificateAuthorityCertificateError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetCertificateAuthorityCertificateError {
    fn from(err: serde_json::error::Error) -> GetCertificateAuthorityCertificateError {
        GetCertificateAuthorityCertificateError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetCertificateAuthorityCertificateError {
    fn from(err: CredentialsError) -> GetCertificateAuthorityCertificateError {
        GetCertificateAuthorityCertificateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetCertificateAuthorityCertificateError {
    fn from(err: HttpDispatchError) -> GetCertificateAuthorityCertificateError {
        GetCertificateAuthorityCertificateError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetCertificateAuthorityCertificateError {
    fn from(err: io::Error) -> GetCertificateAuthorityCertificateError {
        GetCertificateAuthorityCertificateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetCertificateAuthorityCertificateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetCertificateAuthorityCertificateError {
    fn description(&self) -> &str {
        match *self {
            GetCertificateAuthorityCertificateError::InvalidArn(ref cause) => cause,
            GetCertificateAuthorityCertificateError::InvalidState(ref cause) => cause,
            GetCertificateAuthorityCertificateError::ResourceNotFound(ref cause) => cause,
            GetCertificateAuthorityCertificateError::Validation(ref cause) => cause,
            GetCertificateAuthorityCertificateError::Credentials(ref err) => err.description(),
            GetCertificateAuthorityCertificateError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetCertificateAuthorityCertificateError::ParseError(ref cause) => cause,
            GetCertificateAuthorityCertificateError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetCertificateAuthorityCsr
#[derive(Debug, PartialEq)]
pub enum GetCertificateAuthorityCsrError {
    /// <p>The requested Amazon Resource Name (ARN) does not refer to an existing resource.</p>
    InvalidArn(String),
    /// <p>The private CA is in a state during which a report cannot be generated.</p>
    InvalidState(String),
    /// <p>The request has failed for an unspecified reason.</p>
    RequestFailed(String),
    /// <p>Your request is already in progress.</p>
    RequestInProgress(String),
    /// <p>A resource such as a private CA, S3 bucket, certificate, or audit report cannot be found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetCertificateAuthorityCsrError {
    pub fn from_response(res: BufferedHttpResponse) -> GetCertificateAuthorityCsrError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidArnException" => {
                    return GetCertificateAuthorityCsrError::InvalidArn(String::from(error_message))
                }
                "InvalidStateException" => {
                    return GetCertificateAuthorityCsrError::InvalidState(String::from(
                        error_message,
                    ))
                }
                "RequestFailedException" => {
                    return GetCertificateAuthorityCsrError::RequestFailed(String::from(
                        error_message,
                    ))
                }
                "RequestInProgressException" => {
                    return GetCertificateAuthorityCsrError::RequestInProgress(String::from(
                        error_message,
                    ))
                }
                "ResourceNotFoundException" => {
                    return GetCertificateAuthorityCsrError::ResourceNotFound(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return GetCertificateAuthorityCsrError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetCertificateAuthorityCsrError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetCertificateAuthorityCsrError {
    fn from(err: serde_json::error::Error) -> GetCertificateAuthorityCsrError {
        GetCertificateAuthorityCsrError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetCertificateAuthorityCsrError {
    fn from(err: CredentialsError) -> GetCertificateAuthorityCsrError {
        GetCertificateAuthorityCsrError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetCertificateAuthorityCsrError {
    fn from(err: HttpDispatchError) -> GetCertificateAuthorityCsrError {
        GetCertificateAuthorityCsrError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetCertificateAuthorityCsrError {
    fn from(err: io::Error) -> GetCertificateAuthorityCsrError {
        GetCertificateAuthorityCsrError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetCertificateAuthorityCsrError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetCertificateAuthorityCsrError {
    fn description(&self) -> &str {
        match *self {
            GetCertificateAuthorityCsrError::InvalidArn(ref cause) => cause,
            GetCertificateAuthorityCsrError::InvalidState(ref cause) => cause,
            GetCertificateAuthorityCsrError::RequestFailed(ref cause) => cause,
            GetCertificateAuthorityCsrError::RequestInProgress(ref cause) => cause,
            GetCertificateAuthorityCsrError::ResourceNotFound(ref cause) => cause,
            GetCertificateAuthorityCsrError::Validation(ref cause) => cause,
            GetCertificateAuthorityCsrError::Credentials(ref err) => err.description(),
            GetCertificateAuthorityCsrError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetCertificateAuthorityCsrError::ParseError(ref cause) => cause,
            GetCertificateAuthorityCsrError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ImportCertificateAuthorityCertificate
#[derive(Debug, PartialEq)]
pub enum ImportCertificateAuthorityCertificateError {
    /// <p>The certificate authority certificate you are importing does not comply with conditions specified in the certificate that signed it.</p>
    CertificateMismatch(String),
    /// <p>A previous update to your private CA is still ongoing.</p>
    ConcurrentModification(String),
    /// <p>The requested Amazon Resource Name (ARN) does not refer to an existing resource.</p>
    InvalidArn(String),
    /// <p>The private CA is in a state during which a report cannot be generated.</p>
    InvalidState(String),
    /// <p>One or more fields in the certificate are invalid.</p>
    MalformedCertificate(String),
    /// <p>The request has failed for an unspecified reason.</p>
    RequestFailed(String),
    /// <p>Your request is already in progress.</p>
    RequestInProgress(String),
    /// <p>A resource such as a private CA, S3 bucket, certificate, or audit report cannot be found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ImportCertificateAuthorityCertificateError {
    pub fn from_response(res: BufferedHttpResponse) -> ImportCertificateAuthorityCertificateError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "CertificateMismatchException" => {
                    return ImportCertificateAuthorityCertificateError::CertificateMismatch(
                        String::from(error_message),
                    )
                }
                "ConcurrentModificationException" => {
                    return ImportCertificateAuthorityCertificateError::ConcurrentModification(
                        String::from(error_message),
                    )
                }
                "InvalidArnException" => {
                    return ImportCertificateAuthorityCertificateError::InvalidArn(String::from(
                        error_message,
                    ))
                }
                "InvalidStateException" => {
                    return ImportCertificateAuthorityCertificateError::InvalidState(String::from(
                        error_message,
                    ))
                }
                "MalformedCertificateException" => {
                    return ImportCertificateAuthorityCertificateError::MalformedCertificate(
                        String::from(error_message),
                    )
                }
                "RequestFailedException" => {
                    return ImportCertificateAuthorityCertificateError::RequestFailed(String::from(
                        error_message,
                    ))
                }
                "RequestInProgressException" => {
                    return ImportCertificateAuthorityCertificateError::RequestInProgress(
                        String::from(error_message),
                    )
                }
                "ResourceNotFoundException" => {
                    return ImportCertificateAuthorityCertificateError::ResourceNotFound(
                        String::from(error_message),
                    )
                }
                "ValidationException" => {
                    return ImportCertificateAuthorityCertificateError::Validation(
                        error_message.to_string(),
                    )
                }
                _ => {}
            }
        }
        return ImportCertificateAuthorityCertificateError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ImportCertificateAuthorityCertificateError {
    fn from(err: serde_json::error::Error) -> ImportCertificateAuthorityCertificateError {
        ImportCertificateAuthorityCertificateError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ImportCertificateAuthorityCertificateError {
    fn from(err: CredentialsError) -> ImportCertificateAuthorityCertificateError {
        ImportCertificateAuthorityCertificateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ImportCertificateAuthorityCertificateError {
    fn from(err: HttpDispatchError) -> ImportCertificateAuthorityCertificateError {
        ImportCertificateAuthorityCertificateError::HttpDispatch(err)
    }
}
impl From<io::Error> for ImportCertificateAuthorityCertificateError {
    fn from(err: io::Error) -> ImportCertificateAuthorityCertificateError {
        ImportCertificateAuthorityCertificateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ImportCertificateAuthorityCertificateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ImportCertificateAuthorityCertificateError {
    fn description(&self) -> &str {
        match *self {
            ImportCertificateAuthorityCertificateError::CertificateMismatch(ref cause) => cause,
            ImportCertificateAuthorityCertificateError::ConcurrentModification(ref cause) => cause,
            ImportCertificateAuthorityCertificateError::InvalidArn(ref cause) => cause,
            ImportCertificateAuthorityCertificateError::InvalidState(ref cause) => cause,
            ImportCertificateAuthorityCertificateError::MalformedCertificate(ref cause) => cause,
            ImportCertificateAuthorityCertificateError::RequestFailed(ref cause) => cause,
            ImportCertificateAuthorityCertificateError::RequestInProgress(ref cause) => cause,
            ImportCertificateAuthorityCertificateError::ResourceNotFound(ref cause) => cause,
            ImportCertificateAuthorityCertificateError::Validation(ref cause) => cause,
            ImportCertificateAuthorityCertificateError::Credentials(ref err) => err.description(),
            ImportCertificateAuthorityCertificateError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ImportCertificateAuthorityCertificateError::ParseError(ref cause) => cause,
            ImportCertificateAuthorityCertificateError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by IssueCertificate
#[derive(Debug, PartialEq)]
pub enum IssueCertificateError {
    /// <p>One or more of the specified arguments was not valid.</p>
    InvalidArgs(String),
    /// <p>The requested Amazon Resource Name (ARN) does not refer to an existing resource.</p>
    InvalidArn(String),
    /// <p>The private CA is in a state during which a report cannot be generated.</p>
    InvalidState(String),
    /// <p>An ACM PCA limit has been exceeded. See the exception message returned to determine the limit that was exceeded.</p>
    LimitExceeded(String),
    /// <p>The certificate signing request is invalid.</p>
    MalformedCSR(String),
    /// <p>A resource such as a private CA, S3 bucket, certificate, or audit report cannot be found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl IssueCertificateError {
    pub fn from_response(res: BufferedHttpResponse) -> IssueCertificateError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidArgsException" => {
                    return IssueCertificateError::InvalidArgs(String::from(error_message))
                }
                "InvalidArnException" => {
                    return IssueCertificateError::InvalidArn(String::from(error_message))
                }
                "InvalidStateException" => {
                    return IssueCertificateError::InvalidState(String::from(error_message))
                }
                "LimitExceededException" => {
                    return IssueCertificateError::LimitExceeded(String::from(error_message))
                }
                "MalformedCSRException" => {
                    return IssueCertificateError::MalformedCSR(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return IssueCertificateError::ResourceNotFound(String::from(error_message))
                }
                "ValidationException" => {
                    return IssueCertificateError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return IssueCertificateError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for IssueCertificateError {
    fn from(err: serde_json::error::Error) -> IssueCertificateError {
        IssueCertificateError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for IssueCertificateError {
    fn from(err: CredentialsError) -> IssueCertificateError {
        IssueCertificateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for IssueCertificateError {
    fn from(err: HttpDispatchError) -> IssueCertificateError {
        IssueCertificateError::HttpDispatch(err)
    }
}
impl From<io::Error> for IssueCertificateError {
    fn from(err: io::Error) -> IssueCertificateError {
        IssueCertificateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for IssueCertificateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for IssueCertificateError {
    fn description(&self) -> &str {
        match *self {
            IssueCertificateError::InvalidArgs(ref cause) => cause,
            IssueCertificateError::InvalidArn(ref cause) => cause,
            IssueCertificateError::InvalidState(ref cause) => cause,
            IssueCertificateError::LimitExceeded(ref cause) => cause,
            IssueCertificateError::MalformedCSR(ref cause) => cause,
            IssueCertificateError::ResourceNotFound(ref cause) => cause,
            IssueCertificateError::Validation(ref cause) => cause,
            IssueCertificateError::Credentials(ref err) => err.description(),
            IssueCertificateError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            IssueCertificateError::ParseError(ref cause) => cause,
            IssueCertificateError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListCertificateAuthorities
#[derive(Debug, PartialEq)]
pub enum ListCertificateAuthoritiesError {
    /// <p>The token specified in the <code>NextToken</code> argument is not valid. Use the token returned from your previous call to <a>ListCertificateAuthorities</a>.</p>
    InvalidNextToken(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListCertificateAuthoritiesError {
    pub fn from_response(res: BufferedHttpResponse) -> ListCertificateAuthoritiesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidNextTokenException" => {
                    return ListCertificateAuthoritiesError::InvalidNextToken(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return ListCertificateAuthoritiesError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListCertificateAuthoritiesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListCertificateAuthoritiesError {
    fn from(err: serde_json::error::Error) -> ListCertificateAuthoritiesError {
        ListCertificateAuthoritiesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListCertificateAuthoritiesError {
    fn from(err: CredentialsError) -> ListCertificateAuthoritiesError {
        ListCertificateAuthoritiesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListCertificateAuthoritiesError {
    fn from(err: HttpDispatchError) -> ListCertificateAuthoritiesError {
        ListCertificateAuthoritiesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListCertificateAuthoritiesError {
    fn from(err: io::Error) -> ListCertificateAuthoritiesError {
        ListCertificateAuthoritiesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListCertificateAuthoritiesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListCertificateAuthoritiesError {
    fn description(&self) -> &str {
        match *self {
            ListCertificateAuthoritiesError::InvalidNextToken(ref cause) => cause,
            ListCertificateAuthoritiesError::Validation(ref cause) => cause,
            ListCertificateAuthoritiesError::Credentials(ref err) => err.description(),
            ListCertificateAuthoritiesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListCertificateAuthoritiesError::ParseError(ref cause) => cause,
            ListCertificateAuthoritiesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListTags
#[derive(Debug, PartialEq)]
pub enum ListTagsError {
    /// <p>The requested Amazon Resource Name (ARN) does not refer to an existing resource.</p>
    InvalidArn(String),
    /// <p>A resource such as a private CA, S3 bucket, certificate, or audit report cannot be found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> ListTagsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidArnException" => {
                    return ListTagsError::InvalidArn(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return ListTagsError::ResourceNotFound(String::from(error_message))
                }
                "ValidationException" => {
                    return ListTagsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListTagsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListTagsError {
    fn from(err: serde_json::error::Error) -> ListTagsError {
        ListTagsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListTagsError {
    fn from(err: CredentialsError) -> ListTagsError {
        ListTagsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListTagsError {
    fn from(err: HttpDispatchError) -> ListTagsError {
        ListTagsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListTagsError {
    fn from(err: io::Error) -> ListTagsError {
        ListTagsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListTagsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTagsError {
    fn description(&self) -> &str {
        match *self {
            ListTagsError::InvalidArn(ref cause) => cause,
            ListTagsError::ResourceNotFound(ref cause) => cause,
            ListTagsError::Validation(ref cause) => cause,
            ListTagsError::Credentials(ref err) => err.description(),
            ListTagsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListTagsError::ParseError(ref cause) => cause,
            ListTagsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by RestoreCertificateAuthority
#[derive(Debug, PartialEq)]
pub enum RestoreCertificateAuthorityError {
    /// <p>The requested Amazon Resource Name (ARN) does not refer to an existing resource.</p>
    InvalidArn(String),
    /// <p>The private CA is in a state during which a report cannot be generated.</p>
    InvalidState(String),
    /// <p>A resource such as a private CA, S3 bucket, certificate, or audit report cannot be found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl RestoreCertificateAuthorityError {
    pub fn from_response(res: BufferedHttpResponse) -> RestoreCertificateAuthorityError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidArnException" => {
                    return RestoreCertificateAuthorityError::InvalidArn(String::from(error_message))
                }
                "InvalidStateException" => {
                    return RestoreCertificateAuthorityError::InvalidState(String::from(
                        error_message,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RestoreCertificateAuthorityError::ResourceNotFound(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return RestoreCertificateAuthorityError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return RestoreCertificateAuthorityError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for RestoreCertificateAuthorityError {
    fn from(err: serde_json::error::Error) -> RestoreCertificateAuthorityError {
        RestoreCertificateAuthorityError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for RestoreCertificateAuthorityError {
    fn from(err: CredentialsError) -> RestoreCertificateAuthorityError {
        RestoreCertificateAuthorityError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RestoreCertificateAuthorityError {
    fn from(err: HttpDispatchError) -> RestoreCertificateAuthorityError {
        RestoreCertificateAuthorityError::HttpDispatch(err)
    }
}
impl From<io::Error> for RestoreCertificateAuthorityError {
    fn from(err: io::Error) -> RestoreCertificateAuthorityError {
        RestoreCertificateAuthorityError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RestoreCertificateAuthorityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RestoreCertificateAuthorityError {
    fn description(&self) -> &str {
        match *self {
            RestoreCertificateAuthorityError::InvalidArn(ref cause) => cause,
            RestoreCertificateAuthorityError::InvalidState(ref cause) => cause,
            RestoreCertificateAuthorityError::ResourceNotFound(ref cause) => cause,
            RestoreCertificateAuthorityError::Validation(ref cause) => cause,
            RestoreCertificateAuthorityError::Credentials(ref err) => err.description(),
            RestoreCertificateAuthorityError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RestoreCertificateAuthorityError::ParseError(ref cause) => cause,
            RestoreCertificateAuthorityError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by RevokeCertificate
#[derive(Debug, PartialEq)]
pub enum RevokeCertificateError {
    /// <p>A previous update to your private CA is still ongoing.</p>
    ConcurrentModification(String),
    /// <p>The requested Amazon Resource Name (ARN) does not refer to an existing resource.</p>
    InvalidArn(String),
    /// <p>The private CA is in a state during which a report cannot be generated.</p>
    InvalidState(String),
    /// <p>Your request has already been completed.</p>
    RequestAlreadyProcessed(String),
    /// <p>The request has failed for an unspecified reason.</p>
    RequestFailed(String),
    /// <p>Your request is already in progress.</p>
    RequestInProgress(String),
    /// <p>A resource such as a private CA, S3 bucket, certificate, or audit report cannot be found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl RevokeCertificateError {
    pub fn from_response(res: BufferedHttpResponse) -> RevokeCertificateError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ConcurrentModificationException" => {
                    return RevokeCertificateError::ConcurrentModification(String::from(
                        error_message,
                    ))
                }
                "InvalidArnException" => {
                    return RevokeCertificateError::InvalidArn(String::from(error_message))
                }
                "InvalidStateException" => {
                    return RevokeCertificateError::InvalidState(String::from(error_message))
                }
                "RequestAlreadyProcessedException" => {
                    return RevokeCertificateError::RequestAlreadyProcessed(String::from(
                        error_message,
                    ))
                }
                "RequestFailedException" => {
                    return RevokeCertificateError::RequestFailed(String::from(error_message))
                }
                "RequestInProgressException" => {
                    return RevokeCertificateError::RequestInProgress(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return RevokeCertificateError::ResourceNotFound(String::from(error_message))
                }
                "ValidationException" => {
                    return RevokeCertificateError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return RevokeCertificateError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for RevokeCertificateError {
    fn from(err: serde_json::error::Error) -> RevokeCertificateError {
        RevokeCertificateError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for RevokeCertificateError {
    fn from(err: CredentialsError) -> RevokeCertificateError {
        RevokeCertificateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RevokeCertificateError {
    fn from(err: HttpDispatchError) -> RevokeCertificateError {
        RevokeCertificateError::HttpDispatch(err)
    }
}
impl From<io::Error> for RevokeCertificateError {
    fn from(err: io::Error) -> RevokeCertificateError {
        RevokeCertificateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RevokeCertificateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RevokeCertificateError {
    fn description(&self) -> &str {
        match *self {
            RevokeCertificateError::ConcurrentModification(ref cause) => cause,
            RevokeCertificateError::InvalidArn(ref cause) => cause,
            RevokeCertificateError::InvalidState(ref cause) => cause,
            RevokeCertificateError::RequestAlreadyProcessed(ref cause) => cause,
            RevokeCertificateError::RequestFailed(ref cause) => cause,
            RevokeCertificateError::RequestInProgress(ref cause) => cause,
            RevokeCertificateError::ResourceNotFound(ref cause) => cause,
            RevokeCertificateError::Validation(ref cause) => cause,
            RevokeCertificateError::Credentials(ref err) => err.description(),
            RevokeCertificateError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RevokeCertificateError::ParseError(ref cause) => cause,
            RevokeCertificateError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by TagCertificateAuthority
#[derive(Debug, PartialEq)]
pub enum TagCertificateAuthorityError {
    /// <p>The requested Amazon Resource Name (ARN) does not refer to an existing resource.</p>
    InvalidArn(String),
    /// <p>The private CA is in a state during which a report cannot be generated.</p>
    InvalidState(String),
    /// <p>The tag associated with the CA is not valid. The invalid argument is contained in the message field.</p>
    InvalidTag(String),
    /// <p>A resource such as a private CA, S3 bucket, certificate, or audit report cannot be found.</p>
    ResourceNotFound(String),
    /// <p>You can associate up to 50 tags with a private CA. Exception information is contained in the exception message field.</p>
    TooManyTags(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl TagCertificateAuthorityError {
    pub fn from_response(res: BufferedHttpResponse) -> TagCertificateAuthorityError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidArnException" => {
                    return TagCertificateAuthorityError::InvalidArn(String::from(error_message))
                }
                "InvalidStateException" => {
                    return TagCertificateAuthorityError::InvalidState(String::from(error_message))
                }
                "InvalidTagException" => {
                    return TagCertificateAuthorityError::InvalidTag(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return TagCertificateAuthorityError::ResourceNotFound(String::from(
                        error_message,
                    ))
                }
                "TooManyTagsException" => {
                    return TagCertificateAuthorityError::TooManyTags(String::from(error_message))
                }
                "ValidationException" => {
                    return TagCertificateAuthorityError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return TagCertificateAuthorityError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for TagCertificateAuthorityError {
    fn from(err: serde_json::error::Error) -> TagCertificateAuthorityError {
        TagCertificateAuthorityError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for TagCertificateAuthorityError {
    fn from(err: CredentialsError) -> TagCertificateAuthorityError {
        TagCertificateAuthorityError::Credentials(err)
    }
}
impl From<HttpDispatchError> for TagCertificateAuthorityError {
    fn from(err: HttpDispatchError) -> TagCertificateAuthorityError {
        TagCertificateAuthorityError::HttpDispatch(err)
    }
}
impl From<io::Error> for TagCertificateAuthorityError {
    fn from(err: io::Error) -> TagCertificateAuthorityError {
        TagCertificateAuthorityError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for TagCertificateAuthorityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for TagCertificateAuthorityError {
    fn description(&self) -> &str {
        match *self {
            TagCertificateAuthorityError::InvalidArn(ref cause) => cause,
            TagCertificateAuthorityError::InvalidState(ref cause) => cause,
            TagCertificateAuthorityError::InvalidTag(ref cause) => cause,
            TagCertificateAuthorityError::ResourceNotFound(ref cause) => cause,
            TagCertificateAuthorityError::TooManyTags(ref cause) => cause,
            TagCertificateAuthorityError::Validation(ref cause) => cause,
            TagCertificateAuthorityError::Credentials(ref err) => err.description(),
            TagCertificateAuthorityError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            TagCertificateAuthorityError::ParseError(ref cause) => cause,
            TagCertificateAuthorityError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UntagCertificateAuthority
#[derive(Debug, PartialEq)]
pub enum UntagCertificateAuthorityError {
    /// <p>The requested Amazon Resource Name (ARN) does not refer to an existing resource.</p>
    InvalidArn(String),
    /// <p>The private CA is in a state during which a report cannot be generated.</p>
    InvalidState(String),
    /// <p>The tag associated with the CA is not valid. The invalid argument is contained in the message field.</p>
    InvalidTag(String),
    /// <p>A resource such as a private CA, S3 bucket, certificate, or audit report cannot be found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UntagCertificateAuthorityError {
    pub fn from_response(res: BufferedHttpResponse) -> UntagCertificateAuthorityError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidArnException" => {
                    return UntagCertificateAuthorityError::InvalidArn(String::from(error_message))
                }
                "InvalidStateException" => {
                    return UntagCertificateAuthorityError::InvalidState(String::from(error_message))
                }
                "InvalidTagException" => {
                    return UntagCertificateAuthorityError::InvalidTag(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return UntagCertificateAuthorityError::ResourceNotFound(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return UntagCertificateAuthorityError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return UntagCertificateAuthorityError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UntagCertificateAuthorityError {
    fn from(err: serde_json::error::Error) -> UntagCertificateAuthorityError {
        UntagCertificateAuthorityError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UntagCertificateAuthorityError {
    fn from(err: CredentialsError) -> UntagCertificateAuthorityError {
        UntagCertificateAuthorityError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UntagCertificateAuthorityError {
    fn from(err: HttpDispatchError) -> UntagCertificateAuthorityError {
        UntagCertificateAuthorityError::HttpDispatch(err)
    }
}
impl From<io::Error> for UntagCertificateAuthorityError {
    fn from(err: io::Error) -> UntagCertificateAuthorityError {
        UntagCertificateAuthorityError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UntagCertificateAuthorityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UntagCertificateAuthorityError {
    fn description(&self) -> &str {
        match *self {
            UntagCertificateAuthorityError::InvalidArn(ref cause) => cause,
            UntagCertificateAuthorityError::InvalidState(ref cause) => cause,
            UntagCertificateAuthorityError::InvalidTag(ref cause) => cause,
            UntagCertificateAuthorityError::ResourceNotFound(ref cause) => cause,
            UntagCertificateAuthorityError::Validation(ref cause) => cause,
            UntagCertificateAuthorityError::Credentials(ref err) => err.description(),
            UntagCertificateAuthorityError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UntagCertificateAuthorityError::ParseError(ref cause) => cause,
            UntagCertificateAuthorityError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateCertificateAuthority
#[derive(Debug, PartialEq)]
pub enum UpdateCertificateAuthorityError {
    /// <p>A previous update to your private CA is still ongoing.</p>
    ConcurrentModification(String),
    /// <p>One or more of the specified arguments was not valid.</p>
    InvalidArgs(String),
    /// <p>The requested Amazon Resource Name (ARN) does not refer to an existing resource.</p>
    InvalidArn(String),
    /// <p>The S3 bucket policy is not valid. The policy must give ACM PCA rights to read from and write to the bucket and find the bucket location.</p>
    InvalidPolicy(String),
    /// <p>The private CA is in a state during which a report cannot be generated.</p>
    InvalidState(String),
    /// <p>A resource such as a private CA, S3 bucket, certificate, or audit report cannot be found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UpdateCertificateAuthorityError {
    pub fn from_response(res: BufferedHttpResponse) -> UpdateCertificateAuthorityError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ConcurrentModificationException" => {
                    return UpdateCertificateAuthorityError::ConcurrentModification(String::from(
                        error_message,
                    ))
                }
                "InvalidArgsException" => {
                    return UpdateCertificateAuthorityError::InvalidArgs(String::from(error_message))
                }
                "InvalidArnException" => {
                    return UpdateCertificateAuthorityError::InvalidArn(String::from(error_message))
                }
                "InvalidPolicyException" => {
                    return UpdateCertificateAuthorityError::InvalidPolicy(String::from(
                        error_message,
                    ))
                }
                "InvalidStateException" => {
                    return UpdateCertificateAuthorityError::InvalidState(String::from(
                        error_message,
                    ))
                }
                "ResourceNotFoundException" => {
                    return UpdateCertificateAuthorityError::ResourceNotFound(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return UpdateCertificateAuthorityError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return UpdateCertificateAuthorityError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateCertificateAuthorityError {
    fn from(err: serde_json::error::Error) -> UpdateCertificateAuthorityError {
        UpdateCertificateAuthorityError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateCertificateAuthorityError {
    fn from(err: CredentialsError) -> UpdateCertificateAuthorityError {
        UpdateCertificateAuthorityError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateCertificateAuthorityError {
    fn from(err: HttpDispatchError) -> UpdateCertificateAuthorityError {
        UpdateCertificateAuthorityError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateCertificateAuthorityError {
    fn from(err: io::Error) -> UpdateCertificateAuthorityError {
        UpdateCertificateAuthorityError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateCertificateAuthorityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateCertificateAuthorityError {
    fn description(&self) -> &str {
        match *self {
            UpdateCertificateAuthorityError::ConcurrentModification(ref cause) => cause,
            UpdateCertificateAuthorityError::InvalidArgs(ref cause) => cause,
            UpdateCertificateAuthorityError::InvalidArn(ref cause) => cause,
            UpdateCertificateAuthorityError::InvalidPolicy(ref cause) => cause,
            UpdateCertificateAuthorityError::InvalidState(ref cause) => cause,
            UpdateCertificateAuthorityError::ResourceNotFound(ref cause) => cause,
            UpdateCertificateAuthorityError::Validation(ref cause) => cause,
            UpdateCertificateAuthorityError::Credentials(ref err) => err.description(),
            UpdateCertificateAuthorityError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateCertificateAuthorityError::ParseError(ref cause) => cause,
            UpdateCertificateAuthorityError::Unknown(_) => "unknown error",
        }
    }
}
/// Trait representing the capabilities of the ACM-PCA API. ACM-PCA clients implement this trait.
pub trait AcmPca {
    /// <p>Creates a private subordinate certificate authority (CA). You must specify the CA configuration, the revocation configuration, the CA type, and an optional idempotency token. The CA configuration specifies the name of the algorithm and key size to be used to create the CA private key, the type of signing algorithm that the CA uses to sign, and X.500 subject information. The CRL (certificate revocation list) configuration specifies the CRL expiration period in days (the validity period of the CRL), the Amazon S3 bucket that will contain the CRL, and a CNAME alias for the S3 bucket that is included in certificates issued by the CA. If successful, this operation returns the Amazon Resource Name (ARN) of the CA.</p>
    fn create_certificate_authority(
        &self,
        input: CreateCertificateAuthorityRequest,
    ) -> RusotoFuture<CreateCertificateAuthorityResponse, CreateCertificateAuthorityError>;

    /// <p>Creates an audit report that lists every time that the your CA private key is used. The report is saved in the Amazon S3 bucket that you specify on input. The <a>IssueCertificate</a> and <a>RevokeCertificate</a> operations use the private key. You can generate a new report every 30 minutes.</p>
    fn create_certificate_authority_audit_report(
        &self,
        input: CreateCertificateAuthorityAuditReportRequest,
    ) -> RusotoFuture<
        CreateCertificateAuthorityAuditReportResponse,
        CreateCertificateAuthorityAuditReportError,
    >;

    /// <p>Deletes a private certificate authority (CA). You must provide the ARN (Amazon Resource Name) of the private CA that you want to delete. You can find the ARN by calling the <a>ListCertificateAuthorities</a> operation. Before you can delete a CA, you must disable it. Call the <a>UpdateCertificateAuthority</a> operation and set the <b>CertificateAuthorityStatus</b> parameter to <code>DISABLED</code>. </p> <p>Additionally, you can delete a CA if you are waiting for it to be created (the <b>Status</b> field of the <a>CertificateAuthority</a> is <code>CREATING</code>). You can also delete it if the CA has been created but you haven't yet imported the signed certificate (the <b>Status</b> is <code>PENDING_CERTIFICATE</code>) into ACM PCA. </p> <p>If the CA is in one of the aforementioned states and you call <a>DeleteCertificateAuthority</a>, the CA's status changes to <code>DELETED</code>. However, the CA won't be permentantly deleted until the restoration period has passed. By default, if you do not set the <code>PermanentDeletionTimeInDays</code> parameter, the CA remains restorable for 30 days. You can set the parameter from 7 to 30 days. The <a>DescribeCertificateAuthority</a> operation returns the time remaining in the restoration window of a Private CA in the <code>DELETED</code> state. To restore an eligable CA, call the <a>RestoreCertificateAuthority</a> operation.</p>
    fn delete_certificate_authority(
        &self,
        input: DeleteCertificateAuthorityRequest,
    ) -> RusotoFuture<(), DeleteCertificateAuthorityError>;

    /// <p><p>Lists information about your private certificate authority (CA). You specify the private CA on input by its ARN (Amazon Resource Name). The output contains the status of your CA. This can be any of the following: </p> <ul> <li> <p> <code>CREATING</code> - ACM PCA is creating your private certificate authority.</p> </li> <li> <p> <code>PENDING_CERTIFICATE</code> - The certificate is pending. You must use your on-premises root or subordinate CA to sign your private CA CSR and then import it into PCA. </p> </li> <li> <p> <code>ACTIVE</code> - Your private CA is active.</p> </li> <li> <p> <code>DISABLED</code> - Your private CA has been disabled.</p> </li> <li> <p> <code>EXPIRED</code> - Your private CA certificate has expired.</p> </li> <li> <p> <code>FAILED</code> - Your private CA has failed. Your CA can fail because of problems such a network outage or backend AWS failure or other errors. A failed CA can never return to the pending state. You must create a new CA. </p> </li> <li> <p> <code>DELETED</code> - Your private CA is within the restoration period, after which it will be permanently deleted. The length of time remaining in the CA&#39;s restoration period will also be included in this operation&#39;s output.</p> </li> </ul></p>
    fn describe_certificate_authority(
        &self,
        input: DescribeCertificateAuthorityRequest,
    ) -> RusotoFuture<DescribeCertificateAuthorityResponse, DescribeCertificateAuthorityError>;

    /// <p>Lists information about a specific audit report created by calling the <a>CreateCertificateAuthorityAuditReport</a> operation. Audit information is created every time the certificate authority (CA) private key is used. The private key is used when you call the <a>IssueCertificate</a> operation or the <a>RevokeCertificate</a> operation. </p>
    fn describe_certificate_authority_audit_report(
        &self,
        input: DescribeCertificateAuthorityAuditReportRequest,
    ) -> RusotoFuture<
        DescribeCertificateAuthorityAuditReportResponse,
        DescribeCertificateAuthorityAuditReportError,
    >;

    /// <p>Retrieves a certificate from your private CA. The ARN of the certificate is returned when you call the <a>IssueCertificate</a> operation. You must specify both the ARN of your private CA and the ARN of the issued certificate when calling the <b>GetCertificate</b> operation. You can retrieve the certificate if it is in the <b>ISSUED</b> state. You can call the <a>CreateCertificateAuthorityAuditReport</a> operation to create a report that contains information about all of the certificates issued and revoked by your private CA. </p>
    fn get_certificate(
        &self,
        input: GetCertificateRequest,
    ) -> RusotoFuture<GetCertificateResponse, GetCertificateError>;

    /// <p>Retrieves the certificate and certificate chain for your private certificate authority (CA). Both the certificate and the chain are base64 PEM-encoded. The chain does not include the CA certificate. Each certificate in the chain signs the one before it. </p>
    fn get_certificate_authority_certificate(
        &self,
        input: GetCertificateAuthorityCertificateRequest,
    ) -> RusotoFuture<
        GetCertificateAuthorityCertificateResponse,
        GetCertificateAuthorityCertificateError,
    >;

    /// <p>Retrieves the certificate signing request (CSR) for your private certificate authority (CA). The CSR is created when you call the <a>CreateCertificateAuthority</a> operation. Take the CSR to your on-premises X.509 infrastructure and sign it by using your root or a subordinate CA. Then import the signed certificate back into ACM PCA by calling the <a>ImportCertificateAuthorityCertificate</a> operation. The CSR is returned as a base64 PEM-encoded string. </p>
    fn get_certificate_authority_csr(
        &self,
        input: GetCertificateAuthorityCsrRequest,
    ) -> RusotoFuture<GetCertificateAuthorityCsrResponse, GetCertificateAuthorityCsrError>;

    /// <p><p>Imports your signed private CA certificate into ACM PCA. Before you can call this operation, you must create the private certificate authority by calling the <a>CreateCertificateAuthority</a> operation. You must then generate a certificate signing request (CSR) by calling the <a>GetCertificateAuthorityCsr</a> operation. Take the CSR to your on-premises CA and use the root certificate or a subordinate certificate to sign it. Create a certificate chain and copy the signed certificate and the certificate chain to your working directory. </p> <note> <p>Your certificate chain must not include the private CA certificate that you are importing.</p> </note> <note> <p>Your on-premises CA certificate must be the last certificate in your chain. The subordinate certificate, if any, that your root CA signed must be next to last. The subordinate certificate signed by the preceding subordinate CA must come next, and so on until your chain is built. </p> </note> <note> <p>The chain must be PEM-encoded.</p> </note></p>
    fn import_certificate_authority_certificate(
        &self,
        input: ImportCertificateAuthorityCertificateRequest,
    ) -> RusotoFuture<(), ImportCertificateAuthorityCertificateError>;

    /// <p><p>Uses your private certificate authority (CA) to issue a client certificate. This operation returns the Amazon Resource Name (ARN) of the certificate. You can retrieve the certificate by calling the <a>GetCertificate</a> operation and specifying the ARN. </p> <note> <p>You cannot use the ACM <b>ListCertificateAuthorities</b> operation to retrieve the ARNs of the certificates that you issue by using ACM PCA.</p> </note></p>
    fn issue_certificate(
        &self,
        input: IssueCertificateRequest,
    ) -> RusotoFuture<IssueCertificateResponse, IssueCertificateError>;

    /// <p>Lists the private certificate authorities that you created by using the <a>CreateCertificateAuthority</a> operation.</p>
    fn list_certificate_authorities(
        &self,
        input: ListCertificateAuthoritiesRequest,
    ) -> RusotoFuture<ListCertificateAuthoritiesResponse, ListCertificateAuthoritiesError>;

    /// <p>Lists the tags, if any, that are associated with your private CA. Tags are labels that you can use to identify and organize your CAs. Each tag consists of a key and an optional value. Call the <a>TagCertificateAuthority</a> operation to add one or more tags to your CA. Call the <a>UntagCertificateAuthority</a> operation to remove tags. </p>
    fn list_tags(&self, input: ListTagsRequest) -> RusotoFuture<ListTagsResponse, ListTagsError>;

    /// <p>Restores a certificate authority (CA) that is in the <code>DELETED</code> state. You can restore a CA during the period that you defined in the <b>PermanentDeletionTimeInDays</b> parameter of the <a>DeleteCertificateAuthority</a> operation. Currently, you can specify 7 to 30 days. If you did not specify a <b>PermanentDeletionTimeInDays</b> value, by default you can restore the CA at any time in a 30 day period. You can check the time remaining in the restoration period of a private CA in the <code>DELETED</code> state by calling the <a>DescribeCertificateAuthority</a> or <a>ListCertificateAuthorities</a> operations. The status of a restored CA is set to its pre-deletion status when the <b>RestoreCertificateAuthority</b> operation returns. To change its status to <code>ACTIVE</code>, call the <a>UpdateCertificateAuthority</a> operation. If the private CA was in the <code>PENDING_CERTIFICATE</code> state at deletion, you must use the <a>ImportCertificateAuthorityCertificate</a> operation to import a certificate authority into the private CA before it can be activated. You cannot restore a CA after the restoration period has ended.</p>
    fn restore_certificate_authority(
        &self,
        input: RestoreCertificateAuthorityRequest,
    ) -> RusotoFuture<(), RestoreCertificateAuthorityError>;

    /// <p>Revokes a certificate that you issued by calling the <a>IssueCertificate</a> operation. If you enable a certificate revocation list (CRL) when you create or update your private CA, information about the revoked certificates will be included in the CRL. ACM PCA writes the CRL to an S3 bucket that you specify. For more information about revocation, see the <a>CrlConfiguration</a> structure. ACM PCA also writes revocation information to the audit report. For more information, see <a>CreateCertificateAuthorityAuditReport</a>. </p>
    fn revoke_certificate(
        &self,
        input: RevokeCertificateRequest,
    ) -> RusotoFuture<(), RevokeCertificateError>;

    /// <p>Adds one or more tags to your private CA. Tags are labels that you can use to identify and organize your AWS resources. Each tag consists of a key and an optional value. You specify the private CA on input by its Amazon Resource Name (ARN). You specify the tag by using a key-value pair. You can apply a tag to just one private CA if you want to identify a specific characteristic of that CA, or you can apply the same tag to multiple private CAs if you want to filter for a common relationship among those CAs. To remove one or more tags, use the <a>UntagCertificateAuthority</a> operation. Call the <a>ListTags</a> operation to see what tags are associated with your CA. </p>
    fn tag_certificate_authority(
        &self,
        input: TagCertificateAuthorityRequest,
    ) -> RusotoFuture<(), TagCertificateAuthorityError>;

    /// <p>Remove one or more tags from your private CA. A tag consists of a key-value pair. If you do not specify the value portion of the tag when calling this operation, the tag will be removed regardless of value. If you specify a value, the tag is removed only if it is associated with the specified value. To add tags to a private CA, use the <a>TagCertificateAuthority</a>. Call the <a>ListTags</a> operation to see what tags are associated with your CA. </p>
    fn untag_certificate_authority(
        &self,
        input: UntagCertificateAuthorityRequest,
    ) -> RusotoFuture<(), UntagCertificateAuthorityError>;

    /// <p>Updates the status or configuration of a private certificate authority (CA). Your private CA must be in the <code>ACTIVE</code> or <code>DISABLED</code> state before you can update it. You can disable a private CA that is in the <code>ACTIVE</code> state or make a CA that is in the <code>DISABLED</code> state active again.</p>
    fn update_certificate_authority(
        &self,
        input: UpdateCertificateAuthorityRequest,
    ) -> RusotoFuture<(), UpdateCertificateAuthorityError>;
}
/// A client for the ACM-PCA API.
pub struct AcmPcaClient {
    client: Client,
    region: region::Region,
}

impl AcmPcaClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> AcmPcaClient {
        AcmPcaClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> AcmPcaClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        AcmPcaClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl AcmPca for AcmPcaClient {
    /// <p>Creates a private subordinate certificate authority (CA). You must specify the CA configuration, the revocation configuration, the CA type, and an optional idempotency token. The CA configuration specifies the name of the algorithm and key size to be used to create the CA private key, the type of signing algorithm that the CA uses to sign, and X.500 subject information. The CRL (certificate revocation list) configuration specifies the CRL expiration period in days (the validity period of the CRL), the Amazon S3 bucket that will contain the CRL, and a CNAME alias for the S3 bucket that is included in certificates issued by the CA. If successful, this operation returns the Amazon Resource Name (ARN) of the CA.</p>
    fn create_certificate_authority(
        &self,
        input: CreateCertificateAuthorityRequest,
    ) -> RusotoFuture<CreateCertificateAuthorityResponse, CreateCertificateAuthorityError> {
        let mut request = SignedRequest::new("POST", "acm-pca", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ACMPrivateCA.CreateCertificateAuthority");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateCertificateAuthorityResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateCertificateAuthorityError::from_response(response))
                }))
            }
        })
    }

    /// <p>Creates an audit report that lists every time that the your CA private key is used. The report is saved in the Amazon S3 bucket that you specify on input. The <a>IssueCertificate</a> and <a>RevokeCertificate</a> operations use the private key. You can generate a new report every 30 minutes.</p>
    fn create_certificate_authority_audit_report(
        &self,
        input: CreateCertificateAuthorityAuditReportRequest,
    ) -> RusotoFuture<
        CreateCertificateAuthorityAuditReportResponse,
        CreateCertificateAuthorityAuditReportError,
    > {
        let mut request = SignedRequest::new("POST", "acm-pca", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "ACMPrivateCA.CreateCertificateAuthorityAuditReport",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateCertificateAuthorityAuditReportResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateCertificateAuthorityAuditReportError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Deletes a private certificate authority (CA). You must provide the ARN (Amazon Resource Name) of the private CA that you want to delete. You can find the ARN by calling the <a>ListCertificateAuthorities</a> operation. Before you can delete a CA, you must disable it. Call the <a>UpdateCertificateAuthority</a> operation and set the <b>CertificateAuthorityStatus</b> parameter to <code>DISABLED</code>. </p> <p>Additionally, you can delete a CA if you are waiting for it to be created (the <b>Status</b> field of the <a>CertificateAuthority</a> is <code>CREATING</code>). You can also delete it if the CA has been created but you haven't yet imported the signed certificate (the <b>Status</b> is <code>PENDING_CERTIFICATE</code>) into ACM PCA. </p> <p>If the CA is in one of the aforementioned states and you call <a>DeleteCertificateAuthority</a>, the CA's status changes to <code>DELETED</code>. However, the CA won't be permentantly deleted until the restoration period has passed. By default, if you do not set the <code>PermanentDeletionTimeInDays</code> parameter, the CA remains restorable for 30 days. You can set the parameter from 7 to 30 days. The <a>DescribeCertificateAuthority</a> operation returns the time remaining in the restoration window of a Private CA in the <code>DELETED</code> state. To restore an eligable CA, call the <a>RestoreCertificateAuthority</a> operation.</p>
    fn delete_certificate_authority(
        &self,
        input: DeleteCertificateAuthorityRequest,
    ) -> RusotoFuture<(), DeleteCertificateAuthorityError> {
        let mut request = SignedRequest::new("POST", "acm-pca", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ACMPrivateCA.DeleteCertificateAuthority");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteCertificateAuthorityError::from_response(response))
                }))
            }
        })
    }

    /// <p><p>Lists information about your private certificate authority (CA). You specify the private CA on input by its ARN (Amazon Resource Name). The output contains the status of your CA. This can be any of the following: </p> <ul> <li> <p> <code>CREATING</code> - ACM PCA is creating your private certificate authority.</p> </li> <li> <p> <code>PENDING_CERTIFICATE</code> - The certificate is pending. You must use your on-premises root or subordinate CA to sign your private CA CSR and then import it into PCA. </p> </li> <li> <p> <code>ACTIVE</code> - Your private CA is active.</p> </li> <li> <p> <code>DISABLED</code> - Your private CA has been disabled.</p> </li> <li> <p> <code>EXPIRED</code> - Your private CA certificate has expired.</p> </li> <li> <p> <code>FAILED</code> - Your private CA has failed. Your CA can fail because of problems such a network outage or backend AWS failure or other errors. A failed CA can never return to the pending state. You must create a new CA. </p> </li> <li> <p> <code>DELETED</code> - Your private CA is within the restoration period, after which it will be permanently deleted. The length of time remaining in the CA&#39;s restoration period will also be included in this operation&#39;s output.</p> </li> </ul></p>
    fn describe_certificate_authority(
        &self,
        input: DescribeCertificateAuthorityRequest,
    ) -> RusotoFuture<DescribeCertificateAuthorityResponse, DescribeCertificateAuthorityError> {
        let mut request = SignedRequest::new("POST", "acm-pca", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ACMPrivateCA.DescribeCertificateAuthority");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeCertificateAuthorityResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeCertificateAuthorityError::from_response(response))
                }))
            }
        })
    }

    /// <p>Lists information about a specific audit report created by calling the <a>CreateCertificateAuthorityAuditReport</a> operation. Audit information is created every time the certificate authority (CA) private key is used. The private key is used when you call the <a>IssueCertificate</a> operation or the <a>RevokeCertificate</a> operation. </p>
    fn describe_certificate_authority_audit_report(
        &self,
        input: DescribeCertificateAuthorityAuditReportRequest,
    ) -> RusotoFuture<
        DescribeCertificateAuthorityAuditReportResponse,
        DescribeCertificateAuthorityAuditReportError,
    > {
        let mut request = SignedRequest::new("POST", "acm-pca", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "ACMPrivateCA.DescribeCertificateAuthorityAuditReport",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeCertificateAuthorityAuditReportResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeCertificateAuthorityAuditReportError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Retrieves a certificate from your private CA. The ARN of the certificate is returned when you call the <a>IssueCertificate</a> operation. You must specify both the ARN of your private CA and the ARN of the issued certificate when calling the <b>GetCertificate</b> operation. You can retrieve the certificate if it is in the <b>ISSUED</b> state. You can call the <a>CreateCertificateAuthorityAuditReport</a> operation to create a report that contains information about all of the certificates issued and revoked by your private CA. </p>
    fn get_certificate(
        &self,
        input: GetCertificateRequest,
    ) -> RusotoFuture<GetCertificateResponse, GetCertificateError> {
        let mut request = SignedRequest::new("POST", "acm-pca", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ACMPrivateCA.GetCertificate");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetCertificateResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetCertificateError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves the certificate and certificate chain for your private certificate authority (CA). Both the certificate and the chain are base64 PEM-encoded. The chain does not include the CA certificate. Each certificate in the chain signs the one before it. </p>
    fn get_certificate_authority_certificate(
        &self,
        input: GetCertificateAuthorityCertificateRequest,
    ) -> RusotoFuture<
        GetCertificateAuthorityCertificateResponse,
        GetCertificateAuthorityCertificateError,
    > {
        let mut request = SignedRequest::new("POST", "acm-pca", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "ACMPrivateCA.GetCertificateAuthorityCertificate",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetCertificateAuthorityCertificateResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetCertificateAuthorityCertificateError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Retrieves the certificate signing request (CSR) for your private certificate authority (CA). The CSR is created when you call the <a>CreateCertificateAuthority</a> operation. Take the CSR to your on-premises X.509 infrastructure and sign it by using your root or a subordinate CA. Then import the signed certificate back into ACM PCA by calling the <a>ImportCertificateAuthorityCertificate</a> operation. The CSR is returned as a base64 PEM-encoded string. </p>
    fn get_certificate_authority_csr(
        &self,
        input: GetCertificateAuthorityCsrRequest,
    ) -> RusotoFuture<GetCertificateAuthorityCsrResponse, GetCertificateAuthorityCsrError> {
        let mut request = SignedRequest::new("POST", "acm-pca", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ACMPrivateCA.GetCertificateAuthorityCsr");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetCertificateAuthorityCsrResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetCertificateAuthorityCsrError::from_response(response))
                }))
            }
        })
    }

    /// <p><p>Imports your signed private CA certificate into ACM PCA. Before you can call this operation, you must create the private certificate authority by calling the <a>CreateCertificateAuthority</a> operation. You must then generate a certificate signing request (CSR) by calling the <a>GetCertificateAuthorityCsr</a> operation. Take the CSR to your on-premises CA and use the root certificate or a subordinate certificate to sign it. Create a certificate chain and copy the signed certificate and the certificate chain to your working directory. </p> <note> <p>Your certificate chain must not include the private CA certificate that you are importing.</p> </note> <note> <p>Your on-premises CA certificate must be the last certificate in your chain. The subordinate certificate, if any, that your root CA signed must be next to last. The subordinate certificate signed by the preceding subordinate CA must come next, and so on until your chain is built. </p> </note> <note> <p>The chain must be PEM-encoded.</p> </note></p>
    fn import_certificate_authority_certificate(
        &self,
        input: ImportCertificateAuthorityCertificateRequest,
    ) -> RusotoFuture<(), ImportCertificateAuthorityCertificateError> {
        let mut request = SignedRequest::new("POST", "acm-pca", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "ACMPrivateCA.ImportCertificateAuthorityCertificate",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ImportCertificateAuthorityCertificateError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p><p>Uses your private certificate authority (CA) to issue a client certificate. This operation returns the Amazon Resource Name (ARN) of the certificate. You can retrieve the certificate by calling the <a>GetCertificate</a> operation and specifying the ARN. </p> <note> <p>You cannot use the ACM <b>ListCertificateAuthorities</b> operation to retrieve the ARNs of the certificates that you issue by using ACM PCA.</p> </note></p>
    fn issue_certificate(
        &self,
        input: IssueCertificateRequest,
    ) -> RusotoFuture<IssueCertificateResponse, IssueCertificateError> {
        let mut request = SignedRequest::new("POST", "acm-pca", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ACMPrivateCA.IssueCertificate");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<IssueCertificateResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(IssueCertificateError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists the private certificate authorities that you created by using the <a>CreateCertificateAuthority</a> operation.</p>
    fn list_certificate_authorities(
        &self,
        input: ListCertificateAuthoritiesRequest,
    ) -> RusotoFuture<ListCertificateAuthoritiesResponse, ListCertificateAuthoritiesError> {
        let mut request = SignedRequest::new("POST", "acm-pca", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ACMPrivateCA.ListCertificateAuthorities");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListCertificateAuthoritiesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListCertificateAuthoritiesError::from_response(response))
                }))
            }
        })
    }

    /// <p>Lists the tags, if any, that are associated with your private CA. Tags are labels that you can use to identify and organize your CAs. Each tag consists of a key and an optional value. Call the <a>TagCertificateAuthority</a> operation to add one or more tags to your CA. Call the <a>UntagCertificateAuthority</a> operation to remove tags. </p>
    fn list_tags(&self, input: ListTagsRequest) -> RusotoFuture<ListTagsResponse, ListTagsError> {
        let mut request = SignedRequest::new("POST", "acm-pca", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ACMPrivateCA.ListTags");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListTagsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListTagsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Restores a certificate authority (CA) that is in the <code>DELETED</code> state. You can restore a CA during the period that you defined in the <b>PermanentDeletionTimeInDays</b> parameter of the <a>DeleteCertificateAuthority</a> operation. Currently, you can specify 7 to 30 days. If you did not specify a <b>PermanentDeletionTimeInDays</b> value, by default you can restore the CA at any time in a 30 day period. You can check the time remaining in the restoration period of a private CA in the <code>DELETED</code> state by calling the <a>DescribeCertificateAuthority</a> or <a>ListCertificateAuthorities</a> operations. The status of a restored CA is set to its pre-deletion status when the <b>RestoreCertificateAuthority</b> operation returns. To change its status to <code>ACTIVE</code>, call the <a>UpdateCertificateAuthority</a> operation. If the private CA was in the <code>PENDING_CERTIFICATE</code> state at deletion, you must use the <a>ImportCertificateAuthorityCertificate</a> operation to import a certificate authority into the private CA before it can be activated. You cannot restore a CA after the restoration period has ended.</p>
    fn restore_certificate_authority(
        &self,
        input: RestoreCertificateAuthorityRequest,
    ) -> RusotoFuture<(), RestoreCertificateAuthorityError> {
        let mut request = SignedRequest::new("POST", "acm-pca", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ACMPrivateCA.RestoreCertificateAuthority");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(RestoreCertificateAuthorityError::from_response(response))
                }))
            }
        })
    }

    /// <p>Revokes a certificate that you issued by calling the <a>IssueCertificate</a> operation. If you enable a certificate revocation list (CRL) when you create or update your private CA, information about the revoked certificates will be included in the CRL. ACM PCA writes the CRL to an S3 bucket that you specify. For more information about revocation, see the <a>CrlConfiguration</a> structure. ACM PCA also writes revocation information to the audit report. For more information, see <a>CreateCertificateAuthorityAuditReport</a>. </p>
    fn revoke_certificate(
        &self,
        input: RevokeCertificateRequest,
    ) -> RusotoFuture<(), RevokeCertificateError> {
        let mut request = SignedRequest::new("POST", "acm-pca", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ACMPrivateCA.RevokeCertificate");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(RevokeCertificateError::from_response(response))),
                )
            }
        })
    }

    /// <p>Adds one or more tags to your private CA. Tags are labels that you can use to identify and organize your AWS resources. Each tag consists of a key and an optional value. You specify the private CA on input by its Amazon Resource Name (ARN). You specify the tag by using a key-value pair. You can apply a tag to just one private CA if you want to identify a specific characteristic of that CA, or you can apply the same tag to multiple private CAs if you want to filter for a common relationship among those CAs. To remove one or more tags, use the <a>UntagCertificateAuthority</a> operation. Call the <a>ListTags</a> operation to see what tags are associated with your CA. </p>
    fn tag_certificate_authority(
        &self,
        input: TagCertificateAuthorityRequest,
    ) -> RusotoFuture<(), TagCertificateAuthorityError> {
        let mut request = SignedRequest::new("POST", "acm-pca", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ACMPrivateCA.TagCertificateAuthority");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(TagCertificateAuthorityError::from_response(response))
                }))
            }
        })
    }

    /// <p>Remove one or more tags from your private CA. A tag consists of a key-value pair. If you do not specify the value portion of the tag when calling this operation, the tag will be removed regardless of value. If you specify a value, the tag is removed only if it is associated with the specified value. To add tags to a private CA, use the <a>TagCertificateAuthority</a>. Call the <a>ListTags</a> operation to see what tags are associated with your CA. </p>
    fn untag_certificate_authority(
        &self,
        input: UntagCertificateAuthorityRequest,
    ) -> RusotoFuture<(), UntagCertificateAuthorityError> {
        let mut request = SignedRequest::new("POST", "acm-pca", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ACMPrivateCA.UntagCertificateAuthority");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UntagCertificateAuthorityError::from_response(response))
                }))
            }
        })
    }

    /// <p>Updates the status or configuration of a private certificate authority (CA). Your private CA must be in the <code>ACTIVE</code> or <code>DISABLED</code> state before you can update it. You can disable a private CA that is in the <code>ACTIVE</code> state or make a CA that is in the <code>DISABLED</code> state active again.</p>
    fn update_certificate_authority(
        &self,
        input: UpdateCertificateAuthorityRequest,
    ) -> RusotoFuture<(), UpdateCertificateAuthorityError> {
        let mut request = SignedRequest::new("POST", "acm-pca", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ACMPrivateCA.UpdateCertificateAuthority");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateCertificateAuthorityError::from_response(response))
                }))
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
