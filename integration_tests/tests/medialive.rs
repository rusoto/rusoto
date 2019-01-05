#![cfg(feature = "medialive")]

extern crate rusoto_core;
extern crate rusoto_medialive;

use rusoto_core::Region;
use rusoto_medialive::{ListChannelsRequest, MediaLive, MediaLiveClient};

#[test]
fn should_list_channels() {
    let client = MediaLiveClient::new(Region::UsEast1);
    let request = ListChannelsRequest::default();

    match client.list_channels(request).sync() {
        Ok(resp) => println!("Got success response of {:?}", resp),
        Err(err) => panic!("Should get list of channels, got: {:?}", err),
    }
}
