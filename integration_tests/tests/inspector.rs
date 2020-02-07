#![cfg(feature = "inspector")]

extern crate rusoto_core;
extern crate rusoto_inspector;

use rusoto_core::Region;
use rusoto_inspector::{Inspector, InspectorClient, ListAssessmentRunsRequest};

#[tokio::test]
async fn should_list_assessment_runs() {
    let client = InspectorClient::new(Region::UsEast1);
    let request = ListAssessmentRunsRequest::default();

    client.list_assessment_runs(request).await.unwrap();
}
