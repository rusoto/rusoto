
//! AWS Device Farm
//!
//! If you're using the service, you're probably looking for [DeviceFarmClient](struct.DeviceFarmClient.html) and [DeviceFarm](trait.DeviceFarm.html).

extern crate rusoto_core;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod generated;
mod custom;

pub use generated::*;
pub use custom::*;
            