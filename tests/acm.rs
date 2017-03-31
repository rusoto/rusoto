#![cfg(feature = "acm")]

extern crate rusoto;

use rusoto::acm::{AcmClient, ListCertificatesRequest};
use rusoto::{DefaultCredentialsProvider, Region};
use rusoto::default_tls_client;

#[test]
fn should_list_certificates() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = AcmClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListCertificatesRequest::default();

    client.list_certificates(&request).unwrap();
}
