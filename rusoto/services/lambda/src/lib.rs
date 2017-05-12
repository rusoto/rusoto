
//! AWS Lambda
//!
//! If you're using the service, you're probably looking for [LambdaClient](struct.LambdaClient.html) and [Lambda](trait.Lambda.html).

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
            