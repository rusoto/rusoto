#![cfg(feature = "logs")]

extern crate rusoto;

use rusoto::logs::{CloudWatchLogsClient, DescribeLogGroupsRequest};
use rusoto::{DefaultCredentialsProvider, Region};
use rusoto::default_tls_client;

#[test]
fn should_describe_log_groups() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = CloudWatchLogsClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = DescribeLogGroupsRequest::default();

    client.describe_log_groups(&request).unwrap();
}
