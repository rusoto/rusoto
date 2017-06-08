#![cfg(feature = "acm")]

extern crate rusoto_core;
extern crate rusoto_acm;

use rusoto_acm::{Acm, AcmClient, ListCertificatesRequest};
use rusoto_core::{DefaultCredentialsProvider, Region};
use rusoto_core::default_tls_client;

#[test]
fn should_list_certificates() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = AcmClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListCertificatesRequest::default();

    client.list_certificates(&request).unwrap();
}
