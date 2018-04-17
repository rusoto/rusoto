#![cfg(feature = "serverlessrepo")]
extern crate rusoto_core;
extern crate rusoto_serverlessrepo;

use rusoto_serverlessrepo::{ServerlessRepo, ServerlessRepoClient, ListApplicationsRequest};
use rusoto_core::Region;

#[test]
fn should_list_applications() {
    let client = ServerlessRepoClient::simple(Region::UsEast1);
    let request = ListApplicationsRequest::default();

    let result = client.list_applications(request).sync();
    println!("{:#?}", result);
    assert!(result.is_ok());
}
