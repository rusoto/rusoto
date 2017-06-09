
//! Amazon Simple Systems Manager (SSM)
//!
//! If you're using the service, you're probably looking for [SsmClient](struct.SsmClient.html) and [Ssm](trait.Ssm.html).

extern crate rusoto_core;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod generated;
mod custom;

pub use generated::*;
pub use custom::*;
            