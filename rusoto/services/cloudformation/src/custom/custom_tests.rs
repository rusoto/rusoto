extern crate rusoto_mock;

use crate::generated::{CloudFormation, CloudFormationClient, ListStacksInput};

use self::rusoto_mock::*;
use rusoto_core::param::Params;
use rusoto_core::signature::SignedRequest;
use rusoto_core::signature::SignedRequestPayload;
use rusoto_core::Region;
use serde_urlencoded;

#[tokio::test]
async fn should_serialize_list_parameters_in_request_body() {
    let mock = MockRequestDispatcher::with_status(200)
        .with_body(
            r#"<?xml version="1.0" encoding="UTF-8"?>
        <ListStacksResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
            <ListStacksResult>
            <StackSummaries>
                <member>
                <StackId>
                    arn:aws:cloudformation:us-east-1:1234567:stack/TestCreate1/aaaaa
                </StackId>
                <StackStatus>CREATE_IN_PROGRESS</StackStatus>
                <StackName>vpc1</StackName>
                <CreationTime>2011-05-23T15:47:44Z</CreationTime>
                <TemplateDescription>
                    Creates one EC2 instance and a load balancer.
                </TemplateDescription>
                </member>
                <member>
                <StackId>
                    arn:aws:cloudformation:us-east-1:1234567:stack/TestDelete2/bbbbb
                </StackId>
                <StackStatus>DELETE_COMPLETE</StackStatus>
                <DeletionTime>2011-03-10T16:20:51Z</DeletionTime>
                <StackName>WP1</StackName>
                <CreationTime>2011-03-05T19:57:58Z</CreationTime>
                <TemplateDescription>
                    A simple basic Cloudformation Template.
                </TemplateDescription>
                </member>
            </StackSummaries>
            </ListStacksResult>
            <ResponseMetadata>
            <RequestId>b9b4b068-3a41-11e5-94eb-example</RequestId>
            </ResponseMetadata>
        </ListStacksResponse>"#,
        )
        .with_request_checker(|request: &SignedRequest| {
            assert_eq!("POST", request.method);
            assert_eq!("/", request.path);
            if let Some(SignedRequestPayload::Buffer(ref buffer)) = request.payload {
                let params: Params = serde_urlencoded::from_bytes(buffer).unwrap();
                assert_eq!(
                    Some(&Some("CREATE_IN_PROGRESS".to_owned())),
                    params.get("StackStatusFilter.member.1")
                );
                assert_eq!(
                    Some(&Some("DELETE_COMPLETE".to_owned())),
                    params.get("StackStatusFilter.member.2")
                );
            } else {
                panic!("Unexpected request.payload: {:?}", request.payload);
            }
        });

    let filters = vec![
        "CREATE_IN_PROGRESS".to_owned(),
        "DELETE_COMPLETE".to_owned(),
    ];
    let request = ListStacksInput {
        stack_status_filter: Some(filters),
        ..Default::default()
    };

    let client = CloudFormationClient::new_with(mock, MockCredentialsProvider, Region::UsEast1);
    let _result = client.list_stacks(request).await.unwrap();
}