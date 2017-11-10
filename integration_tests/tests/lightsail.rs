#![cfg(feature = "lightsail")]

extern crate rusoto_core;
extern crate rusoto_lightsail;

use rusoto_lightsail::{Lightsail, LightsailClient, GetDomainsRequest};
use rusoto_core::{DefaultCredentialsProvider, Region, default_tls_client};

#[test]
fn should_list_domains() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = LightsailClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = GetDomainsRequest::default();

    let result = client.get_domains(&request).sync().unwrap();
	println!("{:#?}", result);
}