#![cfg(feature = "appstream")]

extern crate env_logger;
extern crate rusoto_appstream;
extern crate rusoto_core;

use rusoto_appstream::{AppStream, AppStreamClient, DescribeFleetsRequest};
use rusoto_core::Region;

#[tokio::test]
async fn should_describe_fleets() {
    let _ = env_logger::try_init();
    let client = AppStreamClient::new(Region::UsEast1);
    let request = DescribeFleetsRequest::default();

    let result = client.describe_fleets(request).await.unwrap();
    println!("{:#?}", result);
}
