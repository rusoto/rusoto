#![cfg(feature = "guardduty")]

extern crate env_logger;
extern crate rusoto_core;
extern crate rusoto_guardduty;

use rusoto_core::Region;
use rusoto_guardduty::{GuardDuty, GuardDutyClient, ListInvitationsRequest};

#[tokio::test]
async fn should_list_invitations() {
    let _ = env_logger::try_init();
    let client = GuardDutyClient::new(Region::UsWest2);
    let request = ListInvitationsRequest {
        ..Default::default()
    };

    let result = client.list_invitations(request).await;
    println!("{:#?}", result);
    assert!(result.is_ok());
}
