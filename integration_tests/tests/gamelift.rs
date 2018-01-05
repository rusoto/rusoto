#![cfg(feature = "gamelift")]

extern crate rusoto_core;
extern crate rusoto_gamelift;

use rusoto_gamelift::{GameLift, GameLiftClient, ListFleetsInput};
use rusoto_core::Region;

#[test]
fn should_list_fleets() {
    let client = GameLiftClient::simple(Region::UsWest2);
    let request = ListFleetsInput::default();

    let result = client.list_fleets(&request).sync().unwrap();
	println!("{:#?}", result);
}