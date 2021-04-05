#![cfg(feature = "securityhub")]

extern crate env_logger;
extern crate rusoto_core;
extern crate rusoto_securityhub;

use rusoto_core::Region;
use rusoto_securityhub::{ListInvitationsRequest, SecurityHub, SecurityHubClient};

#[tokio::test]
#[ignore]
async fn should_list_invitations() {
    let _ = env_logger::try_init();
    let client = SecurityHubClient::new(Region::UsWest2);
    let request = ListInvitationsRequest {
        ..Default::default()
    };

    client.list_invitations(request).await.unwrap();
}
