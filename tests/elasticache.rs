#![cfg(feature = "elasticache")]

extern crate rusoto;

use rusoto::elasticache::{ElastiCacheClient, DescribeCacheClustersMessage};
use rusoto::{DefaultCredentialsProvider, Region};

#[test]
fn should_describe_cache_clusters() {
    let client = ElastiCacheClient::new(DefaultCredentialsProvider::new().unwrap(), Region::UsEast1);
    let request = DescribeCacheClustersMessage::default();

 	let response = client.describe_cache_clusters(&request).unwrap();
    println!("{:#?}", response);
}

