
//! Amazon Elastic Compute Cloud
//!
//! If you're using the service, you're probably looking for [Ec2Client](struct.Ec2Client.html) and [Ec2](trait.Ec2.html).

extern crate hyper;
extern crate rusoto_core;
extern crate xml;

mod generated;
mod custom;

pub use generated::*;
pub use custom::*;
            