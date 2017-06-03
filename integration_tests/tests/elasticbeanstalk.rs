#![cfg(feature = "elasticbeanstalk")]

extern crate rusoto;

use rusoto::elasticbeanstalk::{ElasticBeanstalk, ElasticBeanstalkClient,
                               DescribeApplicationsMessage};
use rusoto::{DefaultCredentialsProvider, Region};
use rusoto::default_tls_client;

#[test]
fn should_describe_applications() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client =
        ElasticBeanstalkClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = DescribeApplicationsMessage::default();

    let result = client.describe_applications(&request).unwrap();
    println!("{:#?}", result);
}
