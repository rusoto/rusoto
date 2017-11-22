#![cfg(feature = "lex-models")]

extern crate rusoto_core;
extern crate rusoto_lex_models;

use rusoto_lex_models::{LexModels, LexModelsClient, GetBotsRequest};
use rusoto_core::{DefaultCredentialsProvider, Region};
use rusoto_core::default_tls_client;

#[test]
fn should_get_bots() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = LexModelsClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = GetBotsRequest::default();

    let result = client.get_bots(&request).unwrap();
    println!("{:#?}", result);
}
