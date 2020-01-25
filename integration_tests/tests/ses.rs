#![cfg(feature = "ses")]

extern crate rusoto_core;
extern crate rusoto_ses;

use rusoto_core::Region;
use rusoto_ses::{Ses, SesClient};

#[tokio::test]
async fn should_list_verified_email_addresses() {
    let client = SesClient::new(Region::UsEast1);

    let result = client.list_verified_email_addresses().await.unwrap();
    println!("{:#?}", result);
}
