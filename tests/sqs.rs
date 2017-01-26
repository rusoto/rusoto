#![cfg(feature = "sqs")]

extern crate rusoto;

use rusoto::sqs::SqsClient;
use rusoto::sqs::ListQueuesRequest;
use rusoto::{DefaultCredentialsProvider, Region};
use rusoto::default_tls_client;

#[test]
fn list_queues() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let sqs = SqsClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);

    let request = ListQueuesRequest {
        ..Default::default()
    };

    let result = sqs.list_queues(&request).unwrap();
    println!("{:#?}", result);
}
