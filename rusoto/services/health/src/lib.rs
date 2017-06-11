
//! AWS Health APIs and Notifications
//!
//! If you're using the service, you're probably looking for [AWSHealthClient](struct.AWSHealthClient.html) and [AWSHealth](trait.AWSHealth.html).

extern crate hyper;
extern crate rusoto_core;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod generated;
mod custom;

pub use generated::*;
pub use custom::*;
            