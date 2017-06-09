
//! Amazon Relational Database Service
//!
//! If you're using the service, you're probably looking for [RdsClient](struct.RdsClient.html) and [Rds](trait.Rds.html).

extern crate rusoto_core;
extern crate xml;

mod generated;
mod custom;

pub use generated::*;
pub use custom::*;
            