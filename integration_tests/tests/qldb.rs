#![cfg(feature = "qldb")]

extern crate rusoto_core;
extern crate rusoto_qldb;

use rusoto_core::Region;
use rusoto_qldb::{ListLedgersRequest, Qldb, QldbClient};

#[tokio::test]
async fn should_list_ledgers() {
    let client = QldbClient::new(Region::UsEast1);
    let response = client
        .list_ledgers(ListLedgersRequest::default())
        .await
        .expect("expected an ok response");
    println!("response is {:#?}", response);
}
