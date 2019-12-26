#![cfg(feature = "emr")]

extern crate rusoto_core;
extern crate rusoto_emr;

use rusoto_core::{Region, RusotoError};
use rusoto_emr::{DescribeJobFlowsInput, Emr, EmrClient, ListClustersInput};

#[tokio::test]
async fn should_list_clusters() {
    let client = EmrClient::new(Region::UsEast1);
    let request = ListClustersInput::default();

    client.list_clusters(request).await.unwrap();
}

#[tokio::test]
async fn should_handle_deprecation_gracefully() {
    let client = EmrClient::new(Region::UsEast1);
    let request = DescribeJobFlowsInput::default();

    match client.describe_job_flows(request).await {
        Err(RusotoError::Validation(msg)) => {
            assert!(msg.contains("DescribeJobFlows API is deprecated."))
        }
        err @ _ => panic!("Expected OK response, got {:#?}", err),
    };
}
