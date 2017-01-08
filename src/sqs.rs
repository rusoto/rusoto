//! The AWS SQS API.

#![cfg_attr(feature = "nightly-testing", allow(while_let_loop))]

include!(concat!(env!("OUT_DIR"), "/sqs.rs"));

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use sqs::{SqsClient, SendMessageRequest, MessageAttributeValue};

    use super::super::{Region, SignedRequest};
    use super::super::mock::*;

    extern crate env_logger;

    #[test]
    fn should_serialize_map_parameters_in_query_string() {
        let mock = MockRequestDispatcher::with_status(200)
            .with_body(r#"<?xml version="1.0" encoding="UTF-8"?>
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
			</SendMessageResponse>"#)
            .with_request_checker(|request: &SignedRequest| {
                assert_eq!("POST", request.method);
                assert_eq!("/", request.path);
                assert_eq!(None, request.payload);
                assert_eq!(Some(&Some("test_attribute_name".to_owned())),
                           request.params.get("MessageAttribute.1.Name"));
                assert_eq!(Some(&Some("test_attribute_value".to_owned())),
                           request.params.get("MessageAttribute.1.Value.StringValue"));
                assert_eq!(Some(&Some("String".to_owned())),
                           request.params.get("MessageAttribute.1.Value.DataType"));
            });

        let mut message_attributes = HashMap::new();

        message_attributes.insert("test_attribute_name".to_owned(),
                                  MessageAttributeValue {
                                      string_value: Some("test_attribute_value".to_owned()),
                                      data_type: "String".to_owned(),
                                      ..Default::default()
                                  });
        let request = SendMessageRequest {
            message_body: "foo".to_owned(),
            queue_url: "bar".to_owned(),
            message_attributes: Some(message_attributes),
            ..Default::default()
        };

        let client =
            SqsClient::with_request_dispatcher(mock, MockCredentialsProvider, Region::UsEast1);
        let _result = client.send_message(&request).unwrap();
    }
}
