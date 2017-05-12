#![cfg(feature = "redshift")]
extern crate rusoto;

use rusoto::redshift::{Redshift, RedshiftClient, DescribeClustersMessage};
use rusoto::{DefaultCredentialsProvider, Region, default_tls_client};

#[test]
fn should_describe_clusters() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = RedshiftClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = DescribeClustersMessage::default();

    let result = client.describe_clusters(&request);
    println!("{:#?}", result);
}
