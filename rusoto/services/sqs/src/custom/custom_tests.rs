extern crate rusoto_mock;

use crate::generated::{
    GetQueueUrlError, GetQueueUrlRequest, MessageAttributeValue, ReceiveMessageRequest,
    SendMessageRequest, Sqs, SqsClient,
};
use std::collections::HashMap;

use self::rusoto_mock::*;
use rusoto_core::param::Params;
use rusoto_core::signature::SignedRequest;
use rusoto_core::signature::SignedRequestPayload;
use rusoto_core::{Region, RusotoError};
use serde_urlencoded;

#[tokio::test]
async fn should_serialize_map_parameters_in_request_body() {
    let mock = MockRequestDispatcher::with_status(200)
        .with_body(
            r#"<?xml version="1.0" encoding="UTF-8"?>
        <SendMessageResponse>
            <SendMessageResult>
                <MD5OfMessageBody>
                    fafb00f5732ab283681e124bf8747ed1
                </MD5OfMessageBody>
                <MD5OfMessageAttributes>
            3ae8f24a165a8cedc005670c81a27295
                </MD5OfMessageAttributes>
                <MessageId>
                    5fea7756-0ea4-451a-a703-a558b933e274
                </MessageId>
            </SendMessageResult>
            <ResponseMetadata>
                <RequestId>
                    27daac76-34dd-47df-bd01-1f6e873584a0
                </RequestId>
            </ResponseMetadata>
        </SendMessageResponse>"#,
        )
        .with_request_checker(|request: &SignedRequest| {
            println!("{:#?}", request.params);

            assert_eq!("POST", request.method);
            assert_eq!("/", request.path);
            if let Some(SignedRequestPayload::Buffer(ref buffer)) = request.payload {
                let params: Params = serde_urlencoded::from_bytes(buffer).unwrap();
                assert_eq!(
                    Some(&Some("test_attribute_name".to_owned())),
                    params.get("MessageAttribute.1.Name")
                );
                assert_eq!(
                    Some(&Some("test_attribute_value".to_owned())),
                    params.get("MessageAttribute.1.Value.StringValue")
                );
                assert_eq!(
                    Some(&Some("String".to_owned())),
                    params.get("MessageAttribute.1.Value.DataType")
                );
            } else {
                panic!("Unexpected request.payload: {:?}", request.payload);
            }
        });

    let mut message_attributes = HashMap::new();

    message_attributes.insert(
        "test_attribute_name".to_owned(),
        MessageAttributeValue {
            string_value: Some("test_attribute_value".to_owned()),
            data_type: "String".to_owned(),
            ..Default::default()
        },
    );
    let request = SendMessageRequest {
        message_body: "foo".to_owned(),
        queue_url: "bar".to_owned(),
        message_attributes: Some(message_attributes),
        ..Default::default()
    };

    let client = SqsClient::new_with(mock, MockCredentialsProvider, Region::UsEast1);
    let _result = client.send_message(request).await.unwrap();
}

#[tokio::test]
async fn should_fix_issue_323() {
    let mock = MockRequestDispatcher::with_status(200)
        .with_body(
            r#"<?xml version="1.0" encoding="UTF-8"?>
        <ReceiveMessageResponse>
            <ReceiveMessageResult>
            <Message>
                <MessageId>
                5fea7756-0ea4-451a-a703-a558b933e274
                </MessageId>
                <ReceiptHandle>
                MbZj6wDWli+JvwwJaBV+3dcjk2YW2vA3+STFFljTM8tJJg6HRG6PYSasuWXPJB+Cw
                Lj1FjgXUv1uSj1gUPAWV66FU/WeR4mq2OKpEGYWbnLmpRCJVAyeMjeU5ZBdtcQ+QE
                auMZc8ZRv37sIW2iJKq3M9MFx1YvV11A2x/KSbkJ0=
                </ReceiptHandle>
                <MD5OfBody>
                fafb00f5732ab283681e124bf8747ed1
                </MD5OfBody>
                <Body>This is a test message</Body>
                <Attribute>
                <Name>SenderId</Name>
                <Value>195004372649</Value>
                </Attribute>
            </Message>
            </ReceiveMessageResult>
            <ResponseMetadata>
            <RequestId>
                b6633655-283d-45b4-aee4-4e84e0ae6afa
            </RequestId>
            </ResponseMetadata>
        </ReceiveMessageResponse>"#,
        )
        .with_request_checker(|request: &SignedRequest| {
            assert_eq!("POST", request.method);
            assert_eq!("/", request.path);
            if let Some(SignedRequestPayload::Buffer(ref buffer)) = request.payload {
                let params: Params = serde_urlencoded::from_bytes(buffer).unwrap();
                assert_eq!(
                    params.get("Action"),
                    Some(&Some("ReceiveMessage".to_owned()))
                );
                assert_eq!(
                    params.get("MaxNumberOfMessages"),
                    Some(&Some("1".to_owned()))
                );
                assert_eq!(params.get("VisibilityTimeout"), Some(&Some("2".to_owned())));
                assert_eq!(params.get("WaitTimeSeconds"), Some(&Some("3".to_owned())));
                assert_eq!(params.get("Integer"), None);
            } else {
                panic!("Unexpected request.payload: {:?}", request.payload);
            }
        });

    let request = ReceiveMessageRequest {
        max_number_of_messages: Some(1),
        queue_url: "foo".to_owned(),
        visibility_timeout: Some(2),
        wait_time_seconds: Some(3),
        ..Default::default()
    };

    let client = SqsClient::new_with(mock, MockCredentialsProvider, Region::UsEast1);
    let _result = client.receive_message(request).await.unwrap();
}

#[tokio::test]
async fn test_parse_queue_does_not_exist_error() {
    let mock = MockRequestDispatcher::with_status(400).with_body(
        r#"<?xml version="1.0"?>
        <ErrorResponse xmlns="http://queue.amazonaws.com/doc/2012-11-05/">
            <Error>
                <Type>Sender</Type>
                <Code>AWS.SimpleQueueService.NonExistentQueue</Code>
                <Message>The specified queue does not exist for this wsdl version.</Message>
                <Detail/>
            </Error>
            <RequestId>8f8f9957-c0d9-536a-9ca6-ca7483be06ad</RequestId>
        </ErrorResponse>"#,
    );

    let request = GetQueueUrlRequest {
        queue_name: "no-such-queue".to_owned(),
        ..Default::default()
    };

    let client = SqsClient::new_with(mock, MockCredentialsProvider, Region::UsEast1);
    let result = client.get_queue_url(request).await;
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(
        RusotoError::Service(GetQueueUrlError::QueueDoesNotExist(
            "The specified queue does not exist for this wsdl version.".to_owned()
        )),
        err
    );
}
