#![cfg(feature = "cloudsearch")]

extern crate rusoto_core;
extern crate rusoto_cloudsearch;

use rusoto_cloudsearch::{CloudSearch, CloudSearchClient, DescribeDomainsRequest};
use rusoto_core::{DefaultCredentialsProvider, Region};
use rusoto_core::default_tls_client;

#[test]
fn should_describe_domains() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client =
        CloudSearchClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = DescribeDomainsRequest::default();

    let response = client.describe_domains(&request).unwrap();
    println!("{:#?}", response);
}
