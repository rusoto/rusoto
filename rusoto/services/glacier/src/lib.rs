
//! Amazon Glacier
//!
//! If you're using the service, you're probably looking for [GlacierClient](struct.GlacierClient.html) and [Glacier](trait.Glacier.html).

extern crate hyper;
#[macro_use]
extern crate log;
extern crate rusoto_core;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod generated;
mod custom;

pub use generated::*;
pub use custom::*;
            