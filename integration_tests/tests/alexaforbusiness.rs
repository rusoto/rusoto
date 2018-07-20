#![cfg(feature = "alexaforbusiness")]

extern crate rusoto_core;
extern crate rusoto_alexaforbusiness;

use rusoto_alexaforbusiness::{AlexaForBusiness, AlexaForBusinessClient, ListSkillsRequest};
use rusoto_core::Region;

#[test]
fn should_list_skills() {
    let client = AlexaForBusinessClient::new(Region::UsEast1);
    let request = ListSkillsRequest::default();

    let response = client.list_skills(request).sync();
    println!("response is {:#?}", response);
    // expected failure for accounts without access to Alexa for Business:
    match response {
        Ok(_) => (),
        Err(e) => {
            assert!(format!("{}", e).contains("Organization does not exist for the given aws account"))
        },
    }
}
