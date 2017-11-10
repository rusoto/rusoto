#![cfg(feature = "codecommit")]

extern crate rusoto_core;
extern crate rusoto_codecommit;

use rusoto_codecommit::{CodeCommit, CodeCommitClient, ListRepositoriesInput};
use rusoto_core::{DefaultCredentialsProvider, Region};
use rusoto_core::default_tls_client;

#[test]
fn should_list_repositories() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = CodeCommitClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListRepositoriesInput::default();

    client.list_repositories(&request).sync().unwrap();
}
