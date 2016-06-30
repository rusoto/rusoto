#![cfg(feature = "cloudhsm")]

extern crate rusoto;

use rusoto::cloudhsm::{CloudHsmClient, ListHapgsRequest, ListHsmsRequest, ListLunaClientsRequest};
use rusoto::{DefaultCredentialsProvider, Region};

#[test]
fn should_list_hapgs() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = CloudHsmClient::new(credentials, Region::UsEast1);
    let request = ListHapgsRequest::default();

    client.list_hapgs(&request).unwrap();
}

#[test]
fn should_list_hsms() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = CloudHsmClient::new(credentials, Region::UsEast1);
    let request = ListHsmsRequest::default();

    client.list_hsms(&request).unwrap();
}
#[test]
fn should_list_luna_clients() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = CloudHsmClient::new(credentials, Region::UsEast1);
    let request = ListLunaClientsRequest::default();

    client.list_luna_clients(&request).unwrap();
}
