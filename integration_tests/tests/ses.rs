#![cfg(feature = "ses")]

extern crate rusoto_core;
extern crate rusoto_ses;

use rusoto_ses::{Ses, SesClient};
use rusoto_core::Region;

#[test]
fn should_list_verified_email_addresses() {
    let client = SesClient::new(Region::UsEast1);

    let result = client.list_verified_email_addresses().sync().unwrap();
    println!("{:#?}", result);
}
