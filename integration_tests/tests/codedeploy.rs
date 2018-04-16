#![cfg(feature = "codedeploy")]

extern crate rusoto_core;
extern crate rusoto_codedeploy;

use rusoto_codedeploy::{CodeDeploy, CodeDeployClient, ListApplicationsInput};
use rusoto_core::Region;

#[test]
fn should_list_applications() {
    let client = CodeDeployClient::simple(Region::UsEast1);
    let request = ListApplicationsInput::default();

    client.list_applications(request).sync().unwrap();
}
