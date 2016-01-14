/*
#![allow(dead_code)]
#[macro_use]
extern crate rusoto;
extern crate xml;
extern crate time;
extern crate regex;
extern crate rustc_serialize;

use rusoto::credentials::*;
use rusoto::regions::*;
use rusoto::kms::{KMSHelper, KMSError};

#[cfg(feature = "kms")]
#[test]
fn main() {
    let creds = DefaultAWSCredentialsProviderChain::new();
    let region = Region::UsWest2;

    let mut kms = KMSHelper::new(creds, &region);

    match kms_list_keys_tests(&mut kms) {
        Ok(_) => {
            println!("List keys OK");
        }
        Err(err) => {
            println!("Error getting keys list: {:#?}", err);
        }
    }
}

fn kms_list_keys_tests(kms: &mut KMSHelper) -> Result<(), KMSError> {
    let response = try!(kms.list_keys());
    println!("{:#?}", response);
    Ok(())
}
*/
