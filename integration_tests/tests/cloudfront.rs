#![cfg(feature = "cloudfront")]

extern crate env_logger;
extern crate rusoto_cloudfront;
extern crate rusoto_core;

use rusoto_cloudfront::{CloudFront, CloudFrontClient, ListDistributionsRequest};
use rusoto_core::Region;

#[tokio::test]
async fn should_list_distributions() {
    let _ = env_logger::try_init();
    let client = CloudFrontClient::new(Region::UsEast1);
    let request = ListDistributionsRequest::default();

    let result = client.list_distributions(request).await;
    println!("{:#?}", result);
    assert!(result.is_ok());
}
