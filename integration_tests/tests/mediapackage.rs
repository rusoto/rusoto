#![cfg(feature = "mediapackage")]

extern crate rusoto_core;
extern crate rusoto_mediapackage;

use rusoto_core::Region;
use rusoto_mediapackage::{ListOriginEndpointsRequest, MediaPackage, MediaPackageClient};

#[tokio::test]
async fn should_list_origin_endpoints() {
    let client = MediaPackageClient::new(Region::UsEast1);
    let request = ListOriginEndpointsRequest::default();

    match client.list_origin_endpoints(request).await {
        Ok(resp) => println!("Got success response of {:?}", resp),
        Err(err) => panic!("Should get list of origin endpoints, got: {:?}", err),
    }
}
