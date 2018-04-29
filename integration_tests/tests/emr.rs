#![cfg(feature = "emr")]

extern crate rusoto_core;
extern crate rusoto_emr;

use rusoto_emr::{Emr, EmrClient, ListClustersInput, DescribeJobFlowsInput, DescribeJobFlowsError};
use rusoto_core::Region;

#[test]
fn should_list_clusters() {
    let client = EmrClient::simple(Region::UsEast1);
    let request = ListClustersInput::default();

    client.list_clusters(request).sync().unwrap();
}

#[test]
fn should_handle_deprecation_gracefully() {
    let client = EmrClient::simple(Region::UsEast1);
    let request = DescribeJobFlowsInput::default();

    match client.describe_job_flows(request).sync() {
        Err(DescribeJobFlowsError::Validation(msg)) => {
            assert!(msg.contains("DescribeJobFlows API is deprecated."))
        }
        err @ _ => panic!("Expected OK response, got {:#?}", err),
    };
}
