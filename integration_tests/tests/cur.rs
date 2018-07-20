#![cfg(feature = "cur")]

extern crate rusoto_core;
extern crate rusoto_cur;

use rusoto_cur::{CostAndUsageReport, CostAndUsageReportClient, DescribeReportDefinitionsRequest};
use rusoto_core::Region;

#[test]
fn should_describe_report_definitions() {
    let client = CostAndUsageReportClient::new(Region::UsEast1);
    let request = DescribeReportDefinitionsRequest::default();

    let result = client.describe_report_definitions(request).sync().unwrap();
	println!("{:#?}", result);
}