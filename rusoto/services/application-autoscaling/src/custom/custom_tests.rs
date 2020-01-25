extern crate rusoto_mock;

use crate::generated::*;

use self::rusoto_mock::*;
use rusoto_core::Region;

#[tokio::test]
// regression test for #1002
async fn register_scalable_target_happy_path() {
    let body = "{}".to_string();
    let mock = MockRequestDispatcher::with_status(200).with_body(&body);

    let client =
        ApplicationAutoScalingClient::new_with(mock, MockCredentialsProvider, Region::UsEast1);
    let result = client.register_scalable_target(Default::default()).await;

    result.expect("Couldn't parse register_scalable_target");
}

#[tokio::test]
// another regression test for #1002
async fn register_scalable_target_returs_empty_body() {
    let body = "".to_string();
    let mock = MockRequestDispatcher::with_status(200).with_body(&body);

    let client =
        ApplicationAutoScalingClient::new_with(mock, MockCredentialsProvider, Region::UsEast1);
    let result = client.register_scalable_target(Default::default()).await;

    result.expect("Couldn't parse register_scalable_target");
}