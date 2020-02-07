#![cfg(feature = "macie")]

extern crate rusoto_core;
extern crate rusoto_macie;

use rusoto_core::Region;
use rusoto_macie::{ListS3ResourcesRequest, Macie, MacieClient};

#[tokio::test]
async fn should_list_s3_resources() {
    let client = MacieClient::new(Region::UsEast1);
    let request = ListS3ResourcesRequest::default();

    // If Macie isn't turned on, don't fail the test
    match client.list_s3_resources(request).await {
        Err(e) => assert!(format!("{}", e).contains("Macie is not enabled for this AWS account")),
        Ok(result) => println!("S3 resources for Macie: {:?}", result),
    }
}
