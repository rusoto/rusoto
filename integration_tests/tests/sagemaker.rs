#![cfg(feature = "sagemaker")]

extern crate rusoto_core;
extern crate rusoto_sagemaker;

use rusoto_sagemaker::{SageMaker, SageMakerClient, ListModelsInput};
use rusoto_core::Region;

#[test]
fn main() {
    let sm = SageMakerClient::new(Region::UsEast1);
    let req = ListModelsInput::default();
    let result = sm.list_models(req).sync().unwrap();
    println!("Got models: {:?}", result);
}
