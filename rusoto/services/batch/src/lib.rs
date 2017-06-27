
//! AWS Batch
//!
//! If you're using the service, you're probably looking for [BatchClient](struct.BatchClient.html) and [Batch](trait.Batch.html).

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
            