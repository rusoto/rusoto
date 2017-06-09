
//! Amazon Elastic Transcoder
//!
//! If you're using the service, you're probably looking for [EtsClient](struct.EtsClient.html) and [Ets](trait.Ets.html).

extern crate rusoto_core;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod generated;
mod custom;

pub use generated::*;
pub use custom::*;
            