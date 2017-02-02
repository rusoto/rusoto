#![cfg(feature = "lightsail")]
extern crate rusoto;

use rusoto::lightsail::{LightsailClient, GetDomainsRequest};
use rusoto::{DefaultCredentialsProvider, Region, default_tls_client};

#[test]
fn should_list_domains() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = LightsailClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = GetDomainsRequest::default();

    let result = client.get_domains(&request).unwrap();
	println!("{:#?}", result);
}