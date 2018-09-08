#![cfg(feature = "guardduty")]

extern crate rusoto_core;
extern crate rusoto_guardduty;
extern crate env_logger;

use rusoto_guardduty::{GuardDuty, GuardDutyClient, ListInvitationsRequest};
use rusoto_core::Region;

#[test]
fn should_list_invitations() {
    let _ = env_logger::try_init();
    let client = GuardDutyClient::new(Region::UsWest2);
    let request = ListInvitationsRequest{
        ..Default::default()
    };

    let result = client.list_invitations(request).sync();
    println!("{:#?}", result);
    assert!(result.is_ok());
}