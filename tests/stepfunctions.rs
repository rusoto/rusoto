#![cfg(feature = "stepfunctions")]
extern crate rusoto;

use rusoto::stepfunctions::{StepFunctionsClient, ListStateMachinesInput};
use rusoto::{DefaultCredentialsProvider, Region};
use rusoto::default_tls_client;

#[test]
fn should_list_state_machines() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = StepFunctionsClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListStateMachinesInput::default();

    let result = client.list_state_machines(&request).unwrap();
    println!("{:#?}", result);
}


