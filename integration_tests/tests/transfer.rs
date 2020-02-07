#![cfg(feature = "transfer")]

extern crate rusoto_core;
extern crate rusoto_transfer;

use rusoto_core::Region;
use rusoto_transfer::{ListServersRequest, Transfer, TransferClient};

#[tokio::test]
async fn should_list_servers() {
    let client = TransferClient::new(Region::UsEast1);
    let request = ListServersRequest::default();

    println!("{:?}", client.list_servers(request).await.unwrap());
}
