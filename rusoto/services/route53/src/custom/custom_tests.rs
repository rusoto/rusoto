use crate::custom::util::quote_txt_record;
use crate::generated::{
    ListResourceRecordSetsError, ListResourceRecordSetsRequest, Route53, Route53Client,
};
use rusoto_core::{Region, RusotoError};

use rusoto_mock::*;

#[tokio::test]
async fn test_parse_no_such_hosted_zone_error() {
    let mock = MockRequestDispatcher::with_status(404).with_body(
        r#"<?xml version="1.0"?>
            <ErrorResponse xmlns="https://route53.amazonaws.com/doc/2013-04-01/">
                <Error>
                    <Type>Sender</Type>
                    <Code>NoSuchHostedZone</Code>
                    <Message>No hosted zone found with ID: NO-SUCH-ZONE</Message>
                </Error>
                <RequestId>20c2984f-279e-11e8-9a16-83e7725d8022</RequestId>
            </ErrorResponse>"#,
    );

    let request = ListResourceRecordSetsRequest {
        hosted_zone_id: "NO-SUCH-ZONE".to_owned(),
        ..Default::default()
    };

    let client = Route53Client::new_with(mock, MockCredentialsProvider, Region::UsEast1);
    let result = client.list_resource_record_sets(request).await;
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(
        RusotoError::Service(ListResourceRecordSetsError::NoSuchHostedZone(
            "No hosted zone found with ID: NO-SUCH-ZONE".to_owned()
        )),
        err
    );
}

#[test]
fn test_quoter() {
    assert_eq!(quote_txt_record("foo"), "\"foo\"");
    assert_eq!(quote_txt_record("\"foo\""), "\"foo\"");
    assert_eq!(quote_txt_record("\"foo"), "\"foo\"");
    assert_eq!(quote_txt_record("foo\""), "\"foo\"");
}
