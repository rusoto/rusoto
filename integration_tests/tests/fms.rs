#![cfg(feature = "fms")]

extern crate rusoto_core;
extern crate rusoto_fms;

use rusoto_core::{Region, RusotoError};
use rusoto_fms::{Fms, FmsClient, ListPoliciesRequest};

use std::str;

#[tokio::test]
async fn should_list_policies() {
    let client = FmsClient::new(Region::UsEast1);
    let request = ListPoliciesRequest {
        max_results: Some(1),
        ..Default::default()
    };

    // If our account doesn't have access, assume everything is fine:
    match client.list_policies(request).await {
        Err(e) => match e {
            RusotoError::Unknown(ref e) => assert!(
                str::from_utf8(&e.body)
                    .unwrap()
                    .contains("is not currently delegated by AWS FM"),
                "Missing error message"
            ),
            _ => panic!("Should have a typed error from FMS, got {:?}", e),
        },
        Ok(res) => println!("Got these policies: {:?}", res),
    }
}
