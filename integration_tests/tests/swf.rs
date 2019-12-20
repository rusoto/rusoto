#![cfg(feature = "swf")]

extern crate rusoto_core;
extern crate rusoto_swf;

use rusoto_core::Region;
use rusoto_swf::{ListDomainsInput, Swf, SwfClient};

#[tokio::test]
async fn should_list_domains() {
    let client = SwfClient::new(Region::UsEast1);

    let mut request = ListDomainsInput::default();
    request.maximum_page_size = Some(10);
    request.registration_status = "REGISTERED".to_string();

    client.list_domains(request).await.unwrap();
}
