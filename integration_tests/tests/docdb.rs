#![cfg(feature = "docdb")]

extern crate rusoto_core;
extern crate rusoto_docdb;

use rusoto_core::Region;
use rusoto_docdb::{
    Docdb, DocdbClient, DescribeDBClustersRequest,
};

#[test]
fn should_describe_tags() {
    let client = DocdbClient::new(Region::UsEast1);
    let request = DescribeDBClustersRequest::default();

    let result = client.describe_db_clusters(request).sync().unwrap();
    println!("{:#?}", result);
}
