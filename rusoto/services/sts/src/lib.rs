
//! AWS Security Token Service
//!
//! If you're using the service, you're probably looking for [StsClient](struct.StsClient.html) and [Sts](trait.Sts.html).

extern crate chrono;
extern crate rusoto_core;
extern crate xml;

mod generated;
mod custom;

pub use generated::*;
pub use custom::*;
            