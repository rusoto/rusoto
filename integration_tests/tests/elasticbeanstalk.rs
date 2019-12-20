#![cfg(feature = "elasticbeanstalk")]

extern crate rusoto_core;
extern crate rusoto_elasticbeanstalk;

use rusoto_core::Region;
use rusoto_elasticbeanstalk::{
    DescribeApplicationsMessage, ElasticBeanstalk, ElasticBeanstalkClient,
};

#[tokio::test]
async fn should_describe_applications() {
    let client = ElasticBeanstalkClient::new(Region::UsEast1);
    let request = DescribeApplicationsMessage::default();

    let result = client.describe_applications(request).await.unwrap();
    println!("{:#?}", result);
}
