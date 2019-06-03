#![cfg(feature = "elasticbeanstalk")]

extern crate rusoto_core;
extern crate rusoto_elasticbeanstalk;

use rusoto_core::Region;
use rusoto_elasticbeanstalk::{
    DescribeApplicationsRequest, ElasticBeanstalk, ElasticBeanstalkClient,
};

#[test]
fn should_describe_applications() {
    let client = ElasticBeanstalkClient::new(Region::UsEast1);
    let request = DescribeApplicationsRequest::default();

    let result = client.describe_applications(request).sync().unwrap();
    println!("{:#?}", result);
}
