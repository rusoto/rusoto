#![cfg(feature = "devicefarm")]

extern crate rusoto;

use rusoto::devicefarm::{DeviceFarmClient, ListDevicesRequest};
use rusoto::{DefaultCredentialsProvider, Region};

#[test]
pub fn should_list_devices() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = DeviceFarmClient::new(credentials, Region::UsWest2);
    let request = ListDevicesRequest::default();

    client.list_devices(&request).unwrap();
}
