#![cfg(feature = "stepfunctions")]

extern crate rusoto_core;
extern crate rusoto_stepfunctions;

use rusoto_core::Region;
use rusoto_stepfunctions::{ListStateMachinesInput, StepFunctions, StepFunctionsClient};

#[tokio::test]
async fn should_list_state_machines() {
    let client = StepFunctionsClient::new(Region::UsEast1);
    let request = ListStateMachinesInput::default();

    let result = client.list_state_machines(request).await.unwrap();
    println!("{:#?}", result);
}
