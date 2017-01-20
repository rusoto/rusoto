#![cfg(feature = "cloudsearch")]

extern crate rusoto;

use rusoto::cloudsearch::{CloudSearchClient, DescribeDomainsRequest};
use rusoto::{DefaultCredentialsProvider, Region};
use rusoto::default_tls_client;

#[test]
fn should_describe_domains() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = CloudSearchClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = DescribeDomainsRequest::default();

 	let response = client.describe_domains(&request).unwrap();
    println!("{:#?}", response);
}

