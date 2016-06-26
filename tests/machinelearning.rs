#![cfg(feature = "machinelearning")]

extern crate rusoto;

use rusoto::machinelearning::{MachineLearningClient, DescribeDataSourcesInput, DescribeBatchPredictionsInput, DescribeEvaluationsInput};
use rusoto::{DefaultCredentialsProvider, Region};

#[test]
fn should_describe_batch_predictions() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = MachineLearningClient::new(credentials, Region::UsEast1);

    let request = DescribeBatchPredictionsInput::default();

    match client.describe_batch_predictions(&request) {
        Ok(response) => {
            println!("{:#?}", response); 
            assert!(true)            
        },
        Err(err) => panic!("Expected OK response, got {:#?}", err)
    };
}
#[test]
fn should_describe_data_sources() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = MachineLearningClient::new(credentials, Region::UsEast1);

    let request = DescribeDataSourcesInput::default();

    match client.describe_data_sources(&request) {
        Ok(response) => {
            println!("{:#?}", response); 
            assert!(true)            
        },
        Err(err) => panic!("Expected OK response, got {:#?}", err)
    };
}
#[test]
fn should_describe_evaluations() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = MachineLearningClient::new(credentials, Region::UsEast1);

    let request = DescribeEvaluationsInput::default();

    match client.describe_evaluations(&request) {
        Ok(response) => {
            println!("{:#?}", response); 
            assert!(true)            
        },
        Err(err) => panic!("Expected OK response, got {:#?}", err)
    };
}
