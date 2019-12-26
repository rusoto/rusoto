#![cfg(feature = "redshift")]
extern crate env_logger;
extern crate rusoto_core;
extern crate rusoto_redshift;

use rusoto_core::Region;
use rusoto_redshift::{DescribeClustersMessage, Redshift, RedshiftClient};

#[tokio::test]
async fn should_describe_clusters() {
    let _ = env_logger::try_init();
    let client = RedshiftClient::new(Region::UsEast1);
    let request = DescribeClustersMessage::default();

    let result = client.describe_clusters(request).await;
    println!("{:#?}", result);
    assert!(result.is_ok());
}
