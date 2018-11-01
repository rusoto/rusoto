#![cfg(feature = "fms")]

extern crate rusoto_core;
extern crate rusoto_fms;

use rusoto_fms::{Fms, FmsClient, ListPoliciesRequest, ListPoliciesError};
use rusoto_core::Region;

use std::str;

#[test]
fn should_list_policies() {
    let client = FmsClient::new(Region::UsEast1);
    let request = ListPoliciesRequest::default();

    // If our account doesn't have access, assume everything is fine:
    match client.list_policies(request).sync() {
        Err(e) => {
            match e {
                ListPoliciesError::Unknown(ref e) => assert!(str::from_utf8(&e.body).unwrap().contains("is not currently delegated by AWS FM"), "Missing error message"),
                _ => panic!("Should have a typed error from FMS"),
            }
        },
        Ok(res) => println!("Got these policies: {:?}", res),
    }
    
}
