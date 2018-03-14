extern crate rusoto_mock;

use ::*;

use rusoto_core::Region;
use self::rusoto_mock::*;

#[test]
// regression test for #1002
fn register_scalable_target_happy_path() {
    let body = "{}".to_string();
    let mock = MockRequestDispatcher::with_status(200).with_body(&body);

    let client = ApplicationAutoScalingClient::new(mock, MockCredentialsProvider, Region::UsEast1);
    let result = client.register_scalable_target(&Default::default()).sync();

    result.expect("Couldn't parse register_scalable_target");
}
