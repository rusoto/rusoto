#![cfg(feature = "kinesisanalytics")]

extern crate rusoto_core;
extern crate rusoto_kinesisanalytics;

use rusoto_kinesisanalytics::{KinesisAnalytics, KinesisAnalyticsClient, ListApplicationsRequest};
use rusoto_core::Region;

#[test]
fn should_list_applications() {
    let client = KinesisAnalyticsClient::simple(Region::UsEast1);
    let request = ListApplicationsRequest::default();

    let result = client.list_applications(&request).sync().unwrap();
	println!("{:#?}", result);
}