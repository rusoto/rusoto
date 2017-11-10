#![cfg(feature = "cloudhsm")]

extern crate rusoto_core;
extern crate rusoto_cloudhsm;

use rusoto_cloudhsm::{CloudHsm, CloudHsmClient, ListHapgsRequest, ListHsmsRequest, ListLunaClientsRequest};
use rusoto_core::{DefaultCredentialsProvider, Region};
use rusoto_core::default_tls_client;

#[test]
fn should_list_hapgs() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = CloudHsmClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListHapgsRequest::default();

    match client.list_hapgs(&request).sync() {
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
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = CloudHsmClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListHsmsRequest::default();

    match client.list_hsms(&request).sync() {
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
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = CloudHsmClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListLunaClientsRequest::default();

    match client.list_luna_clients(&request).sync() {
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
