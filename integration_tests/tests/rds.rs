#![cfg(feature = "rds")]
extern crate env_logger;
extern crate rusoto_core;
extern crate rusoto_rds;

use rusoto_rds::{Rds, RdsClient, DescribeDBClustersMessage};
use rusoto_core::Region;

#[test]
fn should_describe_db_clusters() {
    let _ = env_logger::try_init();
    let client = RdsClient::simple(Region::UsEast1);
    let request = DescribeDBClustersMessage::default();

    let result = client.describe_db_clusters(&request).sync();
    println!("{:#?}", result);
}
