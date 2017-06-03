
//! Amazon SimpleDB
//!
//! If you're using the service, you're probably looking for [SimpleDbClient](struct.SimpleDbClient.html) and [SimpleDb](trait.SimpleDb.html).

extern crate rusoto_core;
extern crate xml;

mod generated;
mod custom;

pub use generated::*;
pub use custom::*;
            