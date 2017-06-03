
//! AWS Import/Export
//!
//! If you're using the service, you're probably looking for [ImportExportClient](struct.ImportExportClient.html) and [ImportExport](trait.ImportExport.html).

extern crate rusoto_core;
extern crate xml;

mod generated;
mod custom;

pub use generated::*;
pub use custom::*;
            