#![cfg(feature = "ses")]

extern crate rusoto;

use rusoto::ses::{Ses, SesClient};
use rusoto::{DefaultCredentialsProvider, Region};
use rusoto::default_tls_client;

#[test]
fn should_list_verified_email_addresses() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = SesClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);

    let result = client.list_verified_email_addresses().unwrap();
    println!("{:#?}", result);
}
