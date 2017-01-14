#![cfg(feature = "inspector")]

extern crate rusoto;

use rusoto::inspector::{InspectorClient, ListAssessmentRunsRequest};
use rusoto::{DefaultCredentialsProvider, Region};

#[test]
fn should_list_assessment_runs() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = InspectorClient::new(credentials, Region::UsEast1).unwrap();
    let request = ListAssessmentRunsRequest::default();

    client.list_assessment_runs(&request).unwrap();
}
