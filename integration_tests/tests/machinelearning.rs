#![cfg(feature = "machinelearning")]

extern crate rusoto_core;
extern crate rusoto_machinelearning;

use rusoto_core::Region;
use rusoto_machinelearning::{
    DescribeBatchPredictionsInput, DescribeDataSourcesInput, DescribeEvaluationsInput,
    MachineLearning, MachineLearningClient,
};

#[test]
fn should_describe_batch_predictions() {
    let client = MachineLearningClient::new(Region::UsEast1);
    let request = DescribeBatchPredictionsInput::default();

    client.describe_batch_predictions(request).sync().unwrap();
}
#[test]
fn should_describe_data_sources() {
    let client = MachineLearningClient::new(Region::UsEast1);
    let request = DescribeDataSourcesInput::default();

    client.describe_data_sources(request).sync().unwrap();
}
#[test]
fn should_describe_evaluations() {
    let client = MachineLearningClient::new(Region::UsEast1);

    let request = DescribeEvaluationsInput::default();

    client.describe_evaluations(request).sync().unwrap();
}
