/*
#![allow(dead_code)]
#[macro_use]
extern crate rusoto;

use rusoto::credentials::DefaultAWSCredentialsProviderChain;
use rusoto::regions::Region;
use rusoto::kms::{KMSHelper, KMSError};

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
