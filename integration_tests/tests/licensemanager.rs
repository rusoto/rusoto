#![cfg(feature = "license-manager")]

extern crate rusoto_core;
extern crate rusoto_license_manager;

use rusoto_core::{Region, RusotoError};
use rusoto_license_manager::{LicenseManager, LicenseManagerClient};

#[tokio::test]
async fn should_list_domains() {
    let client = LicenseManagerClient::new(Region::UsEast1);

    let result = client.get_service_settings().await;
    println!("{:#?}", result);
    match result {
        Ok(_) => (),
        Err(e) => {
            match e {
                RusotoError::Service(err) => {
                    assert!(format!("{:?}", err).contains("Denied"));
                }
                _ => (),
            };
        }
    }
}
