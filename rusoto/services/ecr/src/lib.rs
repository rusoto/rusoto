
//! Amazon EC2 Container Registry
//!
//! If you're using the service, you're probably looking for [EcrClient](struct.EcrClient.html) and [Ecr](trait.Ecr.html).

extern crate rusoto_core;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod generated;
mod custom;

pub use generated::*;
pub use custom::*;
            