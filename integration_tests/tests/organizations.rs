#![cfg(feature = "organizations")]
extern crate env_logger;
extern crate rusoto_core;
extern crate rusoto_organizations;

use rusoto_core::Region;
use rusoto_organizations::{Organizations, OrganizationsClient};

#[tokio::test]
#[ignore]
async fn should_describe_organizations() {
    let _ = env_logger::try_init();
    let client = OrganizationsClient::new(Region::UsEast1);

    let result = client.describe_organization().await.unwrap();
    println!("{:#?}", result);
}
