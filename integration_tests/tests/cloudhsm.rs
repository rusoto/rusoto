#![cfg(feature = "cloudhsm")]

extern crate rusoto_cloudhsm;
extern crate rusoto_core;

use rusoto_cloudhsm::{
    CloudHsm, CloudHsmClient, ListHapgsRequest, ListHsmsRequest, ListLunaClientsRequest,
};
use rusoto_core::Region;

#[test]
fn should_list_hapgs() {
    let client = CloudHsmClient::new(Region::UsEast1);
    let request = ListHapgsRequest::default();

    match client.list_hapgs(request).sync() {
        Ok(_) => (),
        Err(e) => {
            if e.to_string().contains("This service is unavailable.") {
                ()
            } else {
                panic!("Error unrelated to service being unavailable: {:?}", e);
            }
        }
    }
}

#[test]
fn should_list_hsms() {
    let client = CloudHsmClient::new(Region::UsEast1);
    let request = ListHsmsRequest::default();

    match client.list_hsms(request).sync() {
        Ok(_) => (),
        Err(e) => {
            if e.to_string().contains("This service is unavailable.") {
                ()
            } else {
                panic!("Error unrelated to service being unavailable: {:?}", e);
            }
        }
    }
}
#[test]
fn should_list_luna_clients() {
    let client = CloudHsmClient::new(Region::UsEast1);
    let request = ListLunaClientsRequest::default();

    match client.list_luna_clients(request).sync() {
        Ok(_) => (),
        Err(e) => {
            if e.to_string().contains("This service is unavailable.") {
                ()
            } else {
                panic!("Error unrelated to service being unavailable: {:?}", e);
            }
        }
    }
}
