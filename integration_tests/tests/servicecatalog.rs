#![cfg(feature = "servicecatalog")]

extern crate rusoto_core;
extern crate rusoto_servicecatalog;

use rusoto_servicecatalog::{ServiceCatalog, ServiceCatalogClient, ListPortfoliosInput};
use rusoto_core::{DefaultCredentialsProvider, Region, default_tls_client};

#[test]
fn should_list_portfolios() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = ServiceCatalogClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListPortfoliosInput::default();

    let result = client.list_portfolios(&request).unwrap();
	println!("{:#?}", result);
}