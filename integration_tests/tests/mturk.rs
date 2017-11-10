#![cfg(feature = "mturk")]

extern crate rusoto_core;
extern crate rusoto_mturk;

use rusoto_core::{DefaultCredentialsProvider, Region, default_tls_client};
use rusoto_mturk::{MechanicalTurkClient, MechanicalTurk, ListHITsRequest};

#[test]
fn should_list_hits() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = MechanicalTurkClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListHITsRequest::default();

    // If your AWS account isn't linked to a Mechanical Turk account, AWS returns an error
    match client.list_hi_ts(&request).sync() {
        Err(e) => assert!(format!("{}", e).contains("Your AWS account must be linked to your Amazon Mechanical Turk Account")),
        Ok(_) => (),
    }
}