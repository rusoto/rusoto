#![cfg(feature = "swf")]

extern crate rusoto;

use rusoto::swf::{SwfClient, ListDomainsInput};
use rusoto::{DefaultCredentialsProvider, Region};
use rusoto::default_tls_client;

#[test]
fn should_list_domains() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = SwfClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);

    let mut request = ListDomainsInput::default();
    request.maximum_page_size = Some(10);
    request.registration_status = "REGISTERED".to_string();

    client.list_domains(&request).unwrap();
}
