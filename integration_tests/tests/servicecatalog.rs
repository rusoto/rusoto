#![cfg(feature = "servicecatalog")]

extern crate rusoto_core;
extern crate rusoto_servicecatalog;

use rusoto_servicecatalog::{ServiceCatalog, ServiceCatalogClient, ListPortfoliosInput};
use rusoto_core::Region;

#[test]
fn should_list_portfolios() {
    let client = ServiceCatalogClient::new(Region::UsEast1);
    let request = ListPortfoliosInput::default();

    let result = client.list_portfolios(request).sync().unwrap();
	println!("{:#?}", result);
}