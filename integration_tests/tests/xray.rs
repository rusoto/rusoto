#![cfg(feature = "xray")]

extern crate rusoto_core;
extern crate rusoto_xray;

use rusoto_xray::{XRay, XRayClient, GetServiceGraphRequest};
use rusoto_core::{DefaultCredentialsProvider, Region};
use rusoto_core::default_tls_client;

// duplicates the AWS X-Ray CLI example, which gets an (empty) service graph
// for the last 600 seconds
#[test]
fn should_get_service_graph() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = XRayClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);

    let time: f64 = 1503868350.0; // time at the writing of this test

    let request = GetServiceGraphRequest {
        start_time: time - 600.0,
        end_time: time,
        ..Default::default()
    };

    let result = client.get_service_graph(&request);
    println!("{:#?}", result);
    result.unwrap();
}
