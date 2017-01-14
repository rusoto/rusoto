#![cfg(feature = "sqs")]

extern crate rusoto;

use rusoto::sqs::SqsClient;
use rusoto::sqs::ListQueuesRequest;
use rusoto::{DefaultCredentialsProvider, Region};
use rusoto::default_tls_client;

#[test]
fn list_queues() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let sqs = SqsClient::new(default_tls_client().unwrap(), credentials, Region::EuWest1);

    // http://docs.aws.amazon.com/AWSSimpleQueueService/latest/APIReference/Welcome.html
    let request = ListQueuesRequest {
        ..Default::default()
    };
    sqs.list_queues(&request).unwrap();
}
