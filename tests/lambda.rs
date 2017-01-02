#![cfg(feature = "lambda")]

extern crate rusoto;

use rusoto::lambda::{LambdaClient, ListFunctionsRequest};
use rusoto::{DefaultCredentialsProvider, Region};

#[test]
fn should_list_functions() {
    let client = LambdaClient::new(DefaultCredentialsProvider::new().unwrap(), Region::UsEast1);
    let request = ListFunctionsRequest::default();

    let result = client.list_functions(&request).unwrap();
    println!("{:#?}", result);
}


