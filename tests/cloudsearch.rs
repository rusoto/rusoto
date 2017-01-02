#![cfg(feature = "cloudsearch")]

extern crate rusoto;

use rusoto::cloudsearch::{CloudSearchClient, DescribeDomainsRequest};
use rusoto::{DefaultCredentialsProvider, Region};

#[test]
fn should_describe_domains() {
    let client = CloudSearchClient::new(DefaultCredentialsProvider::new().unwrap(), Region::UsEast1);
    let request = DescribeDomainsRequest::default();

 	let response = client.describe_domains(&request).unwrap();
    println!("{:#?}", response);
}

