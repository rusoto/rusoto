#![cfg(feature = "glacier")]

extern crate rusoto_core;
extern crate rusoto_glacier;
extern crate env_logger;

use rusoto_glacier::{Glacier, GlacierClient, ListVaultsInput};
use rusoto_core::{DefaultCredentialsProvider, Region, default_tls_client};

#[test]
fn should_list_fleets() {
    let _ = env_logger::init();
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = GlacierClient::new(default_tls_client().unwrap(), credentials, Region::UsWest2);
    let request = ListVaultsInput{
        account_id: "-".to_string(),
        ..Default::default()
    };

    let result = client.list_vaults(&request).unwrap();
	println!("{:#?}", result);
}