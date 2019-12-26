#![cfg(feature = "budgets")]

extern crate rusoto_budgets;
extern crate rusoto_core;

use rusoto_budgets::{Budgets, BudgetsClient, DescribeBudgetsRequest};
use rusoto_core::Region;

// Switch to DescribeReportDefinitions when botocore is updated?
#[tokio::test]
#[ignore] // Ignore until we get this working
async fn should_describe_budgets() {
    let client = BudgetsClient::new(Region::UsEast1);
    // This request needs the accountId set:
    let request = DescribeBudgetsRequest::default();

    let response = client.describe_budgets(request).await.unwrap();
    println!("response: {:?}", response);
}
