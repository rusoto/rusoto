
//! Amazon Redshift
//!
//! If you're using the service, you're probably looking for [RedshiftClient](struct.RedshiftClient.html) and [Redshift](trait.Redshift.html).

extern crate hyper;
extern crate rusoto_core;
extern crate xml;

mod generated;
mod custom;

pub use generated::*;
pub use custom::*;
            