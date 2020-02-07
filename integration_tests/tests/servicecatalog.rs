#![cfg(feature = "servicecatalog")]

extern crate rusoto_core;
extern crate rusoto_servicecatalog;

use rusoto_core::Region;
use rusoto_servicecatalog::{ListPortfoliosInput, ServiceCatalog, ServiceCatalogClient};

#[tokio::test]
async fn should_list_portfolios() {
    let client = ServiceCatalogClient::new(Region::UsEast1);
    let request = ListPortfoliosInput::default();

    let result = client.list_portfolios(request).await.unwrap();
    println!("{:#?}", result);
}
