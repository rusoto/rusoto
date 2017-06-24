#![cfg(feature = "sms")]

extern crate rusoto_core;
extern crate rusoto_sms;

use rusoto_sms::{ServerMigrationService, ServerMigrationServiceClient, GetServersRequest};
use rusoto_core::{default_tls_client, DefaultCredentialsProvider, Region};

#[test]
fn should_get_servers() {
    let provider = DefaultCredentialsProvider::new().unwrap();
    let client = ServerMigrationServiceClient::new(default_tls_client().unwrap(), provider, Region::UsEast1);

    let response = client.get_servers(&GetServersRequest::default()).unwrap();
    println!("{:#?}", response);
}