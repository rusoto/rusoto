
//! AWS Elastic Beanstalk
//!
//! If you're using the service, you're probably looking for [ElasticBeanstalkClient](struct.ElasticBeanstalkClient.html) and [ElasticBeanstalk](trait.ElasticBeanstalk.html).

extern crate hyper;
extern crate rusoto_core;
extern crate xml;

mod generated;
mod custom;

pub use generated::*;
pub use custom::*;
            