#![cfg(feature = "servicecatalog")]
extern crate rusoto;
extern crate env_logger;

use rusoto::servicecatalog::{ServiceCatalogClient, ListPortfoliosInput};
use rusoto::{DefaultCredentialsProvider, Region, default_tls_client};

#[test]
fn should_list_portfolios() {
	let _ = env_logger::init();
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = ServiceCatalogClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListPortfoliosInput {
//    	page_size: Some(10),
    	..Default::default()
    };

    let result = client.list_portfolios(&request).unwrap();
	println!("{:#?}", result);
}