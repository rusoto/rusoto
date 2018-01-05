#![cfg(feature = "machinelearning")]

extern crate rusoto_core;
extern crate rusoto_machinelearning;

use rusoto_machinelearning::{MachineLearning, MachineLearningClient,
                              DescribeDataSourcesInput, DescribeBatchPredictionsInput,
                              DescribeEvaluationsInput};
use rusoto_core::Region;

#[test]
fn should_describe_batch_predictions() {
    let client = MachineLearningClient::simple(Region::UsEast1);
    let request = DescribeBatchPredictionsInput::default();

    client.describe_batch_predictions(&request).sync().unwrap();
}
#[test]
fn should_describe_data_sources() {
    let client = MachineLearningClient::simple(Region::UsEast1);
    let request = DescribeDataSourcesInput::default();

    client.describe_data_sources(&request).sync().unwrap();
}
#[test]
fn should_describe_evaluations() {
    let client = MachineLearningClient::simple(Region::UsEast1);

    let request = DescribeEvaluationsInput::default();

    client.describe_evaluations(&request).sync().unwrap();
}
