#![cfg(feature = "organizations")]
extern crate env_logger;
extern crate rusoto_core;
extern crate rusoto_organizations;

use rusoto_organizations::{Organizations, OrganizationsClient };
use rusoto_core::Region;

#[test]
#[ignore]
fn should_describe_organizations() {
    let _ = env_logger::try_init();
    let client = OrganizationsClient::simple(Region::UsEast1);

    let result = client.describe_organization().sync().unwrap();
    println!("{:#?}", result);
}
