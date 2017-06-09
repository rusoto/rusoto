
//! Amazon CloudWatch
//!
//! If you're using the service, you're probably looking for [CloudWatchClient](struct.CloudWatchClient.html) and [CloudWatch](trait.CloudWatch.html).

extern crate rusoto_core;
extern crate xml;

mod generated;
mod custom;

pub use generated::*;
pub use custom::*;
            