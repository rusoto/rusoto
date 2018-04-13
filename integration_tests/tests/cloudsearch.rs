#![cfg(feature = "cloudsearch")]

extern crate rusoto_core;
extern crate rusoto_cloudsearch;

use rusoto_cloudsearch::{CloudSearch, CloudSearchClient, DescribeDomainsRequest};
use rusoto_core::Region;


#[test]
fn should_describe_domains() {
    let client = CloudSearchClient::simple(Region::UsEast1);
    let request = DescribeDomainsRequest::default();

    let response = client.describe_domains(request).sync().unwrap();
    println!("{:#?}", response);
}
