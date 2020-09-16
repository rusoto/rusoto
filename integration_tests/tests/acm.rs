#![cfg(feature = "acm")]

extern crate rusoto_acm;
extern crate rusoto_core;

use rusoto_acm::{Acm, AcmClient, ListCertificatesRequest};
use rusoto_core::Region;

#[tokio::test]
async fn should_list_certificates() {
    let client = AcmClient::new(Region::UsEast1);
    let request = ListCertificatesRequest::default();

    client.list_certificates(request).await.unwrap();
}
