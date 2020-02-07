#![cfg(feature = "serverlessrepo")]
extern crate rusoto_core;
extern crate rusoto_serverlessrepo;

use rusoto_core::Region;
use rusoto_serverlessrepo::{ListApplicationsRequest, ServerlessRepo, ServerlessRepoClient};

#[tokio::test]
async fn should_list_applications() {
    let client = ServerlessRepoClient::new(Region::UsEast1);
    let request = ListApplicationsRequest::default();

    let result = client.list_applications(request).await;
    println!("{:#?}", result);
    assert!(result.is_ok());
}
