#![cfg(feature = "lambda")]

extern crate rusoto_core;
extern crate rusoto_lambda;

use rusoto_lambda::{Lambda, LambdaClient, ListFunctionsRequest};
use rusoto_core::{DefaultCredentialsProvider, Region};
use rusoto_core::default_tls_client;

#[test]
fn should_list_functions() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = LambdaClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListFunctionsRequest::default();

    let result = client.list_functions(&request).unwrap();
    println!("{:#?}", result);
}
