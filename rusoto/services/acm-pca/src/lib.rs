
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

#![doc(html_logo_url = "https://raw.githubusercontent.com/rusoto/rusoto/master/assets/logo-square.png")]
//! <p><p>You can use the ACM PCA API to create a private certificate authority (CA). You must first call the <a>CreateCertificateAuthority</a> operation. If successful, the operation returns an Amazon Resource Name (ARN) for your private CA. Use this ARN as input to the <a>GetCertificateAuthorityCsr</a> operation to retrieve the certificate signing request (CSR) for your private CA certificate. Sign the CSR using the root or an intermediate CA in your on-premises PKI hierarchy, and call the <a>ImportCertificateAuthorityCertificate</a> to import your signed private CA certificate into ACM PCA. </p> <p>Use your private CA to issue and revoke certificates. These are private certificates that identify and secure client computers, servers, applications, services, devices, and users over SSLS/TLS connections within your organization. Call the <a>IssueCertificate</a> operation to issue a certificate. Call the <a>RevokeCertificate</a> operation to revoke a certificate. </p> <note> <p>Certificates issued by your private CA can be trusted only within your organization, not publicly.</p> </note> <p>Your private CA can optionally create a certificate revocation list (CRL) to track the certificates you revoke. To create a CRL, you must specify a <a>RevocationConfiguration</a> object when you call the <a>CreateCertificateAuthority</a> operation. ACM PCA writes the CRL to an S3 bucket that you specify. You must specify a bucket policy that grants ACM PCA write permission. </p> <p>You can also call the <a>CreateCertificateAuthorityAuditReport</a> to create an optional audit report, which enumerates all of the issued, valid, expired, and revoked certificates from the CA.</p> <note> <p>Each ACM PCA API operation has a throttling limit which determines the number of times the operation can be called per second. For more information, see <a href="acm-pca/latest/userguide/PcaLimits.html#PcaLimits-api">API Rate Limits in ACM PCA</a> in the ACM PCA user guide.</p> </note></p>
//!
//! If you're using the service, you're probably looking for [AcmPcaClient](struct.AcmPcaClient.html) and [AcmPca](trait.AcmPca.html).

extern crate futures;
extern crate rusoto_core;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod generated;
mod custom;

pub use generated::*;
pub use custom::*;
            
