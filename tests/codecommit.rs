#![cfg(feature = "codecommit")]

extern crate rusoto;

use rusoto::codecommit::{CodeCommit, CodeCommitClient, ListRepositoriesInput};
use rusoto::{DefaultCredentialsProvider, Region};
use rusoto::default_tls_client;

#[test]
fn should_list_repositories() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = CodeCommitClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListRepositoriesInput::default();

    client.list_repositories(&request).unwrap();
}
