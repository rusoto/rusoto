#![cfg(feature = "budgets")]

extern crate rusoto_core;
extern crate rusoto_budgets;

use rusoto_budgets::{Budgets, BudgetsClient, DescribeBudgetsRequest};
use rusoto_core::{DefaultCredentialsProvider, Region, default_tls_client};

// Switch to DescribeReportDefinitions when botocore is updated?
#[test]
fn should_describe_budgets() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = BudgetsClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    // This request needs the accountId set:
    let request = DescribeBudgetsRequest::default();

    let response = client.describe_budgets(&request).unwrap();
    println!("response: {:?}", response);
}