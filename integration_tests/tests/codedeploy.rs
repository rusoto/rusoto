#![cfg(feature = "codedeploy")]

extern crate rusoto_codedeploy;
extern crate rusoto_core;

use rusoto_codedeploy::{CodeDeploy, CodeDeployClient, ListApplicationsInput};
use rusoto_core::Region;

#[tokio::test]
async fn should_list_applications() {
    let client = CodeDeployClient::new(Region::UsEast1);
    let request = ListApplicationsInput::default();

    client.list_applications(request).await.unwrap();
}
