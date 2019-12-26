#![cfg(feature = "sagemaker")]

extern crate rusoto_core;
extern crate rusoto_sagemaker;

use rusoto_core::Region;
use rusoto_sagemaker::{ListModelsInput, SageMaker, SageMakerClient};

#[tokio::test]
async fn main() {
    let sm = SageMakerClient::new(Region::UsEast1);
    let req = ListModelsInput::default();
    let result = sm.list_models(req).await.unwrap();
    println!("Got models: {:?}", result);
}
