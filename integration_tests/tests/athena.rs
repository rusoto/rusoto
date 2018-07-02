#![cfg(feature = "athena")]

extern crate rusoto_core;
extern crate rusoto_athena;

use rusoto_athena::{Athena, AthenaClient, ListNamedQueriesInput};
use rusoto_core::Region;

#[test]
fn should_list_named_queries() {
    let client = AthenaClient::new(Region::UsEast1);
    let request = ListNamedQueriesInput::default();

    client.list_named_queries(request).sync().unwrap();

}