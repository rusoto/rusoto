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
use rusoto_core::compression::CompressRequestPayload;
use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError, RusotoFuture};

use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
use serde_json;
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
#[cfg_attr(test, derive(Serialize))]
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

/// <p>Contains configuration information for your private certificate authority (CA). This includes information about the class of public key algorithm and the key pair that your private CA creates when it issues a certificate. It also includes the signature algorithm that it uses when issuing certificates, and its X.500 distinguished name. You must specify this information when you call the <a>CreateCertificateAuthority</a> operation. </p>
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
    /// <p>The format in which to create the report. This can be either <b>JSON</b> or <b>CSV</b>.</p>
    #[serde(rename = "AuditReportResponseFormat")]
    pub audit_report_response_format: String,
    /// <p>The Amazon Resource Name (ARN) of the CA to be audited. This is of the form:</p> <p> <code>arn:aws:acm-pca:<i>region</i>:<i>account</i>:certificate-authority/<i>12345678-1234-1234-1234-123456789012</i> </code>.</p>
    #[serde(rename = "CertificateAuthorityArn")]
    pub certificate_authority_arn: String,
    /// <p>The name of the S3 bucket that will contain the audit report.</p>
    #[serde(rename = "S3BucketName")]
    pub s3_bucket_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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
    /// <p>Key-value pairs that will be attached to the new private CA. You can associate up to 50 tags with a private CA.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateCertificateAuthorityResponse {
    /// <p>If successful, the Amazon Resource Name (ARN) of the certificate authority (CA). This is of the form: </p> <p> <code>arn:aws:acm-pca:<i>region</i>:<i>account</i>:certificate-authority/<i>12345678-1234-1234-1234-123456789012</i> </code>. </p>
    #[serde(rename = "CertificateAuthorityArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_authority_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreatePermissionRequest {
    /// <p>The actions that the specified AWS service principal can use. These include <code>IssueCertificate</code>, <code>GetCertificate</code>, and <code>ListPermissions</code>.</p>
    #[serde(rename = "Actions")]
    pub actions: Vec<String>,
    /// <p>The Amazon Resource Name (ARN) of the CA that grants the permissions. You can find the ARN by calling the <a>ListCertificateAuthorities</a> operation. This must have the following form: </p> <p> <code>arn:aws:acm-pca:<i>region</i>:<i>account</i>:certificate-authority/<i>12345678-1234-1234-1234-123456789012</i> </code>. </p>
    #[serde(rename = "CertificateAuthorityArn")]
    pub certificate_authority_arn: String,
    /// <p>The AWS service or identity that receives the permission. At this time, the only valid principal is <code>acm.amazonaws.com</code>.</p>
    #[serde(rename = "Principal")]
    pub principal: String,
    /// <p>The ID of the calling account.</p>
    #[serde(rename = "SourceAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_account: Option<String>,
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
pub struct DeletePermissionRequest {
    /// <p>The Amazon Resource Number (ARN) of the private CA that issued the permissions. You can find the CA's ARN by calling the <a>ListCertificateAuthorities</a> operation. This must have the following form: </p> <p> <code>arn:aws:acm-pca:<i>region</i>:<i>account</i>:certificate-authority/<i>12345678-1234-1234-1234-123456789012</i> </code>. </p>
    #[serde(rename = "CertificateAuthorityArn")]
    pub certificate_authority_arn: String,
    /// <p>The AWS service or identity that will have its CA permissions revoked. At this time, the only valid service principal is <code>acm.amazonaws.com</code> </p>
    #[serde(rename = "Principal")]
    pub principal: String,
    /// <p>The AWS account that calls this operation.</p>
    #[serde(rename = "SourceAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_account: Option<String>,
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
    pub certificate: bytes::Bytes,
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
    pub certificate_chain: bytes::Bytes,
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
    pub csr: bytes::Bytes,
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
pub struct ListPermissionsRequest {
    /// <p>The Amazon Resource Number (ARN) of the private CA to inspect. You can find the ARN by calling the <a>ListCertificateAuthorities</a> operation. This must be of the form: <code>arn:aws:acm-pca:region:account:certificate-authority/12345678-1234-1234-1234-123456789012</code> You can get a private CA's ARN by running the <a>ListCertificateAuthorities</a> operation.</p>
    #[serde(rename = "CertificateAuthorityArn")]
    pub certificate_authority_arn: String,
    /// <p>When paginating results, use this parameter to specify the maximum number of items to return in the response. If additional items exist beyond the number you specify, the <b>NextToken</b> element is sent in the response. Use this <b>NextToken</b> value in a subsequent request to retrieve additional items.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>When paginating results, use this parameter in a subsequent request after you receive a response with truncated results. Set it to the value of <b>NextToken</b> from the response you just received.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListPermissionsResponse {
    /// <p>When the list is truncated, this value is present and should be used for the <b>NextToken</b> parameter in a subsequent pagination request. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Summary information about each permission assigned by the specified private CA, including the action enabled, the policy provided, and the time of creation.</p>
    #[serde(rename = "Permissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<Permission>>,
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
#[cfg_attr(test, derive(Serialize))]
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

/// <p>Permissions designate which private CA operations can be performed by an AWS service or entity. In order for ACM to automatically renew private certificates, you must give the ACM service principal all available permissions (<code>IssueCertificate</code>, <code>GetCertificate</code>, and <code>ListPermissions</code>). Permissions can be assigned with the <a>CreatePermission</a> operation, removed with the <a>DeletePermission</a> operation, and listed with the <a>ListPermissions</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Permission {
    /// <p>The private CA operations that can be performed by the designated AWS service.</p>
    #[serde(rename = "Actions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<String>>,
    /// <p>The Amazon Resource Number (ARN) of the private CA from which the permission was issued.</p>
    #[serde(rename = "CertificateAuthorityArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_authority_arn: Option<String>,
    /// <p>The time at which the permission was created.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The name of the policy that is associated with the permission.</p>
    #[serde(rename = "Policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    /// <p>The AWS service or entity that holds the permission. At this time, the only valid principal is <code>acm.amazonaws.com</code>.</p>
    #[serde(rename = "Principal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal: Option<String>,
    /// <p>The ID of the account that assigned the permission.</p>
    #[serde(rename = "SourceAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_account: Option<String>,
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
    /// <p>The tag associated with the CA is not valid. The invalid argument is contained in the message field.</p>
    InvalidTag(String),
    /// <p>An ACM PCA limit has been exceeded. See the exception message returned to determine the limit that was exceeded.</p>
    LimitExceeded(String),
}

impl CreateCertificateAuthorityError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateCertificateAuthorityError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArgsException" => {
                    return RusotoError::Service(CreateCertificateAuthorityError::InvalidArgs(
                        err.msg,
                    ))
                }
                "InvalidPolicyException" => {
                    return RusotoError::Service(CreateCertificateAuthorityError::InvalidPolicy(
                        err.msg,
                    ))
                }
                "InvalidTagException" => {
                    return RusotoError::Service(CreateCertificateAuthorityError::InvalidTag(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateCertificateAuthorityError::LimitExceeded(
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
            CreateCertificateAuthorityError::InvalidTag(ref cause) => cause,
            CreateCertificateAuthorityError::LimitExceeded(ref cause) => cause,
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
    /// <p>The private CA is in a state during which a report or certificate cannot be generated.</p>
    InvalidState(String),
    /// <p>The request has failed for an unspecified reason.</p>
    RequestFailed(String),
    /// <p>Your request is already in progress.</p>
    RequestInProgress(String),
    /// <p>A resource such as a private CA, S3 bucket, certificate, or audit report cannot be found.</p>
    ResourceNotFound(String),
}

impl CreateCertificateAuthorityAuditReportError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateCertificateAuthorityAuditReportError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArgsException" => {
                    return RusotoError::Service(
                        CreateCertificateAuthorityAuditReportError::InvalidArgs(err.msg),
                    )
                }
                "InvalidArnException" => {
                    return RusotoError::Service(
                        CreateCertificateAuthorityAuditReportError::InvalidArn(err.msg),
                    )
                }
                "InvalidStateException" => {
                    return RusotoError::Service(
                        CreateCertificateAuthorityAuditReportError::InvalidState(err.msg),
                    )
                }
                "RequestFailedException" => {
                    return RusotoError::Service(
                        CreateCertificateAuthorityAuditReportError::RequestFailed(err.msg),
                    )
                }
                "RequestInProgressException" => {
                    return RusotoError::Service(
                        CreateCertificateAuthorityAuditReportError::RequestInProgress(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        CreateCertificateAuthorityAuditReportError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by CreatePermission
#[derive(Debug, PartialEq)]
pub enum CreatePermissionError {
    /// <p>The requested Amazon Resource Name (ARN) does not refer to an existing resource.</p>
    InvalidArn(String),
    /// <p>The private CA is in a state during which a report or certificate cannot be generated.</p>
    InvalidState(String),
    /// <p>An ACM PCA limit has been exceeded. See the exception message returned to determine the limit that was exceeded.</p>
    LimitExceeded(String),
    /// <p>The designated permission has already been given to the user.</p>
    PermissionAlreadyExists(String),
    /// <p>The request has failed for an unspecified reason.</p>
    RequestFailed(String),
    /// <p>A resource such as a private CA, S3 bucket, certificate, or audit report cannot be found.</p>
    ResourceNotFound(String),
}

impl CreatePermissionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreatePermissionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArnException" => {
                    return RusotoError::Service(CreatePermissionError::InvalidArn(err.msg))
                }
                "InvalidStateException" => {
                    return RusotoError::Service(CreatePermissionError::InvalidState(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreatePermissionError::LimitExceeded(err.msg))
                }
                "PermissionAlreadyExistsException" => {
                    return RusotoError::Service(CreatePermissionError::PermissionAlreadyExists(
                        err.msg,
                    ))
                }
                "RequestFailedException" => {
                    return RusotoError::Service(CreatePermissionError::RequestFailed(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreatePermissionError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreatePermissionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreatePermissionError {
    fn description(&self) -> &str {
        match *self {
            CreatePermissionError::InvalidArn(ref cause) => cause,
            CreatePermissionError::InvalidState(ref cause) => cause,
            CreatePermissionError::LimitExceeded(ref cause) => cause,
            CreatePermissionError::PermissionAlreadyExists(ref cause) => cause,
            CreatePermissionError::RequestFailed(ref cause) => cause,
            CreatePermissionError::ResourceNotFound(ref cause) => cause,
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
    /// <p>The private CA is in a state during which a report or certificate cannot be generated.</p>
    InvalidState(String),
    /// <p>A resource such as a private CA, S3 bucket, certificate, or audit report cannot be found.</p>
    ResourceNotFound(String),
}

impl DeleteCertificateAuthorityError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteCertificateAuthorityError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        DeleteCertificateAuthorityError::ConcurrentModification(err.msg),
                    )
                }
                "InvalidArnException" => {
                    return RusotoError::Service(DeleteCertificateAuthorityError::InvalidArn(
                        err.msg,
                    ))
                }
                "InvalidStateException" => {
                    return RusotoError::Service(DeleteCertificateAuthorityError::InvalidState(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteCertificateAuthorityError::ResourceNotFound(
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
        }
    }
}
/// Errors returned by DeletePermission
#[derive(Debug, PartialEq)]
pub enum DeletePermissionError {
    /// <p>The requested Amazon Resource Name (ARN) does not refer to an existing resource.</p>
    InvalidArn(String),
    /// <p>The private CA is in a state during which a report or certificate cannot be generated.</p>
    InvalidState(String),
    /// <p>The request has failed for an unspecified reason.</p>
    RequestFailed(String),
    /// <p>A resource such as a private CA, S3 bucket, certificate, or audit report cannot be found.</p>
    ResourceNotFound(String),
}

impl DeletePermissionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeletePermissionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArnException" => {
                    return RusotoError::Service(DeletePermissionError::InvalidArn(err.msg))
                }
                "InvalidStateException" => {
                    return RusotoError::Service(DeletePermissionError::InvalidState(err.msg))
                }
                "RequestFailedException" => {
                    return RusotoError::Service(DeletePermissionError::RequestFailed(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeletePermissionError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeletePermissionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeletePermissionError {
    fn description(&self) -> &str {
        match *self {
            DeletePermissionError::InvalidArn(ref cause) => cause,
            DeletePermissionError::InvalidState(ref cause) => cause,
            DeletePermissionError::RequestFailed(ref cause) => cause,
            DeletePermissionError::ResourceNotFound(ref cause) => cause,
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
}

impl DescribeCertificateAuthorityError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeCertificateAuthorityError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArnException" => {
                    return RusotoError::Service(DescribeCertificateAuthorityError::InvalidArn(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DescribeCertificateAuthorityError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl DescribeCertificateAuthorityAuditReportError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeCertificateAuthorityAuditReportError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArgsException" => {
                    return RusotoError::Service(
                        DescribeCertificateAuthorityAuditReportError::InvalidArgs(err.msg),
                    )
                }
                "InvalidArnException" => {
                    return RusotoError::Service(
                        DescribeCertificateAuthorityAuditReportError::InvalidArn(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DescribeCertificateAuthorityAuditReportError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by GetCertificate
#[derive(Debug, PartialEq)]
pub enum GetCertificateError {
    /// <p>The requested Amazon Resource Name (ARN) does not refer to an existing resource.</p>
    InvalidArn(String),
    /// <p>The private CA is in a state during which a report or certificate cannot be generated.</p>
    InvalidState(String),
    /// <p>The request has failed for an unspecified reason.</p>
    RequestFailed(String),
    /// <p>Your request is already in progress.</p>
    RequestInProgress(String),
    /// <p>A resource such as a private CA, S3 bucket, certificate, or audit report cannot be found.</p>
    ResourceNotFound(String),
}

impl GetCertificateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetCertificateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArnException" => {
                    return RusotoError::Service(GetCertificateError::InvalidArn(err.msg))
                }
                "InvalidStateException" => {
                    return RusotoError::Service(GetCertificateError::InvalidState(err.msg))
                }
                "RequestFailedException" => {
                    return RusotoError::Service(GetCertificateError::RequestFailed(err.msg))
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
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by GetCertificateAuthorityCertificate
#[derive(Debug, PartialEq)]
pub enum GetCertificateAuthorityCertificateError {
    /// <p>The requested Amazon Resource Name (ARN) does not refer to an existing resource.</p>
    InvalidArn(String),
    /// <p>The private CA is in a state during which a report or certificate cannot be generated.</p>
    InvalidState(String),
    /// <p>A resource such as a private CA, S3 bucket, certificate, or audit report cannot be found.</p>
    ResourceNotFound(String),
}

impl GetCertificateAuthorityCertificateError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetCertificateAuthorityCertificateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArnException" => {
                    return RusotoError::Service(
                        GetCertificateAuthorityCertificateError::InvalidArn(err.msg),
                    )
                }
                "InvalidStateException" => {
                    return RusotoError::Service(
                        GetCertificateAuthorityCertificateError::InvalidState(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        GetCertificateAuthorityCertificateError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by GetCertificateAuthorityCsr
#[derive(Debug, PartialEq)]
pub enum GetCertificateAuthorityCsrError {
    /// <p>The requested Amazon Resource Name (ARN) does not refer to an existing resource.</p>
    InvalidArn(String),
    /// <p>The private CA is in a state during which a report or certificate cannot be generated.</p>
    InvalidState(String),
    /// <p>The request has failed for an unspecified reason.</p>
    RequestFailed(String),
    /// <p>Your request is already in progress.</p>
    RequestInProgress(String),
    /// <p>A resource such as a private CA, S3 bucket, certificate, or audit report cannot be found.</p>
    ResourceNotFound(String),
}

impl GetCertificateAuthorityCsrError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetCertificateAuthorityCsrError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArnException" => {
                    return RusotoError::Service(GetCertificateAuthorityCsrError::InvalidArn(
                        err.msg,
                    ))
                }
                "InvalidStateException" => {
                    return RusotoError::Service(GetCertificateAuthorityCsrError::InvalidState(
                        err.msg,
                    ))
                }
                "RequestFailedException" => {
                    return RusotoError::Service(GetCertificateAuthorityCsrError::RequestFailed(
                        err.msg,
                    ))
                }
                "RequestInProgressException" => {
                    return RusotoError::Service(
                        GetCertificateAuthorityCsrError::RequestInProgress(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetCertificateAuthorityCsrError::ResourceNotFound(
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
    /// <p>The private CA is in a state during which a report or certificate cannot be generated.</p>
    InvalidState(String),
    /// <p>One or more fields in the certificate are invalid.</p>
    MalformedCertificate(String),
    /// <p>The request has failed for an unspecified reason.</p>
    RequestFailed(String),
    /// <p>Your request is already in progress.</p>
    RequestInProgress(String),
    /// <p>A resource such as a private CA, S3 bucket, certificate, or audit report cannot be found.</p>
    ResourceNotFound(String),
}

impl ImportCertificateAuthorityCertificateError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ImportCertificateAuthorityCertificateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CertificateMismatchException" => {
                    return RusotoError::Service(
                        ImportCertificateAuthorityCertificateError::CertificateMismatch(err.msg),
                    )
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        ImportCertificateAuthorityCertificateError::ConcurrentModification(err.msg),
                    )
                }
                "InvalidArnException" => {
                    return RusotoError::Service(
                        ImportCertificateAuthorityCertificateError::InvalidArn(err.msg),
                    )
                }
                "InvalidStateException" => {
                    return RusotoError::Service(
                        ImportCertificateAuthorityCertificateError::InvalidState(err.msg),
                    )
                }
                "MalformedCertificateException" => {
                    return RusotoError::Service(
                        ImportCertificateAuthorityCertificateError::MalformedCertificate(err.msg),
                    )
                }
                "RequestFailedException" => {
                    return RusotoError::Service(
                        ImportCertificateAuthorityCertificateError::RequestFailed(err.msg),
                    )
                }
                "RequestInProgressException" => {
                    return RusotoError::Service(
                        ImportCertificateAuthorityCertificateError::RequestInProgress(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        ImportCertificateAuthorityCertificateError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
    /// <p>The private CA is in a state during which a report or certificate cannot be generated.</p>
    InvalidState(String),
    /// <p>An ACM PCA limit has been exceeded. See the exception message returned to determine the limit that was exceeded.</p>
    LimitExceeded(String),
    /// <p>The certificate signing request is invalid.</p>
    MalformedCSR(String),
    /// <p>A resource such as a private CA, S3 bucket, certificate, or audit report cannot be found.</p>
    ResourceNotFound(String),
}

impl IssueCertificateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<IssueCertificateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArgsException" => {
                    return RusotoError::Service(IssueCertificateError::InvalidArgs(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(IssueCertificateError::InvalidArn(err.msg))
                }
                "InvalidStateException" => {
                    return RusotoError::Service(IssueCertificateError::InvalidState(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(IssueCertificateError::LimitExceeded(err.msg))
                }
                "MalformedCSRException" => {
                    return RusotoError::Service(IssueCertificateError::MalformedCSR(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(IssueCertificateError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by ListCertificateAuthorities
#[derive(Debug, PartialEq)]
pub enum ListCertificateAuthoritiesError {
    /// <p>The token specified in the <code>NextToken</code> argument is not valid. Use the token returned from your previous call to <a>ListCertificateAuthorities</a>.</p>
    InvalidNextToken(String),
}

impl ListCertificateAuthoritiesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListCertificateAuthoritiesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListCertificateAuthoritiesError::InvalidNextToken(
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
impl fmt::Display for ListCertificateAuthoritiesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListCertificateAuthoritiesError {
    fn description(&self) -> &str {
        match *self {
            ListCertificateAuthoritiesError::InvalidNextToken(ref cause) => cause,
        }
    }
}
/// Errors returned by ListPermissions
#[derive(Debug, PartialEq)]
pub enum ListPermissionsError {
    /// <p>The requested Amazon Resource Name (ARN) does not refer to an existing resource.</p>
    InvalidArn(String),
    /// <p>The token specified in the <code>NextToken</code> argument is not valid. Use the token returned from your previous call to <a>ListCertificateAuthorities</a>.</p>
    InvalidNextToken(String),
    /// <p>The private CA is in a state during which a report or certificate cannot be generated.</p>
    InvalidState(String),
    /// <p>The request has failed for an unspecified reason.</p>
    RequestFailed(String),
    /// <p>A resource such as a private CA, S3 bucket, certificate, or audit report cannot be found.</p>
    ResourceNotFound(String),
}

impl ListPermissionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListPermissionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArnException" => {
                    return RusotoError::Service(ListPermissionsError::InvalidArn(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListPermissionsError::InvalidNextToken(err.msg))
                }
                "InvalidStateException" => {
                    return RusotoError::Service(ListPermissionsError::InvalidState(err.msg))
                }
                "RequestFailedException" => {
                    return RusotoError::Service(ListPermissionsError::RequestFailed(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListPermissionsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListPermissionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListPermissionsError {
    fn description(&self) -> &str {
        match *self {
            ListPermissionsError::InvalidArn(ref cause) => cause,
            ListPermissionsError::InvalidNextToken(ref cause) => cause,
            ListPermissionsError::InvalidState(ref cause) => cause,
            ListPermissionsError::RequestFailed(ref cause) => cause,
            ListPermissionsError::ResourceNotFound(ref cause) => cause,
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
}

impl ListTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArnException" => {
                    return RusotoError::Service(ListTagsError::InvalidArn(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListTagsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by RestoreCertificateAuthority
#[derive(Debug, PartialEq)]
pub enum RestoreCertificateAuthorityError {
    /// <p>The requested Amazon Resource Name (ARN) does not refer to an existing resource.</p>
    InvalidArn(String),
    /// <p>The private CA is in a state during which a report or certificate cannot be generated.</p>
    InvalidState(String),
    /// <p>A resource such as a private CA, S3 bucket, certificate, or audit report cannot be found.</p>
    ResourceNotFound(String),
}

impl RestoreCertificateAuthorityError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<RestoreCertificateAuthorityError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArnException" => {
                    return RusotoError::Service(RestoreCertificateAuthorityError::InvalidArn(
                        err.msg,
                    ))
                }
                "InvalidStateException" => {
                    return RusotoError::Service(RestoreCertificateAuthorityError::InvalidState(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        RestoreCertificateAuthorityError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
    /// <p>The private CA is in a state during which a report or certificate cannot be generated.</p>
    InvalidState(String),
    /// <p>An ACM PCA limit has been exceeded. See the exception message returned to determine the limit that was exceeded.</p>
    LimitExceeded(String),
    /// <p>Your request has already been completed.</p>
    RequestAlreadyProcessed(String),
    /// <p>The request has failed for an unspecified reason.</p>
    RequestFailed(String),
    /// <p>Your request is already in progress.</p>
    RequestInProgress(String),
    /// <p>A resource such as a private CA, S3 bucket, certificate, or audit report cannot be found.</p>
    ResourceNotFound(String),
}

impl RevokeCertificateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RevokeCertificateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(RevokeCertificateError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(RevokeCertificateError::InvalidArn(err.msg))
                }
                "InvalidStateException" => {
                    return RusotoError::Service(RevokeCertificateError::InvalidState(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(RevokeCertificateError::LimitExceeded(err.msg))
                }
                "RequestAlreadyProcessedException" => {
                    return RusotoError::Service(RevokeCertificateError::RequestAlreadyProcessed(
                        err.msg,
                    ))
                }
                "RequestFailedException" => {
                    return RusotoError::Service(RevokeCertificateError::RequestFailed(err.msg))
                }
                "RequestInProgressException" => {
                    return RusotoError::Service(RevokeCertificateError::RequestInProgress(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(RevokeCertificateError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            RevokeCertificateError::LimitExceeded(ref cause) => cause,
            RevokeCertificateError::RequestAlreadyProcessed(ref cause) => cause,
            RevokeCertificateError::RequestFailed(ref cause) => cause,
            RevokeCertificateError::RequestInProgress(ref cause) => cause,
            RevokeCertificateError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by TagCertificateAuthority
#[derive(Debug, PartialEq)]
pub enum TagCertificateAuthorityError {
    /// <p>The requested Amazon Resource Name (ARN) does not refer to an existing resource.</p>
    InvalidArn(String),
    /// <p>The private CA is in a state during which a report or certificate cannot be generated.</p>
    InvalidState(String),
    /// <p>The tag associated with the CA is not valid. The invalid argument is contained in the message field.</p>
    InvalidTag(String),
    /// <p>A resource such as a private CA, S3 bucket, certificate, or audit report cannot be found.</p>
    ResourceNotFound(String),
    /// <p>You can associate up to 50 tags with a private CA. Exception information is contained in the exception message field.</p>
    TooManyTags(String),
}

impl TagCertificateAuthorityError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagCertificateAuthorityError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArnException" => {
                    return RusotoError::Service(TagCertificateAuthorityError::InvalidArn(err.msg))
                }
                "InvalidStateException" => {
                    return RusotoError::Service(TagCertificateAuthorityError::InvalidState(
                        err.msg,
                    ))
                }
                "InvalidTagException" => {
                    return RusotoError::Service(TagCertificateAuthorityError::InvalidTag(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(TagCertificateAuthorityError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "TooManyTagsException" => {
                    return RusotoError::Service(TagCertificateAuthorityError::TooManyTags(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by UntagCertificateAuthority
#[derive(Debug, PartialEq)]
pub enum UntagCertificateAuthorityError {
    /// <p>The requested Amazon Resource Name (ARN) does not refer to an existing resource.</p>
    InvalidArn(String),
    /// <p>The private CA is in a state during which a report or certificate cannot be generated.</p>
    InvalidState(String),
    /// <p>The tag associated with the CA is not valid. The invalid argument is contained in the message field.</p>
    InvalidTag(String),
    /// <p>A resource such as a private CA, S3 bucket, certificate, or audit report cannot be found.</p>
    ResourceNotFound(String),
}

impl UntagCertificateAuthorityError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagCertificateAuthorityError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArnException" => {
                    return RusotoError::Service(UntagCertificateAuthorityError::InvalidArn(
                        err.msg,
                    ))
                }
                "InvalidStateException" => {
                    return RusotoError::Service(UntagCertificateAuthorityError::InvalidState(
                        err.msg,
                    ))
                }
                "InvalidTagException" => {
                    return RusotoError::Service(UntagCertificateAuthorityError::InvalidTag(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UntagCertificateAuthorityError::ResourceNotFound(
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
    /// <p>The private CA is in a state during which a report or certificate cannot be generated.</p>
    InvalidState(String),
    /// <p>A resource such as a private CA, S3 bucket, certificate, or audit report cannot be found.</p>
    ResourceNotFound(String),
}

impl UpdateCertificateAuthorityError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateCertificateAuthorityError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        UpdateCertificateAuthorityError::ConcurrentModification(err.msg),
                    )
                }
                "InvalidArgsException" => {
                    return RusotoError::Service(UpdateCertificateAuthorityError::InvalidArgs(
                        err.msg,
                    ))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(UpdateCertificateAuthorityError::InvalidArn(
                        err.msg,
                    ))
                }
                "InvalidPolicyException" => {
                    return RusotoError::Service(UpdateCertificateAuthorityError::InvalidPolicy(
                        err.msg,
                    ))
                }
                "InvalidStateException" => {
                    return RusotoError::Service(UpdateCertificateAuthorityError::InvalidState(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateCertificateAuthorityError::ResourceNotFound(
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

    /// <p>Creates an audit report that lists every time that your CA private key is used. The report is saved in the Amazon S3 bucket that you specify on input. The <a>IssueCertificate</a> and <a>RevokeCertificate</a> operations use the private key. You can generate a new report every 30 minutes.</p>
    fn create_certificate_authority_audit_report(
        &self,
        input: CreateCertificateAuthorityAuditReportRequest,
    ) -> RusotoFuture<
        CreateCertificateAuthorityAuditReportResponse,
        CreateCertificateAuthorityAuditReportError,
    >;

    /// <p>Assigns permissions from a private CA to a designated AWS service. Services are specified by their service principals and can be given permission to create and retrieve certificates on a private CA. Services can also be given permission to list the active permissions that the private CA has granted. For ACM to automatically renew your private CA's certificates, you must assign all possible permissions from the CA to the ACM service principal.</p> <p>At this time, you can only assign permissions to ACM (<code>acm.amazonaws.com</code>). Permissions can be revoked with the <a>DeletePermission</a> operation and listed with the <a>ListPermissions</a> operation.</p>
    fn create_permission(
        &self,
        input: CreatePermissionRequest,
    ) -> RusotoFuture<(), CreatePermissionError>;

    /// <p>Deletes a private certificate authority (CA). You must provide the ARN (Amazon Resource Name) of the private CA that you want to delete. You can find the ARN by calling the <a>ListCertificateAuthorities</a> operation. Before you can delete a CA, you must disable it. Call the <a>UpdateCertificateAuthority</a> operation and set the <b>CertificateAuthorityStatus</b> parameter to <code>DISABLED</code>. </p> <p>Additionally, you can delete a CA if you are waiting for it to be created (the <b>Status</b> field of the <a>CertificateAuthority</a> is <code>CREATING</code>). You can also delete it if the CA has been created but you haven't yet imported the signed certificate (the <b>Status</b> is <code>PENDING_CERTIFICATE</code>) into ACM PCA. </p> <p>If the CA is in one of the previously mentioned states and you call <a>DeleteCertificateAuthority</a>, the CA's status changes to <code>DELETED</code>. However, the CA won't be permanently deleted until the restoration period has passed. By default, if you do not set the <code>PermanentDeletionTimeInDays</code> parameter, the CA remains restorable for 30 days. You can set the parameter from 7 to 30 days. The <a>DescribeCertificateAuthority</a> operation returns the time remaining in the restoration window of a Private CA in the <code>DELETED</code> state. To restore an eligible CA, call the <a>RestoreCertificateAuthority</a> operation.</p>
    fn delete_certificate_authority(
        &self,
        input: DeleteCertificateAuthorityRequest,
    ) -> RusotoFuture<(), DeleteCertificateAuthorityError>;

    /// <p>Revokes permissions that a private CA assigned to a designated AWS service. Permissions can be created with the <a>CreatePermission</a> operation and listed with the <a>ListPermissions</a> operation. </p>
    fn delete_permission(
        &self,
        input: DeletePermissionRequest,
    ) -> RusotoFuture<(), DeletePermissionError>;

    /// <p><p>Lists information about your private certificate authority (CA). You specify the private CA on input by its ARN (Amazon Resource Name). The output contains the status of your CA. This can be any of the following: </p> <ul> <li> <p> <code>CREATING</code> - ACM PCA is creating your private certificate authority.</p> </li> <li> <p> <code>PENDING_CERTIFICATE</code> - The certificate is pending. You must use your on-premises root or subordinate CA to sign your private CA CSR and then import it into PCA. </p> </li> <li> <p> <code>ACTIVE</code> - Your private CA is active.</p> </li> <li> <p> <code>DISABLED</code> - Your private CA has been disabled.</p> </li> <li> <p> <code>EXPIRED</code> - Your private CA certificate has expired.</p> </li> <li> <p> <code>FAILED</code> - Your private CA has failed. Your CA can fail because of problems such a network outage or backend AWS failure or other errors. A failed CA can never return to the pending state. You must create a new CA. </p> </li> <li> <p> <code>DELETED</code> - Your private CA is within the restoration period, after which it is permanently deleted. The length of time remaining in the CA&#39;s restoration period is also included in this operation&#39;s output.</p> </li> </ul></p>
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

    /// <p>Lists all the permissions, if any, that have been assigned by a private CA. Permissions can be granted with the <a>CreatePermission</a> operation and revoked with the <a>DeletePermission</a> operation.</p>
    fn list_permissions(
        &self,
        input: ListPermissionsRequest,
    ) -> RusotoFuture<ListPermissionsResponse, ListPermissionsError>;

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
#[derive(Clone)]
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
            region,
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
        D: DispatchSignedRequest + CompressRequestPayload + Send + Sync + 'static,
        D::Future: Send,
    {
        AcmPcaClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateCertificateAuthorityResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateCertificateAuthorityError::from_response(response))
                }))
            }
        })
    }

    /// <p>Creates an audit report that lists every time that your CA private key is used. The report is saved in the Amazon S3 bucket that you specify on input. The <a>IssueCertificate</a> and <a>RevokeCertificate</a> operations use the private key. You can generate a new report every 30 minutes.</p>
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateCertificateAuthorityAuditReportResponse, _>()
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

    /// <p>Assigns permissions from a private CA to a designated AWS service. Services are specified by their service principals and can be given permission to create and retrieve certificates on a private CA. Services can also be given permission to list the active permissions that the private CA has granted. For ACM to automatically renew your private CA's certificates, you must assign all possible permissions from the CA to the ACM service principal.</p> <p>At this time, you can only assign permissions to ACM (<code>acm.amazonaws.com</code>). Permissions can be revoked with the <a>DeletePermission</a> operation and listed with the <a>ListPermissions</a> operation.</p>
    fn create_permission(
        &self,
        input: CreatePermissionRequest,
    ) -> RusotoFuture<(), CreatePermissionError> {
        let mut request = SignedRequest::new("POST", "acm-pca", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ACMPrivateCA.CreatePermission");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreatePermissionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a private certificate authority (CA). You must provide the ARN (Amazon Resource Name) of the private CA that you want to delete. You can find the ARN by calling the <a>ListCertificateAuthorities</a> operation. Before you can delete a CA, you must disable it. Call the <a>UpdateCertificateAuthority</a> operation and set the <b>CertificateAuthorityStatus</b> parameter to <code>DISABLED</code>. </p> <p>Additionally, you can delete a CA if you are waiting for it to be created (the <b>Status</b> field of the <a>CertificateAuthority</a> is <code>CREATING</code>). You can also delete it if the CA has been created but you haven't yet imported the signed certificate (the <b>Status</b> is <code>PENDING_CERTIFICATE</code>) into ACM PCA. </p> <p>If the CA is in one of the previously mentioned states and you call <a>DeleteCertificateAuthority</a>, the CA's status changes to <code>DELETED</code>. However, the CA won't be permanently deleted until the restoration period has passed. By default, if you do not set the <code>PermanentDeletionTimeInDays</code> parameter, the CA remains restorable for 30 days. You can set the parameter from 7 to 30 days. The <a>DescribeCertificateAuthority</a> operation returns the time remaining in the restoration window of a Private CA in the <code>DELETED</code> state. To restore an eligible CA, call the <a>RestoreCertificateAuthority</a> operation.</p>
    fn delete_certificate_authority(
        &self,
        input: DeleteCertificateAuthorityRequest,
    ) -> RusotoFuture<(), DeleteCertificateAuthorityError> {
        let mut request = SignedRequest::new("POST", "acm-pca", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ACMPrivateCA.DeleteCertificateAuthority");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

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

    /// <p>Revokes permissions that a private CA assigned to a designated AWS service. Permissions can be created with the <a>CreatePermission</a> operation and listed with the <a>ListPermissions</a> operation. </p>
    fn delete_permission(
        &self,
        input: DeletePermissionRequest,
    ) -> RusotoFuture<(), DeletePermissionError> {
        let mut request = SignedRequest::new("POST", "acm-pca", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ACMPrivateCA.DeletePermission");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeletePermissionError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Lists information about your private certificate authority (CA). You specify the private CA on input by its ARN (Amazon Resource Name). The output contains the status of your CA. This can be any of the following: </p> <ul> <li> <p> <code>CREATING</code> - ACM PCA is creating your private certificate authority.</p> </li> <li> <p> <code>PENDING_CERTIFICATE</code> - The certificate is pending. You must use your on-premises root or subordinate CA to sign your private CA CSR and then import it into PCA. </p> </li> <li> <p> <code>ACTIVE</code> - Your private CA is active.</p> </li> <li> <p> <code>DISABLED</code> - Your private CA has been disabled.</p> </li> <li> <p> <code>EXPIRED</code> - Your private CA certificate has expired.</p> </li> <li> <p> <code>FAILED</code> - Your private CA has failed. Your CA can fail because of problems such a network outage or backend AWS failure or other errors. A failed CA can never return to the pending state. You must create a new CA. </p> </li> <li> <p> <code>DELETED</code> - Your private CA is within the restoration period, after which it is permanently deleted. The length of time remaining in the CA&#39;s restoration period is also included in this operation&#39;s output.</p> </li> </ul></p>
    fn describe_certificate_authority(
        &self,
        input: DescribeCertificateAuthorityRequest,
    ) -> RusotoFuture<DescribeCertificateAuthorityResponse, DescribeCertificateAuthorityError> {
        let mut request = SignedRequest::new("POST", "acm-pca", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ACMPrivateCA.DescribeCertificateAuthority");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeCertificateAuthorityResponse, _>()
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeCertificateAuthorityAuditReportResponse, _>()
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetCertificateResponse, _>()
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetCertificateAuthorityCertificateResponse, _>()
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetCertificateAuthorityCsrResponse, _>()
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
        request.set_payload(Some(encoded));

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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<IssueCertificateResponse, _>()
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListCertificateAuthoritiesResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListCertificateAuthoritiesError::from_response(response))
                }))
            }
        })
    }

    /// <p>Lists all the permissions, if any, that have been assigned by a private CA. Permissions can be granted with the <a>CreatePermission</a> operation and revoked with the <a>DeletePermission</a> operation.</p>
    fn list_permissions(
        &self,
        input: ListPermissionsRequest,
    ) -> RusotoFuture<ListPermissionsResponse, ListPermissionsError> {
        let mut request = SignedRequest::new("POST", "acm-pca", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ACMPrivateCA.ListPermissions");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListPermissionsResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListPermissionsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists the tags, if any, that are associated with your private CA. Tags are labels that you can use to identify and organize your CAs. Each tag consists of a key and an optional value. Call the <a>TagCertificateAuthority</a> operation to add one or more tags to your CA. Call the <a>UntagCertificateAuthority</a> operation to remove tags. </p>
    fn list_tags(&self, input: ListTagsRequest) -> RusotoFuture<ListTagsResponse, ListTagsError> {
        let mut request = SignedRequest::new("POST", "acm-pca", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ACMPrivateCA.ListTags");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListTagsResponse, _>()
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
        request.set_payload(Some(encoded));

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
        request.set_payload(Some(encoded));

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
        request.set_payload(Some(encoded));

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
        request.set_payload(Some(encoded));

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
        request.set_payload(Some(encoded));

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
