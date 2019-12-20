#![cfg(feature = "lex-models")]

extern crate rusoto_core;
extern crate rusoto_lex_models;

use rusoto_core::Region;
use rusoto_lex_models::{GetBotsRequest, LexModels, LexModelsClient};

#[tokio::test]
async fn should_get_bots() {
    let client = LexModelsClient::new(Region::UsEast1);
    let request = GetBotsRequest::default();

    let result = client.get_bots(request).await.unwrap();
    println!("{:#?}", result);
}
