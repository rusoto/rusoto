#![cfg(feature = "inspector")]

extern crate rusoto_core;
extern crate rusoto_inspector;

use rusoto_inspector::{Inspector, InspectorClient, ListAssessmentRunsRequest};
use rusoto_core::{DefaultCredentialsProvider, Region};
use rusoto_core::default_tls_client;

#[test]
fn should_list_assessment_runs() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = InspectorClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListAssessmentRunsRequest::default();

    client.list_assessment_runs(&request).sync().unwrap();
}
