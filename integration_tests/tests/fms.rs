#![cfg(feature = "fms")]

extern crate rusoto_core;
extern crate rusoto_fms;

use rusoto_fms::{Fms, FmsClient, ListPoliciesRequest};
use rusoto_core::Region;

#[test]
fn should_list_policies() {
    let client = FmsClient::new(Region::UsEast1);
    let request = ListPoliciesRequest::default();

    // If our account doesn't have access, assume everything is fine:
    match client.list_policies(request).sync() {
        Err(e) => assert!(format!("{}", e).contains("is not currently delegated by AWS FM")),
        Ok(res) => println!("Got these policies: {:?}", res),
    }
    
}
