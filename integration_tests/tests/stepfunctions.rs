#![cfg(feature = "stepfunctions")]

extern crate rusoto_core;
extern crate rusoto_stepfunctions;

use rusoto_stepfunctions::{StepFunctions, StepFunctionsClient, ListStateMachinesInput};
use rusoto_core::{DefaultCredentialsProvider, Region};
use rusoto_core::default_tls_client;

#[test]
fn should_list_state_machines() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = StepFunctionsClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListStateMachinesInput::default();

    let result = client.list_state_machines(&request).sync().unwrap();
    println!("{:#?}", result);
}
