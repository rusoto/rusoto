#![cfg(feature = "logs")]

extern crate rusoto;

use rusoto::logs::{CloudWatchLogsClient, DescribeLogGroupsRequest};
use rusoto::{DefaultCredentialsProvider, Region};

#[test]
fn should_describe_log_groups() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = CloudWatchLogsClient::new(credentials, Region::UsEast1);

    let request = DescribeLogGroupsRequest::default();

    match client.describe_log_groups(&request) {
        Ok(response) => {
            println!("{:#?}", response); 
            assert!(true)            
        },
        Err(err) => panic!("Expected OK response, got {:#?}", err)
    };
}
