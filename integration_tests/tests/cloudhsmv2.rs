#![cfg(feature = "cloudhsmv2")]

extern crate rusoto_core;
extern crate rusoto_cloudhsmv2;

use rusoto_cloudhsmv2::{CloudHsmv2, CloudHsmv2Client, DescribeClustersRequest};
use rusoto_core::{DefaultCredentialsProvider, Region};
use rusoto_core::default_tls_client;

#[test]
fn should_describe_clusters() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = CloudHsmv2Client::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = DescribeClustersRequest::default();

    let response = client.describe_clusters(&request).unwrap();
    println!("response is {:?}", response);
}

