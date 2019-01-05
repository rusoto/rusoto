#![cfg(feature = "xray")]

extern crate rusoto_core;
extern crate rusoto_xray;
extern crate time;

use rusoto_core::Region;
use rusoto_xray::{GetServiceGraphRequest, XRay, XRayClient};
use time::get_time;

// duplicates the AWS X-Ray CLI example, which gets an (empty) service graph
// for the last 600 seconds
#[test]
fn should_get_service_graph() {
    let client = XRayClient::new(Region::UsEast1);

    let time = (get_time().sec - 30) as f64; // 30 seconds in the past

    let request = GetServiceGraphRequest {
        start_time: time - 600.0,
        end_time: time,
        ..Default::default()
    };

    let result = client.get_service_graph(request).sync();
    println!("{:#?}", result);
    result.unwrap();
}
