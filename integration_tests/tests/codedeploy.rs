#![cfg(feature = "codedeploy")]

extern crate rusoto_codedeploy;
extern crate rusoto_core;

use rusoto_codedeploy::{CodeDeploy, CodeDeployClient, ListApplicationsRequest};
use rusoto_core::Region;

#[test]
fn should_list_applications() {
    let client = CodeDeployClient::new(Region::UsEast1);
    let request = ListApplicationsRequest::default();

    client.list_applications(request).sync().unwrap();
}
