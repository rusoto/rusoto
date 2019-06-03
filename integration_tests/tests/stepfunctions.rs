#![cfg(feature = "stepfunctions")]

extern crate rusoto_core;
extern crate rusoto_stepfunctions;

use rusoto_core::Region;
use rusoto_stepfunctions::{ListStateMachinesRequest, StepFunctions, StepFunctionsClient};

#[test]
fn should_list_state_machines() {
    let client = StepFunctionsClient::new(Region::UsEast1);
    let request = ListStateMachinesRequest::default();

    let result = client.list_state_machines(request).sync().unwrap();
    println!("{:#?}", result);
}
