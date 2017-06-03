#![cfg(feature = "inspector")]

extern crate rusoto;

use rusoto::inspector::{Inspector, InspectorClient, ListAssessmentRunsRequest};
use rusoto::{DefaultCredentialsProvider, Region};
use rusoto::default_tls_client;

#[test]
fn should_list_assessment_runs() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = InspectorClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListAssessmentRunsRequest::default();

    client.list_assessment_runs(&request).unwrap();
}
