#![cfg(feature = "codecommit")]

extern crate rusoto;

use rusoto::codecommit::{CodeCommitClient, ListRepositoriesInput};
use rusoto::{DefaultCredentialsProvider, Region};

#[test]
fn should_list_repositories() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = CodeCommitClient::new(credentials, Region::UsEast1).unwrap();
    let request = ListRepositoriesInput::default();

    client.list_repositories(&request).unwrap();
}
