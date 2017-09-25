#![cfg(feature = "athena")]

extern crate rusoto_core;
extern crate rusoto_athena;

use rusoto_athena::{Athena, AthenaClient, ListNamedQueriesInput};
use rusoto_core::{DefaultCredentialsProvider, Region, default_tls_client};


#[test]
fn should_list_named_queries() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = AthenaClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListNamedQueriesInput::default();

    client.list_named_queries(&request).unwrap();

}