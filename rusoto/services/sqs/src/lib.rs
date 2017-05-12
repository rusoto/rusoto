
//! Amazon Simple Queue Service
//!
//! If you're using the service, you're probably looking for [SqsClient](struct.SqsClient.html) and [Sqs](trait.Sqs.html).

extern crate hyper;
extern crate rusoto_core;
extern crate xml;

mod generated;
mod custom;

pub use generated::*;
pub use custom::*;
            