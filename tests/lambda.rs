#![cfg(feature = "lambda")]

extern crate rusoto;

use rusoto::lambda::{LambdaClient, ListFunctionsRequest};
use rusoto::{DefaultCredentialsProvider, Region};
use rusoto::default_tls_client;

#[test]
fn should_list_functions() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = LambdaClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListFunctionsRequest::default();

    let result = client.list_functions(&request).unwrap();
    println!("{:#?}", result);
}
