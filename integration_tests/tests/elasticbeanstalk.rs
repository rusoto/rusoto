#![cfg(feature = "elasticbeanstalk")]

extern crate rusoto_core;
extern crate rusoto_elasticbeanstalk;

use rusoto_elasticbeanstalk::{ElasticBeanstalk, ElasticBeanstalkClient,
                               DescribeApplicationsMessage};
use rusoto_core::Region;

#[test]
fn should_describe_applications() {
    let client = ElasticBeanstalkClient::new(Region::UsEast1);
    let request = DescribeApplicationsMessage::default();

    let result = client.describe_applications(request).sync().unwrap();
    println!("{:#?}", result);
}
