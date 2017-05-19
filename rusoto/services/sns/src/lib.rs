
//! Amazon Simple Notification Service
//!
//! If you're using the service, you're probably looking for [SnsClient](struct.SnsClient.html) and [Sns](trait.Sns.html).

extern crate hyper;
extern crate rusoto_core;
extern crate xml;

mod generated;
mod custom;

pub use generated::*;
pub use custom::*;
            