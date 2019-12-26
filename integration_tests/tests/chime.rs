#![cfg(feature = "chime")]

extern crate rusoto_chime;
extern crate rusoto_core;

use rusoto_chime::{Chime, ChimeClient, ListAccountsRequest};
use rusoto_core::Region;

#[tokio::test]
async fn should_list_environments() {
    let client = ChimeClient::new(Region::UsEast1);
    let request = ListAccountsRequest::default();

    let result = client.list_accounts(request).await.unwrap();
    println!("{:#?}", result);
}
