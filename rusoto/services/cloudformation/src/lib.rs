
//! AWS CloudFormation
//!
//! If you're using the service, you're probably looking for [CloudFormationClient](struct.CloudFormationClient.html) and [CloudFormation](trait.CloudFormation.html).

extern crate hyper;
extern crate rusoto_core;
extern crate xml;

mod generated;
mod custom;

pub use generated::*;
pub use custom::*;
            