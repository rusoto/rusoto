#![cfg(feature = "devicefarm")]

extern crate rusoto_core;
extern crate rusoto_devicefarm;

use rusoto_devicefarm::{DeviceFarm, DeviceFarmClient, ListDevicesRequest};
use rusoto_core::{DefaultCredentialsProvider, Region};
use rusoto_core::default_tls_client;

#[test]
pub fn should_list_devices() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = DeviceFarmClient::new(default_tls_client().unwrap(), credentials, Region::UsWest2);
    let request = ListDevicesRequest::default();

    client.list_devices(&request).unwrap();
}
