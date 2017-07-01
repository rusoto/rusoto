#![cfg(feature = "glacier")]

extern crate rusoto_core;
extern crate rusoto_glacier;
extern crate env_logger;

use rusoto_glacier::{Glacier, GlacierClient, ListVaultsInput};
use rusoto_core::{DefaultCredentialsProvider, Region, default_tls_client};

#[test]
fn should_list_vaults() {
    let _ = env_logger::init();
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = GlacierClient::new(default_tls_client().unwrap(), credentials, Region::UsWest2);
    // account id can be provided or use the account that signed the request with `-`.
    // http://docs.aws.amazon.com/amazonglacier/latest/dev/api-vaults-get.html
    let request = ListVaultsInput{
        account_id: "-".to_string(),
        ..Default::default()
    };

    let result = client.list_vaults(&request).unwrap();
    println!("{:#?}", result);
}