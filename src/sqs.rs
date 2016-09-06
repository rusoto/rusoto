//! The AWS SQS API.

#![cfg_attr(feature = "nightly-testing", allow(while_let_loop))]

include!(concat!(env!("OUT_DIR"), "/sqs.rs"));

#[cfg(test)]
mod test {
	use sqs::{SqsClient, CreateQueueRequest};
    use super::super::{Region, SignedRequest};
    use super::super::mock::*;

    extern crate env_logger;

    #[test]
    // sample response from the SQS documentation
    fn should_parse_example_create_queue_response() {
        let mock = MockRequestDispatcher::with_status(200)
            .with_body(r#"
				<?xml version="1.0" encoding="UTF-8"?>
				<CreateQueueResponse>
					<CreateQueueResult>
						<QueueUrl>http://queue.amazonaws.com/123456789012/testQueue</QueueUrl>
					</CreateQueueResult>
					<ResponseMetadata>
						<RequestId>7a62c49f-347e-4fc4-9331-6e8e7a96aa73</RequestId>
					</ResponseMetadata>
				</CreateQueueResponse>
            "#)
            .with_request_checker(|request: &SignedRequest| {
                assert_eq!(request.method, "POST");
                assert_eq!(request.path, "/");
                assert_eq!(request.params.get("Action"), Some(&"CreateQueue".to_string()));
                assert_eq!(request.params.get("QueueName"), Some(&"testQueue".to_string()));
                assert_eq!(request.payload, None);
            });

        let client = SqsClient::with_request_dispatcher(mock, MockCredentialsProvider, Region::UsEast1);

        let mut request = CreateQueueRequest::default();
        request.queue_name = "testQueue".to_string();

        let result = client.create_queue(&request).unwrap();
        assert_eq!(result.queue_url, Some("http://queue.amazonaws.com/123456789012/testQueue".to_string()));
    }
}