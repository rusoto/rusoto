#![cfg(feature = "comprehendmedical")]

extern crate rusoto_comprehendmedical;
extern crate rusoto_core;

use rusoto_comprehendmedical::{ComprehendMedical, ComprehendMedicalClient, DetectPHIRequest};
use rusoto_core::Region;

#[tokio::test]
async fn should_detect_phi() {
    let client = ComprehendMedicalClient::new(Region::UsEast1);

    let request = DetectPHIRequest {
        text: "patient age is 42".to_owned(),
        ..Default::default()
    };

    match client.detect_phi(request).await {
        Ok(response) => {
            println!("{:#?}", response);
            assert_eq!(1, response.entities.len());
        }
        Err(err) => panic!("Expected OK response, got {:#?}", err),
    };
}
