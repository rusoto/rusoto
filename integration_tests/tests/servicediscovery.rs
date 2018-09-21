#![cfg(feature = "servicediscovery")]

extern crate rusoto_core;
extern crate rusoto_servicediscovery;

use rusoto_servicediscovery::{ServiceDiscovery, ServiceDiscoveryClient, ListServicesRequest};
use rusoto_core::Region;

#[test]
fn should_list_services() {
    let client = ServiceDiscoveryClient::new(Region::UsEast1);
    let request = ListServicesRequest::default();

    let res = client.list_services(request).sync().unwrap();
    println!("Res: {:?}", res);
}
