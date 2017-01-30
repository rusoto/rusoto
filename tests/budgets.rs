#![cfg(feature = "budgets")]

extern crate rusoto;
extern crate env_logger;

use rusoto::budgets::{BudgetsClient, DescribeBudgetsRequest, DescribeBudgetsError};
use rusoto::{DefaultCredentialsProvider, Region, default_tls_client};

#[test]
fn should_describe_budgets() {
	let _ = env_logger::init();
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = BudgetsClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = DescribeBudgetsRequest::default();

    // all methods in the budgets api require the account ID as part of the request
    // which makes integration testing tricky for just this service.  Instead ensure
    // that the expected error is returned when it isn't provided

    let result = client.describe_budgets(&request);
	match result {
		// the raw error indicates this is access denied due to the account ID not matching
		Err(DescribeBudgetsError::Unknown(_)) => println!("Got expected error"),
		_ => panic!("Didn't get expected error")
	};
	println!("{:#?}", result);
}


