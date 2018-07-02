#![cfg(feature = "glue")]

extern crate rusoto_core;
extern crate rusoto_glue;

use rusoto_glue::{Glue, GlueClient, GetDatabasesRequest};
use rusoto_core::Region;

#[test]
fn should_get_databases() {
    let client = GlueClient::new(Region::UsWest2);
    let request = GetDatabasesRequest::default();

    let result = client.get_databases(request).sync().unwrap();
	println!("{:#?}", result);
}