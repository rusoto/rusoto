#![cfg(feature = "acm-pca")]

extern crate rusoto_acm_pca;
extern crate rusoto_core;

use rusoto_acm_pca::{AcmPca, AcmPcaClient, ListCertificateAuthoritiesRequest};
use rusoto_core::Region;

#[tokio::test]
async fn should_list_certificate_authoritiess() {
    let client = AcmPcaClient::new(Region::UsEast1);
    let request = ListCertificateAuthoritiesRequest::default();

    let res = client.list_certificate_authorities(request).await.unwrap();
    println!("Got these certificate authorities: {:?}", res);
}
