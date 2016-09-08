//! The AWS SQS API.

#![cfg_attr(feature = "nightly-testing", allow(while_let_loop))]

include!(concat!(env!("OUT_DIR"), "/sqs.rs"));

#[cfg(test)]
mod test {
	use std::fs::File;
	use std::io::Read;

	use sqs::{SqsClient, CreateQueueRequest};
    use super::super::{Region, SignedRequest};
    use super::super::mock::*;

    extern crate env_logger;

    #[test]
    // sample response from the SQS documentation
    fn should_parse_example_create_queue_response() {
		let sample_creation_response_location = "./codegen/botocore/tests/unit/response_parsing/xml/responses/sqs-create-queue.xml";
		let mut input_file = File::open(sample_creation_response_location)
			.expect("couldn't find file");

	    let mut mock_response = String::new();

	    input_file.read_to_string(&mut mock_response).expect(&format!(
	        "Failed to read {:?}",
	        sample_creation_response_location,
	    ));

        let mock = MockRequestDispatcher::with_status(200)
            .with_body(&mock_response)
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
        assert_eq!(result.queue_url, Some("http://sqs.us-east-1.amazonaws.com/123456789012/testQueue".to_string()));
    }
}
