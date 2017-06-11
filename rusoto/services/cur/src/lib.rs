
//! AWS Cost and Usage Report Service
//!
//! If you're using the service, you're probably looking for [CostAndUsageReportClient](struct.CostAndUsageReportClient.html) and [CostAndUsageReport](trait.CostAndUsageReport.html).

extern crate hyper;
extern crate rusoto_core;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod generated;
mod custom;

pub use generated::*;
pub use custom::*;
            