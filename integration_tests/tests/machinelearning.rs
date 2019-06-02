#![cfg(feature = "machinelearning")]

extern crate rusoto_core;
extern crate rusoto_machinelearning;

use rusoto_core::Region;
use rusoto_machinelearning::{
    DescribeBatchPredictionsRequest, DescribeDataSourcesRequest, DescribeEvaluationsRequest,
    MachineLearning, MachineLearningClient,
};

// This service isn't available for new customers, but existing ones
// should still pass in this test.

#[test]
fn should_describe_batch_predictions() {
    let client = MachineLearningClient::new(Region::UsEast1);
    let request = DescribeBatchPredictionsRequest::default();

    match client.describe_batch_predictions(request).sync() {
        Ok(_) => (),
        Err(e) => assert!(e.to_string().contains("AmazonML is no longer available to new customers")),
    };
}
#[test]
fn should_describe_data_sources() {
    let client = MachineLearningClient::new(Region::UsEast1);
    let request = DescribeDataSourcesRequest::default();

    match client.describe_data_sources(request).sync() {
        Ok(_) => (),
        Err(e) => assert!(e.to_string().contains("AmazonML is no longer available to new customers")),
    };
}
#[test]
fn should_describe_evaluations() {
    let client = MachineLearningClient::new(Region::UsEast1);
    let request = DescribeEvaluationsRequest::default();

    match client.describe_evaluations(request).sync() {
        Ok(_) => (),
        Err(e) => assert!(e.to_string().contains("AmazonML is no longer available to new customers")),
    };
}
