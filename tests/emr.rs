#![cfg(feature = "emr")]

extern crate rusoto;

use rusoto::emr::{EmrClient, ListClustersInput, DescribeJobFlowsInput, DescribeJobFlowsError};
use rusoto::{DefaultCredentialsProvider, Region};
use rusoto::default_tls_client;

#[test]
fn should_list_clusters() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = EmrClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListClustersInput::default();

    client.list_clusters(&request).unwrap();
}

#[test]
fn should_handle_deprecation_gracefully() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = EmrClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = DescribeJobFlowsInput::default();

    match client.describe_job_flows(&request) {
        Err(DescribeJobFlowsError::Validation(msg)) => {
            assert!(msg.contains("DescribeJobFlows API is deprecated."))
        }
        err @ _ => panic!("Expected OK response, got {:#?}", err),
    };
}
