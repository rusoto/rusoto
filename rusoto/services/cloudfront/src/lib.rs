
//! Amazon CloudFront
//!
//! If you're using the service, you're probably looking for [CloudFrontClient](struct.CloudFrontClient.html) and [CloudFront](trait.CloudFront.html).

extern crate hyper;
extern crate rusoto_core;
extern crate xml;

mod generated;
mod custom;

pub use generated::*;
pub use custom::*;
            