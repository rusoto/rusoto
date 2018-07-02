#![cfg(feature = "sms")]

extern crate rusoto_core;
extern crate rusoto_sms;

use rusoto_sms::{ServerMigrationService, ServerMigrationServiceClient, GetServersRequest};
use rusoto_core::Region;

#[test]
fn should_get_servers() {
    let client = ServerMigrationServiceClient::new(Region::UsEast1);

    let response = client.get_servers(GetServersRequest::default()).sync().unwrap();
    println!("{:#?}", response);
}