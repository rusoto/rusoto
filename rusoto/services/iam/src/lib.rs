
//! AWS Identity and Access Management
//!
//! If you're using the service, you're probably looking for [IamClient](struct.IamClient.html) and [Iam](trait.Iam.html).

extern crate hyper;
extern crate rusoto_core;
extern crate xml;

mod generated;
mod custom;

pub use generated::*;
pub use custom::*;
            