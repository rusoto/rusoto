#![cfg(feature = "codecommit")]

extern crate rusoto_codecommit;
extern crate rusoto_core;

use rusoto_codecommit::{CodeCommit, CodeCommitClient, ListRepositoriesRequest};
use rusoto_core::Region;

#[test]
fn should_list_repositories() {
    let client = CodeCommitClient::new(Region::UsEast1);
    let request = ListRepositoriesRequest::default();

    client.list_repositories(request).sync().unwrap();
}
