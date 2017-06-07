#![cfg(feature = "elasticache")]

extern crate rusoto_core;
extern crate rusoto_elasticache;

use rusoto_elasticache::{ElastiCache, ElastiCacheClient, DescribeCacheClustersMessage};
use rusoto_core::{DefaultCredentialsProvider, Region};
use rusoto_core::default_tls_client;

#[test]
fn should_describe_cache_clusters() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client =
        ElastiCacheClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = DescribeCacheClustersMessage::default();

    let response = client.describe_cache_clusters(&request).unwrap();
    println!("{:#?}", response);
}
