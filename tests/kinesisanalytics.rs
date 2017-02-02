#![cfg(feature = "kinesisanalytics")]
extern crate rusoto;

use rusoto::kinesisanalytics::{KinesisAnalyticsClient, ListApplicationsRequest};
use rusoto::{DefaultCredentialsProvider, Region, default_tls_client};

#[test]
fn should_list_applications() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = KinesisAnalyticsClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListApplicationsRequest::default();

    let result = client.list_applications(&request).unwrap();
	println!("{:#?}", result);
}