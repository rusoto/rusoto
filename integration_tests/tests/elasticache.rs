#![cfg(feature = "elasticache")]

extern crate rusoto_core;
extern crate rusoto_elasticache;

use rusoto_core::Region;
use rusoto_elasticache::{DescribeCacheClustersRequest, ElastiCache, ElastiCacheClient};

#[test]
fn should_describe_cache_clusters() {
    let client = ElastiCacheClient::new(Region::UsEast1);
    let request = DescribeCacheClustersRequest::default();

    let response = client.describe_cache_clusters(request).sync().unwrap();
    println!("{:#?}", response);
}
