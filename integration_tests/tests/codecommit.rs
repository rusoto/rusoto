#![cfg(feature = "codecommit")]

extern crate rusoto_codecommit;
extern crate rusoto_core;

use rusoto_codecommit::{CodeCommit, CodeCommitClient, ListRepositoriesInput};
use rusoto_core::Region;

#[tokio::test]
async fn should_list_repositories() {
    let client = CodeCommitClient::new(Region::UsEast1);
    let request = ListRepositoriesInput::default();

    client.list_repositories(request).await.unwrap();
}
