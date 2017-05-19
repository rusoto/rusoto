
//! Amazon CloudSearch
//!
//! If you're using the service, you're probably looking for [CloudSearchClient](struct.CloudSearchClient.html) and [CloudSearch](trait.CloudSearch.html).

extern crate hyper;
extern crate rusoto_core;
extern crate xml;

mod generated;
mod custom;

pub use generated::*;
pub use custom::*;
            