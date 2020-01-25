#![cfg(feature = "efs")]

extern crate rusoto_core;
extern crate rusoto_efs;

use rusoto_core::Region;
use rusoto_efs::{DescribeFileSystemsRequest, Efs, EfsClient};

#[tokio::test]
async fn should_describe_filesystems() {
    let client = EfsClient::new(Region::UsEast1);
    let request = DescribeFileSystemsRequest::default();

    let result = client.describe_file_systems(request).await.unwrap();
    println!("{:#?}", result);
}
