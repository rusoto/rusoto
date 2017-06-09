
//! Amazon Route 53 Domains
//!
//! If you're using the service, you're probably looking for [Route53DomainsClient](struct.Route53DomainsClient.html) and [Route53Domains](trait.Route53Domains.html).

extern crate rusoto_core;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod generated;
mod custom;

pub use generated::*;
pub use custom::*;
            