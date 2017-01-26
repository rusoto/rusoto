#![cfg(feature = "rds")]
extern crate env_logger;
extern crate rusoto;

use rusoto::rds::{RdsClient, DescribeDBClustersMessage};
use rusoto::{DefaultCredentialsProvider, Region};
use rusoto::default_tls_client;

#[test]
fn should_describe_db_clusters() {
	let _ = env_logger::init();
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = RdsClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = DescribeDBClustersMessage::default();

    let result = client.describe_db_clusters(&request);
    println!("{:#?}", result);
}


