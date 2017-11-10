#![cfg(feature = "logs")]

extern crate rusoto_core;
extern crate rusoto_logs;

use rusoto_logs::{CloudWatchLogs, CloudWatchLogsClient, DescribeLogGroupsRequest};
use rusoto_core::{DefaultCredentialsProvider, Region};
use rusoto_core::default_tls_client;

#[test]
fn should_describe_log_groups() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client =
        CloudWatchLogsClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = DescribeLogGroupsRequest::default();

    client.describe_log_groups(&request).sync().unwrap();
}
