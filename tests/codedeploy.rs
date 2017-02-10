#![cfg(feature = "codedeploy")]

extern crate rusoto;

use rusoto::codedeploy::{CodeDeployClient, ListApplicationsInput};
use rusoto::{DefaultCredentialsProvider, Region};
use rusoto::default_tls_client;

#[test]
fn should_list_applications() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = CodeDeployClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListApplicationsInput::default();

    client.list_applications(&request).unwrap();
}
