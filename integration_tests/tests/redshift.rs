#![cfg(feature = "redshift")]
extern crate rusoto_core;
extern crate rusoto_redshift;

use rusoto_redshift::{Redshift, RedshiftClient, DescribeClustersMessage};
use rusoto_core::Region;

#[test]
fn should_describe_clusters() {
    let client = RedshiftClient::simple(Region::UsEast1);
    let request = DescribeClustersMessage::default();

    let result = client.describe_clusters(&request).sync().unwrap();
    println!("{:#?}", result);
}
