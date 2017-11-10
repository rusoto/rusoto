#![cfg(feature = "ses")]

extern crate rusoto_core;
extern crate rusoto_ses;

use rusoto_ses::{Ses, SesClient};
use rusoto_core::{DefaultCredentialsProvider, Region};
use rusoto_core::default_tls_client;

#[test]
fn should_list_verified_email_addresses() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = SesClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);

    let result = client.list_verified_email_addresses().sync().unwrap();
    println!("{:#?}", result);
}
