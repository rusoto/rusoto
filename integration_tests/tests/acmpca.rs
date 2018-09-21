#![cfg(feature = "acm-pca")]

extern crate rusoto_core;
extern crate rusoto_acm_pca;

use rusoto_acm_pca::{AcmPca, AcmPcaClient, ListCertificateAuthoritiesRequest};
use rusoto_core::Region;

#[test]
fn should_list_certificate_authoritiess() {
    let client = AcmPcaClient::new(Region::UsEast1);
    let request = ListCertificateAuthoritiesRequest::default();

    let res = client.list_certificate_authorities(request).sync().unwrap();
    println!("Got these certificate authorities: {:?}", res);
}
