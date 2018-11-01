#![cfg(feature = "mturk")]

extern crate rusoto_core;
extern crate rusoto_mturk;

use rusoto_core::Region;
use rusoto_mturk::{MechanicalTurkClient, MechanicalTurk, ListHITsRequest, ListHITsError};

use std::str;

#[test]
fn should_list_hits() {
    let client = MechanicalTurkClient::new(Region::UsEast1);
    let request = ListHITsRequest::default();

    // If your AWS account isn't linked to a Mechanical Turk account, AWS returns an error
    match client.list_hi_ts(request).sync() {
        Err(e) => {
            match e {
                ListHITsError::Unknown(ref e) => assert!(str::from_utf8(&e.body).unwrap().contains("Your AWS account must be linked to your Amazon Mechanical Turk Account"), "Missing error message"),
                ListHITsError::RequestError(_) => (), // request doesn't work without a linked mturk account, this is ok
                _ => panic!("Should have a typed error from MTurk, got {:?}", e),
            }
        },
        Ok(_) => (),
    }
}