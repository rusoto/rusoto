#![cfg(feature = "servicecatalog")]

extern crate rusoto_core;
extern crate rusoto_servicecatalog;

use rusoto_core::Region;
use rusoto_servicecatalog::{ListPortfoliosInput, ServiceCatalog, ServiceCatalogClient};

#[test]
fn should_list_portfolios() {
    let client = ServiceCatalogClient::new(Region::UsEast1);
    let request = ListPortfoliosInput::default();

    let result = client.list_portfolios(request).sync().unwrap();
    println!("{:#?}", result);
}
