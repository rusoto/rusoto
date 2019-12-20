#![cfg(feature = "docdb")]

extern crate rusoto_core;
extern crate rusoto_docdb;

use rusoto_core::Region;
use rusoto_docdb::{DescribeDBClustersMessage, Docdb, DocdbClient};

#[tokio::test]
async fn should_describe_tags() {
    let client = DocdbClient::new(Region::UsEast1);
    let request = DescribeDBClustersMessage::default();

    let result = client.describe_db_clusters(request).await.unwrap();
    println!("{:#?}", result);
}
