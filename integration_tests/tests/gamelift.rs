#![cfg(feature = "gamelift")]

extern crate rusoto_core;
extern crate rusoto_gamelift;

use rusoto_gamelift::{GameLift, GameLiftClient, ListFleetsInput};
use rusoto_core::{DefaultCredentialsProvider, Region, default_tls_client};

#[test]
fn should_list_fleets() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = GameLiftClient::new(default_tls_client().unwrap(), credentials, Region::UsWest2);
    let request = ListFleetsInput::default();

    let result = client.list_fleets(&request).unwrap();
	println!("{:#?}", result);
}