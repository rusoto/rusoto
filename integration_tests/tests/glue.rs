#![cfg(feature = "glue")]

extern crate rusoto_core;
extern crate rusoto_glue;

use rusoto_glue::{Glue, GlueClient, GetDatabasesRequest};
use rusoto_core::{DefaultCredentialsProvider, Region, default_tls_client};

#[test]
fn should_get_databases() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = GlueClient::new(default_tls_client().unwrap(), credentials, Region::UsWest2);
    let request = GetDatabasesRequest::default();

    let result = client.get_databases(&request).sync().unwrap();
	println!("{:#?}", result);
}