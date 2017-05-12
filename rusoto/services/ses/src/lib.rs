
//! Amazon Simple Email Service
//!
//! If you're using the service, you're probably looking for [SesClient](struct.SesClient.html) and [Ses](trait.Ses.html).

extern crate hyper;
extern crate rusoto_core;
extern crate xml;

mod generated;
mod custom;

pub use generated::*;
pub use custom::*;
            