#![cfg(feature = "budgets")]

extern crate rusoto_core;
extern crate rusoto_budgets;

use rusoto_budgets::{Budgets, BudgetsClient, DescribeBudgetsRequest};
use rusoto_core::Region;

// Switch to DescribeReportDefinitions when botocore is updated?
#[test]
#[ignore] // Ignore until we get this working
fn should_describe_budgets() {
    let client = BudgetsClient::simple(Region::UsEast1);
    // This request needs the accountId set:
    let request = DescribeBudgetsRequest::default();

    let response = client.describe_budgets(&request).sync().unwrap();
    println!("response: {:?}", response);
}