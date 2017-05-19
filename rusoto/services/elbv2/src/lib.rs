
//! Elastic Load Balancing
//!
//! If you're using the service, you're probably looking for [ElbClient](struct.ElbClient.html) and [Elb](trait.Elb.html).

extern crate hyper;
extern crate rusoto_core;
extern crate xml;

mod generated;
mod custom;

pub use generated::*;
pub use custom::*;
            