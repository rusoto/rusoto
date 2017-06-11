#![cfg(feature = "kinesisanalytics")]

extern crate rusoto_core;
extern crate rusoto_kinesisanalytics;

use rusoto_kinesisanalytics::{KinesisAnalytics, KinesisAnalyticsClient, ListApplicationsRequest};
use rusoto_core::{DefaultCredentialsProvider, Region, default_tls_client};

#[test]
fn should_list_applications() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = KinesisAnalyticsClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListApplicationsRequest::default();

    let result = client.list_applications(&request).unwrap();
	println!("{:#?}", result);
}