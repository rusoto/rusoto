#![cfg(feature = "cloudsearch")]

extern crate rusoto_cloudsearch;
extern crate rusoto_core;

use rusoto_cloudsearch::{CloudSearch, CloudSearchClient, DescribeDomainsRequest};
use rusoto_core::Region;

#[tokio::test]
async fn should_describe_domains() {
    let client = CloudSearchClient::new(Region::UsEast1);
    let request = DescribeDomainsRequest::default();

    let response = client.describe_domains(request).await.unwrap();
    println!("{:#?}", response);
}
