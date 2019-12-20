#![cfg(feature = "machinelearning")]

extern crate rusoto_core;
extern crate rusoto_machinelearning;

use rusoto_core::Region;
use rusoto_machinelearning::{
    DescribeBatchPredictionsInput, DescribeDataSourcesInput, DescribeEvaluationsInput,
    MachineLearning, MachineLearningClient,
};

// This service isn't available for new customers, but existing ones
// should still pass in this test.

#[tokio::test]
async fn should_describe_batch_predictions() {
    let client = MachineLearningClient::new(Region::UsEast1);
    let request = DescribeBatchPredictionsInput::default();

    match client.describe_batch_predictions(request).await {
        Ok(_) => (),
        Err(e) => assert!(e
            .to_string()
            .contains("AmazonML is no longer available to new customers")),
    };
}
#[tokio::test]
async fn should_describe_data_sources() {
    let client = MachineLearningClient::new(Region::UsEast1);
    let request = DescribeDataSourcesInput::default();

    match client.describe_data_sources(request).await {
        Ok(_) => (),
        Err(e) => assert!(e
            .to_string()
            .contains("AmazonML is no longer available to new customers")),
    };
}
#[tokio::test]
async fn should_describe_evaluations() {
    let client = MachineLearningClient::new(Region::UsEast1);
    let request = DescribeEvaluationsInput::default();

    match client.describe_evaluations(request).await {
        Ok(_) => (),
        Err(e) => assert!(e
            .to_string()
            .contains("AmazonML is no longer available to new customers")),
    };
}
