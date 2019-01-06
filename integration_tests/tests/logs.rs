#![cfg(feature = "logs")]

extern crate rusoto_core;
extern crate rusoto_logs;

use rusoto_core::Region;
use rusoto_logs::{CloudWatchLogs, CloudWatchLogsClient, DescribeLogGroupsRequest};

#[test]
fn should_describe_log_groups() {
    let client = CloudWatchLogsClient::new(Region::UsEast1);
    let request = DescribeLogGroupsRequest::default();

    client.describe_log_groups(request).sync().unwrap();
}
