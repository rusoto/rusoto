#![cfg(feature = "organizations")]
extern crate env_logger;
extern crate rusoto_core;
extern crate rusoto_organizations;

use rusoto_organizations::{Organizations, OrganizationsClient };
use rusoto_core::{DefaultCredentialsProvider, Region};
use rusoto_core::default_tls_client;

#[test]
fn should_describe_organizations() {
    let _ = env_logger::init();
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = OrganizationsClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);

    let result = client.describe_organization();
    println!("{:#?}", result);
}
