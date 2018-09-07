#![cfg(feature = "neptune")]

extern crate rusoto_core;
extern crate rusoto_neptune;

use rusoto_core::Region;
use rusoto_neptune::{NeptuneClient, Neptune, DescribeDBClustersMessage};

#[test]
fn should_list_hits() {
    let client = NeptuneClient::new(Region::UsEast1);
    let request = DescribeDBClustersMessage::default();

    match client.describe_db_clusters(request).sync() {
        Err(e) => panic!("Error listing Neptune clusters: {}", e),
        Ok(clusters) => println!("Found clusters: {:?}", clusters),
    }
}