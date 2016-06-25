#![cfg(feature = "devicefarm")]

extern crate rusoto;

use rusoto::devicefarm::{DeviceFarmClient, ListDevicesRequest};
use rusoto::{DefaultCredentialsProvider, Region};

#[test]
pub fn should_list_devices() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = DeviceFarmClient::new(credentials, Region::UsWest2);

    let request = ListDevicesRequest::default();

    match client.list_devices(&request) {
        Ok(response) => {
            assert!(!response.devices.unwrap().is_empty())
        },
        Err(err) => panic!("Expected OK response, got {:#?}", err)
    };
}
