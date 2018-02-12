#![cfg(feature = "redshift")]
extern crate rusoto_core;
extern crate rusoto_redshift;
extern crate env_logger;

use rusoto_redshift::{Redshift, RedshiftClient, DescribeClustersMessage};
use rusoto_core::Region;

#[test]
fn should_describe_clusters() {
    let _ = env_logger::try_init();
    let client = RedshiftClient::simple(Region::UsEast1);
    let request = DescribeClustersMessage::default();

    let result = client.describe_clusters(&request).sync();
    println!("{:#?}", result);
    assert!(result.is_ok());
}
