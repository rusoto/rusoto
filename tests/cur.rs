#![cfg(feature = "cur")]
extern crate rusoto;

use rusoto::cur::{CostAndUsageReportClient, DescribeReportDefinitionsRequest};
use rusoto::{DefaultCredentialsProvider, Region, default_tls_client};

#[test]
fn should_describe_report_definitions() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = CostAndUsageReportClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = DescribeReportDefinitionsRequest::default();

    let result = client.describe_report_definitions(&request).unwrap();
	println!("{:#?}", result);
}


