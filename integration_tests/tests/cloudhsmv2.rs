#![cfg(feature = "cloudhsmv2")]

extern crate rusoto_cloudhsmv2;
extern crate rusoto_core;

use rusoto_cloudhsmv2::{CloudHsmv2, CloudHsmv2Client, DescribeClustersRequest};
use rusoto_core::Region;

#[tokio::test]
async fn should_describe_clusters() {
    let client = CloudHsmv2Client::new(Region::UsEast1);
    let request = DescribeClustersRequest::default();

    let response = client.describe_clusters(request).await.unwrap();
    println!("response is {:?}", response);
}
