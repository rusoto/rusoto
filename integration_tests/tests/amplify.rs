#![cfg(feature = "amplify")]

extern crate rusoto_amplify;
extern crate rusoto_core;

use rusoto_amplify::{Amplify, AmplifyClient};
use rusoto_core::Region;

#[test]
fn should_work() {
    let client = AmplifyClient::new(Region::UsEast1);
    let response = client.list_apps(Default::default()).sync();
    println!("response is {:#?}", response);
}
