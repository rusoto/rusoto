#![cfg(feature = "opsworkscm")]

extern crate rusoto_core;
extern crate rusoto_opsworkscm;

use rusoto_core::Region;
use rusoto_opsworkscm::{DescribeServersRequest, OpsWorksCM, OpsWorksCMClient};

#[tokio::test]
async fn should_describe_servers() {
    let client = OpsWorksCMClient::new(Region::UsEast1);
    let request = DescribeServersRequest::default();

    let result = client.describe_servers(request).await.unwrap();
    println!("{:#?}", result);
}
