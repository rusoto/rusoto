
//! Amazon ElastiCache
//!
//! If you're using the service, you're probably looking for [ElastiCacheClient](struct.ElastiCacheClient.html) and [ElastiCache](trait.ElastiCache.html).

extern crate hyper;
extern crate rusoto_core;
extern crate xml;

mod generated;
mod custom;

pub use generated::*;
pub use custom::*;
            