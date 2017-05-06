#![cfg(feature = "devicefarm")]

extern crate rusoto;

use rusoto::devicefarm::{DeviceFarm, DeviceFarmClient, ListDevicesRequest};
use rusoto::{DefaultCredentialsProvider, Region};
use rusoto::default_tls_client;

#[test]
pub fn should_list_devices() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = DeviceFarmClient::new(default_tls_client().unwrap(), credentials, Region::UsWest2);
    let request = ListDevicesRequest::default();

    client.list_devices(&request).unwrap();
}
