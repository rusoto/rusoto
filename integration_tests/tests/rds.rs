#![cfg(feature = "rds")]
extern crate env_logger;
extern crate rusoto_core;
extern crate rusoto_rds;

use rusoto_rds::{Rds, RdsClient, DescribeDBClustersMessage};
use rusoto_core::{DefaultCredentialsProvider, Region};
use rusoto_core::default_tls_client;

#[test]
fn should_describe_db_clusters() {
    let _ = env_logger::init();
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = RdsClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = DescribeDBClustersMessage::default();

    let result = client.describe_db_clusters(&request).sync();
    println!("{:#?}", result);
}
