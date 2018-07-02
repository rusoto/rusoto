#![cfg(feature = "cloudhsmv2")]

extern crate rusoto_core;
extern crate rusoto_cloudhsmv2;

use rusoto_cloudhsmv2::{CloudHsmv2, CloudHsmv2Client, DescribeClustersRequest};
use rusoto_core::Region;

#[test]
fn should_describe_clusters() {
    let client = CloudHsmv2Client::new(Region::UsEast1);
    let request = DescribeClustersRequest::default();

    let response = client.describe_clusters(request).sync().unwrap();
    println!("response is {:?}", response);
}

