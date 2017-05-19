
//! Amazon Route 53
//!
//! If you're using the service, you're probably looking for [Route53Client](struct.Route53Client.html) and [Route53](trait.Route53.html).

extern crate hyper;
extern crate rusoto_core;
extern crate xml;

mod generated;
mod custom;

pub use generated::*;
pub use custom::*;
            