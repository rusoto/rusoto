#![cfg(feature = "lambda")]

extern crate rusoto_core;
extern crate rusoto_lambda;

use rusoto_lambda::{Lambda, LambdaClient, ListFunctionsRequest};
use rusoto_core::Region;

#[test]
fn should_list_functions() {
    let client = LambdaClient::simple(Region::UsEast1);
    let request = ListFunctionsRequest::default();

    let result = client.list_functions(&request).sync().unwrap();
    println!("{:#?}", result);
}
