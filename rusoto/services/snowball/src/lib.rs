
//! Amazon Import/Export Snowball
//!
//! If you're using the service, you're probably looking for [SnowballClient](struct.SnowballClient.html) and [Snowball](trait.Snowball.html).

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
            