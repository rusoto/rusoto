#![cfg(feature = "servicecatalog")]

extern crate rusoto_core;
extern crate rusoto_servicecatalog;

use rusoto_core::Region;
use rusoto_servicecatalog::{ListPortfoliosRequest, ServiceCatalog, ServiceCatalogClient};

#[test]
fn should_list_portfolios() {
    let client = ServiceCatalogClient::new(Region::UsEast1);
    let request = ListPortfoliosRequest::default();

    let result = client.list_portfolios(request).sync().unwrap();
    println!("{:#?}", result);
}
