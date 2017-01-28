#![cfg(feature = "elasticache")]

extern crate rusoto;

use rusoto::elasticache::{ElastiCacheClient, DescribeCacheClustersMessage};
use rusoto::{DefaultCredentialsProvider, Region};
use rusoto::default_tls_client;

#[test]
fn should_describe_cache_clusters() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = ElastiCacheClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = DescribeCacheClustersMessage::default();

    let response = client.describe_cache_clusters(&request).unwrap();
    println!("{:#?}", response);
}

