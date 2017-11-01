#![cfg(feature = "pinpoint")]

extern crate rusoto_core;
extern crate rusoto_pinpoint;
extern crate env_logger;
extern crate log;

use rusoto_pinpoint::{Pinpoint, PinpointClient, GetAppsRequest};
use rusoto_core::{DefaultCredentialsProvider, Region, default_tls_client};

#[test]
fn should_describe_apps() {
    let _ = env_logger::init();
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = PinpointClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = GetAppsRequest::default();

    let result = client.get_apps(&request).unwrap();
    println!("Result: {:?}", result);
}
