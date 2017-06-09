
//! Amazon Simple Storage Service
//!
//! If you're using the service, you're probably looking for [S3Client](struct.S3Client.html) and [S3](trait.S3.html).

extern crate md5;
extern crate rusoto_core;
extern crate rustc_serialize;
extern crate xml;

mod generated;
mod custom;

pub use generated::*;
pub use custom::*;
            