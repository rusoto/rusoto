#![cfg(feature = "sns")]

extern crate rusoto_core;
extern crate rusoto_sns;

use rusoto_core::Region;
use rusoto_sns::{ListTopicsInput, Sns, SnsClient};

#[tokio::test]
async fn should_list_topics() {
    let client = SnsClient::new(Region::UsEast1);
    let request = ListTopicsInput::default();

    let result = client.list_topics(request).await.unwrap();
    println!("{:#?}", result);
}
